#!/usr/bin/env bash
set -eoux pipefail

echo "=== 01: Enabling COPR Repositories ==="
dnf5 -y copr enable yalter/niri
dnf5 -y copr enable duxlabs/dms
dnf5 -y copr enable duxlabs/dux-repo
dnf5 -y copr enable bieszczaders/kernel-cachyos-addons
dnf5 -y copr enable bieszczaders/kernel-cachyos-lto
