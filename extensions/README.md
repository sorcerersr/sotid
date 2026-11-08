# Sotid Extensions

This directory contains extensions for Sotid that are maintained alongside the Sotid repository. They currently live in the Sotid repository for ease of maintenance.

## Structure

Sotid includes support for a number of languages without requiring installing an extension. Their grammars, queries, and configs can be found under [`crates/grammars/src`](https://github.com/sotid/sotid/tree/main/crates/grammars/src), and their language server adapters under [`crates/languages/src`](https://github.com/sotid/sotid/tree/main/crates/languages/src).

Support for all other languages is done via extensions. This directory ([extensions/](https://github.com/sotid/sotid/tree/main/extensions/)) contains some of the maintained extensions. These extensions use the [zed_extension_api](https://docs.rs/zed_extension_api/latest/zed_extension_api/) for providing [language servers](https://github.com/sotid/sotid/blob/main/docs/src/extensions/languages.md#language-servers), [tree-sitter grammars](https://github.com/sotid/sotid/blob/main/docs/src/extensions/languages.md#grammar) and [tree-sitter queries](https://github.com/sotid/sotid/blob/main/docs/src/extensions/languages.md#tree-sitter-queries).

## Dev Extensions

See the docs for [Developing an Extension Locally](https://github.com/sotid/sotid/blob/main/docs/src/extensions/developing-extensions.md#developing-an-extension-locally) for how to work with one of these extensions.
