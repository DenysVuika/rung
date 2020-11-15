use log::{error, info};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub trait TemplateLoader {
    fn load(&self, path: &Path) -> String;
}

pub struct FileLoader {}

impl FileLoader {
    pub fn new() -> Box<FileLoader> {
        Box::new(FileLoader {})
    }
}

impl TemplateLoader for FileLoader {
    fn load(&self, path: &Path) -> String {
        let path_str = &path.to_str().unwrap();
        info!("Loading template {}", path_str);

        match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                error!("Error loading `{}`. {}", path_str, err);
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
    pub fn get(&mut self, key: &Path) -> Option<&String> {
        let loader = &self.loader;
        let file_path = String::from(key.to_str().unwrap());

        self.cache
            .entry(file_path)
            .or_insert_with(|| loader.load(&key));

        self.cache.get(&String::from(key.to_str().unwrap()))
    }

    /// Gets content as a vector of strings from the cache or file
    pub fn get_lines(&mut self, key: &Path) -> Option<Vec<String>> {
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
        fn load(&self, _path: &Path) -> String {
            String::from(&self.content)
        }
    }

    #[test]
    fn returns_template_from_loader() {
        let loader = TestLoader::new("test template");
        let expected = String::from("test template");

        assert_eq!(
            TemplateManager::with_loader(loader).get(Path::new("test.txt")),
            Some(&expected)
        );
    }

    #[test]
    fn returns_multiple_lines() {
        let loader = TestLoader::new("test\ntemplate");
        let content = TemplateManager::with_loader(loader)
            .get_lines(Path::new("test.txt"))
            .unwrap();

        assert_eq!(2, content.len());
        assert_eq!("test", content[0]);
        assert_eq!("template", content[1]);
    }
}
