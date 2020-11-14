use rung::verify_files;
use std::error::Error;
use tempfile::tempdir;

mod common;

#[test]
fn should_validate_file() -> Result<(), Box<dyn Error>> {
    common::setup();

    let dir = tempdir()?;
    let file = common::create_temp_file(&dir, "file-1.txt", "test")?;

    let file_path = &dir.path().join("file-1.txt");
    let file_path = file_path.to_str().unwrap();

    assert_eq!(true, verify_files(&vec![file_path]));

    drop(file);
    dir.close()?;

    Ok(())
}

#[test]
fn should_fail_validation_when_one_file_missing() -> Result<(), Box<dyn Error>> {
    common::setup();

    let dir = tempdir()?;
    let file = common::create_temp_file(&dir, "file-1.txt", "test")?;

    let file_path = &dir.path().join("file-1.txt");
    let file_path = file_path.to_str().unwrap();

    assert_eq!(false, verify_files(&vec![file_path, "missing-file.txt"]));

    drop(file);
    dir.close()?;

    Ok(())
}

#[test]
fn should_not_validate_missing_file() {
    common::setup();

    assert_eq!(false, verify_files(&vec!["missing-file.txt"]));
}

#[test]
fn should_validate_multiple_files() -> Result<(), Box<dyn Error>> {
    common::setup();

    let dir = tempdir()?;

    let file1 = common::create_temp_file(&dir, "file-1.txt", "test")?;
    let file2 = common::create_temp_file(&dir, "file-2.txt", "test")?;

    let file1_path = &dir.path().join("file-1.txt");
    let file1_path = file1_path.to_str().unwrap();

    let file2_path = &dir.path().join("file-2.txt");
    let file2_path = file2_path.to_str().unwrap();

    assert_eq!(true, verify_files(&vec![file1_path, file2_path]));

    drop(file1);
    drop(file2);
    dir.close()?;

    Ok(())
}
