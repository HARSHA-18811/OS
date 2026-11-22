#!/bin/bash
# ==============================================================================
# Debian Micro OS -- Automated Live Hybrid ISO Build Script
# Packages: Linux Kernel, Systemd, WiFi/LAN (NetworkManager), APT/Aptitude,
#           Snapstore (snapd), and Producer-Consumer Research Lab
# ==============================================================================
set -e

if [ "$(id -u)" -ne 0 ]; then
    echo "ERROR: This script must be run as root (use 'wsl -u root ...')"
    exit 1
fi

WORKDIR="/var/tmp/micro-os-build"
CHROOT="$WORKDIR/chroot"
ISODIR="$WORKDIR/iso"
FINAL_ISO="/mnt/d/OS/micro-os/build/micro-os.iso"
RESEARCH_SRC="/mnt/d/OS/micro-os/research"
LAUNCHER_SRC="/mnt/d/OS/micro-os/scripts/micro-os-launcher.sh"

echo "=== [Step 1/7] Compiling Producer-Consumer Research Lab (Rust) ==="
cd "$RESEARCH_SRC"
cargo build --release
PCLAB_BIN="$RESEARCH_SRC/target/release/pclab"
if [ ! -f "$PCLAB_BIN" ]; then
    echo "ERROR: pclab binary was not built."
    exit 1
fi
echo "[OK] pclab binary compiled successfully."

echo "=== [Step 2/7] Preparing Build Directories in WSL ==="
# Unmount if previous unclean exit
umount -lf "$CHROOT/dev/pts" 2>/dev/null || true
umount -lf "$CHROOT/dev" 2>/dev/null || true
umount -lf "$CHROOT/proc" 2>/dev/null || true
umount -lf "$CHROOT/sys" 2>/dev/null || true

rm -rf "$WORKDIR"
mkdir -p "$CHROOT" "$ISODIR/live" "$ISODIR/boot/grub"
mkdir -p "$(dirname "$FINAL_ISO")"

echo "=== [Step 3/7] Debootstrapping Minimal Debian 12 (Bookworm amd64) ==="
debootstrap --arch=amd64 --variant=minbase bookworm "$CHROOT" http://deb.debian.org/debian

echo "=== [Step 4/7] Configuring Chroot Environment ==="
# Configure Hostname & Hosts
echo "micro-os" > "$CHROOT/etc/hostname"
cat << 'EOF' > "$CHROOT/etc/hosts"
127.0.0.1   localhost micro-os
::1         localhost ip6-localhost ip6-loopback
EOF

# Configure APT Repositories
cat << 'EOF' > "$CHROOT/etc/apt/sources.list"
deb http://deb.debian.org/debian bookworm main contrib non-free non-free-firmware
deb http://security.debian.org/debian-security bookworm-security main contrib non-free non-free-firmware
deb http://deb.debian.org/debian bookworm-updates main contrib non-free non-free-firmware
EOF

# Mount virtual filesystems for chroot operations
mount --bind /dev "$CHROOT/dev"
mount --bind /dev/pts "$CHROOT/dev/pts"
mount -t proc none "$CHROOT/proc"
mount -t sysfs none "$CHROOT/sys"

# Run internal configuration inside chroot
cat << 'CHROOT_SCRIPT' > "$CHROOT/tmp/setup.sh"
#!/bin/bash
set -e
export DEBIAN_FRONTEND=noninteractive

echo "--- Updating Package Lists ---"
apt-get update

echo "--- Installing Linux Kernel, Live-Boot, and Systemd ---"
apt-get install -y --no-install-recommends \
    linux-image-amd64 \
    live-boot \
    systemd \
    systemd-sysv

echo "--- Installing WiFi, LAN & Networking Drivers ---"
apt-get install -y --no-install-recommends \
    network-manager \
    wireless-tools \
    wpasupplicant \
    rfkill \
    firmware-linux-free \
    iproute2 \
    isc-dhcp-client \
    iputils-ping

echo "--- Installing Package Managers (APT, Aptitude) & Store (Snapd) ---"
apt-get install -y \
    apt \
    apt-utils \
    aptitude \
    snapd

echo "--- Installing Default Web Browser (Firefox ESR) ---"
apt-get install -y --no-install-recommends \
    firefox-esr

echo "--- Installing Essential Utilities & Diagnostics ---"
apt-get install -y --no-install-recommends \
    sudo \
    nano \
    curl \
    wget \
    htop \
    pciutils \
    usbutils \
    kmod \
    bash-completion

echo "--- Creating Default User 'user' (Password: user) ---"
useradd -m -s /bin/bash -G sudo,netdev,adm user
echo "user:user" | chpasswd
echo "root:root" | chpasswd
echo "user ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/user
chmod 0440 /etc/sudoers.d/user

echo "--- Enabling Autologin on tty1 ---"
mkdir -p /etc/systemd/system/getty@tty1.service.d
cat << 'EOF' > /etc/systemd/system/getty@tty1.service.d/override.conf
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin user --noclear %I $TERM
EOF

echo "--- Enabling NetworkManager and Snapd Services ---"
systemctl enable NetworkManager.service || true
systemctl enable snapd.service || true

echo "--- Cleaning Package Cache to Minimize ISO Size ---"
apt-get clean
rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
CHROOT_SCRIPT

chmod +x "$CHROOT/tmp/setup.sh"
chroot "$CHROOT" /tmp/setup.sh
rm -f "$CHROOT/tmp/setup.sh"

echo "=== [Step 5/7] Installing Research Lab & Welcome Launcher ==="
cp "$PCLAB_BIN" "$CHROOT/usr/local/bin/pclab"
chmod 755 "$CHROOT/usr/local/bin/pclab"
ln -sf /usr/local/bin/pclab "$CHROOT/usr/local/bin/research"

cp "$LAUNCHER_SRC" "$CHROOT/usr/local/bin/micro-os-launcher"
chmod 755 "$CHROOT/usr/local/bin/micro-os-launcher"

# Auto-launch on user login
cat << 'EOF' >> "$CHROOT/home/user/.bashrc"
if [ -z "$MICRO_OS_LAUNCHED" ] && [ "$(tty)" = "/dev/tty1" ]; then
    export MICRO_OS_LAUNCHED=1
    /usr/local/bin/micro-os-launcher
fi
EOF
chown -R 1000:1000 "$CHROOT/home/user"

# Copy kernel and initrd out of chroot before unmounting
echo "--- Copying Kernel and Initramfs for Bootloader ---"
cp "$CHROOT/boot/vmlinuz-"* "$ISODIR/live/vmlinuz"
cp "$CHROOT/boot/initrd.img-"* "$ISODIR/live/initrd.img"

# Unmount chroot
umount -lf "$CHROOT/dev/pts" || true
umount -lf "$CHROOT/dev" || true
umount -lf "$CHROOT/proc" || true
umount -lf "$CHROOT/sys" || true

echo "=== [Step 6/7] Compressing Root Filesystem into SquashFS ==="
mksquashfs "$CHROOT" "$ISODIR/live/filesystem.squashfs" -comp xz -e boot

echo "=== [Step 7/7] Generating Hybrid UEFI + Legacy BIOS Bootable ISO ==="
cat << 'EOF' > "$ISODIR/boot/grub/grub.cfg"
search --set=root --file /live/vmlinuz
set default="0"
set timeout=3

menuentry "Debian Micro OS (Live with Research Lab)" {
    linux /live/vmlinuz boot=live quiet splash components
    initrd /live/initrd.img
}

menuentry "Debian Micro OS (Safe Mode - Single User)" {
    linux /live/vmlinuz boot=live components single
    initrd /live/initrd.img
}
EOF

grub-mkrescue -o "$FINAL_ISO" "$ISODIR"
chmod 666 "$FINAL_ISO"

ISO_SIZE=$(du -h "$FINAL_ISO" | cut -f1)
echo "=============================================================================="
echo " [SUCCESS] Debian Micro OS Bootable ISO Generated: $FINAL_ISO ($ISO_SIZE)"
echo " Hybrid Boot: UEFI + Legacy BIOS (Laptop USB & VirtualBox compatible)"
echo " Included: WiFi/LAN, APT, Aptitude, Snapstore, and Producer-Consumer Lab"
echo "=============================================================================="
