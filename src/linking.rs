

/// Unlinks the given php version
pub fn unlink_php_version(version: String) {
    std::process::Command::new("brew")
        .args(vec!["unlink", format!("php@{}", version).as_str()])
        .output()
        .expect("Cannot unlink installed php version");
}

/// Links the given php version
pub fn link_php_version(version: String) {
    std::process::Command::new("brew")
        .args(vec!["link", format!("php@{}", version).as_str()])
        .output()
        .expect("Cannot link installed php version");
}