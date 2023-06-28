use crate::gitconfig::GitConfig;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalGitConfig {
    pub name: String,
    pub index: usize,
    pub git_config: GitConfig,
}

impl InternalGitConfig {
    pub fn new(name: String, index: usize, git_config: GitConfig) -> InternalGitConfig {
        InternalGitConfig {
            name,
            index,
            git_config,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub active_config: Option<usize>,
    pub global_gitconfig_path: String,
    pub configs: Vec<InternalGitConfig>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            active_config: None,
            global_gitconfig_path: "~/.gitconfig".to_string(),
            configs: Vec::new(),
        }
    }

    pub fn read_config(path: &Path) -> Config {
        match fs::read_to_string(path) {
            Ok(contents) => match serde_json::from_str::<Config>(&contents) {
                Ok(config) => config,
                Err(err) => {
                    eprintln!("Failed to parse config file: {}", err);
                    Config::new()
                }
            },
            Err(err) => {
                eprintln!("Failed to read config file: {}", err);
                Config::new()
            }
        }
    }

    pub fn save_config(&self, path: &Path) {
        match serde_json::to_string_pretty(self) {
            Ok(contents) => match fs::create_dir_all(path.parent().unwrap()) {
                Ok(_) => match File::create(path) {
                    Ok(mut file) => {
                        if let Err(err) = file.write_all(contents.as_bytes()) {
                            eprintln!("Failed to write config file: {}", err);
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to create config file: {}", err);
                    }
                },
                Err(err) => {
                    eprintln!("Failed to create config directory: {}", err);
                }
            },
            Err(err) => {
                eprintln!("Failed to serialize config: {}", err);
            }
        }
    }
}
