#!/bin/bash
set -ex

WORKDIR="/var/tmp/micro-os-build"
CHROOT="$WORKDIR/chroot"
ISODIR="$WORKDIR/iso"
LOCAL_ISO="$WORKDIR/micro-os.iso"
FINAL_ISO="/mnt/d/OS/micro-os/build/micro-os.iso"

echo "=== [1/6] Cleaning up any mounts ==="
umount -lf "$CHROOT/dev/pts" 2>/dev/null || true
umount -lf "$CHROOT/dev" 2>/dev/null || true
umount -lf "$CHROOT/proc" 2>/dev/null || true
umount -lf "$CHROOT/sys" 2>/dev/null || true

echo "=== [2/6] Ensuring static /dev nodes and mount points exist in chroot ==="
mkdir -p "$CHROOT/proc" "$CHROOT/sys" "$CHROOT/run" "$CHROOT/tmp" "$CHROOT/dev"
chmod 755 "$CHROOT/proc" "$CHROOT/sys" "$CHROOT/run"
chmod 1777 "$CHROOT/tmp"
rm -rf "$CHROOT/run"/* "$CHROOT/tmp"/*

# Static dev nodes
[ -e "$CHROOT/dev/console" ] || mknod -m 600 "$CHROOT/dev/console" c 5 1
[ -e "$CHROOT/dev/null" ] || mknod -m 666 "$CHROOT/dev/null" c 1 3
[ -e "$CHROOT/dev/zero" ] || mknod -m 666 "$CHROOT/dev/zero" c 1 5
[ -e "$CHROOT/dev/tty" ] || mknod -m 666 "$CHROOT/dev/tty" c 5 0
[ -e "$CHROOT/dev/ptmx" ] || mknod -m 666 "$CHROOT/dev/ptmx" c 5 2
[ -e "$CHROOT/dev/urandom" ] || mknod -m 666 "$CHROOT/dev/urandom" c 1 9
[ -e "$CHROOT/dev/random" ] || mknod -m 666 "$CHROOT/dev/random" c 1 8
mkdir -p "$CHROOT/dev/pts" "$CHROOT/dev/shm"
chmod 755 "$CHROOT/dev/pts"
chmod 1777 "$CHROOT/dev/shm"

echo "=== [3/6] Setting up Snake Game Desktop Entry & Symlinks ==="
ln -sf /usr/games/nsnake "$CHROOT/usr/local/bin/snake"
ln -sf /usr/games/nsnake "$CHROOT/usr/bin/snake"

mkdir -p "$CHROOT/home/user/Desktop" "$CHROOT/etc/skel/Desktop"

cat << 'SNAKE_EOF' > "$CHROOT/home/user/Desktop/snake.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=Snake Game
Comment=Classic Retro Arcade Snake Game
Exec=xfce4-terminal --title="Snake Game - Retro Arcade" --geometry=80x25 -e "/usr/games/nsnake"
Icon=applications-games
Path=/home/user
Terminal=false
StartupNotify=true
Categories=Game;
SNAKE_EOF

cp -f "$CHROOT/home/user/Desktop/snake.desktop" "$CHROOT/etc/skel/Desktop/snake.desktop"
chmod +x "$CHROOT/home/user/Desktop/snake.desktop" "$CHROOT/etc/skel/Desktop/snake.desktop"

# Fix ownership
chown -R 1000:1000 "$CHROOT/home/user"
chmod -R 755 "$CHROOT/home/user/Desktop"

# Reset machine-id
truncate -s 0 "$CHROOT/etc/machine-id"

echo "=== [4/6] Repacking SquashFS (preserving dev, proc, sys, run, tmp) ==="
rm -f "$ISODIR/live/filesystem.squashfs"
mksquashfs "$CHROOT" "$ISODIR/live/filesystem.squashfs" -comp xz -e boot

echo "=== [5/6] Verifying SquashFS contains dev/console, proc, sys, run ==="
unsquashfs -l "$ISODIR/live/filesystem.squashfs" | grep -E "squashfs-root/(dev/console|proc|sys|run)$"

echo "=== [6/6] Generating ISO and syncing to Windows D: drive ==="
rm -f "$LOCAL_ISO"
grub-mkrescue -o "$LOCAL_ISO" "$ISODIR"

mkdir -p /mnt/d/OS/micro-os/build
rm -f "$FINAL_ISO"
cp -f "$LOCAL_ISO" "$FINAL_ISO"
chmod 666 "$FINAL_ISO"
sync

echo "=== [SUCCESS] Bootable GUI ISO with Snake Game is Ready at $FINAL_ISO ==="
ls -lh "$FINAL_ISO"
