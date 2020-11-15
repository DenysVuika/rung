use rung::verify_files;
use std::error::Error;
use std::path::Path;
use tempfile::NamedTempFile;

mod common;

#[test]
fn should_validate_file() -> Result<(), Box<dyn Error>> {
    common::setup();

    let file = NamedTempFile::new()?;

    assert_eq!(true, verify_files(&vec![file.path()]));

    file.close()?;

    Ok(())
}

#[test]
fn should_fail_validation_when_one_file_missing() -> Result<(), Box<dyn Error>> {
    common::setup();

    let file = NamedTempFile::new()?;

    assert_eq!(
        false,
        verify_files(&vec![file.path(), Path::new("missing.txt")])
    );

    file.close()?;

    Ok(())
}

#[test]
fn should_not_validate_missing_file() {
    common::setup();

    assert_eq!(false, verify_files(&vec![Path::new("missing.txt")]));
}

#[test]
fn should_validate_multiple_files() -> Result<(), Box<dyn Error>> {
    common::setup();

    let file1 = NamedTempFile::new()?;
    let file2 = NamedTempFile::new()?;

    assert_eq!(true, verify_files(&vec![file1.path(), file2.path()]));

    file1.close()?;
    file2.close()?;

    Ok(())
}
