use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use crate::config::{Config, Editor};

pub fn start_editor_blocking(config: &Config, rss_file: &Path) -> Result<(), String> {
    let mut directory = rss_file.parent().unwrap().to_path_buf();
    let file_name = rss_file.file_stem().unwrap();
    if *config.new_dir() {
        directory = directory.join(file_name);
    }

    match config.editor() {
        Editor::Code => {
            #[cfg(target_os = "windows")]
            Command::new("code.cmd").args([OsStr::new("-w"), directory.as_os_str()]).status()
                .map_err(|_| "Failed to start VS Code".to_string())?;
            #[cfg(target_os = "linux")]
            Command::new("code").args([OsStr::new("-w"), directory.as_os_str()]).status()
                .map_err(|_| "Failed to start VS Code".to_string())?;
        }
    };

    Ok(())
}