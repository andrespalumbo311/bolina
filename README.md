# bolina ⛵

`bolina` is an immutable, cloud-native desktop Linux operating system built on top of [Universal Blue](https://universal-blue.org/) (`ublue-os/base-main`) and [bootc](https://github.com/bootc-dev/bootc) (Fedora Atomic).

It provides an out-of-the-box Wayland desktop environment powered by the [Niri](https://github.com/YaLTeR/niri) scrollable-tiling compositor, DankMaterialShell (DMS), memory-safe core utilities in Rust, and kernel performance optimizations from CachyOS.

---

## Upstream & Credits

This project directly derives from **[Universal Blue](https://universal-blue.org/)** and adheres to its cloud-native containerized OS architecture:
* Base image: `ghcr.io/ublue-os/base-main:latest`
* Atomic updates, stateless deployments, and rollback via `bootc`.
* Containerized application isolation via Flatpak, Distrobox, and Homebrew.
* Read-only `/usr` host filesystem.

---

## Technical Specifications

### Desktop & Compositor
* **Compositor:** Niri (scrollable-tiling Wayland compositor written in Rust).
* **Shell & Greeter:** DankMaterialShell (DMS) integrated with `greetd`.
* **Session Security:** Pure Wayland session. Tailored PAM integration for `greetd` using `pam_succeed_if.so user = greetd` to enforce user authentication while allowing daemon auto-start. Biometric auth with `fprintd` and GNOME Keyring unlocking.

### Memory Safety
* **`sudo-rs`:** Standard `sudo` replaced with [sudo-rs](https://github.com/trifectatechfoundation/sudo-rs) (Prossimo / Trifecta Tech Foundation), providing a memory-safe SUID implementation in Rust.
* **Core Utilities:** Includes `uutils-coreutils` (Rust rewrite of coreutils).
* **Shell Environment:** `fish` shell with `bass`, `starship`, `zoxide`, and `fzf`.

### Kernel & Performance
* **Kernel:** `kernel-cachyos-lto` with Link-Time Optimization (LTO).
* **CPU Scheduler:** `sched-ext` (`scx`) eBPF extensible scheduler framework.
* **Memory & Storage Tuning:** ZRAM swappiness (`vm.swappiness = 130`, `vm.page-cluster = 0`), ADIOS/BFQ I/O schedulers via udev rules.
* **Network:** TCP BBR congestion control, Cake qdisc, and TCP Fast Open (`net.ipv4.tcp_fastopen = 3`).

### OCI Chunking & Verification
* **Chunked-OCI:** Post-build step executes `rpm-ostree compose build-chunked-oci` with `--max-layers 64` to decompose the image into package-level layers, minimizing update download sizes.
* **Signing:** Container images are cryptographically signed with [Cosign](https://github.com/sigstore/cosign).

---

## Architecture: Base Image vs Fleet Overlays

`bolina` serves as the generic, hardware-agnostic base operating system image. It contains no machine-specific drivers, personal user accounts, or private signing keys.

Hardware-specific drivers, private MOK signing, and device configurations are layered downstream:

```text
[ bolina (Public Base) ] ──(build & rechunker)──► [ GHCR: bolina:latest ]
                                                          │
                                                          ▼ (FROM bolina:latest)
                                                  [ bolina-fleet (Private) ]
                                                          ├── x390-yoga (MOK sign, i915, auto-rotate)
                                                          └── other devices...
```

---

## Deployment

To switch an existing `bootc` system to `bolina`:

```bash
sudo bootc switch ghcr.io/andrespalumbo311/bolina:latest
sudo systemctl reboot
```

### Upgrades and Rollbacks

Automatic updates are handled by `uupd.timer`. To manage updates manually:

```bash
# Check and apply updates
uupd

# Check deployment status
bootc status

# Rollback to the previous deployment
sudo bootc rollback
```

---

## Upstream Projects

* [Universal Blue](https://universal-blue.org/)
* [bootc](https://github.com/bootc-dev/bootc)
* [Niri](https://github.com/YaLTeR/niri)
* [DankMaterialShell](https://github.com/AvengeMedia/dms)
* [CachyOS](https://cachyos.org/)
* [sudo-rs (Trifecta Tech Foundation)](https://github.com/trifectatechfoundation/sudo-rs)

---

## License

Apache-2.0. See [LICENSE](./LICENSE) for details.
