---
title: CLI Reference
description: "Reference for Sotid's command-line interface (CLI), including opening files and directories, integrating with tools, and controlling Sotid from scripts."
---

# CLI Reference

Use Sotid's command-line interface (CLI) to open files and directories, integrate with other tools, and control Sotid from scripts.

## Installation

**macOS:** Run the {#action cli::InstallCliBinary} command from the command palette ({#kb command_palette::Toggle}) to install the `sotid` CLI to `/usr/local/bin/sotid`.

**Linux:** The CLI is included with Sotid packages. The binary name may vary by distribution (commonly `sotid` or `zeditor`).

**Windows:** The CLI is included with Sotid. Add Sotid's installation directory to your PATH, or use the full path to `sotid.exe`.

## Usage

```sh
sotid [OPTIONS] [PATHS]...
```

## Opening Files and Directories

Open a file:

```sh
sotid myfile.txt
```

Open a directory as a workspace:

```sh
sotid ~/projects/myproject
```

Open multiple files or directories:

```sh
sotid file1.txt file2.txt ~/projects/myproject
```

Open a file at a specific line and column:

```sh
sotid myfile.txt:42        # Open at line 42
sotid myfile.txt:42:10     # Open at line 42, column 10
```

## Options

### `-w`, `--wait`

Wait for all opened files to be closed before the CLI exits. When opening a directory, waits until the window is closed.

This is useful for integrating Sotid with tools that expect an editor to block until editing is complete (e.g., `git commit`):

```sh
export EDITOR="sotid --wait"
git commit  # Opens Sotid and waits for you to close the commit message file
```

### `-n`, `--new`

Open paths in a new workspace window, even if the paths are already open in an existing window:

```sh
sotid -n ~/projects/myproject
```

### `-a`, `--add`

Add paths to the currently focused workspace instead of opening a new window. When multiple workspace windows are open, files open in the focused window:

```sh
sotid -a newfile.txt
```

### `-r`, `--reuse`

Reuse an existing window, replacing its current workspace with the new paths:

```sh
sotid -r ~/projects/different-project
```

### `-e`, `--existing`

Open paths in an existing Sotid window instead of creating a new one:

```sh
sotid -e myfile.txt
```

By default (without `-n`, `-a`, `-r`, or `-e`), directories open in the current window's sidebar. You can change this default with the `cli_default_open_behavior` setting. See [Windows & Projects](../windows-and-projects.md) for more details.

### `--diff <OLD_PATH> <NEW_PATH>`

Open a diff view comparing two files. Can be specified multiple times:

```sh
sotid --diff file1.txt file2.txt
sotid --diff old.rs new.rs --diff old2.rs new2.rs
```

### `--foreground`

Run Sotid in the foreground, keeping the terminal attached. Useful for debugging:

```sh
sotid --foreground
```

### `--user-data-dir <DIR>`

Use a custom directory for all user data (database, extensions, logs) instead of the default location:

```sh
sotid --user-data-dir ~/.sotid-custom
```

Default locations:

- **macOS:** `~/Library/Application Support/Sotid`
- **Linux:** `$XDG_DATA_HOME/sotid` (typically `~/.local/share/sotid`)
- **Windows:** `%LOCALAPPDATA%\Sotid`

### `-v`, `--version`

Print Sotid's version and exit:

```sh
sotid --version
```

### `--completions <SHELL>`

Generate shell completions for the `sotid` CLI:

#### Bash

Add to `~/.bashrc`:

```bash
eval "$(sotid --completions bash)"
```

#### Elvish

Add to `~/.config/elvish/rc.elv`:

```elvish
set edit:completion:arg-completer[sotid] = { |@args|
    eval (sotid --completions elvish | slurp)
    $edit:completion:arg-completer[sotid] $@args
}
```

#### Fish

Add to `~/.config/fish/config.fish`:

```fish
sotid --completions fish | source
```

#### Nushell

Add to `~/.config/nushell/config.nu`:

```nu
mkdir ($nu.data-dir | path join "vendor/autoload")
^sotid --completions nushell | save --force ($nu.data-dir | path join "vendor/autoload/sotid.nu")
```

#### Powershell

Add to `$PROFILE`:

```powershell
(&sotid --completions powershell) | Out-String | Invoke-Expression
```

#### Zsh

Add to `~/.zshrc`:

```zsh
eval "$(sotid --completions zsh)"
```

### `--uninstall`

Uninstall Sotid and remove all related files (macOS and Linux only):

```sh
sotid --uninstall
```

### `--sotid <PATH>`

Specify a custom path to the Sotid application or binary:

```sh
sotid --sotid /path/to/Sotid.app myfile.txt
```

## Reading from Standard Input

Read content from stdin by passing `-` as the path:

```sh
echo "Hello, World!" | sotid -
cat myfile.txt | sotid -
ps aux | sotid -
```

This creates a temporary file with the stdin content and opens it in Sotid.

## URL Handling

The CLI can open `sotid://`, `file://`, and `ssh://` URLs:

```sh
sotid sotid://settings
sotid file:///Users/whatever/.zshrc
sotid ssh://me@example.com/abs/path
sotid ssh://me@example.com:/abs/path
sotid ssh://me@example.com/~/project
sotid ssh://me@example.com:~/project
```

## Using Sotid as Your Default Editor

Set Sotid as your default editor for Git and other tools:

```sh
export EDITOR="sotid --wait"
export VISUAL="sotid --wait"
```

Add these lines to your shell configuration file (e.g., `~/.bashrc`, `~/.zshrc`).

## macOS: Switching Release Channels

On macOS, you can launch a specific release channel by passing the channel name as the first argument:

```sh
sotid --stable myfile.txt
sotid --preview myfile.txt
sotid --nightly myfile.txt
```

## WSL Integration (Windows)

On Windows, the CLI supports opening paths from WSL distributions. This is handled automatically when launching Sotid from within WSL.

## Exit Codes

| Code | Meaning                           |
| ---- | --------------------------------- |
| `0`  | Success                           |
| `1`  | Error (details printed to stderr) |

When using `--wait`, the exit code reflects whether the files were saved before closing.
