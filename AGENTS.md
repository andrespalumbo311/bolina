# AI Agent Guidelines & Operating Rules

This repository defines a custom cloud-native Fedora Atomic image (ublue) for a Lenovo ThinkPad X390 Yoga, built as an OCI container via GitHub Actions. Any AI agent operating in this repository must strictly adhere to these guidelines.

## Operating Context & Constraints
- **Hardware Target:** Lenovo ThinkPad X390 Yoga (Intel UHD 620, convertible touchscreen/pen).
- **Host System:** Fedora Atomic / ublue (bootc / ostree).
- **Filesystem Immutability:** `/usr` is read-only. Never use `rpm-ostree install` or hot-patch system directories directly on the host unless explicitly ordered.
- **Persistence Model:**
  - System changes are codified in `Containerfile`, modular scripts in `build_files/`, or runtime configs in `/etc` and `/usr/lib/tmpfiles.d/`.
  - Applications and user tools belong in isolated runtimes: Flatpak, Distrobox, or Homebrew.

## Mandatory Operational Rules

### 1. Default Diagnostic Mode (Read-Only)
- When investigating an issue, bug, or unexpected behavior, **stop at the diagnosis**.
- **Allowed in diagnosis:** Passive, read-only data collection (`journalctl`, `systemctl --user`, `busctl`, `cat`, `grep`, config inspection).
- **Forbidden without explicit confirmation:** Modifying files, staging git commits, applying patches, or running mutating system commands.
- Wait for explicit user authorization (e.g. "procedi a risolvere") before applying fixes.

### 2. Operational Memory Protocol ([historical-errors.md](file:///var/home/andres/Documents/myublue/docs/historical-errors.md))
- **Mandatory Consultation:** Before planning or applying architectural changes (compositor, portals, audio, systemd, bootc, pam, kernel/dracut), consult the living library in [docs/historical-errors.md](file:///var/home/andres/Documents/myublue/docs/historical-errors.md) to avoid known failure modes.
- **Self-Update Post-Fix:** Once a fix is verified and working, document the new error class or refine an existing preventive pattern in [docs/historical-errors.md](file:///var/home/andres/Documents/myublue/docs/historical-errors.md).
- **Technical Abstraction:** Always state the root cause and preventive action in general, abstract terms (e.g. D-Bus portal activation, FUSE unmount race, OCI layer non-determinism) so it serves as a reusable pattern.

### 3. Live Session Validation
- For any changes affecting Wayland sessions (Niri), greeters (`greetd`), portals (`xdg-desktop-portal`), audio (PipeWire/WirePlumber), or user daemons, validate the runtime state live (e.g. with `busctl`, `systemctl --user status`, `pw-cli`, `gsettings`) to verify interfaces and sockets before considering the task complete.

### 4. Build Pipeline & Package Integrity
- Verify exact package names (e.g. with `dnf list` or repo queries) before updating `build_files/`.
- Maintain deterministic build layers: pin versions using `ARG` with Renovate annotations, avoid `date` timestamps, and preserve `.keep` files in empty directories.
