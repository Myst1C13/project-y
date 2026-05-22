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

## Session 2 — 2026-05-22
### Phase 3: Desktop Shell (Wayland)
- Installed Wayland display stack: sway, waybar, wofi, swaybg, foot, xorg-xwayland (~166 packages)
- Enabled seatd (seat management for Wayland)
- Changed UTM display from virtio-ramfb → virtio-gpu-pci (required for DRM/GPU device)
- Loaded virtio_gpu kernel module
- Set up auto-login on tty1 (systemd getty override)
- Set up auto-launch sway from .bash_profile on tty1
- Created sway config with hybrid window management:
  - All windows float by default (Windows-style UX)
  - Super+Space toggles floating/tiling
  - Super+T cycles tiling layouts
  - Super+Return opens foot terminal
  - Super+D opens wofi app launcher
  - Super+Q closes window
  - Alt+Tab switches focus
  - Super+Up fullscreen toggle
- Configured waybar taskbar (CPU, RAM, IP, keyboard, clock)
- Set dark blue background (#1a1a2e) via swaybg
- Fixed locale (LANG=C.UTF-8)

### Phase 3 Results
- RAM idle: 276MB (full graphical desktop with compositor + taskbar)
- Desktop: sway (Wayland compositor) + waybar + foot + wofi
- Auto-boot to desktop: yes (no manual login required)
- Window management: hybrid floating/tiling

### Benchmarks Comparison
| Metric | Stock Arch | Phase 1 (Terminal) | Phase 3 (Desktop) |
|--------|-----------|-------------------|-------------------|
| RAM idle | 63MB | 161MB | 276MB |
| Packages | 135 | 165 | ~330 |
| Services | 11 | 9 | 10 (seatd added) |

---

## Next: Phase 2 — NVIDIA Auto-Config Scripts
## Then: Theming — Modern desktop appearance
