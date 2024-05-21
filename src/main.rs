use std::{env, fs, process};
use std::borrow::Borrow;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::process::Command;

trait Append<Segment : ?Sized> : Sized
    where
        Segment : ToOwned<Owned = Self>,
        Self : Borrow<Segment>,
{
    fn append (self: Self, s: impl AsRef<Segment>)
               -> Self
    ;
}

impl Append<OsStr> for OsString {
    fn append (mut self: OsString, s: impl AsRef<OsStr>)
               -> Self
    {
        self.push(s);
        self
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Specify file");
        process::exit(-1);
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
            process::exit(-1);
        }
    }
    else { false };

    let rss_file = PathBuf::from(file);
    if !rss_file.is_file() {
        println!("Not a file");
        process::exit(-1);
    }

    let directory = rss_file.parent().unwrap();
    let file_name = rss_file.file_stem().unwrap();

    if edit {
        let contents = fs::read(&rss_file).unwrap();

        let (cargo_toml, rust_content) = if contents.len() == 0 {
            (include_str!("default_cargo").replace("$$$$", file_name.to_str().unwrap()), include_str!("default_main").to_string())
        } else {
            let contents = &contents[..contents.len() - 2]; // Remove '*/'
            let (contents, compiled_length) = (&contents[..contents.len() - 4], &contents[contents.len() - 4..]);
            let compiled_length = u32::from_le_bytes(compiled_length.try_into().unwrap());
            let contents = &contents[..contents.len() - compiled_length as usize - 3]; // Remove '\n/*'
            let contents = String::from_utf8_lossy(contents);

            let (cargo_toml, rust_contents) = contents.split_at(contents.find("*/").unwrap());
            let (cargo_toml, rust_contents) = (&cargo_toml[2..], &rust_contents[3..]); // Remove '/*' and '*/\n'

            (cargo_toml.to_string(), rust_contents.to_string())
        };

        fs::create_dir(directory.join("src")).unwrap();

        let src = directory.join("src");
        let main_file = src.join("main.rs");
        fs::write(&main_file, rust_content.as_bytes()).unwrap();
        let cargo_file = directory.join("Cargo.toml");
        fs::write(&cargo_file, cargo_toml.as_bytes()).unwrap();

        Command::new("code.cmd").args([OsStr::new("-w"), directory.as_os_str()]).status().ok();

        Command::new("cargo").args([OsStr::new("build"), OsStr::new("-r")]).current_dir(directory).status().ok();

        let mut output_data: Vec<u8> = Vec::new();

        output_data.extend("/*".as_bytes());
        output_data.extend(&fs::read(&cargo_file).unwrap());
        output_data.extend("*/\n".as_bytes());
        output_data.extend(&fs::read(&main_file).unwrap());
        output_data.extend("\n/*".as_bytes());

        let target = directory.join("target");
        let compiled = fs::read(target.join("release")
            .join(file_name.to_os_string().append(OsStr::new(".exe")))
        ).unwrap();
        output_data.extend(&compiled);
        output_data.extend(&(compiled.len() as u32).to_le_bytes());
        output_data.extend("*/".to_string().as_bytes());

        fs::write(&rss_file, &output_data).unwrap();

        fs::remove_dir_all(&target).unwrap();
        fs::remove_dir_all(&src).unwrap();
        fs::remove_file(&cargo_file).unwrap();
        fs::remove_file(directory.join("Cargo.lock")).unwrap();
    }
    else {
        let compiled = fs::read(&rss_file).unwrap();
        if compiled.is_empty() {
            println!("Empty file");
            process::exit(-1);
        }
        let compiled = &compiled[..compiled.len() - 2]; // Remove '*/'
        let (compiled, compiled_length) = (&compiled[..compiled.len() - 4], &compiled[compiled.len() - 4..]);
        let compiled_length = u32::from_le_bytes(compiled_length.try_into().unwrap());
        let compiled = &compiled[compiled.len() - compiled_length as usize..]; // Remove '\n/*'
        let exe_file = directory.join(file_name.to_os_string().append(OsStr::new(".exe")));
        fs::write(&exe_file, compiled).unwrap();
        Command::new(&exe_file).status().ok();
        fs::remove_file(&exe_file).unwrap();
    }
}
