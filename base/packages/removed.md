# Removed Packages

Packages removed from stock Arch Linux ARM to reduce bloat.

- vim, vim-runtime (heavy editor — will add lighter alternative)
- nano (removed to add single preferred editor later)
- gpm (terminal mouse support — useless with desktop)
- net-tools (deprecated — replaced by iproute2)
- netctl (old network manager — replacing with NetworkManager)
- dhcpcd (DHCP client — NetworkManager handles this)
- openresolv (DNS config — not needed)
- pcre (old regex lib v1 — pcre2 is used everywhere)
- ex-vi-compat (vi compatibility shim)
