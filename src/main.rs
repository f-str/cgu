mod commands;
mod config;
mod gitconfig;
mod utils;

use crate::config::Config;

fn main() {
    let config_path = dirs::config_dir()
        .expect("Failed to determine config directory.")
        .join("cgu")
        .join("config.json");

    let mut config = if config_path.exists() {
        Config::read_config(&config_path)
    } else {
        Config::new()
    };

    commands::parse_arguments(&mut config);

    config.save_config(&config_path);
}
