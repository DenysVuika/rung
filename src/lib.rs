use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;
use std::error::Error;

/// Verify that files have headers according to the templates.
pub fn check_headers(files: &Vec<&str>, templates: &Vec<&str>) -> Result<bool, Box<dyn Error>> {
    println!(
        "checking headers of `{}` with templates `{}`",
        files.join(", "),
        templates.join(", ")
    );

    for file in files {
        let result = check_file_headers(&file, &templates)?;
        if result {
            println!("OK: {}", file);
        } else {
            eprintln!("Error: `{}` has invalid header", file);
        }
    }

    Ok(true)
}

fn get_template_lines(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let result = content
        .lines()
        .map(|line| line.to_string())
        .collect();

    Ok(result)
}

fn check_file_headers(file: &str, templates: &Vec<&str>) -> Result<bool, Box<dyn Error>> {
    for template in templates {
        let equal = check_file_header(&file, &template)?;
        // println!("EQ: {} | {} | {}", equal, file, template);

        if equal {
            return Ok(true);
        }
    }

    Ok(false)
}

fn check_file_header(file: &str, template: &str) -> Result<bool, Box<dyn Error>> {
    let template_lines = get_template_lines(template)?;
    let file_lines = get_file_header(file, template_lines.len())?;

    match compare(&template_lines, &file_lines) {
        Ordering::Equal => Ok(true),
        _ => Ok(false)
    }
}

fn get_file_header(path: &str, size: usize) -> Result<Vec<String>, Box<dyn Error>> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let mut result = Vec::new();

    for line in reader.lines().take(size) {
        result.push(line?);
    }

    Ok(result)
}

fn compare<T: Ord>(a: &[T], b: &[T]) -> Ordering {
    let mut iter_b = b.iter();
    for v in a {
        match iter_b.next() {
            Some(w) => match v.cmp(w) {
                Ordering::Equal => continue,
                ord => return ord,
            },
            None => break,
        }
    }
    return a.len().cmp(&b.len());
}
