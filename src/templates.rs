use log::{error, info};
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
    pub fn get(&mut self, key: &str) -> Option<&String> {
        let loader = &self.loader;

        self.cache
            .entry(key.to_string())
            .or_insert_with(|| loader.load(&key.to_string()));

        self.cache.get(key)
    }

    /// Gets content as a vector of strings from the cache or file
    pub fn get_lines(&mut self, key: &str) -> Option<Vec<String>> {
        match &self.get(key) {
            Some(content) => {
                let result = content.lines().map(|line| line.to_string()).collect();
                return Some(result);
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestLoader {
        content: String,
    }

    impl TestLoader {
        fn new(content: &str) -> Box<TestLoader> {
            Box::new(TestLoader {
                content: content.to_string(),
            })
        }
    }

    impl TemplateLoader for TestLoader {
        fn load(&self, _path: &str) -> String {
            String::from(&self.content)
        }
    }

    #[test]
    fn returns_template_from_loader() {
        let loader = TestLoader::new("test template");
        let expected = String::from("test template");

        assert_eq!(
            TemplateManager::with_loader(loader).get("test.txt"),
            Some(&expected)
        );
    }

    #[test]
    fn returns_multiple_lines() {
        let loader = TestLoader::new("test\ntemplate");
        let content = TemplateManager::with_loader(loader)
            .get_lines("test.txt")
            .unwrap();

        assert_eq!(2, content.len());
        assert_eq!("test", content[0]);
        assert_eq!("template", content[1]);
    }
}
