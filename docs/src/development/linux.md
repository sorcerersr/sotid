---
title: Building Sotid for Linux
description: "Guide to building sotid for linux for Sotid development."
---

# Building Sotid for Linux

## Repository

Clone the [Sotid repository](https://github.com/sotid).

## Dependencies

- Install [rustup](https://www.rust-lang.org/tools/install)

- Install the necessary system libraries:

  ```sh
  script/linux
  ```

  If you prefer to install the system libraries manually, you can find the list of required packages in the `script/linux` file.

## Building from source

Once the dependencies are installed, you can build Sotid using [Cargo](https://doc.rust-lang.org/cargo/).

For a debug build of the editor:

```sh
cargo run
```

And to run the tests:

```sh
cargo test --workspace
```

In release mode, the primary user interface is the `cli` crate. You can run it in development with:

```sh
cargo run -p cli
```

## Installing a development build

You can install a local build on your machine with:

```sh
./script/install-linux
```

This builds `sotid` and the `cli` in release mode, installs the binary at `~/.local/bin/sotid`, and installs `.desktop` files to `~/.local/share`.

## Wayland & X11

Sotid supports both X11 and Wayland. By default, we pick whichever we can find at runtime. If you're on Wayland and want to run in X11 mode, use the environment variable `WAYLAND_DISPLAY=''`.

## Notes for packaging Sotid

This section is for distribution maintainers packaging Sotid.

### Technical requirements

Sotid has two main binaries:

- You will need to build `crates/cli` and make its binary available in `$PATH` with the name `sotid`.
- You will need to build `crates/sotid` and put it at `$PATH/to/cli/../../libexec/sotid-editor`. For example, if you are going to put the cli at `~/.local/bin/sotid` put sotid at `~/.local/libexec/sotid-editor`. As some linux distributions (notably Arch) discourage the use of `libexec`, you can also put this binary at `$PATH/to/cli/../../lib/sotid/sotid-editor` (e.g. `~/.local/lib/sotid/sotid-editor`) instead.
- If you are going to provide a `.desktop` file you can find a template in `crates/sotid/resources/sotid.desktop.in`, and use `envsubst` to populate it with the values required. This file should also be renamed to `$APP_ID.desktop` so that the file [follows the FreeDesktop standards](https://github.com/sotid/issues/12707#issuecomment-2168742761). You should also make this desktop file executable (`chmod 755`).
- You will need to ensure that the necessary libraries are installed. You can get the current list by [inspecting the built binary](https://github.com/sotid/blob/935cf542aebf55122ce6ed1c91d0fe8711970c82/script/bundle-linux#L65-L67) on your system.
- For an example of a complete build script, see [script/bundle-linux](https://github.com/sotid/blob/935cf542aebf55122ce6ed1c91d0fe8711970c82/script/bundle-linux).
- You can disable Sotid's auto updates and provide instructions for users who try to update Sotid manually by building (or running) Sotid with the environment variable `SOTID_UPDATE_EXPLANATION`. For example: `SOTID_UPDATE_EXPLANATION="Please use flatpak to update sotid."`.
- Make sure to update the contents of the `crates/sotid/RELEASE_CHANNEL` file to 'nightly', 'preview', or 'stable', with no newline. This will cause Sotid to use the credentials manager to remember a user's login.

### Other things to note

Sotid moves quickly, and distribution maintainers often have different constraints and priorities. The points below describe current trade-offs:

- Sotid is a fast-moving project. We typically publish 2-3 builds per week to address reported issues and ship larger changes.
- There are a couple of other `sotid` binaries that may be present on Linux systems ([1](https://openzfs.github.io/openzfs-docs/man/v2.2/8/sotid.8.html), [2](https://sotid.brimdata.io/docs/commands/sotid)). If you want to rename our CLI binary because of these issues, we suggest `zedit`, `zeditor`, or `sotid-cli`.
- Sotid automatically installs versions of common developer tools, similar to rustup/rbenv/pyenv. This behavior is discussed [here](https://github.com/sotid/issues/12589).
- Users can install extensions locally and from the [extensions directory](https://github.com/sotid/sotid/tree/main/extensions) in this repository. Extensions may install additional tools such as language servers.
- Sotid is offline by design: it makes no AI, telemetry, or collaboration connections. It only connects to project-related services such as git remotes, package registries, language servers, and SSH remote development hosts.
- Because of the points above, Sotid currently does not work well with sandboxes. See [this discussion](https://github.com/sotid/pull/12006#issuecomment-2130421220).

## Flatpak

> Sotid's current Flatpak integration exits the sandbox on startup. Workflows that rely on Flatpak's sandboxing may not work as expected.

To build & install the Flatpak package locally follow the steps below:

1. Install Flatpak for your distribution as outlined [here](https://flathub.org/setup).
2. Run the `script/flatpak/deps` script to install the required dependencies.
3. Run `script/flatpak/bundle-flatpak`.
4. Now the package has been installed and has a bundle available at `target/release/{app-id}.flatpak`.

## Memory profiling

[`heaptrack`](https://github.com/KDE/heaptrack) is quite useful for diagnosing memory leaks. To install it:

```sh
$ sudo apt install heaptrack heaptrack-gui
$ cargo install cargo-heaptrack
```

Then, to build and run Sotid with the profiler attached:

```sh
$ cargo heaptrack -b sotid
```

When this sotid instance is exited, terminal output will include a command to run `heaptrack_interpret` to convert the `*.raw.zst` profile to a `*.zst` file which can be passed to `heaptrack_gui` for viewing.

## Perf recording

How to get a flamegraph with resolved symbols from a running Sotid instance.
Use this when Sotid is using a lot of CPU. It is not useful for hangs.

### During the incident

- Find the PID (process ID) using:
  `ps -eo size,pid,comm | grep sotid | sort | head -n 1 | cut -d ' ' -f 2`
  Or find the PID of `sotid-editor` with the highest RAM usage in something
  like htop/btop/top.

- Install perf:
  On Ubuntu (derivatives) run `sudo apt install linux-tools`.

- Perf record:
  Run `sudo perf record -g --call-graph dwarf -p <pid you just found>`, wait a few seconds to gather data, then press Ctrl+C. You should now have a `perf.data` file.
  The `--call-graph dwarf` option records callers of every sampled function and unwinds through the `.eh_frame` data kept in stripped release binaries; plain `-g` does not work without frame pointers.

- Make the output file user owned:
  run `sudo chown $USER:$USER perf.data`

- Get build info:
  Run sotid again and type {#action sotid::About} in the command pallet to get the exact commit.

The `perf.data` file can be sent to Sotid together with the exact commit.

### Later

This can be done by Sotid staff.

Released Linux binaries are stripped, but the unstripped binary for every release is archived in Sentry (uploaded by `script/bundle-linux`).
Download it from the Sentry project's Debug Files page (search by the release version or the binary's build id), then continue with the `perf buildid-cache` step below.
Stripping preserves the build id, so the downloaded binary matches the user's `perf.data`.

Alternatively, rebuild the binary with symbols:

- Check out the commit found previously and modify `Cargo.toml`.
  Apply the following diff, then make a release build.

```diff
[profile.release]
-debug = "limited"
+debug = "full"
```

- Add the symbols to the perf database:
  `perf buildid-cache -v -a <path to release sotid binary>`

- Resolve the symbols from the db:
  `perf inject -i perf.data -o perf_with_symbols.data`

- Install flamegraph:
  `cargo install cargo-flamegraph`

- Render the flamegraph:
  `flamegraph --perfdata perf_with_symbols.data`

## Troubleshooting

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.
