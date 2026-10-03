// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use crate::PackageMetadata;
use bake::{Context, Error, Result};
use serde_json::Value;
use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub(crate) fn read_project_package(context: &Context) -> Result<Option<PackageMetadata>> {
    let cargo = cargo_program(env::var_os("CARGO"));
    read_project_package_with(context, cargo)
}

fn cargo_program(configured: Option<OsString>) -> OsString {
    configured.unwrap_or_else(|| "cargo".into())
}

fn read_project_package_with(
    context: &Context,
    cargo: OsString,
) -> Result<Option<PackageMetadata>> {
    read_project_package_with_canonicalizer(context, cargo, canonical_manifest)
}

fn read_project_package_with_canonicalizer(
    context: &Context,
    cargo: OsString,
    canonicalize: fn(&Path) -> Result<PathBuf>,
) -> Result<Option<PackageMetadata>> {
    let manifest_path = context.root().join("Cargo.toml");
    if !manifest_path.is_file() {
        return Ok(None);
    }

    let canonical_manifest = canonicalize(&manifest_path)?;
    let output = context
        .command(cargo)
        .args(["metadata", "--no-deps", "--locked", "--format-version", "1"])
        .arg("--manifest-path")
        .arg(&manifest_path)
        .output()
        .map_err(|error| Error::new(format!("could not run cargo metadata: {error}")))?;

    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr);
        return Err(Error::new(format!(
            "cargo metadata failed: {}",
            message.trim()
        )));
    }

    parse_cargo_metadata(&output.stdout, &canonical_manifest)
}

fn canonical_manifest(path: &Path) -> Result<PathBuf> {
    Ok(path.canonicalize()?)
}

fn parse_cargo_metadata(
    stdout: &[u8],
    canonical_manifest: &Path,
) -> Result<Option<PackageMetadata>> {
    let metadata: Value = serde_json::from_slice(stdout)
        .map_err(|error| Error::new(format!("could not parse cargo metadata: {error}")))?;
    let Some(packages) = metadata.get("packages").and_then(Value::as_array) else {
        return Err(Error::new("cargo metadata did not return a packages array"));
    };

    for package in packages {
        let Some(path) = package.get("manifest_path").and_then(Value::as_str) else {
            continue;
        };
        if !same_path(Path::new(path), canonical_manifest) {
            continue;
        }

        let name = package
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::new("Cargo package metadata is missing its name"))?;
        return Ok(Some(PackageMetadata {
            name: name.to_owned(),
            description: package
                .get("description")
                .and_then(Value::as_str)
                .map(str::to_owned),
            repository: package
                .get("repository")
                .and_then(Value::as_str)
                .map(str::to_owned),
        }));
    }

    Ok(None)
}

fn same_path(left: &Path, right: &Path) -> bool {
    left.canonicalize()
        .map_or_else(|_| left == right, |canonical_left| canonical_left == right)
}

#[cfg(test)]
#[path = "metadata_tests.rs"]
mod tests;
