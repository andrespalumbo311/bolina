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
systemctl --global enable easyeffects.service || true
systemctl disable rpm-ostreed-automatic.timer || true

# Configurazione ID immagine per prevenire kernel panic da ibernazione obsoleta post-upgrade
# Pulizia preventiva da byte nulli e righe IMAGE_ID preesistenti in /usr/lib/os-release
sed -i '/^IMAGE_ID=/d' /usr/lib/os-release || true
tmp_os_rel=$(mktemp)
tr -d '\000' < /usr/lib/os-release > "$tmp_os_rel"
cat "$tmp_os_rel" > /usr/lib/os-release
rm -f "$tmp_os_rel"

if [ "${SHA_HEAD_SHORT:-unknown}" != "unknown" ] && [ -n "${SHA_HEAD_SHORT:-}" ]; then
    echo "IMAGE_ID=\"bolina-${SHA_HEAD_SHORT}\"" >> /usr/lib/os-release
else
    echo "IMAGE_ID=\"bolina-$(date +%Y%m%d)\"" >> /usr/lib/os-release
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

# Pulizia preventiva in-place di /run e /tmp da artefatti di build per bootc lint e rechunker
rm -rf /run/* /run/.* /tmp/* /tmp/.* 2>/dev/null || true

# Pulizia residui di build di DNF5 e cache temporanee per massimizzare la determinazione dell'immagine
rm -rf /var/lib/dnf \
       /var/log/dnf* \
       /usr/lib/sysimage/libdnf5/transaction_history* \
       /usr/lib/fontconfig/cache/* 2>/dev/null || true

