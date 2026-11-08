> [!IMPORTANT]
> Remove this line to confirm you've reviewed this PR before submitting.

# Sotid

Sotid is a high-performance, offline code editor. It is a fork of [Zed](https://zed.dev), with all cloud connections (AI, collaboration, telemetry, update checks, extension marketplace) removed. Sotid keeps only project-related connections: git, package registries, language servers, and SSH remote development.

---

### Features

- **Fully offline** — no cloud dependencies, no telemetry, no AI
- **Fast** — built on the same performance-focused architecture as Zed
- **Extensible** — local extension support (install from file)
- **SSH remote** — connect to remote development environments
- **Language servers** — full LSP support for all major languages

### Installation

Prebuilt artifacts are published to [GitHub Releases](https://github.com/sotid/sotid/releases):

- **Linux** — Flatpak bundle, a portable `sotid-linux-*.tar.gz`, or build from source
- **macOS** — unsigned `.dmg` (the first launch may prompt to confirm the app)
- **Windows** — unsigned installer (Windows may show a SmartScreen warning)

Building from source is first-class on all three platforms. See the [development docs](./docs/src/development/) for build instructions.

### Developing Sotid

- [Building Sotid for macOS](./docs/src/development/macos.md)
- [Building Sotid for Linux](./docs/src/development/linux.md)
- [Building Sotid for Windows](./docs/src/development/windows.md)

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to Sotid.

### Credits

Sotid is a fork of [Zed](https://github.com/zed-industries/zed), created by the Zed team. The original Zed project is licensed under GPL-3.0-or-later.
