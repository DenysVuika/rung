mod utils;

use log::{error, info};
use std::cmp::Ordering;

use std::path::Path;
pub use utils::{get_lines, get_top_lines, verify_files};

pub fn validate_json(json: &Path, schema: &Path) -> bool {
    info!(
        "Validate `{}` with `{}`",
        json.to_str().unwrap(),
        schema.to_str().unwrap()
    );
    return true;
}

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
        .all(|file| compare_file_headers(file, &templates))
}

fn compare_file_headers(file: &Path, templates: &Vec<&Path>) -> bool {
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
