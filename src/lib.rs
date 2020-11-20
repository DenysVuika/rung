mod utils;

use log::{error, info};
use std::cmp::Ordering;

use anyhow::{Context, Result};
use jsonschema::JSONSchema;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub use utils::{get_lines, get_top_lines, verify_files};

pub fn read_json(path: &Path) -> Result<Value> {
    let file = File::open(path).context(format!("Error opening file: {}", path.display()))?;
    let reader = BufReader::new(file);
    let json_value = serde_json::from_reader(reader)
        .context(format!("Error reading from file: {}", path.display()))?;

    Ok(json_value)
}

pub fn validate_json(json_path: &Path, schema_path: &Path) -> Result<bool> {
    info!(
        "Validating `{}` with `{}`",
        json_path.display(),
        schema_path.display()
    );

    let instance = read_json(&json_path)?;
    let schema = read_json(&schema_path)?;
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
