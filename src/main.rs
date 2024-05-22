use std::{env, fs, path, process};
use std::path::PathBuf;
use std::vec::IntoIter;
use config::Config;
use crate::binary_utils::{delete_binary, execute_binary, write_binary};
use crate::editor::start_editor_blocking;
use crate::project_utils::{build_project, delete_project, generate_project, get_cargo_and_source_project};
use crate::rss_utils::{build_rss, check_file, get_binary_rss, get_cargo_and_source_rss};

mod config;
mod rss_utils;
mod project_utils;
mod editor;
mod binary_utils;
mod os_str_utils;

const HELP_TEXT: &'static str = include_str!("help_text");

fn print_err_exit(s: Option<&str>, help_text: bool) -> ! {
    if let Some(s) = s {
        println!("{s}");
        if help_text { println!(); }
    }
    if help_text {
        println!("{HELP_TEXT}");
    }
    process::exit(-1)
}

fn get_file(args: &mut IntoIter<String>, generate: bool) -> Result<PathBuf, String> {
    if let Some(file) = args.next() {
        let f = path::absolute(PathBuf::from(file)).unwrap();
        if generate && !f.is_file() {
            fs::write(&f, &[]).map_err(|_| format!("Failed to create file [{}]", f.display()))?;
        }
        Ok(f)
    } else {
        Err("This command requires a file argument that has not been provided".to_string())
    }
}

fn main() {
    let mut args = env::args().collect::<Vec<_>>().into_iter();

    let self_location = PathBuf::from(args.next().unwrap()).parent().unwrap().to_owned();

    let command = args.next().unwrap_or_else(|| print_err_exit(None, true));

    let config = Config::read(&env::current_exe().unwrap().parent().unwrap()).unwrap_or_else(|e| print_err_exit(Some(&e), false));

    match command.as_str() {
        "help" | "h" => {
            println!("{HELP_TEXT}");
        }
        "edit" | "e" => {
            let rss_file = get_file(&mut args, true).unwrap_or_else(|e| print_err_exit(Some(&e), false));
            check_file(&rss_file).unwrap_or_else(|e| print_err_exit(Some(&e), false));

            let (cargo_content, rust_content) = get_cargo_and_source_rss(&rss_file).unwrap_or_else(|e| print_err_exit(Some(&e), false));

            generate_project(&rss_file, &cargo_content, &rust_content).unwrap_or_else(|e| print_err_exit(Some(&e), false));

            start_editor_blocking(&config, &rss_file).unwrap_or_else(|e| print_err_exit(Some(&e), false));

            let binary;
            loop {
                match build_project(&rss_file) {
                    Ok(b) => {
                        binary = b;
                        break;
                    },
                    Err(Ok(_)) => {}
                    Err(Err(e)) => print_err_exit(Some(&e), false)
                };

                println!("Failed Cargo build, reopening editor");
                start_editor_blocking(&config, &rss_file).unwrap_or_else(|e| print_err_exit(Some(&e), false));
            }


            let (cargo_content, rust_content) = get_cargo_and_source_project(&rss_file).unwrap_or_else(|e| print_err_exit(Some(&e), false));

            build_rss(&config, &rss_file, &cargo_content, &rust_content, &binary).unwrap_or_else(|e| print_err_exit(Some(&e), false));

            println!("RSS file updated");

            delete_project(&rss_file).unwrap_or_else(|e| print_err_exit(Some(&e), false));

            println!("Cleaned project files");
        }
        "run" | "r" => {
            let rss_file = get_file(&mut args, false).unwrap_or_else(|e| print_err_exit(Some(&e), false));
            check_file(&rss_file).unwrap_or_else(|e| print_err_exit(Some(&e), false));

            let contents = get_binary_rss(&rss_file).unwrap_or_else(|e| print_err_exit(Some(&e), false));

            write_binary(&rss_file, &contents).unwrap_or_else(|e| print_err_exit(Some(&e), false));
            drop(contents);

            execute_binary(&rss_file).unwrap_or_else(|e| print_err_exit(Some(&e), false));
            delete_binary(&rss_file).unwrap_or_else(|e| print_err_exit(Some(&e), false));
        }
        "config" | "c" => {
            println!("Config file location: {}", Config::location(&self_location).display());
        }
        c => {
            print_err_exit(Some(&format!("Unrecognised command: {c}")), true);
        }
    }
}
