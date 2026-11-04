#!/usr/bin/env bash
set -eoux pipefail

echo "=== 03: Installing CLI System Tooling & I/O Optimizations ==="
dnf5 install -y \
    git tailscale \
    ntpd-rs powertop power-profiles-daemon freerdp \
    scx-scheds scx-tools scx-manager flatpak udisks2 \
    parted dosfstools exfatprogs e2fsprogs \
    fish zoxide fzf \
    fuse fuse-libs

dnf5 remove -y chrony || true

sed -i 's|SHELL=/bin/bash|SHELL=/usr/bin/fish|' /etc/default/useradd

# Ottimizzazione I/O (ADIOS) - Sintassi Origami OS con deviazione MicroSD su bfq
mkdir -p /etc/udev/rules.d
echo 'ACTION=="add|change", KERNEL=="sd[a-z]*", ATTR{queue/rotational}=="1", ATTR{queue/scheduler}="bfq"' > /etc/udev/rules.d/60-ioschedulers.rules
echo 'ACTION=="add|change", KERNEL=="sd[a-z]*", ATTR{queue/rotational}=="0", ATTR{queue/scheduler}="adios"' >> /etc/udev/rules.d/60-ioschedulers.rules
echo 'ACTION=="add|change", KERNEL=="nvme[0-9]*", ATTR{queue/rotational}=="0", ATTR{queue/scheduler}="adios"' >> /etc/udev/rules.d/60-ioschedulers.rules
echo 'ACTION=="add|change", KERNEL=="mmcblk[0-9]*", ATTR{queue/rotational}=="0", ATTR{queue/scheduler}="bfq"' >> /etc/udev/rules.d/60-ioschedulers.rules
