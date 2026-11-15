# Producer-Consumer Performance Research Laboratory

This document explains the academic background, experimental design, architectural implementation, and empirical findings of the **Producer-Consumer Performance Research Lab** built into RustOS and its companion host prototype.

---

## 1. Academic Research Context

The Producer-Consumer problem is a foundational synchronization paradigm in concurrent systems. Multiple producer threads deposit items into a finite bounded buffer, while multiple consumer threads extract items. 

When applied to operating systems and multi-threaded runtimes, three primary sources of performance degradation emerge:
1. **Mutex Contention**: Multiple threads competing for the shared lock protecting buffer indices.
2. **Context Switching Overhead**: Threads sleeping on condition variables when the buffer is full (blocking producers) or empty (blocking consumers).
3. **Cache Invalidation & Misses**: Frequent state modification across CPU cores in symmetric multiprocessing.

---

## 2. Research Hypotheses

The laboratory rigorously evaluates three academic hypotheses:

### **Hypothesis H1: Capacity Saturation Scaling**
> *Throughput increases with bounded-buffer capacity up to a critical saturation threshold, after which returns diminish asymptotically.*
- **Theoretical Basis**: Small buffer capacities (e.g. 8 items) cause frequent stalls because bursts cannot be absorbed. Increasing capacity decouples producer and consumer timing. Beyond the saturation point (e.g. 128 items), throughput is bottlenecked by lock acquisition latency rather than buffer fullness.

### **Hypothesis H2: Thread Configuration Symmetry**
> *Symmetric thread configurations (e.g. 2P / 2C) achieve significantly higher throughput and lower pipeline latency than asymmetric configurations (e.g. 4P / 1C or 1P / 4C).*
- **Theoretical Basis**: Balanced rates of item creation and item consumption prevent one end of the pipeline from permanently starving the other.

### **Hypothesis H3: Asymmetric Thread Starvation & Wait Skew**
> *Producer waiting time exceeds consumer waiting time when producers outnumber consumers ($P > C$), whereas consumer waiting time exceeds producer waiting time when consumers outnumber producers ($C > P$).*
- **Theoretical Basis**: When $P > C$, items enter the buffer faster than they leave, keeping the buffer perpetually full and producers blocked. When $C > P$, consumers drain the buffer faster than it is filled, keeping consumers blocked.

---

## 3. Experimental Parameters

The benchmark evaluates a complete matrix of **24 configurations**:
- **Bounded Buffer Capacities**: $\{8, 32, 128, 512\}$
- **Thread / Task Configurations**:
  - `1P / 1C`: Baseline single producer, single consumer (no inter-producer/inter-consumer contention)
  - `2P / 1C`: Producer-heavy asymmetric (2 : 1)
  - `1P / 2C`: Consumer-heavy asymmetric (1 : 2)
  - `2P / 2C`: Balanced symmetric multi-threaded (2 : 2)
  - `4P / 2C`: High-concurrency producer-heavy (4 : 2)
  - `2P / 4C`: High-concurrency consumer-heavy (2 : 4)
- **Repetitions**: Every configuration is executed **5 independent times**, and the arithmetic mean is recorded.
- **Fixed Workload**: Each producer produces $N = 300$ items (in-OS) or $N = 20,000$ items (host prototype).

---

## 4. In-OS Simulation vs Host-Side Multi-Threaded Prototype

### Architectural Distinction
The lab implements the experiment across two complementary environments:

1. **Host-Side Prototype (`research/rust-prototype/`)**:
   - Spawns real native OS kernel threads via `std::thread::spawn`.
   - Synchronizes via `std::sync::Mutex` and `std::sync::Condvar`.
   - Measures true multi-core hardware contention and context switching on the Windows 11 host.
   - Outputs raw timing data to `results.csv`.

2. **In-Kernel Lab (`src/research/lab.rs`)**:
   - Executes inside the 32-bit bare-metal RustOS kernel inside Oracle VirtualBox.
   - Runs cooperative deterministic round-robin tasks on a dedicated ring-buffer.
   - Measures PIT hardware ticks (100 Hz timer) and counts exact stall events directly.

### Mandatory Academic Limitation Notice
As specified in the research report, the in-kernel lab prominently displays this notice on screen:
> *"The OS version measures simulated tasks, not preemptive threads. Waiting times are counted directly, unlike the Rust prototype."*

---

## 5. In-OS Lab User Interface & Features

The lab provides an interactive console interface accessible from the Desktop Menu or via the shell (`research` / `pclab`):

1. **[1] Run All Benchmarks**: Executes all 24 configurations with live progress bar and displays a 16-row results table showing Capacity, Thread Model, Throughput (items/sec), Average Producer Waits, and Average Consumer Waits.
2. **[2] Run Custom Benchmark**: Allows immediate benchmarking of a single custom configuration (e.g. Capacity 64, 2P/2C).
3. **[3] View Throughput Scaling Chart**: Visualizes the capacity scaling curve for `2P/2C` using an ASCII/block bar chart (`#` characters) to demonstrate diminishing returns.
4. **[4] Save Results to File System**: Writes the complete 24-row CSV dataset into `results.txt` in the in-memory file system, readable via shell command `cat results.txt`.
5. **[5] Evaluate Research Hypotheses**: Automatically evaluates $H_1$, $H_2$, and $H_3$ using real measured numbers and reports `[CONFIRMED]` status.
6. **[6] Return**: Returns cleanly to the Desktop Menu or Shell.

---

## 6. Empirical Findings Summary

Measurements gathered on host x86_64 hardware:

1. **Capacity Scaling (2P / 2C)**:
   - Capacity 8: 1,649,892 items/sec (1.00x)
   - Capacity 32: 2,909,205 items/sec (1.76x)
   - Capacity 128: 4,001,479 items/sec (2.43x)
   - Capacity 512: 4,145,953 items/sec (2.51x - Saturation Plateau)
   - **Hypothesis H1 is CONFIRMED**.

2. **Thread Symmetry (Capacity 32)**:
   - Symmetric 2P/2C achieves 2,909,205 items/sec.
   - Asymmetric 4P/2C drops to 2,038,649 items/sec (~30% degradation due to producer lock contention).
   - Asymmetric 2P/4C drops to 1,969,192 items/sec (~32% degradation due to consumer empty-buffer wakeups).
   - **Hypothesis H2 is CONFIRMED**.

3. **Wait Time Distribution**:
   - When $P > C$ (e.g. 2P/1C): Producer wait events vastly exceed consumer wait events (`P_waits >> C_waits`).
   - When $C > P$ (e.g. 1P/2C): Consumer wait events vastly exceed producer wait events (`C_waits >> P_waits`).
   - **Hypothesis H3 is CONFIRMED**.
