# `bake-readme`

Reusable `readme.md` maintenance tasks for Bake. `readme:update` keeps the short
release summary and package links in a project's `readme.md` up to date.

## Motivation

A project's `readme.md` should help people quickly understand what the project does,
why it exists, and where to start. Release summaries and package links are
useful, but they are easy to forget or leave stale when maintained by hand.
Bake Readme refreshes those derived sections while leaving the project's
explanation and examples under its authors' control.

## Usage

Add this crate to the project's private Bake task executable and link its task
library:

```toml
[dependencies]
bake-readme = "0.1"
```

```rust,ignore
use bake_readme as _;
```

Then run the task from the project root:

```sh
cargo bake readme:update
```

If `readme.md` has no authored `Releases` section, the task adds one with the
three latest versioned entries from `releases.md`. It skips `Unreleased` and
links to the full release history. The task also adds or refreshes a package
entry in `See Also` based on the root `Cargo.toml` name, description, and
repository URL. Existing authored sections and entries are preserved.

A custom path is supported with `--path PATH`.

## Releasing

Prepare a release with `cargo bake cargo:version:patch` (or `minor`, `major`,
or `bump --version X.Y.Z`), then run `cargo bake cargo:release` and open a
pull request. After review and merge, GitHub Actions publishes the release
when the configured `crates-io` environment approves it. Follow the shared
[Releasing skill](https://github.com/socketry/socketry-project-rust/blob/main/context/releasing.md)
for the standard process.

## Releases

<!-- bake-readme:releases:start -->
See [releases.md](releases.md) for the full release history.

### v0.1.5

- Use the shared `socketry-project` Releasing skill for the standard release
  process and remove references to the duplicate Bake Cargo publishing context.

### v0.1.4

- Improve generated README spacing and document the standard release process.
- Add context to errors when Cargo metadata cannot be launched.

### v0.1.3

- Update generated README sections with Markdown AST nodes while preserving existing package links.
<!-- bake-readme:releases:end -->

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/socketry/bake-readme-rust).
The [readme guidance](context/readme-structure.md) describes the intended scope and
structure of project `readme.md` files.

### Agent Context

Before contributing, read `agents.md` and the relevant context files it links. If `agents.md` is missing or out of date, run `cargo bake agent:context:install` to install context from dependencies and update the index.
