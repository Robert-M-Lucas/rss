use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;
#[cfg(target_os = "windows")]
use crate::os_str_utils::Append;

pub fn write_binary(rss_file: &Path, binary: &[u8]) -> Result<(), String> {
    let file_name = rss_file.file_stem().unwrap();
    let directory = rss_file.parent().unwrap();

    #[cfg(target_os = "windows")]
    let exe_file = directory.join(file_name.to_os_string().append(OsStr::new(".exe")));

    #[cfg(target_os = "linux")]
    let exe_file = directory.join(file_name);

    fs::write(&exe_file, binary).map_err(|_| format!("Failed to write binary to {}", exe_file.display()))?;

    #[cfg(target_os = "linux")]
    Command::new("chmod").args([OsStr::new("+x"), exe_file.as_os_str()]).status().map_err(|_| format!("Failed to mark binary as executable {}", exe_file.display()))?;

    Ok(())
}

pub fn execute_binary(rss_file: &Path) -> Result<(), String> {
    let file_name = rss_file.file_stem().unwrap();
    let directory = rss_file.parent().unwrap();
    #[cfg(target_os = "windows")]
    let exe_file = directory.join(file_name.to_os_string().append(OsStr::new(".exe")));

    #[cfg(target_os = "linux")]
    let exe_file = directory.join(file_name);

    Command::new(&exe_file).status().map_err(|_| format!("Failed to execute binary {}", exe_file.display()))?;
    Ok(())
}

pub fn delete_binary(rss_file: &Path) -> Result<(), String> {
    let file_name = rss_file.file_stem().unwrap();
    let directory = rss_file.parent().unwrap();
    #[cfg(target_os = "windows")]
    let exe_file = directory.join(file_name.to_os_string().append(OsStr::new(".exe")));

    #[cfg(target_os = "linux")]
    let exe_file = directory.join(file_name);

    fs::remove_file(&exe_file).map_err(|_| format!("Failed to delete binary {}", exe_file.display()))?;
    Ok(())
}