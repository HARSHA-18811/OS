#!/bin/bash
set -e

WORKDIR="/var/tmp/micro-os-build"
CHROOT="$WORKDIR/chroot"
ISODIR="$WORKDIR/iso"
LOCAL_ISO="$WORKDIR/micro-os.iso"
FINAL_ISO="/mnt/d/OS/micro-os/build/micro-os.iso"

echo "=== [1/5] Configuring LightDM Autologin into XFCE Desktop ==="

# 1. LightDM configuration
cat << 'EOF' > "$CHROOT/etc/lightdm/lightdm.conf"
[LightDM]
run-directory=/run/lightdm

[Seat:*]
autologin-user=user
autologin-user-timeout=0
user-session=xfce
greeter-session=lightdm-gtk-greeter
EOF

# 2. Add autologin group and user
chroot "$CHROOT" bash -c '
groupadd -r autologin 2>/dev/null || true
gpasswd -a user autologin
gpasswd -a user video
gpasswd -a user input
'

# 3. Configure Xwrapper for root permissions
cat << 'EOF' > "$CHROOT/etc/X11/Xwrapper.config"
allowed_users=anybody
needs_root_rights=yes
EOF

# 4. Set systemd target to graphical.target & enable lightdm
chroot "$CHROOT" bash -c '
systemctl set-default graphical.target
systemctl enable lightdm
'
rm -rf "$CHROOT/etc/systemd/system/getty@tty1.service.d"

# 5. Clean bashrc
cat << 'EOF' > "$CHROOT/home/user/.bashrc"
export PATH=/usr/local/bin:/usr/bin:/bin:/usr/local/games:/usr/games
export PS1='\[\033[01;32m\]\u@micro-os\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '
alias pclab="/usr/local/bin/pclab"
alias research="/usr/local/bin/pclab"
EOF
chown -R 1000:1000 "$CHROOT/home/user"

echo "=== [2/5] Cleaning Package Cache in Chroot ==="
chroot "$CHROOT" bash -c '
apt-get clean
rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
'

echo "=== [3/5] Unmounting Virtual Filesystems ==="
umount -lf "$CHROOT/dev/pts" 2>/dev/null || true
umount -lf "$CHROOT/dev" 2>/dev/null || true
umount -lf "$CHROOT/proc" 2>/dev/null || true
umount -lf "$CHROOT/sys" 2>/dev/null || true

echo "=== [4/5] Compressing SquashFS (No pseudo device nodes) ==="
rm -f "$ISODIR/live/filesystem.squashfs"
mksquashfs "$CHROOT" "$ISODIR/live/filesystem.squashfs" -comp xz -e boot proc sys dev run tmp

echo "=== [5/5] Building Hybrid Bootable ISO on ext4 ==="
rm -f "$LOCAL_ISO"
grub-mkrescue -o "$LOCAL_ISO" "$ISODIR"

# Stream copy to Windows NTFS and sync
echo "--- Syncing ISO to Windows Disk ---"
cp -f "$LOCAL_ISO" "$FINAL_ISO"
chmod 666 "$FINAL_ISO"
sync

ISO_SIZE=$(du -h "$FINAL_ISO" | cut -f1)
echo "=============================================================================="
echo " [SUCCESS] Graphical Windows-like Micro OS Ready: $FINAL_ISO ($ISO_SIZE)"
echo " Direct Autologin into XFCE Desktop via LightDM Display Manager"
echo "=============================================================================="
