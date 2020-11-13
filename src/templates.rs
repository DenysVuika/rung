use log::{error, info};
use std::cell::Cell;
use std::collections::HashMap;
use std::fs;

pub trait TemplateLoader {
    fn load(&self, path: &str) -> String;
}

pub struct FileLoader {}

impl FileLoader {
    pub fn new() -> Box<FileLoader> {
        Box::new(FileLoader {})
    }
}

impl TemplateLoader for FileLoader {
    fn load(&self, path: &str) -> String {
        info!("Loading template {}", &path);

        match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                error!("Error loading `{}`. {}", &path, err);
                String::new()
            }
        }
    }
}

pub struct TestLoader {
    content: String,
    // load_calls: u32,
    load_calls: Cell<u32>,
}

impl TestLoader {
    #[allow(dead_code)]
    fn new(content: &str) -> Box<TestLoader> {
        Box::new(TestLoader {
            content: content.to_string(),
            // load_calls: 0,
            load_calls: Cell::new(0),
        })
    }
}

impl TemplateLoader for TestLoader {
    fn load(&self, _path: &str) -> String {
        self.load_calls.set(self.load_calls.get() + 1);
        String::from(&self.content)
    }
}

/// Provides template loading and caching utilities
pub struct TemplateManager {
    cache: HashMap<String, String>,
    loader: Box<dyn TemplateLoader>,
}

impl TemplateManager {
    /// Creates a new instance with the default loader
    pub fn new() -> TemplateManager {
        TemplateManager {
            cache: HashMap::new(),
            loader: FileLoader::new(),
        }
    }

    /// Creates a new instance with a specific loader
    #[allow(dead_code)]
    pub fn with_loader(loader: Box<dyn TemplateLoader>) -> TemplateManager {
        TemplateManager {
            cache: HashMap::new(),
            loader,
        }
    }

    /// Gets content from the cache or file
    pub fn get(&mut self, key: &str) -> Result<String, &str> {
        let loader = &self.loader;
        self.cache
            .entry(key.to_string())
            .or_insert_with(|| loader.load(&key.to_string()));

        match self.cache.get(key) {
            Some(val) => Ok(val.to_string()),
            None => Err("Error loading template"),
        }
    }

    /// Gets content as a vector of strings from the cache or file
    pub fn get_lines(&mut self, key: &str) -> Result<Vec<String>, &str> {
        let content = &self.get(key)?;

        let result = content.lines().map(|line| line.to_string()).collect();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_template_from_loader() {
        let content = TemplateManager::with_loader(TestLoader::new("test template"))
            .get("test.txt")
            .unwrap();
        assert_eq!(content, "test template".to_string());
    }

    /*
    #[test]
    fn loads_template_only_once() {
        let loader = TestLoader::new("test template");
        let mut manager = TemplateManager::with_loader(loader);

        manager.get("test.txt");
        manager.get("test.txt");
        manager.get("test.txt");

        // assert_eq!(1, loader.load_calls.get());
    }

     */
}
