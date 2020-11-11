use std::fs;
use std::fs::File;
use std::io::{BufRead, Error, BufReader};

/// Verify that files have headers according to the templates.
pub fn check_headers(files: Vec<&str>, templates: Vec<&str>) {
    println!(
        "checking headers of `{}` with templates `{}`",
        files.join(", "),
        templates.join(", ")
    );

    println!(
        "result: {}",
        check_file_header(files[0], templates[0]).unwrap()
    );
}

fn get_template_lines(path: &str) -> Result<Vec<String>, Error> {
    let content = fs::read_to_string(path)?;
    let result = content
        .lines()
        .map(|line| line.to_string())
        .collect();

    Ok(result)
}

fn check_file_header(file: &str, template: &str) -> Result<bool, Error> {
    let template = get_template_lines(template)?;
    let lines = get_file_header(file, template.len())?;

    match compare(&template, &lines) {
        std::cmp::Ordering::Equal => Ok(true),
        _ => Ok(false)
    }
}

fn get_file_header(path: &str, size: usize) -> Result<Vec<String>, Error> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let mut result = Vec::new();

    for line in reader.lines().take(size) {
        result.push(line?);
    }

    Ok(result)
}

fn compare<T: Ord>(a: &[T], b: &[T]) -> std::cmp::Ordering {
    let mut iter_b = b.iter();
    for v in a {
        match iter_b.next() {
            Some(w) => match v.cmp(w) {
                std::cmp::Ordering::Equal => continue,
                ord => return ord,
            },
            None => break,
        }
    }
    return a.len().cmp(&b.len());
}
