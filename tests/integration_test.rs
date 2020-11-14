use rung::verify_files;
use std::error::Error;
use tempfile::NamedTempFile;

mod common;

#[test]
fn should_validate_file() -> Result<(), Box<dyn Error>> {
    common::setup();

    let file = NamedTempFile::new()?;
    let file_path = file.path().to_str().unwrap();

    assert_eq!(true, verify_files(&vec![file_path]));

    file.close()?;

    Ok(())
}

#[test]
fn should_fail_validation_when_one_file_missing() -> Result<(), Box<dyn Error>> {
    common::setup();

    let file = NamedTempFile::new()?;
    let file_path = file.path().to_str().unwrap();

    assert_eq!(false, verify_files(&vec![file_path, "missing.txt"]));

    file.close()?;

    Ok(())
}

#[test]
fn should_not_validate_missing_file() {
    common::setup();

    assert_eq!(false, verify_files(&vec!["missing.txt"]));
}

#[test]
fn should_validate_multiple_files() -> Result<(), Box<dyn Error>> {
    common::setup();

    let file1 = NamedTempFile::new()?;
    let file1_path = file1.path().to_str().unwrap();

    let file2 = NamedTempFile::new()?;
    let file2_path = file2.path().to_str().unwrap();

    assert_eq!(true, verify_files(&vec![file1_path, file2_path]));

    file1.close()?;
    file2.close()?;

    Ok(())
}
