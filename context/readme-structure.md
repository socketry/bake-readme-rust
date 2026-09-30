# Readme Structure

Use this guidance when creating or updating a project's `readme.md`. The readme is the human-facing introduction to the project. It should help someone quickly decide what the project does, why it exists, and where to go next. It is not a substitute for detailed user documentation or agent context.

## Project Name

Rust does not define a canonical human-readable title for a project readme. For a Cargo package, use the exact `[package].name` as the title. It is the name users find in the registry and add to their dependencies. Write it as an inline code span so its exact spelling is clear; do not title-case it, add spaces, or otherwise humanize it.

For example, a package named `socketry-executor` with a library crate named `socketry_executor` should use ``# `socketry-executor` `` as the readme title. Use `socketry_executor` in Rust import paths and examples; Cargo normally derives that crate name from the package name by replacing hyphens with underscores, unless `[lib].name` overrides it.

For a binary-only package, the package name remains the title, and the binary target name is used when showing the executable command. For a workspace readme, use the root package name when the root manifest defines a package.

## Recommended structure

Keep the readme short and arrange sections in the order a new reader needs them:

1. **Project name and summary** — a clear title followed by one or two sentences describing the problem solved and the project's role.
2. **Motivation (optional)** — include it when explaining the problem and why the project exists would add useful context. Keep it specific and brief.
3. **Usage** — show installation and one or two small, representative examples. Link to guides for complete examples or less common use cases. If the package publishes agent context, include the standard optional `### Agent Context` block shown below; otherwise omit it.
4. **Releases** — show a short summary of recent releases and link to `releases.md` for the full history. Keep release sections newest first; `bake-readme` summarizes the first three versioned entries and skips `Unreleased`.
5. **See Also** — link to related projects, APIs, or documentation. For Cargo packages, `bake-readme` can add the package name, description, and repository URL from `Cargo.toml`.
6. **Contributing** — use the standard short block shown below, with the repository link filled in. Put detailed contribution instructions elsewhere.

Motivation and Agent Context are optional. Keep the other sections concise and relevant to helping readers understand, use, or contribute to the project.

## Keep detail in the right place

The readme should provide orientation and a useful first example, not try to contain every detail. Put API references, configuration options, architecture explanations, troubleshooting, and extensive examples in guides or other documentation. Put coding-agent instructions and task-specific implementation context in the package's agent context. Link to those resources from the readme when they help a human reader continue.

Avoid copying an entire guide into the readme, long implementation notes, internal roadmaps, and instructions aimed only at coding agents. Keep the readme understandable without requiring readers to know how the project is maintained.

## Example outline

```markdown
# `exact-package-name`

One or two sentences describing what the project does.

## Motivation

Describe the problem and why this project exists.

## Usage

Show how to install or add the project, then give a small common example.

<!-- Include this subsection only when the crate publishes files in context/. -->
### Agent Context

This crate publishes context files for coding agents. Configure Bake Agent Context in your private `bake/` crate, then run `cargo bake agent:context:install --package PACKAGE_NAME` to install them in `.agents/context/` and update `agents.md`.

## Releases

Recent release summaries and a link to `releases.md`.

## See Also

Links to related projects or documentation.

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/OWNER/REPOSITORY).
```

Replace `PACKAGE_NAME` with the exact Cargo package name and `OWNER/REPOSITORY` with the source repository. The Agent Context paragraph is fixed boilerplate; omit the entire subsection when the crate does not publish context. The Contributing paragraph is also standard boilerplate; retain it for public projects and change only the repository link.

Treat this as a starting point for project-specific summary, motivation, and usage content. Preserve authored wording when running `cargo bake readme:update`; the task only manages marked release summaries and the generated Cargo package entry.

The task uses HTML comments to identify the sections it may refresh. Do not edit the contents between its release markers by hand; edit `releases.md` instead. Existing unmarked release sections and manually maintained See Also entries are left intact.
