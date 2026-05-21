# Project Y

A performance-obsessed Linux distribution built for developers and daily use.

## Goals
- **Lowest RAM usage of any usable desktop OS** — target <400MB idle with full desktop
- **NVIDIA/CUDA works on first boot** — no driver hell, no manual setup
- **Windows-familiar UX** — taskbar, start menu, system tray. No learning curve
- **Nothing ever breaks** — immutable base with automatic rollback on failed updates

## Architecture
- **Base:** Minimal Arch Linux (stripped to essentials)
- **Init:** dinit (fast, lightweight replacement for systemd)
- **Display:** Wayland (wlroots-based compositor)
- **Shell:** Custom desktop shell written in Rust
- **Apps:** Flatpak for sandboxed app delivery + Wine/Proton for Windows apps
- **Updates:** OSTree/btrfs snapshots — atomic, rollback-capable

## Benchmarks
_Coming soon — will compare idle RAM, boot time, and GPU performance against Windows 11, Ubuntu, and Fedora._

## Status
🔧 In development — building base system
