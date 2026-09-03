#!/usr/bin/env bash
set -eoux pipefail

echo "=== 02: Swapping Kernel to CachyOS LTO & SecureBoot Signing ==="
mkdir -p /etc/kernel
echo "initrd_generator=none" > /etc/kernel/install.conf

# Rimozione kernel stock e pulizia moduli per evitare duplicati nel linting
dnf5 -y --setopt=protected_packages= remove kernel kernel-core kernel-modules kernel-modules-extra || true
rm -rf /usr/lib/modules/*

# Mocking GRUB per evitare errori in ambiente container durante l'installazione del kernel
mkdir -p /tmp/bin
echo -e '#!/bin/sh\nexit 0' > /tmp/bin/grub2-probe
echo -e '#!/bin/sh\nexit 0' > /tmp/bin/grub2-editenv
chmod +x /tmp/bin/grub2-probe /tmp/bin/grub2-editenv

# Installazione Kernel CachyOS con PATH override per i mock
PATH=/tmp/bin:$PATH dnf5 -y --setopt=protected_packages= install \
    kernel-cachyos-lto sbsigntools libfaketime \
    --allowerasing

rm -f /etc/kernel/install.conf

# Configurazione SUDO-RS (Source Sovereignty)
ln -sf /usr/bin/sudo-rs /usr/bin/sudo
ln -sf /usr/bin/visudo-rs /usr/bin/visudo
ln -sf /usr/bin/sudoedit-rs /usr/bin/sudoedit
chown root:root /usr/bin/sudo-rs /usr/bin/su-rs || true
chmod 4755 /usr/bin/sudo-rs /usr/bin/su-rs || true

KVER=$(ls /lib/modules | grep cachyos | head -n 1)
depmod -a $KVER

# Generatione reproducible dell'initramfs
export SOURCE_DATE_EPOCH=${SOURCE_DATE_EPOCH:-1700000000}
if [ ! -f /lib/modules/$KVER/initramfs.img ]; then
    dracut --kver $KVER --no-hostonly --reproducible --add ostree --force /lib/modules/$KVER/initramfs.img
    chmod 0600 /lib/modules/$KVER/initramfs.img
fi

# Firma SecureBoot deterministica tramite faketime
if command -v faketime &>/dev/null; then
    faketime -f "@${SOURCE_DATE_EPOCH}" sbsign --key /run/secrets/MOK_key --cert /run/secrets/MOK_crt --output /lib/modules/$KVER/vmlinuz /lib/modules/$KVER/vmlinuz
else
    sbsign --key /run/secrets/MOK_key --cert /run/secrets/MOK_crt --output /lib/modules/$KVER/vmlinuz /lib/modules/$KVER/vmlinuz
fi

# Pulizia post-installazione per bootc lint
rm -rf /boot/* /tmp/bin
dnf5 -y copr disable bieszczaders/kernel-cachyos-lto || true
dnf5 -y copr disable dejan/rpms || true
