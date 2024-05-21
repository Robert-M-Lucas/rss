use std::{env, fs};
use std::path::PathBuf;

fn main() -> i16 {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Specify file");
        return -1;
    }

    let mut args = args.into_iter();
    args.next();
    let file = args.next().unwrap();

    let edit = if let Some(a) = args.next() {
        if &a == "edit" {
            true
        }
        else {
            println!("Unknown option: {a}");
            return -1;
        }
    }
    else { false };

    println!("File: {file}");

    let file = PathBuf::from(file);
    if !file.is_file() {
        println!("Not a file");
        return -1;
    }

    let directory = file.parent().unwrap();
    let contents = fs::read(&file).unwrap();



    0
}
