// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use super::{
    canonical_manifest, cargo_program, parse_cargo_metadata, read_project_package_with,
    read_project_package_with_canonicalizer, same_path,
};
use bake::Registry;
use std::ffi::OsString;
use std::path::Path;

#[test]
fn rejects_invalid_cargo_metadata_json() {
    let error = parse_cargo_metadata(b"not json", Path::new("/missing/Cargo.toml")).unwrap_err();

    assert!(error.to_string().contains("could not parse cargo metadata"));
}

#[test]
fn rejects_metadata_without_packages_array() {
    let error = parse_cargo_metadata(b"{}", Path::new("/missing/Cargo.toml")).unwrap_err();

    assert_eq!(
        error.to_string(),
        "cargo metadata did not return a packages array"
    );
}

#[test]
fn skips_packages_without_a_manifest_path() {
    let metadata = br#"{"packages":[{}]}"#;

    assert_eq!(
        parse_cargo_metadata(metadata, Path::new("/missing/Cargo.toml")).unwrap(),
        None
    );
}

#[test]
fn rejects_matching_package_without_a_name() {
    let directory = tempfile::tempdir().unwrap();
    let manifest = directory.path().join("Cargo.toml");
    std::fs::write(&manifest, "").unwrap();
    let canonical_manifest = manifest.canonicalize().unwrap();
    let metadata = format!(
        r#"{{"packages":[{{"manifest_path":"{}"}}]}}"#,
        canonical_manifest.display()
    );

    let error = parse_cargo_metadata(metadata.as_bytes(), &canonical_manifest).unwrap_err();

    assert_eq!(
        error.to_string(),
        "Cargo package metadata is missing its name"
    );
}

#[test]
fn returns_none_when_no_package_matches_the_manifest() {
    let directory = tempfile::tempdir().unwrap();
    let manifest = directory.path().join("Cargo.toml");
    std::fs::write(&manifest, "").unwrap();
    let canonical_manifest = manifest.canonicalize().unwrap();
    let metadata = br#"{"packages":[{"manifest_path":"/other/Cargo.toml","name":"other"}]}"#;

    assert_eq!(
        parse_cargo_metadata(metadata, &canonical_manifest).unwrap(),
        None
    );
}

#[test]
fn matches_original_paths_if_canonicalization_fails() {
    let directory = tempfile::tempdir().unwrap();
    let missing = directory.path().join("missing/Cargo.toml");

    assert!(same_path(&missing, &missing));
    assert!(!same_path(
        &missing,
        &directory.path().join("different/Cargo.toml")
    ));
}

#[test]
fn uses_the_configured_cargo_program_or_default() {
    assert_eq!(cargo_program(None), OsString::from("cargo"));
    assert_eq!(
        cargo_program(Some(OsString::from("custom-cargo"))),
        OsString::from("custom-cargo")
    );
}

#[test]
fn reports_manifest_canonicalization_errors() {
    let directory = tempfile::tempdir().unwrap();
    let missing = directory.path().join("missing/Cargo.toml");

    assert!(canonical_manifest(&missing).is_err());
}

#[test]
fn returns_none_when_the_project_has_no_manifest() {
    let directory = tempfile::tempdir().unwrap();
    let registry = Registry::discover().unwrap();
    let context = registry.context(directory.path());

    let package = read_project_package_with(&context, cargo_program(None)).unwrap();

    assert_eq!(package, None);
}

#[test]
fn propagates_manifest_canonicalization_errors() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("Cargo.toml"), "").unwrap();
    let registry = Registry::discover().unwrap();
    let context = registry.context(directory.path());

    let error = read_project_package_with_canonicalizer(&context, OsString::from("cargo"), |_| {
        Err(bake::Error::new("injected canonicalization error"))
    })
    .unwrap_err();

    assert_eq!(error.to_string(), "injected canonicalization error");
}

#[test]
fn reports_cargo_metadata_process_errors() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("Cargo.toml"),
        "this is not valid TOML[",
    )
    .unwrap();
    let registry = Registry::discover().unwrap();
    let context = registry.context(directory.path());
    let missing_cargo = directory.path().join("missing-cargo").into_os_string();

    let error = read_project_package_with(&context, missing_cargo).unwrap_err();

    assert!(
        error
            .to_string()
            .starts_with("could not run cargo metadata:")
    );
}

#[test]
fn reports_a_nonzero_cargo_metadata_exit_status() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("Cargo.toml"),
        "this is not valid TOML[",
    )
    .unwrap();
    let registry = Registry::discover().unwrap();
    let context = registry.context(directory.path());
    let cargo = cargo_program(None);

    let error = read_project_package_with(&context, cargo).unwrap_err();

    assert!(error.to_string().contains("cargo metadata failed"));
}

#[test]
fn reads_package_metadata_from_a_valid_manifest() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("Cargo.toml"),
        "[package]\nname = \"metadata-fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\ndescription = \"A metadata fixture.\"\nrepository = \"https://example.com/metadata-fixture\"\n",
    )
    .unwrap();
    std::fs::write(
        directory.path().join("Cargo.lock"),
        "version = 4\n\n[[package]]\nname = \"metadata-fixture\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    std::fs::create_dir(directory.path().join("src")).unwrap();
    std::fs::write(directory.path().join("src/lib.rs"), "").unwrap();
    let registry = Registry::discover().unwrap();
    let context = registry.context(directory.path());

    let package = read_project_package_with(&context, cargo_program(std::env::var_os("CARGO")))
        .unwrap()
        .unwrap();

    assert_eq!(package.name, "metadata-fixture");
    assert_eq!(package.description.as_deref(), Some("A metadata fixture."));
    assert_eq!(
        package.repository.as_deref(),
        Some("https://example.com/metadata-fixture")
    );
}
