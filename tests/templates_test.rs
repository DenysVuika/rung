use rung::TemplateManager;
use std::error::Error;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

mod common;

#[test]
fn should_load_template_from_file() -> Result<(), Box<dyn Error>> {
    common::setup();

    let mut file = NamedTempFile::new()?;
    write!(file, "hello\nworld")?;

    let file_path = file.path().to_str().unwrap();
    let mut template_manager = TemplateManager::new();

    let content = template_manager.get(Path::new(file_path));
    let expected = String::from("hello\nworld");

    assert_eq!(Some(&expected), content);

    file.close()?;

    Ok(())
}
