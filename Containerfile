# STAGE 1: Download utility custom
FROM ghcr.io/ublue-os/base-main:latest AS builder

ARG STARSHIP_VERSION="v1.26.0"
ARG TOPGRADE_VERSION="v17.9.0"
ARG UUPD_VERSION="v1.4.0"
ARG SUDO_RS_VERSION="v0.2.14"
ARG COREUTILS_VERSION="0.10.0"
ARG AGY_VERSION="1.1.11"

# Installazione dipendenze per download e verifica
RUN dnf install -y curl jq tar xz

# Download e verifica utility (Starship, Topgrade, uupd, sudo-rs, coreutils)
RUN mkdir -p /tmp/verify /tmp/bin && \
    # Starship
    STARSHIP_ASSETS=$(curl -fsSL https://api.github.com/repos/starship/starship/releases/tags/${STARSHIP_VERSION}) && \
    STARSHIP_URL=$(echo "$STARSHIP_ASSETS" | jq -r '.assets[] | select(.name == "starship-x86_64-unknown-linux-musl.tar.gz") | .browser_download_url') && \
    STARSHIP_SHA=$(echo "$STARSHIP_ASSETS" | jq -r '.assets[] | select(.name == "starship-x86_64-unknown-linux-musl.tar.gz") | .digest' | cut -d: -f2) && \
    curl -fsSL "$STARSHIP_URL" -o /tmp/verify/starship.tar.gz && \
    echo "$STARSHIP_SHA  /tmp/verify/starship.tar.gz" | sha256sum --check && \
    tar -xz -C /tmp/bin -f /tmp/verify/starship.tar.gz starship && \
    # Topgrade
    TOPGRADE_ASSETS=$(curl -fsSL https://api.github.com/repos/topgrade-rs/topgrade/releases/tags/${TOPGRADE_VERSION}) && \
    TOPGRADE_URL=$(echo "$TOPGRADE_ASSETS" | jq -r '.assets[] | select(.name | contains("x86_64-unknown-linux-musl.tar.gz")) | .browser_download_url') && \
    TOPGRADE_SHA=$(echo "$TOPGRADE_ASSETS" | jq -r '.assets[] | select(.name | contains("x86_64-unknown-linux-musl.tar.gz")) | .digest' | cut -d: -f2) && \
    curl -fsSL "$TOPGRADE_URL" -o /tmp/verify/topgrade.tar.gz && \
    echo "$TOPGRADE_SHA  /tmp/verify/topgrade.tar.gz" | sha256sum --check && \
    tar -xz -C /tmp/bin -f /tmp/verify/topgrade.tar.gz topgrade && \
    # uupd
    UUPD_ASSETS=$(curl -fsSL https://api.github.com/repos/ublue-os/uupd/releases/tags/${UUPD_VERSION}) && \
    UUPD_URL=$(echo "$UUPD_ASSETS" | jq -r '.assets[] | select(.name == "uupd_Linux_x86_64.tar.gz") | .browser_download_url') && \
    UUPD_SHA=$(echo "$UUPD_ASSETS" | jq -r '.assets[] | select(.name == "uupd_Linux_x86_64.tar.gz") | .digest' | cut -d: -f2) && \
    curl -fsSL "$UUPD_URL" -o /tmp/verify/uupd.tar.gz && \
    echo "$UUPD_SHA  /tmp/verify/uupd.tar.gz" | sha256sum --check && \
    tar -xz -C /tmp/bin -f /tmp/verify/uupd.tar.gz uupd && \
    # sudo-rs
    SUDO_RS_ASSETS=$(curl -fsSL https://api.github.com/repos/trifectatechfoundation/sudo-rs/releases/tags/${SUDO_RS_VERSION}) && \
    SUDO_RS_URL=$(echo "$SUDO_RS_ASSETS" | jq -r '.assets[] | select(.name | startswith("sudo-") and endswith(".tar.gz")) | .browser_download_url' | head -n 1) && \
    SUDO_RS_SHA=$(echo "$SUDO_RS_ASSETS" | jq -r '.assets[] | select(.name | startswith("sudo-") and endswith(".tar.gz")) | .digest' | cut -d: -f2 | head -n 1) && \
    curl -fsSL "$SUDO_RS_URL" -o /tmp/verify/sudo.tar.gz && \
    echo "$SUDO_RS_SHA  /tmp/verify/sudo.tar.gz" | sha256sum --check && \
    tar -xz -C /tmp/bin -f /tmp/verify/sudo.tar.gz --strip-components=1 && \
    SU_RS_URL=$(echo "$SUDO_RS_ASSETS" | jq -r '.assets[] | select(.name | startswith("su-") and endswith(".tar.gz")) | .browser_download_url' | head -n 1) && \
    SU_RS_SHA=$(echo "$SUDO_RS_ASSETS" | jq -r '.assets[] | select(.name | startswith("su-") and endswith(".tar.gz")) | .digest' | cut -d: -f2 | head -n 1) && \
    curl -fsSL "$SU_RS_URL" -o /tmp/verify/su.tar.gz && \
    echo "$SU_RS_SHA  /tmp/verify/su.tar.gz" | sha256sum --check && \
    tar -xz -C /tmp/bin -f /tmp/verify/su.tar.gz --strip-components=1 && \
    # coreutils (uutils)
    COREUTILS_ASSETS=$(curl -fsSL https://api.github.com/repos/uutils/coreutils/releases/tags/${COREUTILS_VERSION}) && \
    COREUTILS_URL=$(echo "$COREUTILS_ASSETS" | jq -r '.assets[] | select(.name | contains("x86_64-unknown-linux-gnu.tar.gz")) | .browser_download_url') && \
    COREUTILS_SHA=$(echo "$COREUTILS_ASSETS" | jq -r '.assets[] | select(.name | contains("x86_64-unknown-linux-gnu.tar.gz")) | .digest' | cut -d: -f2) && \
    curl -fsSL "$COREUTILS_URL" -o /tmp/verify/coreutils.tar.gz && \
    echo "$COREUTILS_SHA  /tmp/verify/coreutils.tar.gz" | sha256sum --check && \
    tar -xz -C /tmp/bin -f /tmp/verify/coreutils.tar.gz --strip-components=1 && \
    # antigravity-cli
    AGY_ASSETS=$(curl -fsSL https://api.github.com/repos/google-antigravity/antigravity-cli/releases/tags/${AGY_VERSION}) && \
    AGY_URL=$(echo "$AGY_ASSETS" | jq -r '.assets[] | select(.name == "agy_cli_linux_x64.tar.gz") | .browser_download_url') && \
    AGY_SHA=$(echo "$AGY_ASSETS" | jq -r '.assets[] | select(.name == "agy_cli_linux_x64.tar.gz") | .digest' | cut -d: -f2) && \
    curl -fsSL "$AGY_URL" -o /tmp/verify/agy.tar.gz && \
    echo "$AGY_SHA  /tmp/verify/agy.tar.gz" | sha256sum --check && \
    tar -xz -C /tmp/bin -f /tmp/verify/agy.tar.gz antigravity && \
    chmod +x /tmp/bin/* && \
    rm -rf /tmp/verify

# Download JetBrains Mono Nerd Font
RUN mkdir -p /tmp/verify /tmp/fonts && \
    curl -fsSL https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.tar.xz -o /tmp/verify/JetBrainsMono.tar.xz && \
    tar -xf /tmp/verify/JetBrainsMono.tar.xz -C /tmp/fonts && \
    rm -rf /tmp/verify

# STAGE 2: Immagine Finale
FROM ghcr.io/ublue-os/base-main:latest

ARG SHA_HEAD_SHORT=unknown
ARG SOURCE_DATE_EPOCH=1700000000

# Copia dei binari custom dallo stage di build
COPY --from=builder /tmp/bin/starship /usr/bin/starship
COPY --from=builder /tmp/bin/topgrade /usr/bin/topgrade
COPY --from=builder /tmp/bin/uupd /usr/bin/uupd
COPY --from=builder /tmp/bin/sudo /usr/bin/sudo-rs
COPY --from=builder /tmp/bin/visudo /usr/bin/visudo-rs
COPY --from=builder /tmp/bin/sudoedit /usr/bin/sudoedit-rs
COPY --from=builder /tmp/bin/su /usr/bin/su-rs
COPY --from=builder /tmp/bin/coreutils /usr/bin/uutils-coreutils
COPY --from=builder /tmp/bin/antigravity /usr/bin/antigravity
COPY --from=builder /tmp/fonts /usr/share/fonts/JetBrainsMono

# STRATO 1: Repository COPR (Manteniamo per ananicy-cpp e altri tool)
RUN --mount=type=cache,dst=/var/cache --mount=type=cache,dst=/var/log \
    dnf5 -y copr enable yalter/niri && \
    dnf5 -y copr enable avengemedia/dms && \
    dnf5 -y copr enable avengemedia/danklinux && \
    dnf5 -y copr enable bieszczaders/kernel-cachyos-addons && \
    dnf5 -y copr enable bieszczaders/kernel-cachyos-lto && \
    dnf5 -y copr enable dejan/rpms && \
    dnf5 clean all

# SWAP KERNEL E UTILITY RUST + FIRMA SECUREBOOT
RUN --mount=type=secret,id=MOK_key \
    --mount=type=secret,id=MOK_crt \
    --mount=type=cache,dst=/var/cache --mount=type=cache,dst=/var/log \
    mkdir -p /etc/kernel && \
    echo "initrd_generator=none" > /etc/kernel/install.conf && \
    # Rimozione kernel stock e pulizia moduli per evitare duplicati nel linting
    dnf5 -y --setopt=protected_packages= remove kernel kernel-core kernel-modules kernel-modules-extra && \
    rm -rf /usr/lib/modules/* && \
    # Mocking GRUB per evitare errori in ambiente container durante l'installazione del kernel
    mkdir -p /tmp/bin && \
    echo -e '#!/bin/sh\nexit 0' > /tmp/bin/grub2-probe && \
    echo -e '#!/bin/sh\nexit 0' > /tmp/bin/grub2-editenv && \
    chmod +x /tmp/bin/grub2-probe /tmp/bin/grub2-editenv && \
    # Installazione Kernel CachyOS con PATH override per i mock
    PATH=/tmp/bin:$PATH dnf5 -y --setopt=protected_packages= install \
        kernel-cachyos-lto sbsigntools \
        --allowerasing && \
    rm /etc/kernel/install.conf && \
    # Configurazione SUDO-RS (Source Sovereignty)
    ln -sf /usr/bin/sudo-rs /usr/bin/sudo && \
    ln -sf /usr/bin/visudo-rs /usr/bin/visudo && \
    ln -sf /usr/bin/sudoedit-rs /usr/bin/sudoedit && \
    chown root:root /usr/bin/sudo-rs /usr/bin/su-rs && \
    chmod 4755 /usr/bin/sudo-rs /usr/bin/su-rs && \
    KVER=$(ls /lib/modules | grep cachyos | head -n 1) && \
    depmod -a $KVER && \
    if [ ! -f /lib/modules/$KVER/initramfs.img ]; then \
        export SOURCE_DATE_EPOCH=${SOURCE_DATE_EPOCH:-1700000000}; \
        dracut --kver $KVER --no-hostonly --reproducible --add ostree --force /lib/modules/$KVER/initramfs.img; \
        chmod 0600 /lib/modules/$KVER/initramfs.img; \
    fi && \
    sbsign --key /run/secrets/MOK_key --cert /run/secrets/MOK_crt --output /lib/modules/$KVER/vmlinuz /lib/modules/$KVER/vmlinuz && \
    # Pulizia post-installazione per bootc lint
    rm -rf /boot/* /tmp/bin && \
    setsebool -P domain_kernel_load_modules on && \
    dnf5 -y copr disable bieszczaders/kernel-cachyos-lto && \
    dnf5 -y copr disable dejan/rpms && \
    dnf5 clean all


# STRATO 2: Utilità CLI e System Tooling
RUN --mount=type=cache,dst=/var/cache --mount=type=cache,dst=/var/log \
    dnf5 install -y \
    git tailscale \
    inotify-tools powertop power-profiles-daemon freerdp \
    scx-scheds scx-tools scx-manager flatpak udisks2 \
    python3-pyqt6 \
    parted dosfstools exfatprogs e2fsprogs \
    fish zoxide fzf \
    fuse fuse-libs && \
    sed -i 's|SHELL=/bin/bash|SHELL=/usr/bin/fish|' /etc/default/useradd && \
    # Ottimizzazione I/O (ADIOS) - Sintassi Origami OS con deviazione MicroSD su bfq
    echo 'ACTION=="add|change", KERNEL=="sd[a-z]*", ATTR{queue/rotational}=="1", ATTR{queue/scheduler}="bfq"' > /etc/udev/rules.d/60-ioschedulers.rules && \
    echo 'ACTION=="add|change", KERNEL=="sd[a-z]*", ATTR{queue/rotational}=="0", ATTR{queue/scheduler}="adios"' >> /etc/udev/rules.d/60-ioschedulers.rules && \
    echo 'ACTION=="add|change", KERNEL=="nvme[0-9]*", ATTR{queue/rotational}=="0", ATTR{queue/scheduler}="adios"' >> /etc/udev/rules.d/60-ioschedulers.rules && \
    echo 'ACTION=="add|change", KERNEL=="mmcblk[0-9]*", ATTR{queue/rotational}=="0", ATTR{queue/scheduler}="bfq"' >> /etc/udev/rules.d/60-ioschedulers.rules && \
    dnf5 clean all

# Installazione plugin Bass (per compatibilità script Bash in Fish)
RUN git clone https://github.com/edc/bass.git /tmp/bass && \
    mkdir -p /usr/share/fish/vendor_functions.d && \
    cp /tmp/bass/functions/bass.fish /usr/share/fish/vendor_functions.d/ && \
    rm -rf /tmp/bass

# STRATO 3: Ambiente Grafico e Utility
RUN --mount=type=cache,dst=/var/cache --mount=type=cache,dst=/var/log \
    dnf5 install -y \
    niri dms dms-greeter \
    xdg-desktop-portal-wlr \
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
    xdg-desktop-portal-gnome xdg-desktop-portal-gtk xdg-user-dirs-gtk && \
    dnf5 remove -y swaybg swaylock swayidle cliphist fuzzel mako dunst || true && \
    dnf5 clean all

# STRATO 4: Configurazione servizi e finalizzazione
RUN mkdir -p /etc/pki/akmods/certs/
COPY etc /etc
COPY usr /usr
COPY MOK.der /etc/pki/akmods/certs/public_key.der
RUN if id "greetd" &>/dev/null; then \
    usermod -aG video,render,tty,input greetd && \
    chown -R greetd:greetd /etc/greetd/dms-greeter && \
    chown -R greetd:greetd /etc/greetd/niri; \
fi && \
chmod +x /etc/skel/.config/niri/scripts/*.sh && \
    dconf update && \
    systemctl enable tailscaled.service greetd.service uupd.timer scx.service scx_loader.service power-profiles-daemon.service bluetooth.service bluetooth-poweroff.service && \
    systemctl --global enable easyeffects.service taildrop-auto-receive.service tailscale-systray.service && \
    systemctl disable rpm-ostreed-automatic.timer

# STRATO 5: Inizializzazione Flatpak, Valent e Finalizzazione
RUN flatpak remote-delete valent || true && \
    flatpak remote-add --if-not-exists --system valent /etc/flatpak/remotes.d/valent.flatpakrepo && \
    flatpak update --appstream valent && \
    # Configurazione ID immagine per prevenire kernel panic da ibernazione obsoleta post-upgrade
    if [ "$SHA_HEAD_SHORT" != "unknown" ] && [ -n "$SHA_HEAD_SHORT" ]; then \
        echo "IMAGE_ID=\"myublue-${SHA_HEAD_SHORT}\"" >> /usr/lib/os-release; \
    else \
        echo "IMAGE_ID=\"myublue-\$(date +%Y%m%d)\"" >> /usr/lib/os-release; \
    fi && \
    # Segregazione passwd/group in usr/lib per prevenire conflitti ostree/bootc ad ogni upgrade
    if [ -f /etc/passwd ]; then \
        out=$(grep -v "root" /etc/passwd) || true; \
        if [ -n "$out" ]; then \
            echo "$out" >> /usr/lib/passwd; \
            echo "root:x:0:0:root:/root:/bin/bash" > /etc/passwd; \
        fi; \
    fi && \
    if [ -f /etc/group ]; then \
        out=$(grep -v "root\|wheel" /etc/group) || true; \
        if [ -n "$out" ]; then \
            echo "$out" >> /usr/lib/group; \
            echo "root:x:0:" > /etc/group; \
            echo "wheel:x:10:" >> /etc/group; \
        fi; \
    fi

### LINTING
RUN bootc container lint

