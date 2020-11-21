mod utils;

use log::{error, info};
use std::cmp::Ordering;

use anyhow::Result;
use jsonschema::JSONSchema;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub use utils::{get_lines, get_top_lines, verify_files};

pub fn read_json(path: &Path) -> Option<Value> {
    if !path.exists() {
        error!("File not found: {}", path.display());
        return None;
    }

    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            error!("Error opening file: {}", path.display());
            return None;
        }
    };

    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(value) => Some(value),
        Err(_) => {
            error!("Error reading from file: {}", path.display());
            None
        }
    }
}

pub fn validate_json(json_path: &Path, schema_path: &Path) -> Result<bool> {
    info!(
        "Validating `{}` with `{}`",
        json_path.display(),
        schema_path.display()
    );

    let instance = match read_json(&json_path) {
        Some(value) => value,
        None => return Ok(false),
    };

    let schema = match read_json(&schema_path) {
        Some(value) => value,
        None => return Ok(false),
    };

    let compiled = JSONSchema::compile(&schema)?;
    let result = compiled.validate(&instance);

    if let Err(errors) = result {
        for error in errors {
            error!("Validation error: {}", error);
        }

        return Ok(false);
    }

    Ok(true)
}

/// Verify that files have headers matching one of the templates.
pub fn check_headers(files: &[&Path], templates: &[&Path]) -> bool {
    if !verify_files(&files) {
        return false;
    }

    if !verify_files(&templates) {
        return false;
    }

    let result: bool = files
        .iter()
        .all(|file| compare_file_headers(file, &templates));

    result
}

fn compare_file_headers(file: &Path, templates: &[&Path]) -> bool {
    for template in templates {
        let template_lines = get_lines(&template);
        let file_lines = get_top_lines(file, template_lines.len());

        if Ordering::Equal == utils::compare(&template_lines, &file_lines) {
            return true;
        }
    }

    error!("Invalid header: {}", file.display());
    false
}
