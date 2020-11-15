mod templates;
mod utils;

use log::{error, info};
use std::cmp::Ordering;
use std::error::Error;
use std::fs;

use std::path::Path;
pub use templates::TemplateManager;
pub use utils::get_top_lines;

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
        let file_path = Path::new(file);
        let result = match check_file_headers(file_path, &templates, &mut loader) {
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
    file: &Path,
    templates: &Vec<&str>,
    loader: &mut TemplateManager,
) -> Result<bool, Box<dyn Error>> {
    for template in templates {
        let template_path = Path::new(template);
        let equal = check_file_header(&file, template_path, loader)?;
        // debug!("EQ: {} | {} | {}", equal, file, template);

        if equal {
            return Ok(true);
        }
    }

    Ok(false)
}

fn check_file_header(
    file: &Path,
    template: &Path,
    loader: &mut TemplateManager,
) -> Result<bool, Box<dyn Error>> {
    let template_lines = match loader.get_lines(&template) {
        Some(lines) => lines,
        None => Vec::new(),
    };

    let file_lines = get_top_lines(file, template_lines.len())?;

    match utils::compare(&template_lines, &file_lines) {
        Ordering::Equal => Ok(true),
        _ => Ok(false),
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
}
*/
