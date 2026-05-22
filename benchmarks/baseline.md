# Project Y — Baseline Benchmarks

## Stock Arch Linux ARM (UTM VM, Apple Silicon)
Date: 2026-05-21

### System
- Kernel: Linux 5.18.1-1-aarch64-ARCH
- Architecture: aarch64 (ARM64)
- VM: UTM on Apple Silicon Mac

### RAM Usage (idle, terminal only)
- Total: 1.9 GiB
- Used: 63 MiB
- Buff/cache: 62 MiB
- Available: 1.8 GiB
- Swap: None

### Disk Usage
- Root (/): 1.1G used / 9.4G total (12%)
- Boot (/boot): 104M used / 200M total (52%)

### Packages
- Total installed: 135

### Running Services
- Total: 11
  - dbus.service (D-Bus message bus)
  - getty@tty1.service (terminal login)
  - serial-getty@ttyAMA0.service (serial console)
  - sshd.service (SSH daemon)
  - systemd-journald.service (logging)
  - systemd-logind.service (login management)
  - systemd-networkd.service (networking)
  - systemd-resolved.service (DNS)
  - systemd-timesyncd.service (time sync)
  - systemd-udevd.service (hardware detection)
  - user@0.service (user session)

## Comparison
| OS | Idle RAM | Disk | Packages | Services |
|---|---|---|---|---|
| **Project Y (baseline)** | **63 MB** | **1.1 GB** | **135** | **11** |
| Ubuntu 24.04 | ~1,500 MB | ~8 GB | ~1,800 | ~50 |
| Windows 11 | ~4,000-6,000 MB | ~25 GB | N/A | ~100+ |

## Target (after optimization)
- Idle RAM with desktop: < 400 MB
- Disk: < 3 GB
- Services: < 8
- Boot time: < 3 seconds
