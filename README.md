# CGU - ChangeGitUser

CGU (**C**hange**G**it**U**ser) is a command-line application written in Rust that allows you to manage multiple local git configuration files with ease. It provides various commands to Create, List, Delete, Import, and cycle through git configurations.

## Features

- Create and manage your local git configurations
- Quickly switch between different git configurations with just the `cgu` command
- Import existing git configurations from files.


## Installation

To use CGU, you need to have Rust installed on your system. If you don't have Rust installed, you can follow the official Rust installation guide [here](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can proceed with the following steps to install CGU:

1. Clone the repository or download the source code.
2. Open a terminal and navigate to the project directory.
3. Run the command `cargo build --release` to build the application.
4. The compiled binary will be available in the `target/release` directory.
5. You can either run the binary directly or add it to your system's PATH for convenient access.


## Usage

CGU provides the following commands:

- `create`: Create a new git configuration.
- `list`: List all present git configurations.
- `delete <name>`: Delete a specific git configuration by name.
- `import <filepath>`: Import a git configuration file.
- Running the application without any command will cycle through the existing configurations and set the active configuration.

### Creating a Configuration

To Create a new git configuration, use the following command:

```shell
cgu create
```

You will be prompted to enter the required information such as name and email for the new configuration. Once entered, the configuration will be saved.

### Listing Configurations

To List all the present git configurations, use the following command:

```shell
cgu list
```

This will display a List of all the configurations along with their names and emails.

### Deleting a Configuration

To Delete a specific git configuration by name, use the following command:

```shell
cgu delete <name>
```

Replace `<name>` with the name of the configuration you want to Delete. This will remove the specified configuration from the List.

### Importing a Configuration

To Import a git configuration file, use the following command:

```shell
cgu import <filepath>
```

Replace `<filepath>` with the path to the configuration file you want to Import. This will add the configuration from the file to the List of configurations.

### Cycling Configurations

Running the CGU application without any command will cycle through the existing configurations and set the active configuration. This allows you to quickly switch between different git configurations.


## Configuration Storage

CGU stores the git configurations in a JSON file located at `~/.config/cgu/config.json`. Each configuration is represented as a JSON object within an array in the file.


## Roadmap

- [ ] Release on the [AUR](https://aur.archlinux.org/)
- [ ] Renaming configurations
- [ ] Exporting configurations
- [ ] GPG Key-Selection support
- [ ] Support for Windows and macOS
- [ ] Change cycle order
- [ ] integrating [fzf](https://github.com/junegunn/fzf) as alternative to cycling
- [ ] Colorful outputs


## Contributions

Contributions to CGU are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or submit a pull request.


## License

CGU is released under the [MIT License](LICENSE).
