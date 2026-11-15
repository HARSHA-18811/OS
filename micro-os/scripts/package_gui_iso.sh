#!/bin/bash
set -e

WORKDIR="/var/tmp/micro-os-build"
CHROOT="$WORKDIR/chroot"
ISODIR="$WORKDIR/iso"
FINAL_ISO="/mnt/d/OS/micro-os/build/micro-os.iso"

echo "=== [1/4] Configuring Graphical Desktop (XFCE4 Windows-style) ==="

# 1. User .xinitrc
cat << 'EOF' > "$CHROOT/home/user/.xinitrc"
#!/bin/sh
exec startxfce4
EOF
chmod 755 "$CHROOT/home/user/.xinitrc"

# 2. Update .bashrc to launch startx on tty1 autologin
cat << 'EOF' > "$CHROOT/home/user/.bashrc"
# ~/.bashrc: executed by bash(1) for non-login shells.
export PATH=/usr/local/bin:/usr/bin:/bin:/usr/local/games:/usr/games
export PS1='\[\033[01;32m\]\u@micro-os\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '

# Autostart X11 Desktop on tty1
if [ -z "$DISPLAY" ] && [ "$(tty)" = "/dev/tty1" ]; then
    exec startx
fi
EOF

# 3. Create Desktop Shortcuts Directory
mkdir -p "$CHROOT/home/user/Desktop"

# Shortcut 1: Producer-Consumer Performance Research Lab
cat << 'EOF' > "$CHROOT/home/user/Desktop/pclab.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=Producer-Consumer Research Lab
Comment=Academic Research Lab - Concurrency Performance
Exec=xfce4-terminal --title="Producer-Consumer Performance Research Lab" --geometry=100x30 -e "/usr/local/bin/pclab"
Icon=utilities-system-monitor
Path=/home/user
Terminal=false
StartupNotify=true
EOF

# Shortcut 2: Micro OS All-in-One Dashboard
cat << 'EOF' > "$CHROOT/home/user/Desktop/dashboard.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=Micro OS Dashboard
Comment=System Overview and Feature Launcher
Exec=xfce4-terminal --title="Micro OS Control Panel" --geometry=95x28 -e "/usr/local/bin/micro-os-launcher"
Icon=preferences-desktop
Path=/home/user
Terminal=false
StartupNotify=true
EOF

# Shortcut 3: Terminal (Bash Shell)
cat << 'EOF' > "$CHROOT/home/user/Desktop/terminal.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=Terminal (Bash)
Comment=Open command prompt
Exec=xfce4-terminal
Icon=utilities-terminal
Path=/home/user
Terminal=false
StartupNotify=true
EOF

# Shortcut 4: WiFi & Network Settings
cat << 'EOF' > "$CHROOT/home/user/Desktop/network.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=WiFi & Network Settings
Comment=Connect to WiFi or configure Ethernet
Exec=xfce4-terminal --title="Network Configuration" -e "nmtui"
Icon=network-wireless
Path=/home/user
Terminal=false
StartupNotify=true
EOF

# Shortcut 5: APT Package Manager
cat << 'EOF' > "$CHROOT/home/user/Desktop/apt_manager.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=APT Package Manager
Comment=Debian Aptitude Package Manager
Exec=xfce4-terminal --title="APT Package Manager" -e "sudo aptitude"
Icon=system-software-install
Path=/home/user
Terminal=false
StartupNotify=true
EOF

# Shortcut 6: Boot Time Performance
cat << 'EOF' > "$CHROOT/home/user/Desktop/boot_analysis.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=Boot Time Benchmark
Comment=View startup durations and systemd analysis
Exec=xfce4-terminal --title="Boot Performance Analysis" --hold -e "systemd-analyze blame"
Icon=preferences-system
Path=/home/user
Terminal=false
StartupNotify=true
EOF

# Shortcut 7: File Manager
cat << 'EOF' > "$CHROOT/home/user/Desktop/files.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=File Manager
Comment=Browse files and folders
Exec=thunar
Icon=system-file-manager
Path=/home/user
Terminal=false
StartupNotify=true
EOF

chmod +x "$CHROOT/home/user/Desktop/"*.desktop
chown -R 1000:1000 "$CHROOT/home/user"

echo "=== [2/4] Cleaning Package Cache in Chroot ==="
chroot "$CHROOT" bash -c '
apt-get clean
rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
'

echo "=== [3/4] Unmounting Chroot Virtual Filesystems ==="
umount -lf "$CHROOT/dev/pts" 2>/dev/null || true
umount -lf "$CHROOT/dev" 2>/dev/null || true
umount -lf "$CHROOT/proc" 2>/dev/null || true
umount -lf "$CHROOT/sys" 2>/dev/null || true

echo "=== [4/4] Repacking SquashFS and Hybrid ISO ==="
rm -f "$ISODIR/live/filesystem.squashfs"
mksquashfs "$CHROOT" "$ISODIR/live/filesystem.squashfs" -comp xz -e boot

# Update GRUB configuration
cat << 'EOF' > "$ISODIR/boot/grub/grub.cfg"
search --set=root --file /live/vmlinuz
set default="0"
set timeout=3

menuentry "Debian Micro OS (Graphical Desktop - Laptop & VM)" {
    linux /live/vmlinuz boot=live quiet splash components
    initrd /live/initrd.img
}

menuentry "Debian Micro OS (Safe Mode - Text Console)" {
    linux /live/vmlinuz boot=live components single
    initrd /live/initrd.img
}
EOF

rm -f "$FINAL_ISO"
grub-mkrescue -o "$FINAL_ISO" "$ISODIR"
chmod 666 "$FINAL_ISO"

ISO_SIZE=$(du -h "$FINAL_ISO" | cut -f1)
echo "=============================================================================="
echo " [SUCCESS] Graphical Micro OS ISO Built: $FINAL_ISO ($ISO_SIZE)"
echo " Windows-style Desktop Environment (XFCE) with Desktop Icons & Start Menu"
echo "=============================================================================="
