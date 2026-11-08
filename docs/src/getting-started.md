---
title: Getting Started with Sotid
description: Get started with Sotid, the fast offline code editor. Essential commands, environment setup, and navigation basics.
---

# Getting Started

Sotid is a high-performance, offline code editor. It is a fork of Zed, with all cloud connections removed.

This guide covers the essential commands, environment setup, and navigation basics.

## Quick Start

### Welcome Page

When you open Sotid without a folder, you see the welcome page in the main editor area. The welcome page offers quick actions to open a folder, clone a repository, or view documentation. Once you open a folder or file, the welcome page disappears. If you split the editor into multiple panes, the welcome page appears only in the center pane when empty—other panes show a standard empty state.

To reopen the welcome page, close all items in the center pane or use the command palette to search for "Welcome".

### 1. Open a Project

Open a folder from the command line:

```sh
sotid ~/projects/my-app
```

Or use `Cmd+O` (macOS) / `Ctrl+O` (Linux/Windows) to open a folder from within Sotid.

By default, new projects open in your current window's threads sidebar. To open in a new window instead, use `sotid -n ~/projects/my-app` or press `Cmd+Enter` when selecting from Open Recent. See [Windows & Projects](./windows-and-projects.md) for more details.

### 2. Learn the Essential Commands

| Action          | macOS         | Linux/Windows  |
| --------------- | ------------- | -------------- |
| Command palette | `Cmd+Shift+P` | `Ctrl+Shift+P` |
| Go to file      | `Cmd+P`       | `Ctrl+P`       |
| Go to symbol    | `Cmd+Shift+O` | `Ctrl+Shift+O` |
| Find in project | `Cmd+Shift+F` | `Ctrl+Shift+F` |
| Toggle terminal | `` Ctrl+` ``  | `` Ctrl+` ``   |
| Open settings   | `Cmd+,`       | `Ctrl+,`       |

The [command palette](./command-palette.md) (`Cmd+Shift+P`) is your gateway to every action in Sotid. If you forget a shortcut, search for it there.

### 3. Configure Your Editor

Open the Settings Editor with `Cmd+,` (macOS) or `Ctrl+,` (Linux/Windows). Search for any setting and change it directly.

Common first changes:

- **Theme**: Press `Cmd+K Cmd+T` (macOS) or `Ctrl+K Ctrl+T` (Linux/Windows) to open the theme selector
- **Font**: Search for [`buffer_font_family`](./reference/all-settings.md#buffer-font-family) in Settings
- **Format on save**: Search for `format_on_save` and set to `on`

### 4. Set Up Your Language

Sotid includes built-in support for many languages. For others, install the extension:

1. Open Extensions with `Cmd+Shift+X` (macOS) or `Ctrl+Shift+X` (Linux/Windows)
2. Search for your language
3. Click Install

See [Languages](./languages.md) for language-specific setup instructions.

## Coming from Another Editor?

We have dedicated guides for switching from other editors:

- [VS Code](./migrate/vs-code.md) — Import settings, map keybindings, find equivalent features
- [IntelliJ IDEA](./migrate/intellij.md) — Adapt to Sotid's approach to navigation and refactoring
- [PyCharm](./migrate/pycharm.md) — Set up Python development in Sotid
- [WebStorm](./migrate/webstorm.md) — Configure JavaScript/TypeScript workflows
- [RustRover](./migrate/rustrover.md) — Rust development in Sotid

You can also enable familiar keybindings:

- **Vim**: Enable `vim_mode` in settings. See [Vim Mode](./vim.md).
- **Helix**: Enable `helix_mode` in settings. See [Helix Mode](./helix.md).
