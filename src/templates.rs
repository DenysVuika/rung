use std::collections::HashMap;
use std::fs;
use log::{info};

/// Provides template loading and caching utilities
pub struct TemplateLoader {
    cache: HashMap<String, String>
}

impl TemplateLoader {
    /// Creates a new instance of the loader and initializes the cache
    pub fn new() -> TemplateLoader {
        TemplateLoader {
            cache: HashMap::new()
        }
    }

    /// Gets content from the cache or file
    pub fn get(&mut self, key: &str) -> Result<String, &str> {
        self.cache.entry(key.to_string()).or_insert_with(|| {
            info!("Loading template {}", &key);
            fs::read_to_string(&key).unwrap()
        });

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
