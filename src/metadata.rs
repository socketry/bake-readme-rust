// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use crate::PackageMetadata;
use bake::{Context, Error, Result};
use serde_json::Value;
use std::env;
use std::path::Path;

pub(crate) fn read_project_package(context: &Context) -> Result<Option<PackageMetadata>> {
    let manifest_path = context.root().join("Cargo.toml");
    if !manifest_path.is_file() {
        return Ok(None);
    }

    let canonical_manifest = manifest_path.canonicalize()?;
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output = context
        .command(cargo)
        .args(["metadata", "--no-deps", "--locked", "--format-version", "1"])
        .arg("--manifest-path")
        .arg(&manifest_path)
        .output()?;

    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr);
        return Err(Error::new(format!(
            "cargo metadata failed: {}",
            message.trim()
        )));
    }

    let metadata: Value = serde_json::from_slice(&output.stdout)
        .map_err(|error| Error::new(format!("could not parse cargo metadata: {error}")))?;
    let Some(packages) = metadata.get("packages").and_then(Value::as_array) else {
        return Err(Error::new("cargo metadata did not return a packages array"));
    };

    for package in packages {
        let Some(path) = package.get("manifest_path").and_then(Value::as_str) else {
            continue;
        };
        if !same_path(Path::new(path), &canonical_manifest) {
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
