//! Dynamic Memory Allocator for RustOS
//!
//! Provides a linked-list allocator with instant adjacent block coalescing,
//! enabling `alloc::vec::Vec`, `alloc::string::String`, `alloc::boxed::Box`,
//! and other dynamic collections.
//!
//! Heap storage is placed in a static 2 MiB BSS buffer, guaranteeing isolation
//! and avoiding overlaps with kernel text, stack, or multiboot structures.

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use crate::sync::Spinlock;

/// Total heap size: 2 MiB
pub const HEAP_SIZE: usize = 2 * 1024 * 1024;

/// Align allocations to 16 bytes.
const ALIGNMENT: usize = 16;
const HEADER_SIZE: usize = core::mem::size_of::<BlockHeader>();

#[repr(C, align(16))]
struct BlockHeader {
    size: usize,            // Usable payload size in bytes
    is_free: bool,          // Allocation status
    prev: *mut BlockHeader, // Preceding block in memory order
    next: *mut BlockHeader, // Succeeding block in memory order
}

/// In-memory allocator state
struct Heap {
    head: *mut BlockHeader,
    initialized: bool,
    total_allocated: usize,
    total_free: usize,
}

// SAFETY: The Heap raw pointers point to dedicated static memory protected by Spinlock.
unsafe impl Send for Heap {}

impl Heap {
    const fn new() -> Self {
        Heap {
            head: ptr::null_mut(),
            initialized: false,
            total_allocated: 0,
            total_free: 0,
        }
    }

    /// Initializes the heap buffer as a single large free block.
    fn init(&mut self, heap_start: usize, size: usize) {
        // Ensure starting address is aligned to 16 bytes
        let aligned_start = (heap_start + ALIGNMENT - 1) & !(ALIGNMENT - 1);
        let actual_size = size.saturating_sub(aligned_start - heap_start);

        if actual_size <= HEADER_SIZE + ALIGNMENT {
            return;
        }

        let first_block = aligned_start as *mut BlockHeader;
        let payload_size = actual_size - HEADER_SIZE;

        // SAFETY: The memory range is within our static HEAP_MEMORY buffer
        unsafe {
            ptr::write(first_block, BlockHeader {
                size: payload_size,
                is_free: true,
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
            });
        }

        self.head = first_block;
        self.initialized = true;
        self.total_allocated = 0;
        self.total_free = payload_size;
    }

    /// Allocates a memory block with the given layout.
    fn allocate(&mut self, layout: Layout) -> *mut u8 {
        if !self.initialized || self.head.is_null() {
            return ptr::null_mut();
        }

        // Align requested size to a multiple of 16 bytes, minimum 16 bytes
        let needed_size = (layout.size().max(ALIGNMENT) + ALIGNMENT - 1) & !(ALIGNMENT - 1);

        let mut curr = self.head;
        while !curr.is_null() {
            // SAFETY: Traversing validated chain of BlockHeader pointers
            unsafe {
                if (*curr).is_free && (*curr).size >= needed_size {
                    // Check if we can split this block
                    if (*curr).size >= needed_size + HEADER_SIZE + ALIGNMENT {
                        let new_block_addr = (curr as usize) + HEADER_SIZE + needed_size;
                        let new_block = new_block_addr as *mut BlockHeader;
                        let remaining_size = (*curr).size - needed_size - HEADER_SIZE;

                        ptr::write(new_block, BlockHeader {
                            size: remaining_size,
                            is_free: true,
                            prev: curr,
                            next: (*curr).next,
                        });

                        if !(*curr).next.is_null() {
                            (*(*curr).next).prev = new_block;
                        }
                        (*curr).next = new_block;
                        (*curr).size = needed_size;
                    }

                    (*curr).is_free = false;
                    self.total_allocated += (*curr).size;
                    self.total_free = self.total_free.saturating_sub((*curr).size);

                    let payload_addr = (curr as usize) + HEADER_SIZE;
                    return payload_addr as *mut u8;
                }
                curr = (*curr).next;
            }
        }

        // Out of memory
        ptr::null_mut()
    }

    /// Deallocates a previously allocated block and coalesces adjacent free blocks.
    fn deallocate(&mut self, ptr: *mut u8) {
        if ptr.is_null() || !self.initialized {
            return;
        }

        let block_addr = (ptr as usize).saturating_sub(HEADER_SIZE);
        let block = block_addr as *mut BlockHeader;

        // SAFETY: block pointer was created by allocate() and is protected by Heap lock
        unsafe {
            if (*block).is_free {
                return; // Double free prevention
            }

            (*block).is_free = true;
            self.total_allocated = self.total_allocated.saturating_sub((*block).size);
            self.total_free += (*block).size;

            // Coalesce with next block if free
            let next = (*block).next;
            if !next.is_null() && (*next).is_free {
                (*block).size += HEADER_SIZE + (*next).size;
                (*block).next = (*next).next;
                if !(*next).next.is_null() {
                    (*(*next).next).prev = block;
                }
            }

            // Coalesce with previous block if free
            let prev = (*block).prev;
            if !prev.is_null() && (*prev).is_free {
                (*prev).size += HEADER_SIZE + (*block).size;
                (*prev).next = (*block).next;
                if !(*block).next.is_null() {
                    (*(*block).next).prev = prev;
                }
            }
        }
    }
}

/// Global allocator wrapper around Spinlock-protected Heap
pub struct LockedHeap(Spinlock<Heap>);

impl LockedHeap {
    pub const fn empty() -> Self {
        LockedHeap(Spinlock::new(Heap::new()))
    }

    pub fn init(&self, heap_start: usize, size: usize) {
        self.0.lock().init(heap_start, size);
    }

    pub fn stats(&self) -> (usize, usize) {
        let heap = self.0.lock();
        (heap.total_allocated, heap.total_free)
    }
}

// SAFETY: LockedHeap uses Spinlock synchronization for thread safety
unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().allocate(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        self.0.lock().deallocate(ptr);
    }
}

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// 2 MiB static BSS buffer for the heap
#[repr(align(16))]
struct HeapBuffer {
    #[allow(dead_code)]
    raw: [u8; HEAP_SIZE],
}
static mut HEAP_MEMORY: HeapBuffer = HeapBuffer {
    raw: [0; HEAP_SIZE],
};

/// Initialize the global allocator with the 2 MiB heap buffer
pub fn init() {
    let heap_start = core::ptr::addr_of_mut!(HEAP_MEMORY) as usize;
    ALLOCATOR.init(heap_start, HEAP_SIZE);
}
