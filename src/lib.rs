mod utils;

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;
use std::error::Error;
use std::collections::HashMap;

struct TemplateLoader {
    cache: HashMap<String, String>
}

impl TemplateLoader {
    fn new() -> TemplateLoader {
        TemplateLoader {
            cache: HashMap::new()
        }
    }

    fn get(&mut self, key: &String) -> Result<String, &str> {
        self.cache.entry(key.to_string()).or_insert_with(|| {
            println!("Loading template {}", &key);
            fs::read_to_string(&key).unwrap()
        });

        match self.cache.get(key) {
            Some(val) => Ok(val.to_string()),
            None => Err("Error loading template")
        }
    }

    fn get_lines(&mut self, key: &str) -> Result<Vec<String>, &str> {
        let content = self.get(&key.to_string())?;

        let result = content
            .lines()
            .map(|line| line.to_string())
            .collect();

        Ok(result)
    }
}

/// Verify that files have headers according to the templates.
pub fn check_headers(files: &Vec<&str>, templates: &Vec<&str>) -> Result<bool, Box<dyn Error>> {
    println!(
        "checking headers of `{}` with templates `{}`",
        files.join(", "),
        templates.join(", ")
    );

    let mut loader = TemplateLoader::new();

    for file in files {
        let result = check_file_headers(&file, &templates, &mut loader)?;
        if result {
            println!("OK: {}", file);
        } else {
            eprintln!("Error: `{}` has invalid header", file);
        }
    }

    Ok(true)
}

fn check_file_headers(file: &str, templates: &Vec<&str>, loader: &mut TemplateLoader) -> Result<bool, Box<dyn Error>> {
    for template in templates {
        let equal = check_file_header(&file, &template, loader)?;
        // println!("EQ: {} | {} | {}", equal, file, template);

        if equal {
            return Ok(true);
        }
    }

    Ok(false)
}

fn check_file_header(file: &str, template: &str, loader: &mut TemplateLoader) -> Result<bool, Box<dyn Error>> {
    let template_lines = loader.get_lines(&template)?;
    let file_lines = get_file_header(file, template_lines.len())?;

    match utils::compare(&template_lines, &file_lines) {
        Ordering::Equal => Ok(true),
        _ => Ok(false)
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
