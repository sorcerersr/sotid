---
title: Install Sotid - macOS, Linux, Windows
description: Build and install Sotid from source on macOS, Linux, or Windows.
---

# Installing Sotid

Sotid is built from source. There are no prebuilt download packages; you clone the
repository, install the build dependencies, and run the install script.

## Build from source

1. Clone the Sotid repository:

   ```sh
   git clone https://github.com/sotid
   cd sotid
   ```

2. Install the build dependencies for your platform:

   - **Linux:** `script/linux`
   - **macOS:** see [Building for macOS](./development/macos.md)
   - **Windows:** see [Building for Windows](./development/windows.md)

3. Build and install Sotid:

   ```sh
   ./script/install.sh
   ```

   The script builds the `sotid` and `cli` binaries in release mode and installs them
   into `~/.local`. On Linux it assembles `~/.local/sotid.app/` and symlinks
   `~/.local/bin/sotid`. On macOS it symlinks `~/.local/bin/sotid`.

4. If `~/.local/bin` is not already on your `PATH`, add it:

   ```sh
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

You can now run Sotid from your terminal with `sotid`.

## System Requirements

### macOS

Sotid requires macOS 12.0 (Monterey) or later.

| Version            | Codename    | Apple Status   | Sotid Status          |
| ------------------ | ----------- | -------------- | --------------------- |
| macOS 27.x         | Golden Gate | Supported      | Supported             |
| macOS 26.x         | Tahoe       | Supported      | Supported             |
| macOS 15.x         | Sequoia     | Supported      | Supported             |
| macOS 14.x         | Sonoma      | EOL 2026-09-14 | Supported             |
| macOS 13.x         | Ventura     | EOL 2025-09-15 | Supported             |
| macOS 12.x (12.3+) | Monterey    | EOL 2024-09-16 | Supported             |
| macOS 12.0–12.2    | Monterey    | EOL 2024-09-16 | Partially Supported   |

Apple support status and EOL dates are from [endoflife.date](https://endoflife.date/macos).

#### Mac Hardware

Sotid supports machines with Intel (x86_64) or Apple (aarch64) processors that meet the above macOS requirements:

- MacBook Pro (Early 2015 and newer)
- MacBook Air (Early 2015 and newer)
- MacBook (Early 2016 and newer)
- Mac Mini (Late 2014 and newer)
- Mac Pro (Late 2013 or newer)
- iMac (Late 2015 and newer)
- iMac Pro (all models)
- Mac Studio (all models)

### Linux

Sotid supports 64-bit Intel/AMD (x86_64) and 64-bit Arm (aarch64) processors.

Sotid requires a Vulkan 1.3 driver and the following desktop portals:

- `org.freedesktop.portal.FileChooser`
- `org.freedesktop.portal.OpenURI`
- `org.freedesktop.portal.Secret` or `org.freedesktop.Secrets`

### Windows

Sotid supports the following Windows releases:
| Version | Sotid Status |
| ------------------------- | ------------------- |
| Windows 11, version 22H2 and later | Supported |
| Windows 10, version 1903 and later | Supported |

A 64-bit operating system is required to run Sotid.

#### Windows Hardware

Sotid supports machines with x64 (Intel, AMD) or Arm64 (Qualcomm) processors that meet the following requirements:

- Graphics: A GPU that supports DirectX 11 (most PCs from 2012+).
- Driver: Current NVIDIA/AMD/Intel/Qualcomm driver (not the Microsoft Basic Display Adapter).

### FreeBSD

Can be built [from source](./development/freebsd.md).
