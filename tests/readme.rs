// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use bake_readme::{
    PackageMetadata, Release, ensure_releases_section, recent_releases, update_document,
    update_document_with_releases,
};
use std::fs;

#[test]
fn inserts_releases_section_before_contributing() {
    let document =
        "# Project\n\nA short description.\n\n## Contributing\n\nContributions welcome.\n";
    assert_eq!(
        ensure_releases_section(document),
        "# Project\n\nA short description.\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n\n## Contributing\n\nContributions welcome.\n"
    );
}

#[test]
fn appends_when_no_preferred_section_exists() {
    assert_eq!(
        ensure_releases_section("# Project\n\nDescription."),
        "# Project\n\nDescription.\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n"
    );
}

#[test]
fn existing_section_is_preserved_exactly() {
    let document = "# Project\r\n\r\n## Releases\r\nCustom release links.\r\n";
    assert_eq!(ensure_releases_section(document), document);
}

#[test]
fn ignores_headings_in_fenced_code_and_block_quotes() {
    let document =
        "# Project\n\n```markdown\n## Releases\n```\n\n> ## Releases\n\n## Contributing\n";
    assert!(ensure_releases_section(document).contains("\n## Releases\n\nSee [releases.md]"));
}

#[test]
fn preserves_crlf_line_endings_when_inserting() {
    let document = "# Project\r\n\r\n## Contributing\r\n";
    assert_eq!(
        ensure_releases_section(document),
        "# Project\r\n\r\n## Releases\r\n\r\nSee [releases.md](releases.md) for the release history.\r\n\r\n## Contributing\r\n"
    );
}

#[test]
fn selects_the_three_most_recent_versioned_release_entries() {
    let document = "# Releases\n\n## Unreleased\n\nNot published yet.\n\n## v1.2.0\n\n- Third.\n\n## v1.1.0\n\n- Second.\n\n## v1.0.0\n\n- First.\n\n## v0.9.0\n\n- Older.\n";
    assert_eq!(
        recent_releases(document),
        vec![
            Release {
                name: "v1.2.0".to_owned(),
                notes: "- Third.".to_owned()
            },
            Release {
                name: "v1.1.0".to_owned(),
                notes: "- Second.".to_owned()
            },
            Release {
                name: "v1.0.0".to_owned(),
                notes: "- First.".to_owned()
            },
        ]
    );
}

#[test]
fn includes_recent_releases_in_generated_section_before_contributing() {
    let releases = [Release {
        name: "v1.2.0".to_owned(),
        notes: "- Improve startup.".to_owned(),
    }];
    let updated = update_document_with_releases("# Project\n\n## Contributing\n", None, &releases);
    assert!(updated.contains("## Releases\n\n<!-- bake-readme:releases:start -->"));
    assert!(updated.contains("### v1.2.0\n\n- Improve startup."));
    assert!(updated.contains("<!-- bake-readme:releases:end -->\n\n## Contributing"));
}

#[test]
fn refreshes_generated_release_entries_and_preserves_authored_section_text() {
    let initial = [Release {
        name: "v1.0.0".to_owned(),
        notes: "- Initial release.".to_owned(),
    }];
    let document = update_document_with_releases("# Project\n", None, &initial);
    let document = document.replace(
        "<!-- bake-readme:releases:start -->",
        "A short release summary.\n\n<!-- bake-readme:releases:start -->",
    );
    let latest = [Release {
        name: "v1.1.0".to_owned(),
        notes: "- Faster builds.".to_owned(),
    }];
    let updated = update_document_with_releases(&document, None, &latest);
    assert!(updated.contains("A short release summary.\n\n<!-- bake-readme:releases:start -->"));
    assert!(updated.contains("### v1.1.0\n\n- Faster builds."));
    assert!(!updated.contains("v1.0.0"));
}

#[test]
fn preserves_authored_releases_section() {
    let document = "# Project\n\n## Releases\n\nSee the project website for release notes.\n";
    let releases = [Release {
        name: "v1.2.0".to_owned(),
        notes: "- Recent change.".to_owned(),
    }];
    assert_eq!(
        update_document_with_releases(document, None, &releases),
        document
    );
}

#[test]
fn creates_see_also_from_cargo_package_metadata() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: Some("A useful library for examples.".to_owned()),
        repository: Some("https://github.com/example/example-crate".to_owned()),
    };
    let updated = update_document("# Example\n\n## Contributing\n", Some(&metadata));
    assert!(updated.contains(
		"- [example-crate](https://github.com/example/example-crate) — A useful library for examples. <!-- bake-readme:package -->"
	));
    assert!(updated.contains("See [releases.md](releases.md)"));
}

#[test]
fn refreshes_only_the_generated_entry_in_an_existing_see_also_section() {
    let original = "# Example\n\n## See Also\n\n- [Other](https://example.com)\n- [old-name](https://github.com/example/project) — Old description. <!-- bake-readme:package -->\n";
    let metadata = PackageMetadata {
        name: "new-name".to_owned(),
        description: Some("Updated description.".to_owned()),
        repository: Some("https://github.com/example/project".to_owned()),
    };
    let updated = update_document(original, Some(&metadata));
    assert!(updated.contains("- [Other](https://example.com)\n"));
    assert!(updated.contains(
		"- [new-name](https://github.com/example/project) — Updated description. <!-- bake-readme:package -->"
	));
    assert!(!updated.contains("Old description."));
}

#[test]
fn leaves_manual_see_also_content_untouched() {
    let original =
        "# Example\n\n## See Also\n\n- [Project source](https://github.com/example/project)\n";
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: Some("Description.".to_owned()),
        repository: Some("https://github.com/example/project".to_owned()),
    };
    let updated = update_document(original, Some(&metadata));
    assert_eq!(
        updated
            .matches("https://github.com/example/project")
            .count(),
        1
    );
}

#[test]
fn task_updates_a_custom_readme_under_project_root() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("project-notes.md");
    fs::write(&path, "# Project\n").unwrap();

    let registry = bake::Registry::discover().unwrap();
    let mut context = registry.context(directory.path());
    context
        .call("readme:update", &["--path", "project-notes.md"])
        .unwrap();
    let updated = fs::read_to_string(&path).unwrap();
    assert!(updated.contains("[releases.md](releases.md)"));

    context
        .call("readme:update", &["--path", "project-notes.md"])
        .unwrap();
    assert_eq!(fs::read_to_string(path).unwrap(), updated);
}

#[test]
fn task_reads_package_metadata_from_the_root_cargo_manifest() {
    let directory = tempfile::tempdir().unwrap();
    let manifest = directory.path().join("Cargo.toml");
    fs::write(
		manifest,
		"[package]\nname = \"metadata-project\"\nversion = \"0.1.0\"\nedition = \"2024\"\ndescription = \"A project with metadata.\"\nrepository = \"https://example.com/source\"\n",
	)
	.unwrap();
    fs::create_dir(directory.path().join("src")).unwrap();
    fs::write(directory.path().join("src/main.rs"), "fn main() {}\n").unwrap();
    let path = directory.path().join("readme.md");
    fs::write(&path, "# Example\n").unwrap();

    let registry = bake::Registry::discover().unwrap();
    let mut context = registry.context(directory.path());
    context.call("readme:update", &[]).unwrap();
    let updated = fs::read_to_string(path).unwrap();
    assert!(updated.contains(
		"- [metadata-project](https://example.com/source) — A project with metadata. <!-- bake-readme:package -->"
	));
}

#[test]
fn task_reads_recent_releases_from_the_project_root() {
    let directory = tempfile::tempdir().unwrap();
    let readme = directory.path().join("readme.md");
    fs::write(&readme, "# Example\n").unwrap();
    fs::write(
        directory.path().join("releases.md"),
        "# Releases\n\n## Unreleased\n\n- Not published.\n\n## v0.1.0\n\n- Initial release.\n",
    )
    .unwrap();

    let registry = bake::Registry::discover().unwrap();
    let mut context = registry.context(directory.path());
    context.call("readme:update", &[]).unwrap();
    let updated = fs::read_to_string(readme).unwrap();
    assert!(updated.contains("### v0.1.0\n\n- Initial release."));
    assert!(!updated.contains("### Unreleased"));
}
