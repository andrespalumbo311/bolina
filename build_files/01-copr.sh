#!/usr/bin/env bash
set -eoux pipefail

echo "=== 01: Enabling COPR Repositories ==="
dnf5 -y copr enable yalter/niri
dnf5 -y copr enable avengemedia/dms
dnf5 -y copr enable avengemedia/danklinux
dnf5 -y copr enable bieszczaders/kernel-cachyos-addons
dnf5 -y copr enable bieszczaders/kernel-cachyos-lto
dnf5 -y copr enable dejan/rpms
dnf5 clean all
