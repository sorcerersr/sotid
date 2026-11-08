# Contributing to Sotid

Thank you for helping us make Sotid better!

## Before you start

- **Keep no more than three PRs open at a time.**
  This limit helps us keep up with reviews and ensures all contributors get a fair share of our reviewing capacity.
  Start with one and see it through; the pattern we've seen is that landing your first PR dramatically improves the odds for every PR after it.
  A stack of open PRs on the other hand tends to go stale as `main` changes.
- **Discuss features with us before you start writing code for them.**
  If there isn't a GitHub issue confirming that we want it, start a [GitHub discussion](https://github.com/sotid/sotid/discussions), not a PR or a new issue.

All activity in this repository is subject to our [Code of Conduct](./CODE_OF_CONDUCT.md).

### Things we will (probably) not merge

There are few hard-and-fast rules, but we typically don't merge:

- **Changes that can be provided by an extension**, such as new languages or themes. See the [extension development docs](./docs/src/extensions/developing-extensions.md).
- **New file icons.** Our default icons are hand-designed to fit together; please don't submit off-the-shelf SVGs.
- **Features whose complexity outweighs their benefit**, in our judgement, for the number of people who would use them.
- **Giant refactorings.**
- **Non-trivial changes without tests.**
- **Style-only code changes that don't alter app logic.** Reducing allocations, removing `.unwrap()`s, and fixing typos are welcome; making code "more readable" alone may not be.
- **LLM-generated work the author doesn't understand.**

### We love PRs that...

- **Fix or extend the docs**: browse [docs issues](https://github.com/sotid/sotid/issues?q=is%3Aissue%20state%3Aopen%20type%3ADocs).
- **Fix bugs**: start with [triaged bugs with confirmed reproduction steps](https://github.com/sotid/sotid/issues?q=is%3Aissue%20state%3Aopen%20type%3ABug%20label%3Astate%3Areproducible), or browse [area labels](https://github.com/sotid/sotid/labels?q=area%3A*) for parts of Sotid you care about.
- **Make existing features work for more people** through small enhancements, such as support for more platforms or modes.
- **Add small features**, like keybindings or actions you miss from other editors.

## AI Policy

We welcome the use of LLMs for coding, but we hold a high bar for all contributions, and **we expect a human in the loop who genuinely understands the work an LLM produces** on their behalf.

For that reason, we **don't accept contributions from autonomous agents**. Pull requests that appear to violate this may be closed without notice.

**Don't rely on LLMs to write the whole thing for you when communicating with the maintainers** (meaning replies to comments, PR descriptions, and alike). The readers are humans, and we'd like to hear from you, not from a model.

If you think it's helpful/necessary to **share context from a chat with an LLM**, please put the **relevant part of it** in a quote block (e.g., using `>`), **disclose it as AI-generated**, and add your own commentary explaining **why it's relevant and what you take from it**.

This policy was adapted from [ripgrep's AI policy](https://github.com/BurntSushi/ripgrep/blob/f0cec341ab95c25c691ad3d5754d4bd9eedde21f/AI_POLICY.md).

## Sending changes

**Build and run Sotid locally, then manually test your changes before opening a PR.** Sotid is built from source; follow the guide for [macOS](./docs/src/development/macos.md), [Linux](./docs/src/development/linux.md), or [Windows](./docs/src/development/windows.md).

When preparing your PR:

- **Review your own work**, including any AI-assisted code. Follow the [pull request template](./.github/pull_request_template.md?plain=1) for testing details and the self-review checklist.
- **For visual changes, attach screenshots or a video** and work through the [UI/UX checklist](./docs/src/development/ui-checklist.md). For non-visual improvements and changes, include benchmarks or other artifacts produced when testing.
- **Keep each PR about one thing**, leaving unrelated features and refactoring for another PR.

**Opening a PR does not guarantee a merge.** We may decline a change that doesn't fit Sotid's direction or quality standards, even after you've put work into it. Confirming interest early and following this guide give your PR the best chance.

**Pinging maintainers by username or emailing them does not raise your PR's priority**; it takes time away from review.

## Resources

### UI/UX checklist

For UI changes, use the [UI/UX checklist](./docs/src/development/ui-checklist.md) in the development docs.
