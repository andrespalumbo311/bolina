# Contribution Guidelines

This repository defines `bolina`, a hardware-agnostic, cloud-native Fedora Atomic base image (Universal Blue) with Niri and DankMaterialShell, built as an OCI container via GitHub Actions.

Device-specific configurations, proprietary drivers, sensor scripts, and private MOK signing belong strictly in downstream fleet repositories (such as `bolina-fleet`).

## Guidelines for Human Contributors
- **Container Builds:** All system configuration changes must be made via `Containerfile`, the modular scripts in `build_files/`, or runtime declarations in `etc/` and `usr/lib/tmpfiles.d/`.
- **Commit Conventions:** Commits should follow conventional commit formatting (`feat:`, `fix:`, `perf:`, `chore:`, `docs:`).
- **CI Build Triggers:** Documentation changes (`AGENTS.md`, `CONTRIBUTING.md`, `docs/**`, `README.md`) are ignored by the GitHub Actions container build pipeline to avoid unneeded rebuilds.

## Guidelines for AI Agents
- AI agents working on this codebase must follow the operational guidelines codified in [AGENTS.md](./AGENTS.md).
- The living repository of past technical challenges, root causes, and prevention patterns is maintained in [docs/historical-errors.md](./docs/historical-errors.md). Agents must consult this library before making architectural decisions and update it after verified fixes.
