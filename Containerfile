# STAGE 1: Download utility custom
# Verify layer cache performance on GHCR build-cache tag
FROM ghcr.io/ublue-os/base-main:latest@sha256:a3960bb9d4766f97b653948a6f9fe7683b18797e85165a4bb7926c2b0f97500f AS builder

# renovate: datasource=github-releases depName=starship/starship
ARG STARSHIP_VERSION="v1.26.0"
# renovate: datasource=github-releases depName=topgrade-rs/topgrade
ARG TOPGRADE_VERSION="v17.9.0"
# renovate: datasource=github-releases depName=ublue-os/uupd
ARG UUPD_VERSION="v1.4.0"
# renovate: datasource=github-releases depName=trifectatechfoundation/sudo-rs
ARG SUDO_RS_VERSION="v0.2.15"
# renovate: datasource=github-releases depName=uutils/coreutils
ARG COREUTILS_VERSION="0.11.0"
# renovate: datasource=github-releases depName=google-antigravity/antigravity-cli
ARG AGY_VERSION="1.1.27"

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
FROM ghcr.io/ublue-os/base-main:latest@sha256:a3960bb9d4766f97b653948a6f9fe7683b18797e85165a4bb7926c2b0f97500f

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

# Copia asset di build e configurazione nativa
COPY build_files /tmp/build_files
COPY etc /etc
COPY usr /usr

# STRATO 1: Repository COPR
RUN --mount=type=cache,dst=/var/cache --mount=type=cache,dst=/var/log \
    /tmp/build_files/01-copr.sh

# STRATO 2: Swap Kernel CachyOS
ARG KERNEL_EPOCH=1700000000
RUN --mount=type=cache,dst=/var/cache --mount=type=cache,dst=/var/log \
    SOURCE_DATE_EPOCH="${KERNEL_EPOCH}" /tmp/build_files/02-kernel.sh

# STRATO 3: Utilità CLI e System Tooling
RUN --mount=type=cache,dst=/var/cache --mount=type=cache,dst=/var/log \
    /tmp/build_files/03-cli-packages.sh

# STRATO 4: Ambiente Grafico, Wayland e Audio
RUN --mount=type=cache,dst=/var/cache --mount=type=cache,dst=/var/log \
    /tmp/build_files/04-desktop-packages.sh

# STRATO 5: Configurazione Servizi, Flatpak, os-release e Segregazione Utenti
# NOTA: ARG SHA_HEAD_SHORT dichiarata qui per non invalidare gli strati pesanti precedenti ad ogni commit
ARG SHA_HEAD_SHORT=unknown
RUN SHA_HEAD_SHORT="${SHA_HEAD_SHORT}" /tmp/build_files/05-system-config.sh && \
    rm -rf /tmp/build_files

### LINTING
RUN bootc container lint
