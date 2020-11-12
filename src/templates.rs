use std::collections::HashMap;
use std::fs;

pub struct TemplateLoader {
    cache: HashMap<String, String>
}

impl TemplateLoader {
    pub fn new() -> TemplateLoader {
        TemplateLoader {
            cache: HashMap::new()
        }
    }

    pub fn get(&mut self, key: &str) -> Result<String, &str> {
        self.cache.entry(key.to_string()).or_insert_with(|| {
            println!("Loading template {}", &key);
            fs::read_to_string(&key).unwrap()
        });

        match self.cache.get(key) {
            Some(val) => Ok(val.to_string()),
            None => Err("Error loading template")
        }
    }

    pub fn get_lines(&mut self, key: &str) -> Result<Vec<String>, &str> {
        let content = self.get(key)?;

        let result = content
            .lines()
            .map(|line| line.to_string())
            .collect();

        Ok(result)
    }
}
