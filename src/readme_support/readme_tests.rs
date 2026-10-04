// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use super::{TemporaryFile, prepare_temporary_file, write_document_atomically};
use std::fs;
use std::io::{self, Write};

#[derive(Clone, Copy)]
enum Failure {
    Write,
    Permissions,
    Sync,
}

struct FakeTemporaryFile(Failure);

impl Write for FakeTemporaryFile {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if matches!(self.0, Failure::Write) {
            Err(io::Error::other("injected write error"))
        } else {
            Ok(buffer.len())
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl TemporaryFile for FakeTemporaryFile {
    fn set_permissions(&self, _: fs::Permissions) -> io::Result<()> {
        if matches!(self.0, Failure::Permissions) {
            Err(io::Error::other("injected permissions error"))
        } else {
            Ok(())
        }
    }

    fn sync_all(&self) -> io::Result<()> {
        if matches!(self.0, Failure::Sync) {
            Err(io::Error::other("injected sync error"))
        } else {
            Ok(())
        }
    }
}

#[test]
fn reports_temporary_write_errors() {
    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("readme.md");
    fs::write(&target, "original").unwrap();

    assert!(
        prepare_temporary_file(&mut FakeTemporaryFile(Failure::Write), &target, "updated").is_err()
    );
}

#[test]
fn reports_temporary_permission_errors() {
    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("readme.md");
    fs::write(&target, "original").unwrap();

    assert!(
        prepare_temporary_file(
            &mut FakeTemporaryFile(Failure::Permissions),
            &target,
            "updated"
        )
        .is_err()
    );
}

#[test]
fn reports_temporary_sync_errors() {
    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("readme.md");
    fs::write(&target, "original").unwrap();

    assert!(
        prepare_temporary_file(&mut FakeTemporaryFile(Failure::Sync), &target, "updated").is_err()
    );
}

#[test]
fn flushes_temporary_file_writer() {
    let mut file = FakeTemporaryFile(Failure::Write);

    file.flush().unwrap();
}

#[test]
fn applies_permissions_and_syncs_successfully() {
    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("readme.md");
    fs::write(&target, "original").unwrap();
    let file = FakeTemporaryFile(Failure::Write);

    file.set_permissions(fs::metadata(&target).unwrap().permissions())
        .unwrap();
    file.sync_all().unwrap();
}

#[test]
fn reports_temporary_file_creation_errors() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("missing").join("readme.md");

    assert!(write_document_atomically(&path, "updated").is_err());
}

#[test]
fn reports_target_metadata_errors() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("missing.md");

    assert!(write_document_atomically(&path, "updated").is_err());
}

#[test]
fn reports_persist_errors_when_target_is_a_directory() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("readme.md");
    std::fs::create_dir(&path).unwrap();

    assert!(write_document_atomically(&path, "updated").is_err());
}
