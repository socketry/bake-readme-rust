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

### Agent Context

If you use Bake Agent Context, this crate also provides [Readme Structure](context/readme-structure.md), guidance for writing concise, human-focused project readmes. To install it in your project, add Bake Agent Context to the private `bake/` crate and link its task library:

```toml
[dependencies]
bake-agent-context = "0.1"
```

```rust,ignore
use bake_agent_context as _;
```

Then install this crate's context and refresh `agents.md`:

```sh
cargo bake agent:context:install --package bake-readme
```

## Releases

<!-- bake-readme:releases:start -->
See [releases.md](releases.md) for the full release history.

### v0.1.2

- Create or update GitHub Releases after successful crates.io publication.
- Resolve the local task crate during version updates.

### v0.1.1

- Switch the runtime dependency from `socketry-bake` to `bake` 0.17.0.

### v0.1.0

- Create the initial `bake-readme` task library and `readme:update` task.
- Add metadata-driven package links to the `readme.md` See Also section.
<!-- bake-readme:releases:end -->

## See Also

- [bake-readme](https://github.com/socketry/bake-readme-rust) — Reusable readme.md maintenance tasks for Bake <!-- bake-readme:package -->

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/socketry/bake-readme-rust).
The [readme guidance](context/readme-structure.md) describes the intended scope and
structure of project `readme.md` files.
