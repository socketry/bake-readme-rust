# Conventions

- Keep public README task behavior in `src/` and its behavioral guidance in `readme.md`.
- Preserve existing README bytes when no update is needed.
- Mark metadata-generated package links so later updates can refresh them without replacing authored links.
- Keep task names under `readme` and link this crate from the private `bake/` executable.
- Keep shared Rust guidance in the `bake-agent-context` package.
- Keep release notes and license updates composed through `cargo:after_version_bump`.
- The `bake/` package is private; publish only `bake-readme`.
