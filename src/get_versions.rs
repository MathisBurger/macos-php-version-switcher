use std::fs;

/// Gets all current php versions
pub fn get_versions() -> Vec<String> {
    let mut versions: Vec<String> = vec![];
    for file in fs::read_dir("/opt/homebrew/etc/php").unwrap() {
        let display = file.unwrap().path().display().to_string();
        let split: Vec<&str> = display.split("/").collect();
        versions.push(split.last().unwrap().to_string());
    }
    versions
}