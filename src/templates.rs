use std::collections::HashMap;
use std::fs;
use log::{info,error};

/// Provides template loading and caching utilities
pub struct TemplateLoader {
    cache: HashMap<String, String>,
    loader: fn(&str) -> String,
}

/// Loads template from file or empty content if file not found
pub fn load_from_file(path: &str) -> String {
    info!("Loading template {}", &path);

    match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(err) => {
            error!("Error loading `{}`. {}", &path, err);
            String::new()
        }
    }
}

impl TemplateLoader {
    /// Creates a new instance with the default loader
    pub fn new() -> TemplateLoader {
        TemplateLoader {
            cache: HashMap::new(),
            loader: load_from_file
        }
    }

    /// Creates a new instance with custom loader
    #[allow(dead_code)]
    fn with_loader(loader: fn(&str) -> String) -> TemplateLoader {
        TemplateLoader {
            cache: HashMap::new(),
            loader
        }
    }

    /// Gets content from the cache or file
    pub fn get(&mut self, key: &str) -> Result<String, &str> {
        let loader = self.loader;
        self.cache.entry(key.to_string()).or_insert_with(|| loader(&key.to_string()));

        match self.cache.get(key) {
            Some(val) => Ok(val.to_string()),
            None => Err("Error loading template")
        }
    }

    /// Gets content as a vector of strings from the cache or file
    pub fn get_lines(&mut self, key: &str) -> Result<Vec<String>, &str> {
        let content = self.get(key)?;

        let result = content
            .lines()
            .map(|line| line.to_string())
            .collect();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn test_loader(_path: &str) -> String {
        "test template".to_string()
    }

    #[test]
    fn gets_template_from_loader() {
        let content = TemplateLoader::with_loader(test_loader)
            .get("test.txt").unwrap();
        assert_eq!(content, "test template".to_string());
    }

    /*
    #[test]
    fn loads_template_only_once() {
        let mut hits = Cell::new(0);
        let mut loader_fn = |_: &str| -> String {
            // hits.set(hits.get() + 1);
            "test template".to_string()
        };

        let mut loader = TemplateLoader::with_loader(loader_fn);

        loader.get("test.txt");
        loader.get("test.txt");
        loader.get("test.txt");

        assert_eq!(1, hits.get());
    }
    */
}
