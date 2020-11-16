mod templates;
mod utils;

use log::{error, info};
use std::cmp::Ordering;

use std::path::Path;
pub use templates::TemplateManager;
pub use utils::get_top_lines;

/// Verifies that all files exist
pub fn verify_files(paths: &Vec<&Path>) -> bool {
    paths.iter().all(|path| {
        let exists = path.exists();
        if !exists {
            error!("`{}` not found", path.to_str().unwrap());
        }
        exists
    })
}

/// Verify that files have headers according to the templates.
pub fn check_headers(files: &Vec<&Path>, templates: &Vec<&Path>) -> Option<bool> {
    if !verify_files(&files) {
        return Some(false);
    }

    if !verify_files(&templates) {
        return Some(false);
    }

    let mut loader = TemplateManager::new();
    let mut validation_result = true;

    for file in files {
        let file_path = Path::new(file);
        let result = check_file_headers(file_path, &templates, &mut loader);
        let file_path = file_path.to_str().unwrap();

        if result {
            info!("OK: {}", file_path);
        } else {
            error!("Invalid header: {}", file_path);
            validation_result = false;
        }
    }

    Some(validation_result)
}

fn check_file_headers(file: &Path, templates: &Vec<&Path>, loader: &mut TemplateManager) -> bool {
    for template in templates {
        let template_lines = loader.get_lines(&template);
        let file_lines = get_top_lines(file, template_lines.len());

        if Ordering::Equal == utils::compare(&template_lines, &file_lines) {
            return true;
        }
    }

    false
}

/*
#[cfg(test)]
mod tests {
    use super::*;
}
*/
