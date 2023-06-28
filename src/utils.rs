use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn read_from_file(path: &str) -> io::Result<String> {
    std::fs::read_to_string(Path::new(expand_path(path.to_string()).as_str()))
}

pub fn write_to_file(path: &str, config_string: &str) -> io::Result<()> {
    match File::create(Path::new(expand_path(path.to_string()).as_str())) {
        Ok(mut file) => match file.write_all(config_string.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub fn file_exists(path: &str) -> bool {
    Path::new(Path::new(expand_path(path.to_string()).as_str())).is_file()
}

fn expand_path(path: String) -> String {
    if path.starts_with('~') {
        let home = std::env::var("HOME").unwrap();
        return path.replacen('~', home.as_str(), 1);
    }
    path
}
