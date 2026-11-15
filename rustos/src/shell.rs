//! Interactive Shell for RustOS
//!
//! Implements all mandatory shell commands, line editing, command history,
//! argument parsing, and error reporting.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::fs::FILESYSTEM;
use crate::io::outb;
use crate::keyboard::{get_key, Key};
use crate::pit;
use crate::rtc;
use crate::vga::{Color, VGA};

pub enum ShellAction {
    Continue,
    OpenMenu,
    OpenResearchLab,
}

pub struct Shell {
    history: Vec<String>,
    history_idx: usize,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            history: Vec::new(),
            history_idx: 0,
        }
    }

    /// Prints the shell command prompt.
    pub fn print_prompt() {
        let mut vga = VGA.lock();
        vga.set_color(Color::LightGreen, Color::Black);
        vga.write_str("root@rustos");
        vga.set_color(Color::White, Color::Black);
        vga.write_str(":");
        vga.set_color(Color::LightCyan, Color::Black);
        vga.write_str("/# ");
        vga.set_color(Color::White, Color::Black);
    }

    /// Prints an error message in red.
    pub fn print_error(msg: &str) {
        let mut vga = VGA.lock();
        vga.set_color(Color::LightRed, Color::Black);
        vga.write_str(msg);
        vga.write_str("\n");
        vga.set_color(Color::White, Color::Black);
    }

    /// Reads a single line of input with editing and command history.
    pub fn read_line(&mut self) -> String {
        let mut buffer = String::new();
        self.history_idx = self.history.len();

        loop {
            let key = get_key();
            match key {
                Key::Char(c) => {
                    buffer.push(c);
                    let mut vga = VGA.lock();
                    vga.write_char(c);
                }
                Key::Enter => {
                    let mut vga = VGA.lock();
                    vga.write_str("\n");
                    break;
                }
                Key::Backspace => {
                    if !buffer.is_empty() {
                        buffer.pop();
                        let mut vga = VGA.lock();
                        vga.backspace();
                    }
                }
                Key::Up => {
                    if !self.history.is_empty() && self.history_idx > 0 {
                        self.history_idx -= 1;
                        self.replace_input_line(&mut buffer, &self.history[self.history_idx].clone());
                    }
                }
                Key::Down => {
                    if !self.history.is_empty() && self.history_idx + 1 < self.history.len() {
                        self.history_idx += 1;
                        self.replace_input_line(&mut buffer, &self.history[self.history_idx].clone());
                    } else if self.history_idx + 1 >= self.history.len() {
                        self.history_idx = self.history.len();
                        self.replace_input_line(&mut buffer, "");
                    }
                }
                Key::Esc => {
                    // Esc can return to menu if line is empty
                    if buffer.is_empty() {
                        return String::from("games");
                    }
                }
                _ => {}
            }
        }

        let trimmed = buffer.trim().to_string();
        if !trimmed.is_empty() {
            if self.history.last().map(|s| s.as_str()) != Some(&trimmed) {
                if self.history.len() >= 32 {
                    self.history.remove(0);
                }
                self.history.push(trimmed.clone());
            }
        }
        trimmed
    }

    /// Clears currently typed buffer on screen and replaces it with new text.
    fn replace_input_line(&self, buffer: &mut String, new_text: &str) {
        let mut vga = VGA.lock();
        for _ in 0..buffer.len() {
            vga.backspace();
        }
        buffer.clear();
        buffer.push_str(new_text);
        vga.write_str(new_text);
    }

    /// Executes a single command string.
    pub fn execute(&mut self, line: &str) -> ShellAction {
        let line = line.trim();
        if line.is_empty() {
            return ShellAction::Continue;
        }

        // Handle file redirection: echo <text> > <file>
        if line.starts_with("echo ") && line.contains('>') {
            self.execute_echo_redirect(line);
            return ShellAction::Continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let cmd = parts[0];
        let args = &parts[1..];

        match cmd {
            "help" => self.cmd_help(),
            "clear" => {
                VGA.lock().clear_screen();
            }
            "reboot" => {
                crate::println!("Rebooting RustOS...");
                pit::sleep_ms(300);
                // Pulse CPU reset line via keyboard controller port 0x64
                outb(0x64, 0xFE);
                crate::io::io_wait();
                loop {
                    unsafe { core::arch::asm!("hlt") };
                }
            }
            "shutdown" => {
                crate::println!("System halted. You can now power off the virtual machine.");
                unsafe {
                    core::arch::asm!("cli; hlt");
                    loop {
                        core::arch::asm!("hlt");
                    }
                }
            }
            "date" => self.cmd_date(),
            "cal" => self.cmd_cal(),
            "echo" => {
                let text = if line.len() > 5 { &line[5..] } else { "" };
                crate::println!("{}", text);
            }
            "ls" => self.cmd_ls(),
            "lp" => self.cmd_lp(args),
            "whoami" => crate::println!("root"),
            "uname" => crate::println!("RustOS 1.0.0 i686 (VirtualBox bare-metal)"),
            "hostname" => crate::println!("rustos"),
            "uptime" => self.cmd_uptime(),
            "bc" => self.cmd_bc(args),
            "cat" => self.cmd_cat(args),
            "grab" => self.cmd_grab(args),
            "rm" => self.cmd_rm(args),
            "touch" => self.cmd_touch(args),
            "cp" => self.cmd_cp(args),
            "mv" => self.cmd_mv(args),
            "cut" => self.cmd_cut(args),
            "head" => self.cmd_head(args),
            "tail" => self.cmd_tail(args),
            "chmod" => self.cmd_chmod(args),
            "wc" => self.cmd_wc(args),
            "vi" => self.cmd_vi(args),
            "games" => return ShellAction::OpenMenu,
            "research" | "pclab" => return ShellAction::OpenResearchLab,
            _ => {
                Self::print_error(&format!("rustos: command not found: '{}'. Type 'help' for available commands.", cmd));
            }
        }

        ShellAction::Continue
    }

    fn cmd_help(&self) {
        crate::println!("Available RustOS Commands:");
        crate::println!("  help                 - Display this list of available commands");
        crate::println!("  clear                - Clear the VGA display screen");
        crate::println!("  reboot               - Reboot the machine via keyboard controller");
        crate::println!("  shutdown             - Halt the CPU cleanly for VM poweroff");
        crate::println!("  date                 - Display current RTC date, time, and day of week");
        crate::println!("  cal                  - Display calendar of current month with today highlighted");
        crate::println!("  echo <text> [> file] - Print text or redirect text into a file");
        crate::println!("  ls                   - List files with permissions, size, and name");
        crate::println!("  lp <file>            - Print file content with printer header");
        crate::println!("  whoami               - Print current user (root)");
        crate::println!("  uname                - Print operating system name and architecture");
        crate::println!("  hostname             - Print system hostname");
        crate::println!("  uptime               - Print system uptime in hh:mm:ss from PIT timer");
        crate::println!("  bc <n> <op> <n>      - Calculate binary arithmetic (+, -, *, /, %)");
        crate::println!("  cat <file>           - Display file content");
        crate::println!("  grab <word> <file>   - Search for lines containing word");
        crate::println!("  rm <file>            - Remove file from filesystem");
        crate::println!("  touch <file>         - Create empty file");
        crate::println!("  cp <src> <dst>       - Copy file to destination");
        crate::println!("  mv <src> <dst>       - Rename file to destination");
        crate::println!("  cut <start> <end> <f>- Extract character columns (1-based)");
        crate::println!("  head <file> [n]      - Display first n lines (default 10)");
        crate::println!("  tail <file> [n]      - Display last n lines (default 10)");
        crate::println!("  chmod <6|4> <file>   - Change file permission (6=writable, 4=read-only)");
        crate::println!("  wc <file>            - Count lines, words, and bytes");
        crate::println!("  vi <file>            - Line editor (:wq to save and exit)");
        crate::println!("  games                - Open application & games menu");
        crate::println!("  research / pclab     - Open Producer-Consumer Research Lab");
    }

    fn cmd_date(&self) {
        let dt = rtc::read_datetime();
        crate::println!(
            "{}, {:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
            dt.day_name,
            dt.year,
            dt.month,
            dt.day,
            dt.hour,
            dt.minute,
            dt.second
        );
    }

    fn cmd_cal(&self) {
        let dt = rtc::read_datetime();
        let month_name = rustos_core::month_name(dt.month);
        let days_count = rustos_core::days_in_month(dt.year, dt.month);
        let start_dow = rustos_core::first_day_of_month(dt.year, dt.month);

        crate::println!("     {} {}", month_name, dt.year);
        crate::println!(" Su Mo Tu We Th Fr Sa");

        // Leading spaces
        let mut pos = start_dow as usize;
        for _ in 0..pos {
            let mut vga = VGA.lock();
            vga.write_str("   ");
        }

        for day in 1..=days_count {
            let mut vga = VGA.lock();
            if day == dt.day {
                vga.set_color(Color::Yellow, Color::Blue);
                if day < 10 {
                    vga.write_str(" ");
                }
                let s = format!("{}", day);
                vga.write_str(&s);
                vga.set_color(Color::White, Color::Black);
                vga.write_str(" ");
            } else {
                if day < 10 {
                    vga.write_str(" ");
                }
                let s = format!("{} ", day);
                vga.write_str(&s);
            }

            pos += 1;
            if pos == 7 {
                vga.write_str("\n");
                pos = 0;
            }
        }
        if pos != 0 {
            let mut vga = VGA.lock();
            vga.write_str("\n");
        }
    }

    fn execute_echo_redirect(&self, line: &str) {
        // Find '>' separator
        if let Some(idx) = line.find('>') {
            let raw_text = line[5..idx].trim();
            let file_name = line[idx + 1..].trim();

            if file_name.is_empty() {
                Self::print_error("echo: missing destination file name");
                return;
            }

            let mut final_text = String::from(raw_text);
            final_text.push('\n');

            let mut fs = FILESYSTEM.lock();
            if let Err(err) = fs.write_file(file_name, final_text.as_bytes(), false) {
                drop(fs);
                Self::print_error(&format!("echo: cannot write to '{}': {}", file_name, err));
            }
        }
    }

    fn cmd_ls(&self) {
        let fs = FILESYSTEM.lock();
        let files = fs.list_files();
        crate::println!("PERM    SIZE  NAME");
        for f in files {
            let perm = if f.readonly { "r--" } else { "rw-" };
            crate::println!("{:<6} {:>5}  {}", perm, f.size, f.name);
        }
    }

    fn cmd_lp(&self, args: &[&str]) {
        if args.is_empty() {
            Self::print_error("lp: missing file operand\nUsage: lp <file>");
            return;
        }
        let fs = FILESYSTEM.lock();
        match fs.read_file(args[0]) {
            Ok(data) => {
                crate::println!("[lp] no printer attached; redirecting to console:");
                if let Ok(s) = core::str::from_utf8(data) {
                    crate::print!("{}", s);
                    if !s.ends_with('\n') {
                        crate::println!();
                    }
                } else {
                    crate::println!("<binary content, {} bytes>", data.len());
                }
            }
            Err(err) => {
                Self::print_error(&format!("lp: '{}': {}", args[0], err));
            }
        }
    }

    fn cmd_uptime(&self) {
        let (h, m, s) = pit::uptime_hms();
        crate::println!("up {:02}:{:02}:{:02} (ticks: {})", h, m, s, pit::get_ticks());
    }

    fn cmd_bc(&self, args: &[&str]) {
        if args.len() != 3 {
            Self::print_error("bc: wrong arguments\nUsage: bc <n> <op> <n> (e.g., bc 10 + 20)");
            return;
        }

        let a = match args[0].parse::<i64>() {
            Ok(v) => v,
            Err(_) => {
                Self::print_error("bc: invalid first integer operand");
                return;
            }
        };

        let b = match args[2].parse::<i64>() {
            Ok(v) => v,
            Err(_) => {
                Self::print_error("bc: invalid second integer operand");
                return;
            }
        };

        match rustos_core::eval_bc(a, args[1], b) {
            Ok(res) => crate::println!("{}", res),
            Err(err) => Self::print_error(&format!("bc: error: {}", err)),
        }
    }

    fn cmd_cat(&self, args: &[&str]) {
        if args.is_empty() {
            Self::print_error("cat: missing file operand\nUsage: cat <file>");
            return;
        }
        let fs = FILESYSTEM.lock();
        match fs.read_file(args[0]) {
            Ok(data) => {
                if let Ok(s) = core::str::from_utf8(data) {
                    crate::print!("{}", s);
                    if !s.ends_with('\n') {
                        crate::println!();
                    }
                } else {
                    crate::println!("<binary content, {} bytes>", data.len());
                }
            }
            Err(err) => Self::print_error(&format!("cat: '{}': {}", args[0], err)),
        }
    }

    fn cmd_grab(&self, args: &[&str]) {
        if args.len() < 2 {
            Self::print_error("grab: wrong arguments\nUsage: grab <word> <file>");
            return;
        }
        let word = args[0];
        let file = args[1];

        let fs = FILESYSTEM.lock();
        match fs.read_file(file) {
            Ok(data) => {
                if let Ok(s) = core::str::from_utf8(data) {
                    let matches = rustos_core::grab_lines(s, word);
                    for m in matches {
                        crate::println!("{}", m);
                    }
                } else {
                    Self::print_error("grab: cannot search binary file");
                }
            }
            Err(err) => Self::print_error(&format!("grab: '{}': {}", file, err)),
        }
    }

    fn cmd_rm(&self, args: &[&str]) {
        if args.is_empty() {
            Self::print_error("rm: missing file operand\nUsage: rm <file>");
            return;
        }
        let mut fs = FILESYSTEM.lock();
        if let Err(err) = fs.delete_file(args[0]) {
            Self::print_error(&format!("rm: cannot remove '{}': {}", args[0], err));
        }
    }

    fn cmd_touch(&self, args: &[&str]) {
        if args.is_empty() {
            Self::print_error("touch: missing file operand\nUsage: touch <file>");
            return;
        }
        let mut fs = FILESYSTEM.lock();
        // If file already exists, touch succeeds without modifying
        if fs.read_file(args[0]).is_err() {
            if let Err(err) = fs.create_file(args[0], false, b"") {
                Self::print_error(&format!("touch: cannot touch '{}': {}", args[0], err));
            }
        }
    }

    fn cmd_cp(&self, args: &[&str]) {
        if args.len() < 2 {
            Self::print_error("cp: missing file operand\nUsage: cp <src> <dst>");
            return;
        }
        let mut fs = FILESYSTEM.lock();
        if let Err(err) = fs.copy_file(args[0], args[1]) {
            Self::print_error(&format!("cp: cannot copy '{}' to '{}': {}", args[0], args[1], err));
        }
    }

    fn cmd_mv(&self, args: &[&str]) {
        if args.len() < 2 {
            Self::print_error("mv: missing file operand\nUsage: mv <src> <dst>");
            return;
        }
        let mut fs = FILESYSTEM.lock();
        if let Err(err) = fs.rename_file(args[0], args[1]) {
            Self::print_error(&format!("mv: cannot move '{}' to '{}': {}", args[0], args[1], err));
        }
    }

    fn cmd_cut(&self, args: &[&str]) {
        if args.len() < 3 {
            Self::print_error("cut: wrong arguments\nUsage: cut <start> <end> <file>");
            return;
        }
        let start = match args[0].parse::<usize>() {
            Ok(v) => v,
            Err(_) => {
                Self::print_error("cut: invalid start column index");
                return;
            }
        };
        let end = match args[1].parse::<usize>() {
            Ok(v) => v,
            Err(_) => {
                Self::print_error("cut: invalid end column index");
                return;
            }
        };

        let fs = FILESYSTEM.lock();
        match fs.read_file(args[2]) {
            Ok(data) => {
                if let Ok(s) = core::str::from_utf8(data) {
                    for line in s.lines() {
                        let extracted = rustos_core::cut_line(line, start, end);
                        crate::println!("{}", extracted);
                    }
                }
            }
            Err(err) => Self::print_error(&format!("cut: '{}': {}", args[2], err)),
        }
    }

    fn cmd_head(&self, args: &[&str]) {
        if args.is_empty() {
            Self::print_error("head: missing file operand\nUsage: head <file> [n]");
            return;
        }
        let n = if args.len() >= 2 {
            args[1].parse::<usize>().unwrap_or(10)
        } else {
            10
        };

        let fs = FILESYSTEM.lock();
        match fs.read_file(args[0]) {
            Ok(data) => {
                if let Ok(s) = core::str::from_utf8(data) {
                    let lines = rustos_core::get_head(s, n);
                    for l in lines {
                        crate::println!("{}", l);
                    }
                }
            }
            Err(err) => Self::print_error(&format!("head: '{}': {}", args[0], err)),
        }
    }

    fn cmd_tail(&self, args: &[&str]) {
        if args.is_empty() {
            Self::print_error("tail: missing file operand\nUsage: tail <file> [n]");
            return;
        }
        let n = if args.len() >= 2 {
            args[1].parse::<usize>().unwrap_or(10)
        } else {
            10
        };

        let fs = FILESYSTEM.lock();
        match fs.read_file(args[0]) {
            Ok(data) => {
                if let Ok(s) = core::str::from_utf8(data) {
                    let lines = rustos_core::get_tail(s, n);
                    for l in lines {
                        crate::println!("{}", l);
                    }
                }
            }
            Err(err) => Self::print_error(&format!("tail: '{}': {}", args[0], err)),
        }
    }

    fn cmd_chmod(&self, args: &[&str]) {
        if args.len() < 2 {
            Self::print_error("chmod: missing operand\nUsage: chmod <6|4> <file> (6=writable, 4=read-only)");
            return;
        }
        let mode = match args[0].parse::<u8>() {
            Ok(m) => m,
            Err(_) => {
                Self::print_error("chmod: invalid mode (use 6 or 4)");
                return;
            }
        };

        let mut fs = FILESYSTEM.lock();
        if let Err(err) = fs.chmod(args[1], mode) {
            Self::print_error(&format!("chmod: cannot change mode of '{}': {}", args[1], err));
        }
    }

    fn cmd_wc(&self, args: &[&str]) {
        if args.is_empty() {
            Self::print_error("wc: missing file operand\nUsage: wc <file>");
            return;
        }
        let fs = FILESYSTEM.lock();
        match fs.read_file(args[0]) {
            Ok(data) => {
                let res = rustos_core::count_words(data);
                crate::println!("{:4} {:4} {:4} {}", res.lines, res.words, res.bytes, args[0]);
            }
            Err(err) => Self::print_error(&format!("wc: '{}': {}", args[0], err)),
        }
    }

    /// Simple line editor (`vi <file>`)
    fn cmd_vi(&self, args: &[&str]) {
        if args.is_empty() {
            Self::print_error("vi: missing file operand\nUsage: vi <file>");
            return;
        }
        let filename = args[0];

        // Read existing file if any
        let mut content = String::new();
        {
            let fs = FILESYSTEM.lock();
            if let Ok(data) = fs.read_file(filename) {
                if let Ok(s) = core::str::from_utf8(data) {
                    content.push_str(s);
                }
            }
        }

        {
            let mut vga = VGA.lock();
            vga.clear_screen();
            vga.set_color(Color::White, Color::Blue);
            vga.write_str(" RustOS VI Line Editor -- [File: ");
            vga.write_str(filename);
            vga.write_str("] -- Type ':wq' to save and exit, ':q' to abort ");
            vga.new_line();
            vga.set_color(Color::White, Color::Black);
            if !content.is_empty() {
                vga.write_str(&content);
                if !content.ends_with('\n') {
                    vga.write_str("\n");
                }
            }
        }

        let mut lines: Vec<String> = Vec::new();
        let mut curr_line = String::new();

        loop {
            let key = get_key();
            match key {
                Key::Char(c) => {
                    curr_line.push(c);
                    let mut vga = VGA.lock();
                    vga.write_char(c);
                }
                Key::Enter => {
                    let mut vga = VGA.lock();
                    vga.write_str("\n");

                    if curr_line.trim() == ":wq" {
                        // Save and exit
                        let mut final_content = content.clone();
                        for l in &lines {
                            final_content.push_str(l);
                            final_content.push('\n');
                        }
                        let mut fs = FILESYSTEM.lock();
                        if let Err(err) = fs.write_file(filename, final_content.as_bytes(), false) {
                            drop(fs);
                            vga.set_color(Color::LightRed, Color::Black);
                            vga.write_str("Error saving file: ");
                            vga.write_str(err);
                            vga.write_str("\nPress any key to exit...");
                            drop(vga);
                            let _ = get_key();
                        }
                        break;
                    } else if curr_line.trim() == ":q" {
                        // Exit without saving
                        break;
                    } else {
                        lines.push(curr_line.clone());
                        curr_line.clear();
                    }
                }
                Key::Backspace => {
                    if !curr_line.is_empty() {
                        curr_line.pop();
                        let mut vga = VGA.lock();
                        vga.backspace();
                    }
                }
                _ => {}
            }
        }

        // Return to clean shell screen
        let mut vga = VGA.lock();
        vga.clear_screen();
    }
}
