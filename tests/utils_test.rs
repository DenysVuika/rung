use rung::get_top_lines;
use std::error::Error;
use std::io::Write;
use tempfile::NamedTempFile;

mod common;

#[test]
fn should_get_top_lines() -> Result<(), Box<dyn Error>> {
    common::setup();

    let mut file = NamedTempFile::new()?;
    writeln!(file, "one\ntwo\nthree")?;

    let lines = get_top_lines(file.path(), 2)?;

    assert_eq!(2, lines.len());
    assert_eq!("one", lines[0]);
    assert_eq!("two", lines[1]);

    Ok(())
}
