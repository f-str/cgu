use crate::utils;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitConfig {
    pub sections: HashMap<String, HashMap<String, String>>,
}

impl GitConfig {
    pub fn new() -> GitConfig {
        GitConfig {
            sections: Default::default(),
        }
    }

    pub fn new_entry(&mut self, section_name: &str, entry_name: &str, entry_value: &str) {
        let section = match self.sections.get_mut(section_name) {
            Some(section) => section,
            None => {
                self.sections
                    .insert(section_name.to_string(), HashMap::new());
                self.sections.get_mut(section_name).unwrap()
            }
        };
        section.insert(entry_name.to_string(), entry_value.to_string());
    }

    pub fn to_config_string(&self) -> String {
        let mut config_string = String::new();

        for (section_name, section) in &self.sections {
            config_string.push_str(&format!("[{}]\n", section_name));

            for (key, value) in section {
                config_string.push_str(&format!("{} = {}\n", key, value));
            }

            config_string.push('\n');
        }

        config_string
    }

    pub fn from_config_string(config_string: &str) -> GitConfig {
        let mut git_config = GitConfig {
            sections: HashMap::new(),
        };

        let mut current_section = String::new();

        for line in config_string.lines() {
            let trimmed_line = line.trim();

            // Skip empty lines and comments
            if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
                continue;
            }

            if trimmed_line.starts_with('[') && trimmed_line.ends_with(']') {
                // Extract section name
                let section_name = trimmed_line[1..trimmed_line.len() - 1].to_owned();
                current_section = section_name.clone_from(&section_name);

                // Create section entry in the git_config
                git_config.sections.entry(section_name).or_default();
            } else {
                // Parse key-value pair
                if let Some((key, value)) = GitConfig::parse_key_value(trimmed_line) {
                    if let Some(section) = git_config.sections.get_mut(&current_section) {
                        section.insert(key, value);
                    }
                }
            }
        }

        git_config
    }

    fn parse_key_value(line: &str) -> Option<(String, String)> {
        let mut iter = line.splitn(2, '=');
        if let Some(key) = iter.next().map(str::trim) {
            if let Some(value) = iter.next().map(str::trim) {
                return Some((key.to_owned(), value.to_owned()));
            }
        }
        None
    }
}

pub fn generate_gitconfig(config: &GitConfig, path: &str) -> Result<()> {
    let generated_config = config.to_config_string();

    utils::write_to_file(path, &generated_config)
}

pub fn read_gitconfig(path: &str) -> Result<GitConfig> {
    match utils::read_from_file(path) {
        Ok(file_content) => Ok(GitConfig::from_config_string(&file_content)),
        Err(err) => Err(err),
    }
}
