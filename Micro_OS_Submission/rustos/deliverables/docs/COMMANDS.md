# RustOS Shell Commands Manual

RustOS provides a UNIX-inspired interactive shell featuring 27 built-in commands, command history (navigated with Up/Down arrow keys), argument validation, red error messages for failures, and file redirection.

---

## Command Reference Index

| Command | Usage | Description |
|:---|:---|:---|
| **help** | `help` | List all available shell commands with one-line descriptions |
| **clear** | `clear` | Clear the VGA screen buffer and reset cursor to (0, 0) |
| **reboot** | `reboot` | Restart the system via PS/2 keyboard controller reset |
| **shutdown** | `shutdown` | Halt CPU cleanly for VM poweroff |
| **date** | `date` | Display current hardware date, time, and day of week from CMOS |
| **cal** | `cal` | Render current month calendar with today's date highlighted |
| **echo** | `echo <text>` | Print text to the console screen |
| **echo >** | `echo <text> > <file>` | Write or redirect text into an in-memory file |
| **ls** | `ls` | List all files with permissions (`rw-` or `r--`), size, and name |
| **lp** | `lp <file>` | Print file content with a `[lp] no printer attached` header |
| **whoami** | `whoami` | Print active user name (`root`) |
| **uname** | `uname` | Print operating system name, version, and architecture |
| **hostname** | `hostname` | Print system network node hostname (`rustos`) |
| **uptime** | `uptime` | Display elapsed uptime in `hh:mm:ss` based on PIT timer |
| **bc** | `bc <n> <op> <n>` | Binary integer calculator (`+`, `-`, `*`, `/`, `%`) |
| **cat** | `cat <file>` | Output file contents to the screen |
| **grab** | `grab <word> <file>` | Filter and display lines matching the specified substring |
| **rm** | `rm <file>` | Delete a file from the in-memory file system |
| **touch** | `touch <file>` | Create a new empty file if it does not already exist |
| **cp** | `cp <src> <dst>` | Copy a file to a new destination name |
| **mv** | `mv <src> <dst>` | Rename a file (fails if destination already exists) |
| **cut** | `cut <start> <end> <f>`| Extract character column range from each line (1-based) |
| **head** | `head <file> [n]` | Print first `n` lines of a file (default 10) |
| **tail** | `tail <file> [n]` | Print last `n` lines of a file (default 10) |
| **chmod** | `chmod <6\|4> <file>` | Set permission mode: `6` = writable (`rw-`), `4` = read-only (`r--`) |
| **wc** | `wc <file>` | Count lines, words, and bytes in a file |
| **vi** | `vi <file>` | Interactive line editor (`:wq` to save, `:q` to exit) |
| **games** | `games` | Launch the Visual Desktop Application & Games Menu |
| **research** / **pclab** | `research` or `pclab` | Launch the Producer-Consumer Performance Research Lab |

---

## Detailed Command Specifications

### 1. `help`
- **Syntax**: `help`
- **Behavior**: Outputs a tabulated reference list of all available commands.
- **Example**:
  ```
  root@rustos:/# help
  Available RustOS Commands:
    help                 - Display this list of available commands
    clear                - Clear the VGA display screen
    ...
  ```

### 2. `clear`
- **Syntax**: `clear`
- **Behavior**: Clears all 80x25 characters on the VGA text buffer and sets cursor position to top-left (row 0, col 0).

### 3. `reboot`
- **Syntax**: `reboot`
- **Behavior**: Displays a rebooting message, pauses for 300 ms, and writes `0xFE` to the keyboard controller command port `0x64`. This asserts the CPU reset line on x86 PC architecture.

### 4. `shutdown`
- **Syntax**: `shutdown`
- **Behavior**: Prints `"System halted. You can now power off the virtual machine."`, disables CPU hardware interrupts via `cli`, and enters an infinite halt (`hlt`) loop.
- **Poweroff Instructions**: The user powers off the virtual machine from VirtualBox (Menu: **Machine -> Close -> Power off the machine**), or executes on the Windows host:
  ```powershell
  VBoxManage controlvm RustOS poweroff
  ```

### 5. `date`
- **Syntax**: `date`
- **Behavior**: Queries the CMOS Real-Time Clock registers and prints the formatted date, time, and day of week.
- **Example**:
  ```
  root@rustos:/# date
  Sunday, 2026-11-08 10:11:32 UTC
  ```

### 6. `cal`
- **Syntax**: `cal`
- **Behavior**: Computes the number of days and starting day of week for the current month, outputs a calendar grid, and highlights today's date in a distinct color block (Yellow text on Blue background).

### 7. `echo` and Redirection (`>`)
- **Syntax**: `echo <text>` or `echo <text> > <filename>`
- **Behavior**:
  - Without `>`: prints text to the screen.
  - With `>`: writes text to `<filename>`. Creates the file if it does not exist. If the target file is read-only, an error message in red is displayed.
- **Examples**:
  ```
  root@rustos:/# echo Hello World
  Hello World

  root@rustos:/# echo Systems Engineering > test.txt
  root@rustos:/# cat test.txt
  Systems Engineering
  ```

### 8. `ls`
- **Syntax**: `ls`
- **Behavior**: Lists all active files in the in-memory file system with permission flags (`rw-` for writable, `r--` for read-only), file size in bytes, and file name.
- **Example**:
  ```
  root@rustos:/# ls
  PERM    SIZE  NAME
  rw-      154  readme.txt
  rw-      201  notes.txt
  rw-      212  todo.txt
  r--      153  about.txt
  ```

### 9. `lp` (Line Printer Redirect)
- **Syntax**: `lp <filename>`
- **Behavior**: Simulates line printer spooling. Because no physical printer is attached to the virtual machine, prints a clear banner header followed by the file contents.
- **Example**:
  ```
  root@rustos:/# lp readme.txt
  [lp] no printer attached; redirecting to console:
  Welcome to RustOS 1.0.0!
  ...
  ```

### 10. `whoami`, `uname`, `hostname`
- **Syntax**: `whoami`, `uname`, `hostname`
- **Outputs**:
  - `whoami` -> `root`
  - `uname` -> `RustOS 1.0.0 i686 (VirtualBox bare-metal)`
  - `hostname` -> `rustos`

### 11. `uptime`
- **Syntax**: `uptime`
- **Behavior**: Converts the 100 Hz PIT hardware tick counter into `hh:mm:ss` format.
- **Example**:
  ```
  root@rustos:/# uptime
  up 00:03:42 (ticks: 22218)
  ```

### 12. `bc` (Binary Calculator)
- **Syntax**: `bc <n1> <operator> <n2>`
- **Supported Operators**: `+`, `-`, `*`, `/`, `%`
- **Features**: Supports 64-bit signed integers, negative operands, divide-by-zero detection, and modulo-by-zero checks.
- **Examples**:
  ```
  root@rustos:/# bc 15 + 27
  42

  root@rustos:/# bc -10 * 6
  -60

  root@rustos:/# bc 100 / 0
  bc: error: divide by zero
  ```

### 13. `cat`
- **Syntax**: `cat <filename>`
- **Behavior**: Prints the UTF-8 text contents of `<filename>`. Displays an error in red if the file is not found.

### 14. `grab`
- **Syntax**: `grab <word> <filename>`
- **Behavior**: Performs substring search on each line of `<filename>` and outputs all matching lines.
- **Example**:
  ```
  root@rustos:/# grab PIC notes.txt
  - IDT + 8259 PIC remapped to 0x20..0x2F
  ```

### 15. `rm`
- **Syntax**: `rm <filename>`
- **Behavior**: Deletes the specified file. If the file is marked read-only (`r--`), deletion is blocked and a red permission denied error is shown.

### 16. `touch`
- **Syntax**: `touch <filename>`
- **Behavior**: Creates an empty file of size 0 bytes if it does not exist. If it already exists, succeeds without modification.

### 17. `cp` and `mv`
- **Syntax**: `cp <src> <dst>` / `mv <src> <dst>`
- **Behavior**:
  - `cp`: Copies `<src>` content and permissions to a new file `<dst>`. Fails if `<dst>` already exists or if the file table is full.
  - `mv`: Renames `<src>` to `<dst>`. Fails if `<src>` does not exist, `<dst>` exists, or `<src>` is read-only.

### 18. `cut`
- **Syntax**: `cut <start_col> <end_col> <filename>`
- **Behavior**: Extracts characters between 1-based column indices `start_col` and `end_col` inclusive for every line in the file.
- **Example**:
  ```
  root@rustos:/# cut 1 7 readme.txt
  Welcome
  RustOS 
  Type 'h
  ```

### 19. `head` and `tail`
- **Syntax**: `head <file> [n]` / `tail <file> [n]`
- **Behavior**: Outputs the first (or last) `n` lines of the specified file (defaults to 10 lines if `n` is omitted).

### 20. `chmod`
- **Syntax**: `chmod <6|4> <filename>`
- **Behavior**: Sets file permissions. Mode `6` enables read-write access (`rw-`). Mode `4` marks the file as read-only (`r--`).

### 21. `wc`
- **Syntax**: `wc <filename>`
- **Behavior**: Counts lines, whitespace-delimited words, and bytes in the file.
- **Example**:
  ```
  root@rustos:/# wc readme.txt
     3   23  154 readme.txt
  ```

### 22. `vi` (Line Editor)
- **Syntax**: `vi <filename>`
- **Behavior**: Opens an interactive line editor for `<filename>`.
  - Displays existing content if the file exists.
  - Shows top banner with editing instructions.
  - Appends lines typed by user.
  - Typing `:wq` saves the content to the in-memory file system and exits back to the shell.
  - Typing `:q` exits without saving changes.

### 23. `games`
- **Syntax**: `games`
- **Behavior**: Launches the Visual Desktop Application Menu.

### 24. `research` / `pclab`
- **Syntax**: `research` or `pclab`
- **Behavior**: Launches the Producer-Consumer Performance Research Laboratory.
