# Running RustOS in Oracle VirtualBox

This guide provides instructions for running RustOS either via automated PowerShell scripts or manually through the Oracle VirtualBox Graphical User Interface (GUI).

---

## Method 1: Automated Launch via PowerShell (Recommended)

1. Open PowerShell and navigate to the project directory:
   ```powershell
   cd D:\OS\rustos
   ```
2. Run the automated script:
   ```powershell
   .\run.ps1
   ```
   This script will:
   - Compile the kernel and build `build/rustos.iso`
   - Automatically locate `VBoxManage.exe` (checking PATH, `C:\Program Files\Oracle\VirtualBox`, registry, and `D:\VirtualBox`)
   - Create the `RustOS` virtual machine if it does not already exist
   - Set 128 MB RAM, DVD boot order, and BIOS firmware
   - Configure COM1 serial output to `build/serial.log`
   - Attach `build/rustos.iso` to a SATA CD/DVD drive
   - Launch the VirtualBox GUI window

To launch in headless mode (for background testing without a GUI window):
```powershell
.\run.ps1 -Headless
```

To power off the VM from the command line:
```powershell
& 'D:\VirtualBox\VBoxManage.exe' controlvm RustOS poweroff
```

---

## Method 2: Manual Setup via VirtualBox GUI

If you prefer to configure the VM manually using the VirtualBox GUI:

### 1. Build the ISO
Ensure `build/rustos.iso` is generated:
```powershell
.\build.ps1
```

### 2. Create the Virtual Machine
1. Open **Oracle VM VirtualBox Manager**.
2. Click **New** (or press `Ctrl+N`).
3. Set the properties:
   - **Name**: `RustOS`
   - **Type**: `Other`
   - **Version**: `Other/Unknown (32-bit)` or `Other/Unknown`
4. Click **Next**.

### 3. Configure Hardware
1. **Base Memory (RAM)**: Set to `128 MB` (or any value between 64 MB and 512 MB).
2. **Processors**: 1 CPU.
3. Click **Next**.

### 4. Hard Disk
1. Select **Do not add a virtual hard disk** (RustOS runs purely in memory from the boot ISO).
2. Click **Next**, then click **Finish**.

### 5. Attach the ISO
1. Select the `RustOS` VM in the list and click **Settings** (or press `Ctrl+S`).
2. Navigate to the **Storage** tab.
3. If no optical drive is listed under Controller:
   - Click the **Add Optical Drive** icon.
4. Click on the optical drive (it will say `[Optical Drive] Empty`).
5. In the Attributes pane on the right, click the blue disk icon and choose **Choose a disk file...**.
6. Browse to `D:\OS\rustos\build\rustos.iso` and select it.

### 6. Configure System & Boot Order
1. Navigate to the **System** tab.
2. In the **Motherboard** sub-tab:
   - In **Boot Order**, check **Optical** and move it to the top.
   - Ensure **Enable EFI (special OSes only)** is **unchecked** (RustOS boots via BIOS / Multiboot).

### 7. Configure Serial Port (Optional, for logging)
1. Navigate to the **Serial Ports** tab.
2. Under **Port 1**:
   - Check **Enable Serial Port**.
   - **Port Number**: `COM1`.
   - **Port Mode**: `Raw File`.
   - **Path/Address**: Browse or type `D:\OS\rustos\build\serial.log`.

### 8. Start the VM
1. Click **OK** to save settings.
2. Click the green **Start** arrow.
3. The VirtualBox display window will open showing the GRUB bootloader menu:
   ```
   RustOS
   ```
4. Press Enter or wait 3 seconds. RustOS will boot directly to the desktop menu!

---

## Powering Off
When you are finished running RustOS:
1. From the RustOS shell, type `shutdown` (this safely halts CPU execution).
2. Close the VirtualBox VM window and choose **Power off the machine**, or run:
   ```powershell
   & 'D:\VirtualBox\VBoxManage.exe' controlvm RustOS poweroff
   ```
