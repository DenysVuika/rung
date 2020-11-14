mod templates;
mod utils;

use log::{error, info};
use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub use templates::TemplateManager;

/// Verifies that all files exist
pub fn verify_files(paths: &Vec<&str>) -> bool {
    paths.iter().all(|path| {
        let exists = fs::metadata(path).is_ok();
        if !exists {
            error!("`{}` not found", path);
        }
        exists
    })
}

/// Verify that files have headers according to the templates.
pub fn check_headers(files: &Vec<&str>, templates: &Vec<&str>) -> Option<bool> {
    info!(
        "checking headers of `{}` with templates `{}`",
        files.join(", "),
        templates.join(", ")
    );

    if !verify_files(&files) {
        let error = "One of the input files missing";
        error!("{}", error);
        return Some(false);
    }

    if !verify_files(&templates) {
        let error = "One of the templates is missing";
        error!("{}", error);
        return Some(false);
    }

    let mut loader = TemplateManager::new();
    let mut validation_result = true;

    for file in files {
        let result = match check_file_headers(&file, &templates, &mut loader) {
            Ok(val) => val,
            _ => false,
        };
        if result {
            info!("OK: {}", file);
        } else {
            error!("Invalid header: {}", file);
            validation_result = false;
        }
    }

    Some(validation_result)
}

fn check_file_headers(
    file: &str,
    templates: &Vec<&str>,
    loader: &mut TemplateManager,
) -> Result<bool, Box<dyn Error>> {
    for template in templates {
        let equal = check_file_header(&file, &template, loader)?;
        // debug!("EQ: {} | {} | {}", equal, file, template);

        if equal {
            return Ok(true);
        }
    }

    Ok(false)
}

fn check_file_header(
    file: &str,
    template: &str,
    loader: &mut TemplateManager,
) -> Result<bool, Box<dyn Error>> {
    let template_lines = match loader.get_lines(&template) {
        Some(lines) => lines,
        None => Vec::new(),
    };

    let file_lines = get_file_header(file, template_lines.len())?;

    match utils::compare(&template_lines, &file_lines) {
        Ordering::Equal => Ok(true),
        _ => Ok(false),
    }
}

fn get_file_header(path: &str, size: usize) -> Result<Vec<String>, Box<dyn Error>> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let result = reader
        .lines()
        .take(size)
        .map(|item| item.unwrap())
        .collect();

    Ok(result)
}

/*
#[cfg(test)]
mod tests {
    use super::*;
}
*/
