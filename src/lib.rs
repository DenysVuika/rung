mod utils;

use log::error;
use std::cmp::Ordering;

use std::path::Path;
pub use utils::{get_lines, get_top_lines, verify_files};

/// Verify that files have headers matching one of the templates.
pub fn check_headers(files: &Vec<&Path>, templates: &Vec<&Path>) -> bool {
    if !verify_files(&files) {
        return false;
    }

    if !verify_files(&templates) {
        return false;
    }

    files
        .iter()
        .all(|file| check_file_headers(file, &templates))
}

fn check_file_headers(file: &Path, templates: &Vec<&Path>) -> bool {
    for template in templates {
        let template_lines = get_lines(&template);
        let file_lines = get_top_lines(file, template_lines.len());

        if Ordering::Equal == utils::compare(&template_lines, &file_lines) {
            return true;
        }
    }

    let file_path = file.to_str().unwrap();
    error!("Invalid header: {}", file_path);
    false
}
