#!/usr/bin/env bash
set -eoux pipefail

echo "=== 04: Installing Desktop Environment, Wayland & Audio Utilities ==="
dnf5 install -y \
    niri dms dms-greeter \
    xdg-desktop-portal-wlr xdg-desktop-portal-hyprland \
    greetd fprintd fprintd-pam \
    brightnessctl grim slurp \
    pavucontrol kitty pamixer \
    libva-intel-media-driver libva-utils \
    scx-manager python3-pyqt6 \
    easyeffects lsp-plugins \
    nautilus gvfs-mtp gvfs-smb \
    gnome-keyring gnome-keyring-pam \
    cups-pk-helper kf6-kimageformats qt6-qtimageformats \
    accountsservice \
    jstest-gtk joystick \
    xdg-desktop-portal-gnome xdg-desktop-portal-gtk xdg-user-dirs-gtk

dnf5 remove -y swaybg swaylock swayidle cliphist fuzzel mako dunst || true
