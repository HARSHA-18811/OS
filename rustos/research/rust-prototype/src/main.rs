use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Instant;

struct BoundedBuffer<T> {
    queue: Mutex<VecDeque<T>>,
    not_full: Condvar,
    not_empty: Condvar,
    capacity: usize,
    finished: Mutex<bool>,
}

impl<T> BoundedBuffer<T> {
    fn new(capacity: usize) -> Self {
        BoundedBuffer {
            queue: Mutex::new(VecDeque::with_capacity(capacity)),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
            capacity,
            finished: Mutex::new(false),
        }
    }

    fn push(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        while queue.len() >= self.capacity {
            queue = self.not_full.wait(queue).unwrap();
        }
        queue.push_back(item);
        self.not_empty.notify_one();
    }

    fn pop(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        loop {
            if let Some(item) = queue.pop_front() {
                self.not_full.notify_one();
                return Some(item);
            }
            if *self.finished.lock().unwrap() && queue.is_empty() {
                return None;
            }
            queue = self.not_empty.wait(queue).unwrap();
        }
    }

    fn finish(&self) {
        *self.finished.lock().unwrap() = true;
        self.not_empty.notify_all();
    }
}

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
const ITEMS_PER_PRODUCER: usize = 20_000;

#[derive(Clone, Copy)]
struct RunResult {
    capacity: usize,
    producers: usize,
    consumers: usize,
    run_idx: usize,
    duration_ms: f64,
    throughput: f64,
}

fn run_single_test(capacity: usize, p_count: usize, c_count: usize, run_idx: usize) -> RunResult {
    let buffer = Arc::new(BoundedBuffer::new(capacity));
    let total_items = p_count * ITEMS_PER_PRODUCER;

    let start = Instant::now();

    // Spawn Producers
    let mut prod_handles = Vec::with_capacity(p_count);
    for p_id in 0..p_count {
        let b = Arc::clone(&buffer);
        let handle = thread::spawn(move || {
            for i in 0..ITEMS_PER_PRODUCER {
                b.push((p_id * 1_000_000 + i) as u64);
            }
        });
        prod_handles.push(handle);
    }

    // Spawn Consumers
    let mut cons_handles = Vec::with_capacity(c_count);
    for _ in 0..c_count {
        let b = Arc::clone(&buffer);
        let handle = thread::spawn(move || {
            let mut count = 0;
            while let Some(_) = b.pop() {
                count += 1;
            }
            count
        });
        cons_handles.push(handle);
    }

    // Wait for all producers to finish
    for h in prod_handles {
        h.join().unwrap();
    }
    buffer.finish();

    // Wait for all consumers to finish
    for h in cons_handles {
        h.join().unwrap();
    }

    let duration = start.elapsed();
    let duration_ms = duration.as_secs_f64() * 1000.0;
    let throughput = (total_items as f64) / duration.as_secs_f64();

    RunResult {
        capacity,
        producers: p_count,
        consumers: c_count,
        run_idx,
        duration_ms,
        throughput,
    }
}

fn main() {
    println!("=================================================================");
    println!("   PRODUCER-CONSUMER PERFORMANCE RESEARCH: HOST-SIDE PROTOTYPE   ");
    println!("=================================================================");
    println!("Testing 24 configurations (Capacities: 8, 32, 128, 512) x 5 runs each");
    println!("Total workload per producer: {} items\n", ITEMS_PER_PRODUCER);

    let mut all_results: Vec<RunResult> = Vec::new();

    for &cap in &CAPACITIES {
        for &(p, c) in &THREAD_CONFIGS {
            print!("Benchmarking Cap: {:>3}, {:>1}P/{:>1}C ... ", cap, p, c);
            std::io::stdout().flush().unwrap();

            let mut run_tps = Vec::new();
            for run_idx in 1..=REPETITIONS {
                let res = run_single_test(cap, p, c, run_idx);
                run_tps.push(res.throughput);
                all_results.push(res);
            }

            let avg_tp: f64 = run_tps.iter().sum::<f64>() / (REPETITIONS as f64);
            println!("Mean Throughput: {:>10.0} items/sec", avg_tp);
        }
    }

    // Write to results.csv
    let csv_path = "results.csv";
    let mut file = File::create(csv_path).expect("failed to create results.csv");
    writeln!(file, "Capacity,Producers,Consumers,Run,Duration_Ms,Throughput_Items_Per_Sec").unwrap();
    for r in &all_results {
        writeln!(
            file,
            "{},{},{},{},{:.2},{:.0}",
            r.capacity, r.producers, r.consumers, r.run_idx, r.duration_ms, r.throughput
        ).unwrap();
    }
    println!("\n[OK] Raw results saved to {}", csv_path);

    // Print Report Table 1: Throughput vs Capacity (2P / 2C)
    println!("\n--- Table 1: Throughput vs Capacity for Symmetric 2P / 2C ---");
    println!("Capacity | Mean Throughput (items/sec) | Relative Speedup");
    let base_tp: f64 = {
        let runs: Vec<f64> = all_results.iter().filter(|r| r.capacity == 8 && r.producers == 2 && r.consumers == 2).map(|r| r.throughput).collect();
        runs.iter().sum::<f64>() / runs.len() as f64
    };
    for &cap in &CAPACITIES {
        let runs: Vec<f64> = all_results.iter().filter(|r| r.capacity == cap && r.producers == 2 && r.consumers == 2).map(|r| r.throughput).collect();
        let mean_tp = runs.iter().sum::<f64>() / runs.len() as f64;
        let speedup = mean_tp / base_tp;
        println!("{:>8} | {:>27.0} | {:>15.2}x", cap, mean_tp, speedup);
    }

    // Print Report Table 2: Thread Ratio Impact (Capacity 32)
    println!("\n--- Table 2: Thread Ratio Impact at Capacity 32 ---");
    println!("Config | Ratio P:C | Mean Throughput (items/sec)");
    for &(p, c) in &THREAD_CONFIGS {
        let runs: Vec<f64> = all_results.iter().filter(|r| r.capacity == 32 && r.producers == p && r.consumers == c).map(|r| r.throughput).collect();
        let mean_tp = runs.iter().sum::<f64>() / runs.len() as f64;
        println!("{:>1}P/{:>1}C  | {:>4}:{:<4} | {:>26.0}", p, c, p, c, mean_tp);
    }
}
