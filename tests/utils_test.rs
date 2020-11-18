use anyhow::Result;
use rung::get_top_lines;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

mod common;

#[test]
fn should_get_top_lines() -> Result<()> {
    common::setup();

    let mut file = NamedTempFile::new()?;
    writeln!(file, "one\ntwo\nthree")?;

    let lines = get_top_lines(file.path(), 2);

    assert_eq!(2, lines.len());
    assert_eq!("one", lines[0]);
    assert_eq!("two", lines[1]);

    Ok(())
}

#[test]
fn should_not_fail_on_small_files() -> Result<()> {
    common::setup();

    let mut file = NamedTempFile::new()?;
    writeln!(file, "one\ntwo\nthree")?;

    let lines = get_top_lines(file.path(), 20);

    assert_eq!(3, lines.len());
    assert_eq!("one", lines[0]);
    assert_eq!("two", lines[1]);
    assert_eq!("three", lines[2]);

    Ok(())
}

#[test]
fn should_return_empty_array_for_missing_file() -> Result<()> {
    common::setup();

    let lines = get_top_lines(Path::new("missing.txt"), 2);

    assert_eq!(0, lines.len());

    Ok(())
}
