#!/usr/bin/env bash
set -e

BUILD_DIR="/var/tmp/micro-os-build"
CHROOT="${BUILD_DIR}/chroot"
LIVE_DIR="${BUILD_DIR}/iso"
FINAL_ISO="/mnt/d/OS/micro-os/build/micro-os.iso"

echo "==> [1/3] Ensuring static device nodes in chroot..."
mknod -m 600 "${CHROOT}/dev/console" c 5 1 2>/dev/null || true
mknod -m 666 "${CHROOT}/dev/null" c 1 3 2>/dev/null || true
mkdir -p "${CHROOT}/proc" "${CHROOT}/sys" "${CHROOT}/dev" "${CHROOT}/run" "${CHROOT}/tmp"
chmod 1777 "${CHROOT}/tmp"

echo "==> [2/3] Compressing SquashFS filesystem (XZ)..."
rm -f "${LIVE_DIR}/live/filesystem.squashfs"
mksquashfs "${CHROOT}" "${LIVE_DIR}/live/filesystem.squashfs" -comp xz -e boot

echo "==> [3/3] Generating Bootable Hybrid ISO..."
if [ -f "${LIVE_DIR}/isolinux/isolinux.bin" ]; then
    xorriso -as mkisofs -r -V "MicroOS" \
        -J -l -b isolinux/isolinux.bin \
        -c isolinux/boot.cat \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        -isohybrid-mbr /usr/lib/ISOLINUX/isohdpfx.bin \
        -o "${FINAL_ISO}" "${LIVE_DIR}"
elif [ -d "${LIVE_DIR}/boot/grub" ]; then
    grub-mkrescue -o "${FINAL_ISO}" "${LIVE_DIR}"
fi

chmod 666 "${FINAL_ISO}"
ls -lh "${FINAL_ISO}"
echo "==> ISO build complete!"
