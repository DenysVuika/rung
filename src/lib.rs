mod templates;
mod utils;

use log::error;
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

/// Verify that files have headers matching one of the templates.
pub fn check_headers(files: &Vec<&Path>, templates: &Vec<&Path>) -> bool {
    if !verify_files(&files) {
        return false;
    }

    if !verify_files(&templates) {
        return false;
    }

    let mut loader = TemplateManager::new();
    let mut validation_result = true;

    for file in files {
        let result = check_file_headers(file, &templates, &mut loader);
        if !result {
            let file_path = file.to_str().unwrap();
            error!("Invalid header: {}", file_path);
            validation_result = false;
        }
    }

    validation_result
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
