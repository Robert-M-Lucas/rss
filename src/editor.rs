use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use crate::config::{Config, Editor};

pub fn start_editor_blocking(config: &Config, rss_file: &Path) -> Result<(), String> {
    let directory = rss_file.parent().unwrap();
    match config.editor() {
        Editor::Code => {
            Command::new("code.cmd").args([OsStr::new("-w"), directory.as_os_str()]).status()
                .map_err(|_| "Failed to start VS Code".to_string())?;
        }
    };

    Ok(())
}