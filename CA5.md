# Investigating the Impact of Bounded Buffer Sizing and Thread Allocation Ratios on Concurrent Producer–Consumer Throughput: A Linux Systems Engineering Empirical Study
### Operating Systems Continuous Assessment — Linux Systems Engineering Challenge (CA-5)

| Metadata Field | Academic Record Details |
|:---|:---|
| **Student Researcher / Author:** | **Y Harsha Vardhan \| PRN: 25BTRCL289** |
| **Course & Academic Position:** | Operating Systems (CSE) \| Continuous Assessment 5 of 7 (10 Marks) |
| **Research Foundation (CA-4):** | Mini-Project: Producer–Consumer Performance (`25BTRCL289_Harsha_Vardhan_Research.docx`) |
| **Research Category & Track:** | Category C: Synchronization and Concurrency \| Track R10: Producer–Consumer Performance |
| **Core Research Question:** | How do buffer size and producer/consumer ratios influence throughput in a bounded system? |
| **Empirical Systems Testbed:** | Ubuntu 24.04.3 LTS \| Linux Kernel 6.6.87.2-WSL2 \| Intel Core i5-13450HX (16 CPUs) |
| **Systems Toolchain Suite:** | Linux POSIX pthreads, `/proc/thread-self/status`, `pidstat`, `vmstat`, `strace -f -c`, `taskset`, Rust, GCC |

---

## Executive Summary
This research document presents the unified culmination of Continuous Assessment 4 (CA-4 Research Mini-Project) and Continuous Assessment 5 (CA-5 Linux Systems Engineering Challenge) in Operating Systems. Authored by Harsha Vardhan (PRN: 25BTRCL289), this investigation systematically tackles Research Track C (Synchronization and Concurrency) and Topic R10 (Producer–Consumer Performance): *"How do buffer size and producer/consumer ratios influence throughput?"* In CA-4, the author developed a foundational research framework and user-space Rust prototype using `crossbeam-channel`, evaluating throughput across capacities 8 to 512 and ratios 1P/1C to 4P/2C. However, CA-4 explicitly acknowledged a key limitation: user-space benchmarking cannot observe kernel scheduling preemption, futex wait-queue contention, or CPU cache invalidation. CA-5 directly closes this research gap by deploying a comprehensive Linux systems engineering investigation on a real operating system (Ubuntu 24.04.3 LTS, Linux Kernel 6.6.87.2-WSL2, Intel Core i5-13450HX with 16 logical CPUs).

By capturing low-level operating system telemetry via `/proc/thread-self/status`, `pidstat -w -u -t`, `vmstat`, `strace -f -c`, and `taskset`, we expose the exact kernel mechanisms governing concurrency bottlenecks. In small buffers ($B=1$), the system collapses into severe futex lock thrashing, achieving only 33,266.5 items/sec and incurring over 402,605 voluntary context switches per 200,000 items processed. Live thread-level pidstat telemetry proves that under $B=1$, worker threads spend 83.0% of their CPU time trapped inside the Linux kernel (`%system = 83.0%`, `%usr = 4.0%`), enduring 17,704 voluntary context switches per second per thread. Scaling buffer capacity to $B=256$ eliminates 95.3% of voluntary context switches (18,888 switches) and accelerates throughput to 1,378,760 items/sec (41.4x gain), saturating at 6,307,235 items/sec at $B=16384$ (189.6x speedup). We formulate, implement, and validate two systems engineering interventions:
1. **Hardware-aware CPU Affinity Pinning** (`taskset` / `pthread_setaffinity_np`) which restricts threads to shared L2 cache clusters, delivering up to a **+44.2% throughput boost**; and
2. **Adaptive Transfer Batching**, which amortizes mutex acquisition over multi-item chunks, slashing context switches by **89.2%** and unlocking **3.38M items/sec (9.08x gain)**.

Finally, we provide a complete conceptual and implementation bridge to CA-6, mapping Linux synchronization directly into bare-metal Rust Micro-OS kernel task queues, spinlocks, and custom assembly context switching.

---

## 1. Linux Operating System Mechanism Identification

### Dual-Tier Locking Architecture
In the Linux kernel, POSIX synchronization primitives (`pthread_mutex_t`, `pthread_cond_t`) are mediated by the `futex` (Fast Userspace Mutex) system call:
- **Fast-Path (Zero Syscall Overhead):** When a mutex lock is uncontended, glibc executes an atomic Compare-And-Swap (CAS) instruction (e.g., `lock cmpxchg` on x86_64) directly in user space. No kernel transition or context switch occurs, completing in ~15 nanoseconds.
- **Slow-Path (Kernel Descheduling):** When contention arises, or when a thread calls `pthread_cond_wait()`, the user-space CAS fails. The thread invokes `sys_futex(uaddr, FUTEX_WAIT_BITSET, val, ...)`. The Linux kernel transitions the thread's state from `TASK_RUNNING` to `TASK_INTERRUPTIBLE`, enqueues its `task_struct` into a hashed spinlock-protected kernel wait queue (`futex_queues`), and triggers a voluntary context switch.
- **Waking Path (Kernel Runqueue Insertion):** When a producer calls `pthread_cond_signal()`, glibc issues `sys_futex(uaddr, FUTEX_WAKE_BITSET, 1, ...)`. The kernel dequeues the sleeping `task_struct`, updates its state to `TASK_RUNNING`, and places it onto the Completely Fair Scheduler (CFS) run queue.

### Context Switch Typology
- **Voluntary Context Switches (`voluntary_ctxt_switches`):** Occur when a thread explicitly yields CPU execution because it cannot proceed (e.g., waiting for a full buffer to drain or an empty buffer to fill). Each voluntary switch incurs CPU register saving, kernel stack pointer adjustment, Translation Lookaside Buffer (TLB) flushing, and scheduler execution (~2.0–3.5 microseconds).
- **Involuntary Context Switches (`nonvoluntary_ctxt_switches`):** Occur when the Linux CFS scheduler forcibly preempts a thread because its time quantum (`vruntime`) has expired.

---

## 2. Linux Environment & Hardware Specification

| Host Machine / Parameter | Details & Settings |
|:---|:---|
| **Host Machine / Model** | Lenovo LOQ / Intel Core i5-13450HX @ 2.40 GHz (up to 4.60 GHz Turbo) |
| **CPU Microarchitecture** | Intel Raptor Lake (x86_64, 64-bit, Little Endian) |
| **CPU Cores / Topologies** | 16 Logical CPUs (8 Physical Cores: 6 P-Cores + 4 E-Cores / 16 SMT threads) |
| **CPU Cache Geometry** | L1d: 384 KiB \| L1i: 256 KiB \| L2: 10 MiB \| L3: 20 MiB unified |
| **System Memory (RAM)** | 7.6 GiB virtualized physical RAM allocated to Linux (WSL2), 7.1 GiB available |
| **Linux Operating System** | Ubuntu 24.04.3 LTS (Noble Numbat) — x86_64 |
| **Linux Kernel Version** | Linux 6.6.87.2-microsoft-standard-WSL2 (#1 SMP PREEMPT_DYNAMIC x86_64) |
| **C Toolchain & Flags** | GCC 13.3.0 (`-O3 -pthread -D_GNU_SOURCE`) |
| **Python Automation Suite** | Python 3.12.3 (subprocess, json, re, statistics) |
| **Rust Systems Bridge** | rustc 1.99.1 / cargo 1.99.1 (`-O` release optimizations) |

---

## 3. Empirical Linux Benchmark Results

### Experiment 1: Bounded Buffer Capacity Scaling ($B \in [1..16384]$, Workload: 200,000 items, 2P : 2C, 5 trials)

| Buffer ($B$) | Elapsed (s) | Throughput (item/s) | Voluntary Ctx | Invol Ctx | Ctx / 1k items |
|:---:|:---:|:---:|:---:|:---:|:---:|
| **$B = 1$** | 6.0832 ± 0.796 | 33,266.5 | 402,605 | 1.8 | 2013.0 |
| **$B = 4$** | 2.6423 ± 0.165 | 75,925.1 | 323,440 | 1.4 | 1617.2 |
| **$B = 16$** | 1.6278 ± 0.096 | 123,221.9 | 196,950 | 1.4 | 984.8 |
| **$B = 64$** | 0.5286 ± 0.099 | 389,980.2 | 65,117 | 0.2 | 325.6 |
| **$B = 256$** | 0.1514 ± 0.032 | 1,378,760.8 | 18,888 | 1.0 | 94.4 |
| **$B = 1024$** | 0.0616 ± 0.011 | 3,346,327.0 | 9,441 | 0.4 | 47.2 |
| **$B = 4096$** | 0.0408 ± 0.012 | 5,381,044.8 | 6,991 | 0.0 | 35.0 |
| **$B = 16384$** | 0.0343 ± 0.010 | **6,307,235.7** | **6,136** | 0.6 | **30.7** |

> **Key Takeaway:** Expanding buffer capacity from $B=1$ to $B=16384$ slashed voluntary context switches by **98.5%** and delivered a **189.6x speedup** in overall throughput!

### Experiment 2: Thread Allocation Ratio Variations ($B = 64$, Workload: 200,000 items, 5 trials)

| Ratio (P : C) | Total Threads | Elapsed Time (s) | Throughput (item/s) | Voluntary Ctx Switches |
|:---:|:---:|:---:|:---:|:---:|
| **1P : 1C** | 2 threads | 0.0865 ± 0.006 | **2,320,853.8** | **6,238** |
| **2P : 2C** | 4 threads | 0.5364 ± 0.057 | 376,343.6 | 66,388 |
| **4P : 4C** | 8 threads | 0.2102 ± 0.076 | 1,026,433.3 | 31,805 |
| **2P : 1C** | 3 threads | 0.5870 ± 0.079 | 345,297.4 | 68,790 |
| **4P : 1C** | 5 threads | 0.4480 ± 0.032 | 448,156.1 | 60,574 |
| **8P : 2C** | 10 threads | 0.3367 ± 0.018 | 595,393.9 | 56,267 |
| **1P : 2C** | 3 threads | 0.5483 ± 0.065 | 368,997.7 | 66,195 |
| **1P : 4C** | 5 threads | 0.4611 ± 0.052 | 437,902.4 | 63,693 |
| **2P : 8C** | 10 threads | 0.3676 ± 0.036 | 548,445.5 | 63,894 |

---

## 4. Live Linux Telemetry: pidstat & strace Profiling

### Thread-Level pidstat Telemetry (Sampled during $B=1$ execution)
```text
Linux 6.6.87.2-microsoft-standard-WSL2 (DESKTOP-P0306P5)  (16 CPU)
UID    TGID   TID    %usr  %system  %CPU  CPU  Command
1000   375     -     4.00    83.00  87.00   5  pc_posix
1000     -   377     1.00    22.00  23.00  13  |__pc_posix (P1)
1000     -   378     0.00    22.00  22.00   8  |__pc_posix (P2)
1000     -   379     2.00    20.00  22.00  15  |__pc_posix (C1)
1000     -   380     0.00    21.00  21.00   4  |__pc_posix (C2)

UID    TGID   TID    cswch/s  nvcswch/s  Command
1000     -   377    17704.00       0.00  |__pc_posix (Producer Thread 1)
1000     -   378    17668.00       0.00  |__pc_posix (Producer Thread 2)
1000     -   379    17679.00       0.00  |__pc_posix (Consumer Thread 1)
1000     -   380    17683.00       0.00  |__pc_posix (Consumer Thread 2)
```

> **Key Linux Observation:** Under $B=1$, worker threads spend **83.0% of their CPU time inside the Linux kernel** (`%system = 83.0%`) and only 4.0% in user space (`%usr = 4.0%`), enduring ~17,700 voluntary context switches every second per thread.

### System Call Profiling: `strace -f -c` futex Profiling

| Configuration | futex Calls | % Syscall Time | Kernel Time (s) | Throughput | Total Syscalls |
|:---|:---:|:---:|:---:|:---:|:---:|
| **Small Buffer ($B = 4$, Batch = 1)** | 178,583 | 99.99% | 12.335 s | 64,571 item/s | 178,651 |
| **Medium Buffer ($B = 64$, Batch = 1)** | 31,920 | 98.85% | 0.482 s | 389,980 item/s | 31,988 |
| **Large Buffer ($B = 1024$, Batch = 1)** | 1,601 | 97.87% | 0.110 s | 3,346,327 item/s | 1,669 |
| **Batched Buffer ($B = 64$, Batch = 16)** | **7,420** | 98.12% | **0.134 s** | **1,567,222 item/s** | **7,488** |

---

## 5. Engineering Interventions & Performance Optimization

### Intervention 1: Hardware-Aware CPU Affinity Pinning (`taskset` / `pthread_setaffinity_np`)
| Buffer ($B$) | Baseline Unpinned | Pinned (Intervention 1) | Baseline Vol Ctx | Pinned Vol Ctx | Speedup Gain |
|:---:|:---:|:---:|:---:|:---:|:---:|
| **$B = 1$** | 33,266.5 item/s | 34,400.4 item/s | 402,605 | 401,803 | +3.4% |
| **$B = 16$** | 123,221.9 item/s | 169,467.6 item/s | 196,950 | 146,730 | +37.5% |
| **$B = 64$** | 389,980.2 item/s | 520,436.0 item/s | 65,117 | 47,676 | +33.5% |
| **$B = 256$** | 1,378,760.8 item/s | 1,988,246.0 item/s | 18,888 | 13,105 | **+44.2%** |
| **$B = 1024$** | 3,346,327.0 item/s | 4,605,533.5 item/s | 9,441 | 7,939 | +37.6% |
| **$B = 4096$** | 5,381,044.8 item/s | 5,878,415.2 item/s | 6,991 | 5,917 | +9.2% |

### Intervention 2: Adaptive Transfer Batching ($b \in [1..64]$, Capacity $B = 64$, 2P : 2C)
| Batch Size ($b$) | Elapsed (s) | Throughput (item/s) | Voluntary Ctx | Invol Ctx | Throughput Gain |
|:---:|:---:|:---:|:---:|:---:|:---:|
| **$b = 1$** | 0.5461 ± 0.077 | 372,499.2 | 65,525 | 0.6 | **1.00x** (Baseline) |
| **$b = 4$** | 0.1970 ± 0.023 | 1,026,685.4 | 23,857 | 0.0 | **2.76x** |
| **$b = 16$** | 0.1284 ± 0.011 | 1,567,222.0 | 15,423 | 0.2 | **4.21x** |
| **$b = 32$** | 0.0886 ± 0.004 | 2,261,684.7 | 11,030 | 0.0 | **6.07x** |
| **$b = 64$** | 0.0600 ± 0.008 | **3,382,746.8** | **7,063** | 0.0 | **9.08x (89.2% switch drop)** |

---

## 6. Systems Bridge: CA-5 Linux Mechanisms &rarr; CA-6 Rust Micro-OS

| Operating System Concept | Observed Linux Mechanism (CA-5) | Rust Micro-OS Kernel Implementation (CA-6) |
|:---|:---|:---|
| **Thread Context** | `pthread_create()` / `clone3()` creating `task_struct` | `TaskControlBlock` (TCB) with dedicated 16 KiB kernel stack and register state. |
| **CPU Task Scheduler** | Linux CFS / EEVDF scheduler with `vruntime` | Preemptive Round-Robin / Priority RunQueue with PIT timer interrupt tick. |
| **Mutual Exclusion** | `pthread_mutex_t` with atomic CAS and futex slow-path | Kernel Spinlock (`AtomicBool` CAS) with interrupt-disabling (`push_cli` / `pop_cli`). |
| **Condition Variables** | `pthread_cond_wait()` / `futex(FUTEX_WAIT)` sleeping | Kernel `WaitQueue`: enqueuing blocked TCB and marking state as `BlockedOnSemaphore`. |
| **Context Switching** | Kernel `switch_to()` saving RIP, RSP, CR3 | Naked assembly function (`asm!`) swapping RSP and pushing/popping registers. |
| **Processor Affinity** | `taskset` / `sched_setaffinity()` pinning threads | Per-core Local APIC task scheduler binding tasks to specific hardware cores. |
| **Instrumentation** | `/proc/<pid>/status` & `pidstat` tracking | Kernel telemetry subsystem logging context switches over Serial UART interface. |

---

## 7. Conclusion
This unified research investigation demonstrates that concurrent producer-consumer performance on a real Linux operating system is overwhelmingly dictated by the interaction between synchronization primitives and low-level kernel mechanisms. Sizing the buffer to $B=256$ eliminates 95.3% of voluntary context switches, CPU affinity pinning boosts throughput by up to **+44.2%**, and transfer batching delivers a **9.08x speedup**. These empirical insights provide a rigorous empirical foundation for constructing custom kernel task queues and spinlocks in the CA-6 Rust Micro-OS.

**Student Researcher / Author:** Y Harsha Vardhan | **PRN:** 25BTRCL289 | **Section:** CSE AIML-E
