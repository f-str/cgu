use crate::config::{Config, InternalGitConfig};
use crate::gitconfig::{generate_gitconfig, read_gitconfig, GitConfig};
use crate::utils::file_exists;
use clap::{Parser, Subcommand};
use std::io::{self, Error, Write};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a new gitconfig
    Create,

    /// Edit a config
    Edit {
        /// Name of the config to Edit
        name: String,
    },

    /// List all present configs
    List,

    /// Delete a config
    Delete {
        /// Name of the config to Delete
        name: String,
    },

    /// Import a config file
    Import {
        /// Path to the config file to Import
        filepath: String,
    },
}

pub fn parse_arguments(config: &mut Config) {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create) => {
            create_config(config);
        }
        Some(Commands::List) => {
            list_configs(config);
        }
        Some(Commands::Edit { name }) => {
            edit_config(config, name);
        }
        Some(Commands::Delete { name }) => {
            delete_config(config, name);
        }
        Some(Commands::Import { filepath }) => {
            import_config(config, filepath);
        }
        None => {
            cycle_configs(config);
        }
    }
}

fn create_config(config: &mut Config) {
    print!("Enter the name for the new config: ");
    let mut name = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    let gitconfig = create_gitconfig();
    config.configs.push(InternalGitConfig::new(
        name,
        config.configs.len() + 1,
        gitconfig,
    ));

    println!("Config created successfully!");
}

fn create_gitconfig() -> GitConfig {
    let mut new_config = GitConfig::new();

    print!("Enter your name for the new config: ");
    let mut username = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    new_config.new_entry("user", "name", username.trim());

    print!("Enter the email for the new config: ");
    let mut email = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).unwrap();
    new_config.new_entry("user", "email", email.trim());

    loop {
        print!("Do you want to set more custom entries? (y/n): ");

        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "y" => {
                break match custom_edit(&new_config) {
                    Ok(config) => config,
                    Err(err) => {
                        eprintln!("Failed to create config: {}", err);
                        new_config
                    }
                }
            }
            "n" => break new_config,
            _ => println!("Invalid choice. Please choose again."),
        };
    }
}

fn custom_edit(new_config: &GitConfig) -> Result<GitConfig, Error> {
    let config_string = new_config.to_config_string();
    println!("Opening editor...");
    let edited = match edit::edit(config_string) {
        Ok(edited) => edited,
        Err(err) => {
            eprintln!("Failed to open editor: {}", err);
            return Err(err);
        }
    };
    Ok(GitConfig::from_config_string(&edited))
}

fn edit_config(config: &mut Config, name: &str) {
    let index = config.configs.iter().position(|cfg| cfg.name == name);

    if let Some(index) = index {
        let gitconfig = config.configs[index].git_config.clone();
        match custom_edit(&gitconfig) {
            Ok(edited_config) => {
                config.configs[index].git_config = edited_config;
                println!("Config '{}' edited successfully!", name);
            }
            Err(err) => {
                eprintln!("Failed to edit config: {}", err);
            }
        }
    } else {
        println!("Config '{}' not found.", name);
    }
}

fn list_configs(config: &Config) {
    for (index, cfg) in config.configs.iter().enumerate() {
        println!("{}: {}", index, cfg.name);
    }
}

fn delete_config(config: &mut Config, name: &str) {
    let index = config.configs.iter().position(|cfg| cfg.name == name);

    if let Some(index) = index {
        config.configs.remove(index);
    } else {
        println!("Config '{}' not found.", name);
    }
}

fn import_config(config: &mut Config, filepath: &str) {
    if file_exists(filepath) {
        match read_gitconfig(filepath) {
            Ok(imported_config) => {
                println!("Enter the name for the imported config: ");
                let mut input = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();

                config.configs.push(InternalGitConfig {
                    name: input.trim().to_string(),
                    index: config.configs.len(),
                    git_config: imported_config,
                });

                println!("Config imported successfully!");
            }
            Err(err) => {
                eprintln!("Failed to Import config: {}", err);
            }
        }
    } else {
        eprintln!("Config file '{}' not found.", filepath);
    }
}

fn cycle_configs(config: &mut Config) {
    if config.configs.is_empty() {
        handle_no_configs(config);
    } else {
        backup_active_config(config);
    }

    cycle_active_config(config);
}

fn handle_no_configs(config: &mut Config) {
    println!("No configs found.");

    loop {
        println!("1. Create a new config");
        println!("2. Import a config file");
        println!("Choose an option: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => break create_config(config),
            "2" => {
                break {
                    println!("Enter the path to the config file: ");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let filepath = input.trim();
                    import_config(config, filepath);
                }
            }
            _ => println!("Invalid choice. Please choose again."),
        };
    }
}

fn cycle_active_config(config: &mut Config) {
    if (config.configs.len() == 1) && (config.active_config.is_some()) {
        println!("Only one config found. No need to cycle.");
    } else {
        if let Some(active_index) = config.active_config {
            let new_index = (active_index + 1) % config.configs.len();
            config.active_config = Some(new_index);
        } else {
            config.active_config = Some(0);
        }
        write_active_config(config);
    }
}

fn backup_active_config(config: &mut Config) {
    let path = config.global_gitconfig_path.as_str();
    if file_exists(path) {
        if let Some(active_index) = config.active_config {
            match read_gitconfig(path) {
                Ok(global_gitconfig) => {
                    config.configs[active_index].git_config = global_gitconfig;
                }
                Err(err) => {
                    eprintln!("Failed to parse global config: {}", err);
                }
            }
        }
    }
}

fn write_active_config(config: &mut Config) {
    if let Some(active_index) = config.active_config {
        let active_config = &config.configs[active_index];
        match generate_gitconfig(
            &active_config.git_config,
            config.global_gitconfig_path.as_str(),
        ) {
            Ok(_) => {
                println!("Config '{}' activated successfully!", active_config.name);
            }
            Err(err) => {
                eprintln!("Failed to activate config: {}", err);
            }
        }
    } else {
        println!("No active config found.");
    }
}
