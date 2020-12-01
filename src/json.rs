//! JSON utils

use anyhow::Result;
use jsonschema::JSONSchema;
use log::{error, info};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Reads and parses JSON from file
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

/// Validates JSON file with the JSON Schema file
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
