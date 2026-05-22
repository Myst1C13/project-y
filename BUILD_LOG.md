# Project Y — Build Log

## Session 1 — 2026-05-21
### Setup
- Created GitHub repo: https://github.com/Myst1C13/project-y
- Installed UTM on Mac (Apple Silicon)
- Downloaded Arch Linux ARM from UTM gallery
- Booted VM, logged in as root
- Set up SSH access for easier development
- Recorded stock baseline: 63MB RAM, 135 packages, 11 services

### System Update
- Updated kernel from 5.18.1 to 7.0.9
- Cleared /boot space (removed fallback initramfs + compressed kernel)
- Full system upgrade: 172 packages updated

### Phase 1: Base System Optimization
- Disabled systemd-resolved (replaced with static DNS in /etc/resolv.conf)
- Masked serial-getty@ttyAMA0 (VM-only service)
- Masked systemd-journald-audit.socket
- Capped journald: volatile storage, 8MB max
- Removed 10 bloat packages: vim, nano, gpm, net-tools, netctl, dhcpcd, openresolv, pcre, ex-vi-compat
- Installed zram-generator with zstd compression (ram/2 = ~981MB)
- Applied kernel sysctl tuning: swappiness=10, dirty_ratio=5, overcommit_memory=1
- Installed micro (lightweight text editor)
- Fixed DNS after resolved removal

### Phase 1 Results
- RAM idle: 161MB (terminal only, no desktop)
- zram swap: 981MB compressed
- Services: 9
- Packages: 165
- Disk: 2.3GB

---

## Next: Phase 2 — NVIDIA Auto-Config Scripts
## Then: Phase 3 — Desktop Shell (Rust + Wayland)
