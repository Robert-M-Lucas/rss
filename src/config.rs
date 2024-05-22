use std::path::{Path, PathBuf};
use std::fs;
use derive_getters::Getters;

pub enum Editor {
    Code,
    Nvim,
    Nano
}

impl Editor {
    fn from_str(s: &str) -> Result<Editor, String> {
        Ok(match s {
            "code" => Editor::Code,
            "nvim" => Editor::Nvim,
            "nano" => Editor::Nano,
            e => return Err(format!("Code editor [{e}] in config file not supported"))
        })
    }
}

#[derive(Getters)]
pub struct Config {
    check_hash: bool,
    base64: bool,
    new_dir: bool,
    editor: Editor
}

impl Config {
    pub fn location(self_location: &Path) -> PathBuf {
        self_location.join("config")
    }

    pub fn read(self_location: &Path) -> Result<Config, String> {
        let config_location = Self::location(self_location);
        let Ok(contents) = fs::read_to_string(&config_location) else {
            return Err(format!("Failed to open config file [{:?}]", config_location))
        };

        const OPTIONS_COUNT: usize = 4;
        let mut options = Vec::with_capacity(OPTIONS_COUNT);

        for i in 0..OPTIONS_COUNT {
            let f = format!("${i}=");
            let Some(location) = contents.find(&f) else {
                return Err(format!("Could not find option [{i}] in config file [{:?}]", config_location));
            };
            let contents = &contents[location + f.len()..];
            let contents = contents.lines().next().unwrap();
            options.push(contents);
        }

        let check_hash = options[0] == "true";
        let base64 = options[1] == "true";
        let new_dir = options[2] == "true";
        let editor = Editor::from_str(options[3])?;

        Ok(Config {
            check_hash,
            base64,
            new_dir,
            editor,
        })
    }
}
