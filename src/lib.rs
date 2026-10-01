// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

//! Reusable `readme.md` maintenance tasks for Bake.
//!
//! The task adds a short summary of recent entries from `releases.md` when a
//! project `readme.md` does not already contain an authored Releases section. When
//! the project root has a Cargo package manifest, it also adds or refreshes a
//! generated package entry in the `readme.md` See Also section when the source
//! repository link is not already present elsewhere in the readme.
mod document;
mod metadata;

pub use document::{
    PackageMetadata, Release, ensure_releases_section, recent_releases, update_document,
    update_document_with_releases, update_releases_section,
};

/// Tasks exported by this package register beneath the `readme` namespace.
pub mod readme {
    use bake::{Context, Error, Result};
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    use super::{metadata::read_project_package, recent_releases, update_document_with_releases};

    /// Add recent release notes and Cargo package links to `readme.md` when needed.
    #[bake::task]
    pub fn update(
        context: &mut Context,
        #[bake(
            default = "readme.md",
            help = "readme file relative to the project root."
        )]
        path: PathBuf,
    ) -> Result<()> {
        let path = context.root().join(path).canonicalize()?;
        let document = fs::read_to_string(&path)
            .map_err(|error| Error::new(format!("{}: {error}", path.display())))?;
        let package = read_project_package(context)?;
        let release_path = context.root().join("releases.md");
        let releases = if release_path.is_file() {
            let release_document = fs::read_to_string(&release_path)
                .map_err(|error| Error::new(format!("{}: {error}", release_path.display())))?;
            recent_releases(&release_document)
        } else {
            Vec::new()
        };
        let updated = update_document_with_releases(&document, package.as_ref(), &releases);

        if updated == document {
            return Ok(());
        }

        let directory = path
            .parent()
            .ok_or_else(|| Error::new("readme file has no parent directory"))?;
        let mut temporary = tempfile::NamedTempFile::new_in(directory)?;
        temporary.write_all(updated.as_bytes())?;
        temporary
            .as_file()
            .set_permissions(fs::metadata(&path)?.permissions())?;
        temporary.as_file().sync_all()?;
        temporary
            .persist(&path)
            .map_err(|error| Error::from(error.error))?;
        Ok(())
    }
}
