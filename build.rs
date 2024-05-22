use std::{env, fs};
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=config");
    let binding = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target_dir = binding
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    fs::copy("config", target_dir.join("config")).unwrap();
}
