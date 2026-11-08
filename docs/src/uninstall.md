---
title: Uninstall
description: "This guide covers how to uninstall Sotid on different operating systems."
---

# Uninstall

Sotid is installed from source by `script/install.sh`, so uninstallation is handled by
`script/uninstall.sh`. Run it from the root of your Sotid checkout:

```sh
./script/uninstall.sh
```

The script removes the installed binaries and then prompts you to keep or delete your
preferences.

## What gets removed

- `~/.local/sotid.app` — the assembled application directory (Linux)
- `~/.local/bin/sotid` — the binary symlink
- `~/.local/share/applications/de.sorcarion.Sotid.desktop` — the desktop entry (Linux)
- `~/.config/sotid` — your preferences and data (only if you choose to remove them)

## Keeping your preferences

If you want to keep your configuration and data, answer `n` when prompted. Your
`~/.config/sotid` directory will be left in place, so a future reinstall picks up where
you left off.

## Manual removal

If the script is unavailable, you can remove the installed files manually:

- Installation directory: `~/.local/sotid.app`
- Binary symlink: `~/.local/bin/sotid`
- Desktop entry: `~/.local/share/applications/de.sorcarion.Sotid.desktop`
- Configuration and data: `~/.config/sotid`
