#!/usr/bin/env bash
set -eoux pipefail

echo "=== 05: Configuring Services, Skel, Flatpak & User Segregation ==="
mkdir -p /etc/pki/akmods/certs/

if [ -f /tmp/MOK.der ]; then
    cp /tmp/MOK.der /etc/pki/akmods/certs/public_key.der
fi

if id "greetd" &>/dev/null; then
    usermod -aG video,render,tty,input greetd || true
    chown -R greetd:greetd /etc/greetd/dms-greeter || true
    chown -R greetd:greetd /etc/greetd/niri || true
fi

chmod +x /etc/skel/.config/niri/scripts/*.sh || true
dconf update || true

systemctl enable tailscaled.service greetd.service uupd.timer power-profiles-daemon.service bluetooth.service bluetooth-poweroff.service || true
systemctl --global enable easyeffects.service taildrop-auto-receive.service tailscale-systray.service || true
systemctl disable rpm-ostreed-automatic.timer || true

flatpak remote-delete valent || true
flatpak remote-add --if-not-exists --system valent /etc/flatpak/remotes.d/valent.flatpakrepo || true
flatpak update --appstream valent || true

# Configurazione ID immagine per prevenire kernel panic da ibernazione obsoleta post-upgrade
if [ "${SHA_HEAD_SHORT:-unknown}" != "unknown" ] && [ -n "${SHA_HEAD_SHORT:-}" ]; then
    echo "IMAGE_ID=\"myublue-${SHA_HEAD_SHORT}\"" >> /usr/lib/os-release
else
    echo "IMAGE_ID=\"myublue-$(date +%Y%m%d)\"" >> /usr/lib/os-release
fi

# Segregazione passwd/group in usr/lib per prevenire conflitti ostree/bootc ad ogni upgrade
if [ -f /etc/passwd ]; then
    out=$(grep -v "root" /etc/passwd) || true
    if [ -n "$out" ]; then
        echo "$out" >> /usr/lib/passwd
        echo "root:x:0:0:root:/root:/bin/bash" > /etc/passwd
    fi
fi

if [ -f /etc/group ]; then
    out=$(grep -v "root\|wheel" /etc/group) || true
    if [ -n "$out" ]; then
        echo "$out" >> /usr/lib/group
        echo "root:x:0:" > /etc/group
        echo "wheel:x:10:" >> /etc/group
    fi
fi

dnf5 clean all

