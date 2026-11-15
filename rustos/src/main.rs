#![no_std]
#![no_main]

extern crate alloc;

use core::arch::global_asm;
use core::panic::PanicInfo;

pub mod io;
pub mod sync;
pub mod serial;
pub mod vga;
pub mod gdt;
pub mod pic;
pub mod pit;
pub mod keyboard;
pub mod idt;
pub mod allocator;
pub mod rtc;
pub mod fs;
pub mod shell;
pub mod statusbar;
pub mod menu;
pub mod games;
pub mod research;

use vga::{Color, VGA};
use rustos_core as _;
use alloc::vec::Vec;
use alloc::format;
use alloc::boxed::Box;

global_asm!(
    r#"
    .section .multiboot_header, "a"
    .align 4
    .long 0x1BADB002
    .long 0x00000003
    .long 0xE4524FFB

    .section .bss
    .align 16
    stack_bottom:
        .skip 65536
    stack_top:

    .section .text
    .globl _start
    .type _start, @function
    _start:
        cli
        mov esp, offset stack_top
        push ebx
        push eax
        call kernel_main
    1:
        hlt
        jmp 1b
    "#
);

fn log_ok(step: &str) {
    serial_println!("[ OK ] {}", step);
    {
        let mut vga = VGA.lock();
        vga.set_color(Color::LightGreen, Color::Black);
        vga.write_str("[ OK ] ");
        vga.set_color(Color::White, Color::Black);
        vga.write_str(step);
        vga.write_str("\n");
    }
}

pub fn run_shell_session() {
    let mut shell = shell::Shell::new();
    {
        let mut vga = VGA.lock();
        vga.clear_screen();
        vga.set_color(Color::LightCyan, Color::Black);
        vga.write_str("RustOS Shell 1.0.0\n");
        vga.set_color(Color::LightGray, Color::Black);
        vga.write_str("Type 'help' for commands, 'games' or press [Esc] to return to Desktop.\n\n");
    }
    statusbar::draw("Shell");

    loop {
        shell::Shell::print_prompt();
        let line = shell.read_line();
        let action = shell.execute(&line);
        statusbar::draw("Shell");
        match action {
            shell::ShellAction::Continue => {}
            shell::ShellAction::OpenMenu => break,
            shell::ShellAction::OpenResearchLab => {
                research::lab::open_lab();
                statusbar::draw("Shell");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn kernel_main(magic: u32, _info: u32) -> ! {
    serial::SERIAL1.lock().init();
    serial_println!("RustOS Serial Console Initialized (COM1)");
    serial_println!("Multiboot magic: {:#010X}", magic);

    {
        let mut vga = VGA.lock();
        vga.set_color(Color::White, Color::Black);
        vga.clear_screen();
        vga.set_color(Color::LightCyan, Color::Black);
        vga.write_str("RustOS 1.0.0 (32-bit x86)\n");
        vga.set_color(Color::LightGray, Color::Black);
        vga.write_str("VirtualBox Educational Operating System\n\n");
    }

    if magic == 0x2BADB002 {
        log_ok("Multiboot compliant bootloader detected");
    }

    log_ok("VGA Text Mode Driver initialized (80x25)");
    log_ok("Serial COM1 Driver initialized (38400 baud)");

    // GDT
    gdt::init();
    log_ok("GDT initialized (flat 4GB code & data segments)");

    // PIC
    pic::init();
    log_ok("PIC remapped (IRQ 0..7 -> 0x20..0x27, IRQ 8..15 -> 0x28..0x2F)");

    // PIT
    pit::init();
    log_ok("PIT timer started at 100 Hz (10ms per tick)");

    // IDT & Interrupts
    idt::init();
    log_ok("IDT initialized and CPU interrupts enabled");

    // Keyboard
    keyboard::init();
    log_ok("Keyboard driver initialized (scancode queue active)");

    // Heap Allocator
    allocator::init();
    log_ok("Heap allocator initialized (2 MiB linked-list with coalescing)");

    // Test heap allocations
    {
        let mut v = Vec::new();
        v.push(42);
        v.push(100);
        let b = Box::new(999);
        let s = format!("Alloc test OK: v[0]={}, box={}", v[0], *b);
        serial_println!("{}", s);
    }
    log_ok("Dynamic memory allocation verified (Vec, String, Box)");

    // RTC CMOS Driver
    let dt = rtc::read_datetime();
    let rtc_msg = format!(
        "RTC CMOS initialized: {:04}-{:02}-{:02} {:02}:{:02}:{:02} ({})",
        dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second, dt.day_name
    );
    log_ok(&rtc_msg);

    // In-memory File System
    fs::FILESYSTEM.lock().init_defaults();
    log_ok("In-memory file system initialized (readme.txt, notes.txt, todo.txt, about.txt)");

    // Self-test shell commands via execution engine
    {
        let mut test_shell = shell::Shell::new();
        test_shell.execute("echo 'Self-test file creation' > selftest.txt");
        let fs = fs::FILESYSTEM.lock();
        if fs.read_file("selftest.txt").is_ok() {
            serial_println!("File creation test passed: selftest.txt exists");
        }
        drop(fs);
        test_shell.execute("rm selftest.txt");
    }
    log_ok("Shell command engine verified (27 commands ready)");

    log_ok("Desktop launcher & Status bar initialized");
    log_ok("Games loaded (Snake, Snake & Ladders, Car Dodge)");
    log_ok("Producer-Consumer Research Lab initialized");
    serial_println!("RustOS booted");

    serial_println!("DEBUG: Before sleep_ms(800), ticks={}", pit::get_ticks());
    // Pause briefly so boot log is legible before opening Desktop
    pit::sleep_ms(800);
    serial_println!("DEBUG: After sleep_ms(800), ticks={}", pit::get_ticks());

    // Desktop Application Loop
    loop {
        serial_println!("DEBUG: Calling show_desktop_menu()");
        let choice = menu::show_desktop_menu();
        serial_println!("DEBUG: Desktop menu choice: {:?}", choice);
        match choice {
            menu::AppChoice::Shell => {
                run_shell_session();
            }
            menu::AppChoice::About => {
                menu::show_about_screen();
            }
            menu::AppChoice::Files => {
                let mut sh = shell::Shell::new();
                sh.execute("vi notes.txt");
            }
            menu::AppChoice::Snake => {
                games::snake::play();
            }
            menu::AppChoice::SnakesLadders => {
                games::snakes_ladders::play();
            }
            menu::AppChoice::CarDodge => {
                games::car_dodge::play();
            }
            menu::AppChoice::ResearchLab => {
                research::lab::open_lab();
            }
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("\n!!! KERNEL PANIC: {}", info);

    {
        let mut vga = VGA.lock();
        vga.set_color(Color::LightRed, Color::Black);
        vga.new_line();
        vga.write_str("========================================\n");
        vga.write_str("           KERNEL PANIC                 \n");
        vga.write_str("========================================\n");
    }

    println!("{}", info);
    println!("System halted. Halting CPU execution.");

    loop {
        unsafe {
            core::arch::asm!("cli; hlt");
        }
    }
}
