# Releases

## v0.2.1

- Require Bake 0.18.0 for the shared task registry.

## v0.2.0

- Remove the redundant `bake_readme::readme` module; the task adapter is now
  available at `bake_readme::update`.

## v0.1.6

- Declare compatibility with the Bake 0.x API so task libraries can share one task registry
  when upgrading to crate-derived task namespaces.


## v0.1.5

- Use the shared `socketry-project` Releasing skill for the standard release
  process and remove references to the duplicate Bake Cargo publishing context.

## v0.1.4

- Improve generated README spacing and document the standard release process.
- Add context to errors when Cargo metadata cannot be launched.

## v0.1.3

- Update generated README sections with Markdown AST nodes while preserving existing package links.

## v0.1.2

- Create or update GitHub Releases after successful crates.io publication.
- Resolve the local task crate during version updates.

## v0.1.1

- Switch the runtime dependency from `socketry-bake` to `bake` 0.17.0.

## v0.1.0

- Create the initial `bake-readme` task library and `readme:update` task.
- Add metadata-driven package links to the `readme.md` See Also section.
