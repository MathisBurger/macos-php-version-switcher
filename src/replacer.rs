use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

/// Repleaces the old php version in zsh config
pub fn replace_php_version(config_path: String, old: String, new: String) {
    let mut content = fs::read_to_string(config_path.clone().as_str())
        .expect("Cannot read shell config");
    content = str::replace(
        content.clone().as_str(),
        format!("export PATH=\"/opt/homebrew/opt/php@{}/sbin:$PATH\"", current_version.clone()).as_str(),
        format!("export PATH=\"/opt/homebrew/opt/php@{}/sbin:$PATH\"", selected_version.clone()).as_str()
    );
    content = str::replace(
        content.clone().as_str(),
        format!("export PATH=\"/opt/homebrew/opt/php@{}/bin:$PATH\"", current_version.clone()).as_str(),
        format!("export PATH=\"/opt/homebrew/opt/php@{}/bin:$PATH\"", selected_version.clone()).as_str()
    );
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_path.as_str())
        .unwrap();
    file.write_all(content.as_str().as_ref())
        .expect("Cannot write data");
}