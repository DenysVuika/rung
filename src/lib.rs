mod utils;

use log::{error, info};
use std::cmp::Ordering;

use std::fs::read_to_string;
use std::path::Path;
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

    let mut validation_result = true;

    for file in files {
        let result = check_file_headers(file, &templates);
        if !result {
            let file_path = file.to_str().unwrap();
            error!("Invalid header: {}", file_path);
            validation_result = false;
        }
    }

    validation_result
}

fn get_lines(path: &Path) -> Vec<String> {
    let path_str = path.to_str().unwrap();
    info!("Loading {}", path_str);

    match read_to_string(&path) {
        Ok(content) => content.lines().map(|line| line.to_string()).collect(),
        Err(err) => {
            error!("Error loading `{}`. {}", path_str, err);
            vec![]
        }
    }
}

fn check_file_headers(file: &Path, templates: &Vec<&Path>) -> bool {
    for template in templates {
        let template_lines = get_lines(&template);
        let file_lines = get_top_lines(file, template_lines.len());

        if Ordering::Equal == utils::compare(&template_lines, &file_lines) {
            return true;
        }
    }

    false
}
