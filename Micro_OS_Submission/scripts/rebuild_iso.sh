#!/bin/bash
set -e

WORKDIR="/var/tmp/micro-os-build"
CHROOT="$WORKDIR/chroot"
ISODIR="$WORKDIR/iso"
LOCAL_ISO="$WORKDIR/micro-os.iso"
FINAL_ISO_1="/mnt/d/OS/micro-os/build/micro-os.iso"

echo "=== [1/4] Cleaning Package Cache in Chroot ==="
chroot "$CHROOT" bash -c '
apt-get clean
rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
'

echo "=== [2/4] Unmounting Virtual Filesystems ==="
umount -lf "$CHROOT/dev/pts" 2>/dev/null || true
umount -lf "$CHROOT/dev" 2>/dev/null || true
umount -lf "$CHROOT/proc" 2>/dev/null || true
umount -lf "$CHROOT/sys" 2>/dev/null || true

# Ensure mountpoint directories exist
mkdir -p "$CHROOT/proc" "$CHROOT/sys" "$CHROOT/dev" "$CHROOT/run" "$CHROOT/tmp"
chmod 1777 "$CHROOT/tmp"
chmod 755 "$CHROOT/proc" "$CHROOT/sys" "$CHROOT/dev" "$CHROOT/run"

echo "=== [3/4] Compressing SquashFS (Excluding boot only) ==="
rm -f "$ISODIR/live/filesystem.squashfs"
mksquashfs "$CHROOT" "$ISODIR/live/filesystem.squashfs" -comp xz -e boot

echo "=== [4/4] Generating Hybrid Bootable ISO ==="
rm -f "$LOCAL_ISO"
grub-mkrescue -o "$LOCAL_ISO" "$ISODIR"

echo "=== Syncing New ISO to Windows Drive ==="
cp -f "$LOCAL_ISO" "$FINAL_ISO_1"
chmod 666 "$FINAL_ISO_1"
sync

SIZE1=$(du -h "$FINAL_ISO_1" | cut -f1)
echo "=============================================================================="
echo " [SUCCESS] MicroOS Live ISO Ready: $FINAL_ISO_1 ($SIZE1)"
echo "=============================================================================="
