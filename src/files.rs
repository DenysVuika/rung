//! # File utils
//!
//! Provides various utilities to work with files.

use crate::utils;
use clap::ArgMatches;
use log::{error, info};
use std::cmp::Ordering;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;

pub fn run_header_check(args: &ArgMatches) {
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

    let result = check_headers(&files, &templates);
    if result {
        info!("Validation succeeded");
        process::exit(0);
    } else {
        error!("Validation failed");
        process::exit(1);
    }
}

/// Verifies that files have headers matching one of the templates.
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

/// Verifies that all files exist
pub fn verify_files(paths: &[&Path]) -> bool {
    paths.iter().all(|path| {
        let exists = path.exists();
        if !exists {
            error!("`{}` not found", path.display());
        }
        exists
    })
}

/// Returns certain amount of lines from the top of the file
pub fn get_top_lines(path: &Path, size: usize) -> Vec<String> {
    let input = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            error!("Error opening file {}", path.display());
            return vec![];
        }
    };
    BufReader::new(input)
        .lines()
        .take(size)
        .map(|item| item.unwrap())
        .collect()
}

/// Returns the content of the file as a collection of lines
pub fn get_lines(path: &Path) -> Vec<String> {
    match read_to_string(&path) {
        Ok(content) => content.lines().map(|line| line.to_string()).collect(),
        Err(err) => {
            error!("Error loading `{}`. {}", path.display(), err);
            vec![]
        }
    }
}
