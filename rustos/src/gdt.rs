//! Global Descriptor Table (GDT) for 32-bit x86 Flat Memory Model

use core::arch::asm;
use core::ptr::addr_of;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl GdtEntry {
    pub const fn new(base: u32, limit: u32, access: u8, flags: u8) -> Self {
        Self {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: (((limit >> 16) & 0x0F) as u8) | (flags & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }

    pub const fn null() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

#[repr(C, packed)]
struct GdtDescriptor {
    size: u16,
    offset: u32,
}

static GDT: [GdtEntry; 3] = [
    // 0x00: Null descriptor
    GdtEntry::null(),
    // 0x08: Kernel Code Segment (Base=0, Limit=4GB, Access=0x9A, Flags=0xC0)
    GdtEntry::new(0, 0xFFFFF, 0x9A, 0xC0),
    // 0x10: Kernel Data Segment (Base=0, Limit=4GB, Access=0x92, Flags=0xC0)
    GdtEntry::new(0, 0xFFFFF, 0x92, 0xC0),
];

pub fn init() {
    let gdt_desc = GdtDescriptor {
        size: (core::mem::size_of::<[GdtEntry; 3]>() - 1) as u16,
        offset: addr_of!(GDT) as u32,
    };

    // SAFETY: Loading GDT register, performing far jump to reload CS selector to 0x08, and reloading data segments to 0x10
    unsafe {
        asm!(
            "lgdt [{desc}]",
            "push 0x08",
            "lea eax, [2f]",
            "push eax",
            "retf",
            "2:",
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            desc = in(reg) &gdt_desc,
            out("eax") _,
            options(nomem, nostack)
        );
    }
}
