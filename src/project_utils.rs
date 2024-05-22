use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;
#[cfg(target_os = "windows")]
use crate::os_str_utils::Append;

pub fn generate_project(rss_file: &Path, cargo_content: &str, rust_content: &str) -> Result<(), String> {
    let directory = rss_file.parent().unwrap();
    let src = directory.join("src");
    fs::create_dir(&src).map_err(|_| "Failed to create src directory".to_string())?;

    let main_file = src.join("main.rs");
    fs::write(&main_file, rust_content.as_bytes()).map_err(|_| "Failed to create main.rs".to_string())?;

    let cargo_file = directory.join("Cargo.toml");
    fs::write(&cargo_file, cargo_content.as_bytes()).map_err(|_| "Failed to create Cargo.toml".to_string())?;

    Ok(())
}

pub fn build_project(rss_file: &Path) -> Result<Vec<u8>, Result<(), String>> {
    let file_name = rss_file.file_stem().unwrap();
    let directory = rss_file.parent().unwrap();

    if !Command::new("cargo").args([OsStr::new("build"), OsStr::new("-r")]).current_dir(directory).status()
        .map_err(|e| {println!("{:?}", e); Err("Failed to run Cargo".to_string())})?.success() {
        return Err(Ok(()))
    }

    #[cfg(target_os = "windows")]
    return Ok(fs::read(directory.join("target").join("release")
        .join(file_name.to_os_string().append(OsStr::new(".exe")))
    ).map_err(|_| Err("Failed read built binary".to_string()))?);

    #[cfg(target_os = "linux")]
    return Ok(fs::read(directory.join("target").join("release")
        .join(file_name)
    ).map_err(|_| Err("Failed read built binary".to_string()))?);
}

pub fn get_cargo_and_source_project(rss_file: &Path) -> Result<(String, String), String> {
    let directory = rss_file.parent().unwrap();
    let main_file = directory.join("src").join("main.rs");
    let rust_content = fs::read_to_string(&main_file).map_err(|_| "Failed read src/main.rs".to_string())?;
    let cargo_file = directory.join("Cargo.toml");
    let cargo_content = fs::read_to_string(&cargo_file).map_err(|_| "Failed read Cargo.toml".to_string())?;

    Ok((cargo_content, rust_content))
}

pub fn delete_project(rss_file: &Path) -> Result<(), String> {
    let directory = rss_file.parent().unwrap();
    fs::remove_dir_all(directory.join("target")).map_err(|_| "Failed delete target".to_string())?;
    fs::remove_dir_all(directory.join("src")).map_err(|_| "Failed delete src".to_string())?;
    fs::remove_file(directory.join("Cargo.toml")).map_err(|_| "Failed delete Cargo.toml".to_string())?;
    fs::remove_file(directory.join("Cargo.lock")).map_err(|_| "Failed delete Cargo.lock".to_string())?;
    Ok(())
}