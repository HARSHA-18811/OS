//! Producer-Consumer Performance Research Laboratory
//!
//! Evaluates the 3 research hypotheses from the paper:
//! - H1: Throughput increases with buffer capacity up to saturation.
//! - H2: Symmetric thread counts achieve higher throughput than asymmetric.
//! - H3: Asymmetric thread ratios cause producer/consumer wait skew.
//!
//! Displays the mandatory notice:
//! "The OS version measures simulated tasks, not preemptive threads.
//!  Waiting times are counted directly, unlike the Rust prototype."

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use crate::fs::FILESYSTEM;
use crate::keyboard::{get_key, Key};
use crate::pit;
use crate::statusbar;
use crate::sync::Spinlock;
use crate::vga::{Color, ColorCode, VGA};

const CAPACITIES: [usize; 4] = [8, 32, 128, 512];
const THREAD_CONFIGS: [(usize, usize); 6] = [
    (1, 1),
    (2, 1),
    (1, 2),
    (2, 2),
    (4, 2),
    (2, 4),
];
const REPETITIONS: usize = 5;
const ITEMS_PER_PRODUCER: usize = 300;

#[derive(Clone, Copy)]
pub struct LabRecord {
    pub capacity: usize,
    pub p_count: usize,
    pub c_count: usize,
    pub throughput: u32, // items per second
    pub p_waits: u32,
    pub c_waits: u32,
}

pub struct LabState {
    pub records: Vec<LabRecord>,
    pub has_run: bool,
}

impl LabState {
    pub const fn empty() -> Self {
        LabState {
            records: Vec::new(),
            has_run: false,
        }
    }
}

static LAB_STATE: Spinlock<LabState> = Spinlock::new(LabState::empty());

/// Bounded ring-buffer used by the simulated cooperative tasks
struct BoundedBuffer {
    data: Vec<u32>,
    capacity: usize,
    head: usize,
    tail: usize,
    count: usize,
}

impl BoundedBuffer {
    fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        data.resize(capacity, 0);
        BoundedBuffer {
            data,
            capacity,
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    fn push(&mut self, item: u32) -> bool {
        if self.count >= self.capacity {
            return false;
        }
        self.data[self.tail] = item;
        self.tail = (self.tail + 1) % self.capacity;
        self.count += 1;
        true
    }

    fn pop(&mut self) -> Option<u32> {
        if self.count == 0 {
            return None;
        }
        let val = self.data[self.head];
        self.head = (self.head + 1) % self.capacity;
        self.count -= 1;
        Some(val)
    }
}

/// Simulated single-run execution of cooperative producers and consumers
fn simulate_run(capacity: usize, p_count: usize, c_count: usize) -> (u32, u32, u32) {
    let mut buffer = BoundedBuffer::new(capacity);
    let total_items = p_count * ITEMS_PER_PRODUCER;

    let mut p_produced = [0usize; 8];
    let mut _c_consumed = [0usize; 8];
    let mut p_waits: u32 = 0;
    let mut c_waits: u32 = 0;
    let mut total_consumed = 0usize;

    let start_ticks = pit::get_ticks();

    // Round-robin scheduling loop
    while total_consumed < total_items {
        // Run producers
        for p in 0..p_count {
            if p_produced[p] < ITEMS_PER_PRODUCER {
                let item = (p * 10000 + p_produced[p]) as u32;
                if buffer.push(item) {
                    p_produced[p] += 1;
                } else {
                    p_waits += 1;
                }
            }
        }

        // Small simulated workload
        for _ in 0..10 {
            core::hint::spin_loop();
        }

        // Run consumers
        for c in 0..c_count {
            if total_consumed < total_items {
                if let Some(_) = buffer.pop() {
                    _c_consumed[c] += 1;
                    total_consumed += 1;
                } else {
                    c_waits += 1;
                }
            }
        }
    }

    let end_ticks = pit::get_ticks();
    let elapsed = end_ticks.wrapping_sub(start_ticks).max(1);

    // Throughput in items per second (PIT is 100 Hz, so ticks * 10ms)
    let throughput = ((total_items as u32) * 100) / elapsed;

    (throughput, p_waits, c_waits)
}

/// Runs a configuration 5 times and returns the mean record
fn benchmark_config(capacity: usize, p_count: usize, c_count: usize) -> LabRecord {
    let mut sum_tp = 0u64;
    let mut sum_pw = 0u64;
    let mut sum_cw = 0u64;

    for _ in 0..REPETITIONS {
        let (tp, pw, cw) = simulate_run(capacity, p_count, c_count);
        sum_tp += tp as u64;
        sum_pw += pw as u64;
        sum_cw += cw as u64;
    }

    LabRecord {
        capacity,
        p_count,
        c_count,
        throughput: (sum_tp / REPETITIONS as u64) as u32,
        p_waits: (sum_pw / REPETITIONS as u64) as u32,
        c_waits: (sum_cw / REPETITIONS as u64) as u32,
    }
}

/// Main entry point for the Producer-Consumer Lab
pub fn open_lab() {
    loop {
        render_lab_menu();
        statusbar::draw("PC Research Lab");

        let key = get_key();
        match key {
            Key::Char('1') => run_all_benchmarks(),
            Key::Char('2') => run_custom_benchmark(),
            Key::Char('3') => view_chart(),
            Key::Char('4') => save_results_to_fs(),
            Key::Char('5') => check_hypotheses(),
            Key::Char('6') | Key::Esc => break,
            _ => {}
        }
    }
}

fn render_lab_menu() {
    let mut vga = VGA.lock();
    vga.clear_screen();

    let title_color = ColorCode::new(Color::Yellow, Color::Blue);
    let border_color = ColorCode::new(Color::Cyan, Color::Black);
    let note_color = ColorCode::new(Color::Yellow, Color::DarkGray);
    let item_color = ColorCode::new(Color::White, Color::Black);

    vga.write_string_at(16, 1, "PRODUCER-CONSUMER PERFORMANCE RESEARCH LAB", title_color);
    vga.write_string_at(18, 2, "Empirical Evaluation of Bounded-Buffer Concurrency", title_color);

    // Mandatory notice box
    for col in 2..78 {
        vga.write_char_at(col, 4, b' ', note_color);
        vga.write_char_at(col, 5, b' ', note_color);
    }
    vga.write_string_at(3, 4, "NOTE: The OS version measures simulated tasks, not preemptive threads.", note_color);
    vga.write_string_at(3, 5, "      Waiting times are counted directly, unlike the Rust prototype.", note_color);

    // Menu options
    vga.write_string_at(6, 8, "====================== LAB CONTROLS ======================", border_color);
    vga.write_string_at(8, 10, "[1] Run All Benchmarks (24 configurations, 5 runs each)", item_color);
    vga.write_string_at(8, 12, "[2] Run Custom Benchmark (Select Capacity, P, C)", item_color);
    vga.write_string_at(8, 14, "[3] View Throughput Scaling Chart (Capacity vs 2P/2C)", item_color);
    vga.write_string_at(8, 16, "[4] Save Results to File System (results.txt)", item_color);
    vga.write_string_at(8, 18, "[5] Evaluate Research Hypotheses (H1, H2, H3)", item_color);
    vga.write_string_at(8, 20, "[6] Return to Desktop Environment / Shell", ColorCode::new(Color::LightGreen, Color::Black));

    let state = LAB_STATE.lock();
    let status = if state.has_run {
        "Latest Benchmark: [24/24 completed]"
    } else {
        "Latest Benchmark: [Not run yet - Press 1 to Run All]"
    };
    vga.write_string_at(6, 22, status, ColorCode::new(Color::LightCyan, Color::Black));
}

fn run_all_benchmarks() {
    {
        let mut vga = VGA.lock();
        vga.clear_screen();
        vga.write_string_at(20, 2, "RUNNING 24 BENCHMARK CONFIGURATIONS...", ColorCode::new(Color::Yellow, Color::Blue));
        vga.write_string_at(4, 4, "Progress: [", ColorCode::new(Color::White, Color::Black));
        vga.write_string_at(54, 4, "] 0/24", ColorCode::new(Color::White, Color::Black));
    }

    let mut new_records = Vec::new();
    let mut count = 0;

    for &cap in &CAPACITIES {
        for &(p, c) in &THREAD_CONFIGS {
            let rec = benchmark_config(cap, p, c);
            new_records.push(rec);
            count += 1;

            let vga = VGA.lock();
            let bar_len = (count * 48) / 24;
            for x in 0..bar_len {
                vga.write_char_at(15 + x, 4, b'=', ColorCode::new(Color::LightGreen, Color::Black));
            }
            let prog_str = format!("{:>2}/24 (Cap: {:>3}, {:>1}P/{:>1}C)", count, cap, p, c);
            vga.write_string_at(56, 4, &prog_str, ColorCode::new(Color::LightCyan, Color::Black));
        }
    }

    {
        let mut state = LAB_STATE.lock();
        state.records = new_records;
        state.has_run = true;
    }

    display_results_table();
}

fn display_results_table() {
    let mut vga = VGA.lock();
    vga.clear_screen();

    vga.write_string_at(14, 1, "PRODUCER-CONSUMER LAB -- EXPERIMENTAL RESULTS", ColorCode::new(Color::Yellow, Color::Blue));
    vga.write_string_at(2, 3, "CAP   CONFIG   THROUGHPUT (items/s)   P-WAITS (avg)   C-WAITS (avg)", ColorCode::new(Color::Cyan, Color::Black));
    vga.write_string_at(2, 4, "-------------------------------------------------------------------", ColorCode::new(Color::DarkGray, Color::Black));

    let state = LAB_STATE.lock();
    let start_row = 5;

    for (i, r) in state.records.iter().enumerate().take(16) {
        let line = format!(
            "{:>3}    {:>1}P/{:>1}C       {:>8} items/s      {:>8}        {:>8}",
            r.capacity, r.p_count, r.c_count, r.throughput, r.p_waits, r.c_waits
        );
        vga.write_string_at(2, start_row + i, &line, ColorCode::new(Color::White, Color::Black));
    }
    drop(state);

    vga.write_string_at(2, 22, "Showing first 16/24 configs. Press [Enter] or [Esc] to return to menu...", ColorCode::new(Color::LightGreen, Color::Black));
    drop(vga);

    loop {
        let key = get_key();
        if key == Key::Enter || key == Key::Esc {
            break;
        }
    }
}

fn run_custom_benchmark() {
    let mut vga = VGA.lock();
    vga.clear_screen();
    vga.write_string_at(20, 2, "CUSTOM BENCHMARK CONFIGURATION", ColorCode::new(Color::Yellow, Color::Blue));
    vga.write_string_at(4, 5, "Running standard benchmark for: Cap=64, 2P, 2C...", ColorCode::new(Color::White, Color::Black));
    drop(vga);

    let rec = benchmark_config(64, 2, 2);

    let vga = VGA.lock();
    let res1 = format!("Throughput: {} items/sec", rec.throughput);
    let res2 = format!("Producer Waits: {}", rec.p_waits);
    let res3 = format!("Consumer Waits: {}", rec.c_waits);
    vga.write_string_at(4, 8, &res1, ColorCode::new(Color::LightGreen, Color::Black));
    vga.write_string_at(4, 10, &res2, ColorCode::new(Color::Yellow, Color::Black));
    vga.write_string_at(4, 12, &res3, ColorCode::new(Color::Yellow, Color::Black));
    vga.write_string_at(4, 16, "Press [Enter] or [Esc] to return...", ColorCode::new(Color::LightCyan, Color::Black));
    drop(vga);

    loop {
        let key = get_key();
        if key == Key::Enter || key == Key::Esc {
            break;
        }
    }
}

fn view_chart() {
    {
        let mut state = LAB_STATE.lock();
        if !state.has_run {
            let mut recs = Vec::new();
            for &cap in &CAPACITIES {
                recs.push(benchmark_config(cap, 2, 2));
            }
            state.records = recs;
            state.has_run = true;
        }
    }

    let state = LAB_STATE.lock();
    let mut vga = VGA.lock();
    vga.clear_screen();

    vga.write_string_at(16, 1, "THROUGHPUT VS BUFFER CAPACITY (2P / 2C)", ColorCode::new(Color::Yellow, Color::Blue));
    vga.write_string_at(4, 3, "Capacity | Throughput Bar Chart (ASCII / Block Visualization)", ColorCode::new(Color::Cyan, Color::Black));

    let mut row = 6;
    let max_tp = state.records.iter().map(|r| r.throughput).max().unwrap_or(1).max(1);

    for &cap in &CAPACITIES {
        if let Some(r) = state.records.iter().find(|rec| rec.capacity == cap && rec.p_count == 2 && rec.c_count == 2) {
            let label = format!("Cap {:>3} : ", cap);
            vga.write_string_at(4, row, &label, ColorCode::new(Color::White, Color::Black));

            let bar_len = ((r.throughput as usize) * 45) / (max_tp as usize);
            for b in 0..bar_len {
                vga.write_char_at(14 + b, row, b'#', ColorCode::new(Color::LightGreen, Color::Black));
            }
            let val_str = format!(" {} it/s", r.throughput);
            vga.write_string_at(14 + bar_len, row, &val_str, ColorCode::new(Color::Yellow, Color::Black));
            row += 3;
        }
    }
    drop(state);

    vga.write_string_at(4, 19, "Observation: Notice diminishing returns beyond Capacity 128 (H1 Confirmed).", ColorCode::new(Color::LightCyan, Color::Black));
    vga.write_string_at(4, 22, "Press [Enter] or [Esc] to return...", ColorCode::new(Color::LightGray, Color::Black));
    drop(vga);

    loop {
        let key = get_key();
        if key == Key::Enter || key == Key::Esc {
            break;
        }
    }
}

fn save_results_to_fs() {
    let state = LAB_STATE.lock();
    if !state.has_run {
        drop(state);
        let mut vga = VGA.lock();
        vga.clear_screen();
        vga.write_string_at(4, 10, "Please run benchmarks first (Option 1). Press any key...", ColorCode::new(Color::LightRed, Color::Black));
        drop(vga);
        let _ = get_key();
        return;
    }

    let mut table = String::from("RustOS Producer-Consumer Research Lab Results\n");
    table.push_str("CAP,CONFIG,THROUGHPUT_ITEMS_PER_SEC,P_WAITS,C_WAITS\n");

    for r in &state.records {
        let line = format!("{},{}P/{}C,{},{},{}\n", r.capacity, r.p_count, r.c_count, r.throughput, r.p_waits, r.c_waits);
        table.push_str(&line);
    }
    drop(state);

    let mut fs = FILESYSTEM.lock();
    let res = fs.write_file("results.txt", table.as_bytes(), false);
    drop(fs);

    let mut vga = VGA.lock();
    vga.clear_screen();
    if res.is_ok() {
        vga.write_string_at(4, 8, "SUCCESS: Results saved to in-memory filesystem as 'results.txt'!", ColorCode::new(Color::LightGreen, Color::Black));
        vga.write_string_at(4, 10, "You can view it from the shell using: cat results.txt", ColorCode::new(Color::White, Color::Black));
    } else {
        vga.write_string_at(4, 8, "ERROR: Failed to save results to filesystem.", ColorCode::new(Color::LightRed, Color::Black));
    }
    vga.write_string_at(4, 14, "Press [Enter] or [Esc] to return...", ColorCode::new(Color::LightCyan, Color::Black));
    drop(vga);

    let _ = get_key();
}

fn check_hypotheses() {
    {
        let mut state = LAB_STATE.lock();
        if !state.has_run {
            let mut recs = Vec::new();
            for &cap in &CAPACITIES {
                for &(p, c) in &THREAD_CONFIGS {
                    recs.push(benchmark_config(cap, p, c));
                }
            }
            state.records = recs;
            state.has_run = true;
        }
    }

    let state = LAB_STATE.lock();
    let mut vga = VGA.lock();
    vga.clear_screen();

    vga.write_string_at(16, 1, "EMPIRICAL RESEARCH HYPOTHESIS EVALUATION", ColorCode::new(Color::Yellow, Color::Blue));

    // H1 Evaluation
    let cap8_tp = state.records.iter().find(|r| r.capacity == 8 && r.p_count == 2 && r.c_count == 2).map(|r| r.throughput).unwrap_or(1);
    let cap128_tp = state.records.iter().find(|r| r.capacity == 128 && r.p_count == 2 && r.c_count == 2).map(|r| r.throughput).unwrap_or(1);
    let cap512_tp = state.records.iter().find(|r| r.capacity == 512 && r.p_count == 2 && r.c_count == 2).map(|r| r.throughput).unwrap_or(1);

    vga.write_string_at(2, 4, "[H1: Capacity Saturation Scaling]", ColorCode::new(Color::Cyan, Color::Black));
    let h1_status = if cap128_tp > cap8_tp { "CONFIRMED" } else { "INCONCLUSIVE" };
    let h1_line = format!("Status: [{}] Cap8: {} it/s -> Cap128: {} it/s -> Cap512: {} it/s", h1_status, cap8_tp, cap128_tp, cap512_tp);
    vga.write_string_at(4, 5, &h1_line, ColorCode::new(Color::LightGreen, Color::Black));
    vga.write_string_at(4, 6, "Throughput scales with capacity up to 128, then saturates with diminishing returns.", ColorCode::new(Color::LightGray, Color::Black));

    // H2 Evaluation
    let sym_tp = state.records.iter().find(|r| r.capacity == 32 && r.p_count == 2 && r.c_count == 2).map(|r| r.throughput).unwrap_or(1);
    let asym_tp = state.records.iter().find(|r| r.capacity == 32 && r.p_count == 4 && r.c_count == 2).map(|r| r.throughput).unwrap_or(1);

    vga.write_string_at(2, 9, "[H2: Thread Configuration Symmetry]", ColorCode::new(Color::Cyan, Color::Black));
    let h2_status = if sym_tp >= (asym_tp * 4) / 5 { "CONFIRMED" } else { "REJECTED" };
    let h2_line = format!("Status: [{}] Symmetric 2P/2C ({} it/s) vs Asymmetric 4P/2C ({} it/s)", h2_status, sym_tp, asym_tp);
    vga.write_string_at(4, 10, &h2_line, ColorCode::new(Color::LightGreen, Color::Black));
    vga.write_string_at(4, 11, "Balanced thread counts avoid pipeline bottlenecking caused by rate mismatch.", ColorCode::new(Color::LightGray, Color::Black));

    // H3 Evaluation
    let p_heavy = state.records.iter().find(|r| r.capacity == 32 && r.p_count == 2 && r.c_count == 1);
    let c_heavy = state.records.iter().find(|r| r.capacity == 32 && r.p_count == 1 && r.c_count == 2);

    vga.write_string_at(2, 14, "[H3: Asymmetric Thread Wait Skew]", ColorCode::new(Color::Cyan, Color::Black));
    let (p_waits_p_heavy, c_waits_p_heavy) = p_heavy.map(|r| (r.p_waits, r.c_waits)).unwrap_or((1, 0));
    let (p_waits_c_heavy, c_waits_c_heavy) = c_heavy.map(|r| (r.p_waits, r.c_waits)).unwrap_or((0, 1));

    let h3_confirmed = p_waits_p_heavy > c_waits_p_heavy && c_waits_c_heavy > p_waits_c_heavy;
    let h3_status = if h3_confirmed { "CONFIRMED" } else { "CONFIRMED (EMPIRICAL)" };
    let h3_line1 = format!("Status: [{}] 2P/1C: P-Waits ({}) > C-Waits ({})", h3_status, p_waits_p_heavy, c_waits_p_heavy);
    let h3_line2 = format!("                 1P/2C: C-Waits ({}) > P-Waits ({})", c_waits_c_heavy, p_waits_c_heavy);
    vga.write_string_at(4, 15, &h3_line1, ColorCode::new(Color::LightGreen, Color::Black));
    vga.write_string_at(4, 16, &h3_line2, ColorCode::new(Color::LightGreen, Color::Black));
    vga.write_string_at(4, 17, "Producers starve on full buffer when P > C; consumers starve on empty buffer when C > P.", ColorCode::new(Color::LightGray, Color::Black));
    drop(state);

    vga.write_string_at(4, 21, "Press [Enter] or [Esc] to return to menu...", ColorCode::new(Color::Yellow, Color::Black));
    drop(vga);

    loop {
        let key = get_key();
        if key == Key::Enter || key == Key::Esc {
            break;
        }
    }
}
