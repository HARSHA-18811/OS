//! In-Memory File System for RustOS
//!
//! Implements a static table of files with configurable permissions,
//! size tracking, creation, deletion, renaming, and reading/writing.

use alloc::string::String;
use alloc::vec::Vec;
use crate::sync::Spinlock;

pub const MAX_FILES: usize = 32;
pub const MAX_NAME_LEN: usize = 24;
pub const MAX_FILE_SIZE: usize = 2048;

#[derive(Clone)]
pub struct FileEntry {
    pub in_use: bool,
    pub name: String,
    pub readonly: bool,
    pub size: usize,
    pub data: [u8; MAX_FILE_SIZE],
}

impl FileEntry {
    pub const fn empty() -> Self {
        FileEntry {
            in_use: false,
            name: String::new(),
            readonly: false,
            size: 0,
            data: [0u8; MAX_FILE_SIZE],
        }
    }
}

pub struct FileMetadata {
    pub name: String,
    pub readonly: bool,
    pub size: usize,
}

pub struct FileSystem {
    files: [FileEntry; MAX_FILES],
}

impl FileSystem {
    pub const fn new() -> Self {
        // Const array initialization
        const EMPTY: FileEntry = FileEntry::empty();
        FileSystem {
            files: [EMPTY; MAX_FILES],
        }
    }

    /// Initializes default files at system boot.
    pub fn init_defaults(&mut self) {
        let _ = self.create_file(
            "readme.txt",
            false,
            b"Welcome to RustOS 1.0.0!\nRustOS is an educational 32-bit x86 OS written in Rust.\nType 'help' to see commands, or 'games' to open the app menu.\n",
        );

        let _ = self.create_file(
            "notes.txt",
            false,
            b"RustOS Architecture Notes:\n- VGA text mode 80x25 at 0xB8000\n- IDT + 8259 PIC remapped to 0x20..0x2F\n- PIT channel 0 timer at 100 Hz\n- CMOS RTC for hardware date and time\n- 2 MiB Heap Allocator with coalescing\n",
        );

        let _ = self.create_file(
            "todo.txt",
            false,
            b"RustOS Roadmap:\n[x] Multiboot & VirtualBox ISO pipeline\n[x] VGA & Serial COM1 drivers\n[x] GDT, IDT, PIC, PIT, Keyboard\n[x] CMOS RTC & Heap Allocator\n[x] File System & Shell Commands\n[ ] Desktop Menu & Games\n[ ] Producer-Consumer Lab\n",
        );

        let _ = self.create_file(
            "about.txt",
            true, // Read-only file
            b"RustOS Version 1.0.0 (i686-rustos)\nBuilt for Oracle VirtualBox sandbox virtualization.\nIncludes shell, text editor, 3 playable games, and PC Lab.\n",
        );
    }

    /// Finds a file by name.
    fn find_index(&self, name: &str) -> Option<usize> {
        for i in 0..MAX_FILES {
            if self.files[i].in_use && self.files[i].name == name {
                return Some(i);
            }
        }
        None
    }

    /// Creates a new file.
    pub fn create_file(&mut self, name: &str, readonly: bool, initial_data: &[u8]) -> Result<(), &'static str> {
        if name.is_empty() || name.len() > MAX_NAME_LEN {
            return Err("file name must be 1 to 24 characters");
        }
        if self.find_index(name).is_some() {
            return Err("file already exists");
        }
        if initial_data.len() > MAX_FILE_SIZE {
            return Err("initial data exceeds max file size of 2048 bytes");
        }

        // Find empty slot
        for i in 0..MAX_FILES {
            if !self.files[i].in_use {
                self.files[i].in_use = true;
                self.files[i].name = String::from(name);
                self.files[i].readonly = readonly;
                self.files[i].size = initial_data.len();
                self.files[i].data[..initial_data.len()].copy_from_slice(initial_data);
                return Ok(());
            }
        }

        Err("file table is full (max 32 files)")
    }

    /// Reads content of a file.
    pub fn read_file<'a>(&'a self, name: &str) -> Result<&'a [u8], &'static str> {
        if let Some(idx) = self.find_index(name) {
            let size = self.files[idx].size;
            Ok(&self.files[idx].data[..size])
        } else {
            Err("file not found")
        }
    }

    /// Writes data to a file. Creates file if missing. Fails if read-only.
    pub fn write_file(&mut self, name: &str, data: &[u8], append: bool) -> Result<(), &'static str> {
        if let Some(idx) = self.find_index(name) {
            if self.files[idx].readonly {
                return Err("permission denied (file is read-only)");
            }

            if append {
                let current_size = self.files[idx].size;
                let new_size = current_size + data.len();
                if new_size > MAX_FILE_SIZE {
                    return Err("file size would exceed 2048 bytes");
                }
                self.files[idx].data[current_size..new_size].copy_from_slice(data);
                self.files[idx].size = new_size;
            } else {
                if data.len() > MAX_FILE_SIZE {
                    return Err("data exceeds maximum file size of 2048 bytes");
                }
                self.files[idx].size = data.len();
                self.files[idx].data[..data.len()].copy_from_slice(data);
            }
            Ok(())
        } else {
            // Create file if it does not exist
            self.create_file(name, false, data)
        }
    }

    /// Deletes a file. Fails if read-only.
    pub fn delete_file(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(idx) = self.find_index(name) {
            if self.files[idx].readonly {
                return Err("permission denied (file is read-only)");
            }
            self.files[idx].in_use = false;
            self.files[idx].name.clear();
            self.files[idx].size = 0;
            Ok(())
        } else {
            Err("file not found")
        }
    }

    /// Renames a file from src to dst. Fails if src is missing, dst exists, or src is read-only.
    pub fn rename_file(&mut self, src: &str, dst: &str) -> Result<(), &'static str> {
        if dst.is_empty() || dst.len() > MAX_NAME_LEN {
            return Err("destination name must be 1 to 24 characters");
        }
        if self.find_index(dst).is_some() {
            return Err("destination file already exists");
        }

        if let Some(idx) = self.find_index(src) {
            if self.files[idx].readonly {
                return Err("permission denied (file is read-only)");
            }
            self.files[idx].name = String::from(dst);
            Ok(())
        } else {
            Err("source file not found")
        }
    }

    /// Copies a file from src to dst.
    pub fn copy_file(&mut self, src: &str, dst: &str) -> Result<(), &'static str> {
        if dst.is_empty() || dst.len() > MAX_NAME_LEN {
            return Err("destination name must be 1 to 24 characters");
        }
        if self.find_index(dst).is_some() {
            return Err("destination file already exists");
        }

        let (size, readonly, data_slice) = if let Some(idx) = self.find_index(src) {
            let sz = self.files[idx].size;
            let ro = self.files[idx].readonly;
            (sz, ro, self.files[idx].data)
        } else {
            return Err("source file not found");
        };

        for i in 0..MAX_FILES {
            if !self.files[i].in_use {
                self.files[i].in_use = true;
                self.files[i].name = String::from(dst);
                self.files[i].readonly = readonly;
                self.files[i].size = size;
                self.files[i].data = data_slice;
                return Ok(());
            }
        }

        Err("file table is full (max 32 files)")
    }

    /// Changes file permissions (6 = writable, 4 = read-only).
    pub fn chmod(&mut self, name: &str, mode: u8) -> Result<(), &'static str> {
        if let Some(idx) = self.find_index(name) {
            match mode {
                6 => {
                    self.files[idx].readonly = false;
                    Ok(())
                }
                4 => {
                    self.files[idx].readonly = true;
                    Ok(())
                }
                _ => Err("invalid mode (use 6 for writable or 4 for read-only)"),
            }
        } else {
            Err("file not found")
        }
    }

    /// Returns metadata of all active files.
    pub fn list_files(&self) -> Vec<FileMetadata> {
        let mut list = Vec::new();
        for i in 0..MAX_FILES {
            if self.files[i].in_use {
                list.push(FileMetadata {
                    name: self.files[i].name.clone(),
                    readonly: self.files[i].readonly,
                    size: self.files[i].size,
                });
            }
        }
        list
    }
}

pub static FILESYSTEM: Spinlock<FileSystem> = Spinlock::new(FileSystem::new());
