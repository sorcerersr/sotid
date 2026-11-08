---
title: Developing Sotid
description: "Guide to building and developing Sotid from source."
---

# Developing Sotid

See the platform-specific instructions for building Sotid from source:

- [macOS](./development/macos.md)
- [Linux](./development/linux.md)
- [Windows](./development/windows.md)

## Bird's-eye view of Sotid {#birds-eye-view-of-sotid}

Keep the [Sotid glossary](./development/glossary.md) handy when starting out. It explains structures and terms you'll see throughout the codebase.

Sotid is made up of several smaller crates. Here are the ones you're most likely to work with:

- [`gpui`](https://github.com/sotid/tree/main/crates/gpui) is a GPU-accelerated UI framework which provides all of the building blocks for Sotid. **We recommend familiarizing yourself with the root level GPUI documentation.**
- [`editor`](https://github.com/sotid/tree/main/crates/editor) contains the core `Editor` type that drives both the code editor and all various input fields within Sotid. It also handles a display layer for LSP features such as Inlay Hints or code completions.
- [`project`](https://github.com/sotid/tree/main/crates/project) manages files and navigation within the filetree. It is also Sotid's side of communication with LSP.
- [`workspace`](https://github.com/sotid/tree/main/crates/workspace) handles local state serialization and groups projects together.
- [`vim`](https://github.com/sotid/tree/main/crates/vim) is a thin implementation of Vim workflow over `editor`.
- [`lsp`](https://github.com/sotid/tree/main/crates/lsp) handles communication with external LSP server.
- [`language`](https://github.com/sotid/tree/main/crates/language) drives `editor`'s understanding of language - from providing a list of symbols to the syntax map.
- [`rpc`](https://github.com/sotid/tree/main/crates/rpc) provides a WebSocket-based RPC connection layer used for remote development.
- [`theme`](https://github.com/sotid/tree/main/crates/theme) defines the theme system and provides a default theme.
- [`ui`](https://github.com/sotid/tree/main/crates/ui) is a collection of UI components and common patterns used throughout Sotid.
- [`cli`](https://github.com/sotid/tree/main/crates/cli) is the CLI crate which invokes the Sotid binary.
- [`sotid`](https://github.com/sotid/tree/main/crates/sotid) is where all things come together, and the `main` entry point for Sotid.

## Keychain access

Sotid stores secrets in the system keychain.

However, when running a development build of Sotid on macOS (and perhaps other
platforms) trying to access the keychain results in a lot of keychain prompts
that require entering your password over and over.

On macOS this is caused by the development build not having a stable identity.
Even if you choose the "Always Allow" option, the OS will still prompt you for
your password again the next time something changes in the binary.

This quickly becomes annoying and impedes development speed.

That is why, by default, when running a development build of Sotid an alternative
credential provider is used to bypass the system keychain.

> **Note:** This is **only** the case for development builds. For all non-development
> release channels the system keychain is always used.

If you need to test something out using the real system keychain in a
development build, run Sotid with the following environment variable set:

```
SOTID_DEVELOPMENT_USE_KEYCHAIN=1
```

## Performance Measurements

Sotid includes a frame time measurement system that can be used to profile how long it takes to render each frame. This is particularly useful when comparing rendering performance between different versions or when optimizing frame rendering code.

### Using SOTID_MEASUREMENTS

To enable performance measurements, set the `SOTID_MEASUREMENTS` environment variable:

```sh
export SOTID_MEASUREMENTS=1
```

When enabled, Sotid will print frame rendering timing information to stderr, showing how long each frame takes to render.

### Performance Comparison Workflow

Here's a typical workflow for comparing frame rendering performance between different versions:

1. **Enable measurements:**

   ```sh
   export SOTID_MEASUREMENTS=1
   ```

2. **Test the first version:**

   - Checkout the commit you want to measure
   - Run Sotid in release mode and use it for 5-10 seconds: `cargo run --release &> version-a`

3. **Test the second version:**

   - Checkout another commit you want to compare
   - Run Sotid in release mode and use it for 5-10 seconds: `cargo run --release &> version-b`

4. **Generate comparison:**

   ```sh
   script/histogram version-a version-b
   ```

The `script/histogram` tool can accept as many measurement files as you like and will generate a histogram visualization comparing the frame rendering performance data between the provided versions.

### Using `util_macros::perf`

For benchmarking unit tests, annotate them with the `#[perf]` attribute from the `util_macros` crate. Then run `cargo
perf-test -p $CRATE` to benchmark them. See the rustdoc documentation on `crates/util_macros` and `tooling/perf` for
in-depth examples and explanations.

## ETW Profiling on Windows

Sotid supports performance profiling with Event Tracing for Windows (ETW) to capture detailed performance data, including CPU, GPU, memory, disk, and file I/O activity. Data is saved to an `.etl` file, which can be opened in standard profiling tools for analysis.

ETW recordings may contain personally identifiable or security-sensitive information, such as paths to files and registry keys accessed, as well as process names. Please keep this in mind when sharing traces with others.

### Recording a trace

Open the command palette and run one of the following:

- `sotid: record etw trace`: records CPU, GPU, memory, and I/O activity
- `sotid: record etw trace with heap tracing`: includes heap allocation data for the Sotid process

Sotid will request administrator permission. Once granted, recording will begin.

### Saving or canceling

While a trace is recording, open the command palette and run one of the following:

- `sotid: save etw trace`: stops recording and saves the trace to disk
- `sotid: cancel etw trace`: stops recording without saving

## Contributor links

- [CONTRIBUTING.md](https://github.com/sotid/blob/main/CONTRIBUTING.md)
- [Code of Conduct](https://github.com/sotid/code-of-conduct)
- [Sotid Contributor License](https://github.com/sotid/cla)
