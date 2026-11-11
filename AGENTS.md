# AGENTS.md — Bolina Base OS Operational Playbook

Playbook for the public, hardware-agnostic Fedora Atomic base image (`bolina`).

## 1. Scope & Architectural Reference
- **Scope**: Generic cloud-native bootc base image (Niri, DMS, sudo-rs, CachyOS kernel). All device-specific hardware configs belong strictly downstream in `bolina-fleet`.
- **Architectural Reference**: Core architecture, package layout, directory trees, and Rust tooling matrices reside in [`docs/architecture.md`](file:///var/home/andres/Documents/myublue/bolina/docs/architecture.md).

---

## 2. Operational Rules for Autonomous Agents

### Diagnostic Mode (Read-Only by Default)
- Stop at passive diagnosis (`journalctl`, `systemctl --user`, `busctl`, configuration review).
- Do not modify files, stage git commits, or run mutating commands without explicit user authorization.

### Operational Memory Protocol
- Consult [`docs/historical-errors.md`](file:///var/home/andres/Documents/myublue/bolina/docs/historical-errors.md) before implementing changes to compositors, portals, audio, systemd, bootc, PAM, or initramfs.
- Document new error patterns immediately post-fix using the standard 3-column table format (`Error Class` | `Root Cause (Abstract)` | `Preventive Action`).

### Build Pipeline & Package Integrity
- Verify exact package identifiers (`dnf list` or repo queries) before updating `build_files/`.
- Ensure deterministic layers: pin versions with Renovate annotations, avoid `date` timestamps, and preserve `.keep` files in tracked directories.
- Respect filesystem immutability: `/usr` is read-only. Codify system changes in `Containerfile`, `build_files/`, `/etc/`, or `tmpfiles.d/`.

### Tooling & Language Policies
- **Rust First**: Build background daemons and system tools in Rust under `tools/`. Compile during CI and install statically linked binaries to `/usr/bin/`.
- **Rust Drop-in Policy**: Integrate only drop-in replacements with 100% verified POSIX/GNU CLI parity (`uutils-coreutils`, `uutils-findutils`, `sudo-rs`, `ntpd-rs`). Exclude WIP or incomplete rewrites (`diffutils`, `sed`, `grep`, `tar`).

### Session Validation & Commit Protocol
- Validate Wayland/desktop changes live via `busctl`, `systemctl --user status`, `pw-cli`, or `gsettings`.
- Stage, commit with conventional commit messages (`feat:`, `fix:`, `chore:`, `docs:`), and push immediately (`git push origin main`).
