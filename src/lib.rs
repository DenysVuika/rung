//! # Rung
//!
//! `rung` is a collection of utilities to make working
//! with Angular CLI projects more convenient.

pub mod angular;
pub mod files;
pub mod json;
pub mod logger;
pub mod serve;
pub mod utils;

use anyhow::Result;
use clap::ArgMatches;
use log::{error, info};
use std::path::{Path, PathBuf};
use std::process;

/// Validates JSON with the Schema
pub fn validate_json(args: &ArgMatches) {
    let file = args.value_of("file").unwrap();
    let file_path = Path::new(file);
    let template = args.value_of("template").unwrap();
    let template_path = Path::new(template);

    match json::validate_with_schema(file_path, template_path) {
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

/// Load workspace configuration file using CLI args
pub fn get_workspace_config(args: &ArgMatches) -> Result<angular::WorkspaceConfig> {
    let config_path = match args.value_of("config") {
        Some(value) => PathBuf::from(value),
        None => std::env::current_dir()?.join("angular.json"),
    };

    angular::read_config(config_path)
}

pub fn check_files_headers(args: &ArgMatches) {
    let files: Vec<_> = args
        .values_of("file")
        .unwrap()
        .map(|path| Path::new(path))
        .collect();
    let templates: Vec<_> = args
        .values_of("template")
        .unwrap()
        .map(|path| Path::new(path))
        .collect();

    let result = files::check_headers(&files, &templates);
    if result {
        info!("Validation succeeded");
        process::exit(0);
    } else {
        error!("Validation failed");
        process::exit(1);
    }
}
