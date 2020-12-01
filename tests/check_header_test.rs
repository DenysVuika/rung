use anyhow::Result;
use rung::files::{check_headers, verify_files};
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

mod common;

#[test]
fn verifies_file() -> Result<()> {
    common::setup();

    let file = NamedTempFile::new()?;

    assert_eq!(true, verify_files(&vec![file.path()]));

    file.close()?;

    Ok(())
}

#[test]
fn fails_verifying_when_one_file_missing() -> Result<()> {
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
fn fails_to_verify_missing_file() {
    common::setup();

    assert_eq!(false, verify_files(&vec![Path::new("missing.txt")]));
}

#[test]
fn verifies_multiple_files() -> Result<()> {
    common::setup();

    let file1 = NamedTempFile::new()?;
    let file2 = NamedTempFile::new()?;

    assert_eq!(true, verify_files(&vec![file1.path(), file2.path()]));

    file1.close()?;
    file2.close()?;

    Ok(())
}

#[test]
fn passes_single_file_template() -> Result<()> {
    common::setup();

    let mut file = NamedTempFile::new()?;
    writeln!(file, "hello\nworld")?;

    let mut template = NamedTempFile::new()?;
    writeln!(template, "hello")?;

    assert_eq!(
        true,
        check_headers(&vec![file.path()], &vec![template.path()])
    );

    Ok(())
}

#[test]
fn passes_multiple_file_single_template() -> Result<()> {
    common::setup();

    let mut file1 = NamedTempFile::new()?;
    writeln!(file1, "hello\nworld")?;

    let mut file2 = NamedTempFile::new()?;
    writeln!(file2, "hello\nthere")?;

    let mut template = NamedTempFile::new()?;
    writeln!(template, "hello")?;

    assert_eq!(
        true,
        check_headers(&vec![file1.path(), file2.path()], &vec![template.path()])
    );

    Ok(())
}

#[test]
fn fails_multiple_files_single_template() -> Result<()> {
    common::setup();

    let mut file1 = NamedTempFile::new()?;
    writeln!(file1, "hello\nworld")?;

    let mut file2 = NamedTempFile::new()?;
    writeln!(file2, "hello\nthere")?;

    let mut template = NamedTempFile::new()?;
    writeln!(template, "something else")?;

    assert_eq!(
        false,
        check_headers(&vec![file1.path(), file2.path()], &vec![template.path()])
    );

    Ok(())
}

#[test]
fn fails_single_file_template() -> Result<()> {
    common::setup();

    let mut file = NamedTempFile::new()?;
    writeln!(file, "hello\nworld")?;

    let mut template = NamedTempFile::new()?;
    writeln!(template, "assets")?;

    assert_eq!(
        false,
        check_headers(&vec![file.path()], &vec![template.path()])
    );

    Ok(())
}

#[test]
fn passes_single_file_multiple_templates() -> Result<()> {
    common::setup();

    let mut file = NamedTempFile::new()?;
    writeln!(file, "hello\nworld")?;

    let mut template1 = NamedTempFile::new()?;
    writeln!(template1, "world")?;

    let mut template2 = NamedTempFile::new()?;
    writeln!(template2, "hello")?;

    assert_eq!(
        true,
        check_headers(
            &vec![file.path()],
            &vec![template1.path(), template2.path()]
        )
    );

    Ok(())
}

#[test]
fn fails_single_file_multiple_templates() -> Result<()> {
    common::setup();

    let mut file = NamedTempFile::new()?;
    writeln!(file, "hello\nworld")?;

    let mut template1 = NamedTempFile::new()?;
    writeln!(template1, "template1")?;

    let mut template2 = NamedTempFile::new()?;
    writeln!(template2, "template2")?;

    assert_eq!(
        false,
        check_headers(
            &vec![file.path()],
            &vec![template1.path(), template2.path()]
        )
    );

    Ok(())
}

#[test]
fn passes_multiple_files_multiple_templates() -> Result<()> {
    common::setup();

    let mut file1 = NamedTempFile::new()?;
    writeln!(file1, "hello\nworld")?;

    let mut file2 = NamedTempFile::new()?;
    writeln!(file2, "hey\nthere")?;

    let mut template1 = NamedTempFile::new()?;
    writeln!(template1, "hello")?;

    let mut template2 = NamedTempFile::new()?;
    writeln!(template2, "hey")?;

    assert_eq!(
        true,
        check_headers(
            &vec![file1.path(), file2.path()],
            &vec![template1.path(), template2.path()]
        )
    );

    Ok(())
}

#[test]
fn fails_multiple_files_multiple_templates() -> Result<()> {
    common::setup();

    let mut file1 = NamedTempFile::new()?;
    writeln!(file1, "hello\nworld")?;

    let mut file2 = NamedTempFile::new()?;
    writeln!(file2, "hey\nthere")?;

    let mut template1 = NamedTempFile::new()?;
    writeln!(template1, "something")?;

    let mut template2 = NamedTempFile::new()?;
    writeln!(template2, "else")?;

    assert_eq!(
        false,
        check_headers(
            &vec![file1.path(), file2.path()],
            &vec![template1.path(), template2.path()]
        )
    );

    Ok(())
}
