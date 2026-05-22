# Disabled / Masked Services

## Disabled
- systemd-resolved (DNS resolver — handled by NetworkManager)

## Masked (can never start)
- serial-getty@ttyAMA0 (serial console — VM only)
- systemd-journald-audit.socket (security auditing — not needed for desktop)

## Kept
- dbus-broker (message bus — required)
- getty@tty1 (terminal login — replaced by graphical login later)
- sshd (SSH — useful for dev, optional for end users)
- systemd-journald (logging — capped at 8MB volatile)
- systemd-logind (session management — required)
- systemd-networkd (networking — will replace with NetworkManager)
- systemd-timesyncd (clock sync — tiny, essential)
- systemd-udevd (hardware detection — required)
- user@0 (user session — required)
