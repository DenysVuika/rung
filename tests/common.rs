use env_logger;
use std::fs::File;
use std::io::{Error, Write};
use tempfile::TempDir;

pub fn setup() {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
    let _ = env_logger::builder().is_test(true).try_init();
}

pub fn create_temp_file(dir: &TempDir, name: &str, content: &str) -> Result<File, Error> {
    let file_path = &dir.path().join(name);
    let mut file = File::create(&file_path)?;

    writeln!(file, "{}", content)?;

    Ok(file)
}
