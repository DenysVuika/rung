use anyhow::Result;
use clap::ArgMatches;
use jsonschema::JSONSchema;
use log::{error, info};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process;

pub fn run(args: &ArgMatches) {
    let file = args.value_of("file").unwrap();
    let file_path = Path::new(file);
    let template = args.value_of("template").unwrap();
    let template_path = Path::new(template);

    match validate_json(file_path, template_path) {
        Ok(true) => {
            info!("Validation succeeded");
            process::exit(0);
        }
        Ok(false) => {
            info!("Validation failed");
            process::exit(1);
        }
        Err(err) => {
            error!("{}", err);
            process::exit(1);
        }
    }
}

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
