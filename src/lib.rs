//! # Rung
//!
//! `rung` is a collection of utilities to make working
//! with Angular CLI projects more convenient.

use clap::ArgMatches;
use log::{error, info};
use std::path::Path;
use std::process;

pub mod angular;
pub mod files;
pub mod json;
pub mod logger;
pub mod serve;
pub mod utils;

pub fn validate_json(args: &ArgMatches) {
    let file = args.value_of("file").unwrap();
    let file_path = Path::new(file);
    let template = args.value_of("template").unwrap();
    let template_path = Path::new(template);

    match json::validate_json(file_path, template_path) {
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
