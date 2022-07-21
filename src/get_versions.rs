use std::fs;
use std::process::Stdio;

/// Gets all installed php versions
pub fn get_versions() -> Vec<String> {
    let mut versions: Vec<String> = vec![];
    for file in fs::read_dir("/opt/homebrew/etc/php").unwrap() {
        let display = file.unwrap().path().display().to_string();
        let split: Vec<&str> = display.split("/").collect();
        versions.push(split.last().unwrap().to_string());
    }
    versions
}

/// Gets the current php version
pub fn get_current_version() -> String {
    let current_version_output = std::process::Command::new("php")
        .args(vec!["-v"])
        .stdout(Stdio::piped())
        .output()
        .expect("Cannot get current php version");
    let current_version_res = String::from_utf8(current_version_output.stdout).expect("Cannot parse");
    let current_version_split: Vec<&str> = current_version_res.split(" ").collect();
    let version_split: Vec<&str> = current_version_split.get(1).unwrap().to_string().split(".").collect();
    return format!("{}.{}", version_split.get(0).unwrap(), version_split.get(1).unwrap());
}