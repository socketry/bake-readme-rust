# Releasing

This repository publishes `bake-readme` as an independently versioned crate.

Use the repository Bake tasks to prepare releases. The reviewed release pull
request updates `Cargo.toml`, `Cargo.lock`, the license, and `releases.md`; the
GitHub workflow publishes the crate and creates its version tag after checks
pass.
