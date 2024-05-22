use std::fs;
use std::path::Path;

use crate::config::Config;

pub fn check_file(rss_file: &Path) -> Result<(), String> {
    if !rss_file.is_file() {
        Err(format!("Input file [{}] is not a file", rss_file.display()))
    }
    else {
        Ok(())
    }
}

pub fn get_cargo_and_source_rss(rss_file: &Path) -> Result<(String, String), String> {
    let file_name = rss_file.file_stem().unwrap();

    let contents = fs::read(&rss_file).map_err(|_| format!("Failed read [{}]", rss_file.display()))?;

    if contents.is_empty() {
        return Ok((
            include_str!("default_cargo").replace("$$$$", file_name.to_str().unwrap()),
            include_str!("default_main").to_string()
        ));
    } else {
        'err_loop: loop {
            if contents.len() < 7 {
                break;
            }
            let contents = &contents[..contents.len() - 2]; // Remove raw / base64) and '*/'
            let is_b64 = contents[contents.len() - 1];
            let contents = &contents[..contents.len() - 1];

            let (contents, compiled_length) = if is_b64 == 98 {
                let mut i = contents.len() - 1;
                loop {
                    if contents[i] == 58 {
                        break;
                    }
                    if i == 0 {
                        break 'err_loop;
                    }
                    i -= 1;
                }
                let (contents, compiled_length) = (&contents[..i], &contents[i+1..]);
                let Ok(compiled_length) = String::from_utf8_lossy(compiled_length).parse() else { break; };
                (contents, compiled_length)
            } else {
                let (contents, compiled_length) = (&contents[..contents.len() - 4], &contents[contents.len() - 4..]);
                let compiled_length = u32::from_le_bytes(compiled_length.try_into().unwrap()) as usize;
                (contents, compiled_length)
            };

            if contents.len() < compiled_length as usize + 3 {
                break;
            }
            let contents = &contents[..contents.len() - compiled_length as usize - 3]; // Remove '\n/*'
            let contents = String::from_utf8_lossy(contents);

            let Some(toml_end) = contents.find("*/") else { break; };
            let (cargo_toml, rust_contents) = contents.split_at(toml_end);

            if cargo_toml.len() < 2 { break; }
            let (cargo_toml, rust_contents) = (&cargo_toml[2..], &rust_contents[3..]); // Remove '/*' and '*/\n'

            return Ok((cargo_toml.to_string(), rust_contents.to_string()));
        }
        return Err("Improperly formatted rss file".to_string());
    }
}

pub fn get_binary_rss(rss_file: &Path) -> Result<Vec<u8>, String> {
    let compiled = fs::read(&rss_file).map_err(|_| format!("Failed read [{}]", rss_file.display()))?;

    if compiled.is_empty() {
        return Err("RSS file is empty - cannot run".to_string());
    }

    'err_loop: loop {
        if compiled.len() < 7 {
            break;
        }
        let compiled = &compiled[..compiled.len() - 2]; // Remove '*/'
        let is_b64 = compiled[compiled.len() - 1];
        let compiled = &compiled[..compiled.len() - 1];

        let (compiled, compiled_length) = if is_b64 == 98 {
            let mut i = compiled.len() - 1;
            loop {
                if compiled[i] == 58 {
                    break;
                }
                if i == 0 {
                    break 'err_loop;
                }
                i -= 1;
            }
            let (compiled, compiled_length) = (&compiled[..i], &compiled[i+1..]);
            let Ok(compiled_length) = String::from_utf8_lossy(compiled_length).parse() else { break; };
            (compiled, compiled_length)
        } else {
            let (compiled, compiled_length) = (&compiled[..compiled.len() - 4], &compiled[compiled.len() - 4..]);
            let compiled_length = u32::from_le_bytes(compiled_length.try_into().unwrap()) as usize;
            (compiled, compiled_length)
        };


        if compiled_length > compiled.len() {
            break;
        }
        let compiled = &compiled[compiled.len() - compiled_length..];

        return if is_b64 == 98 {
            let Ok(decoded) = base64::decode(compiled) else {
                break;
            };
            Ok(Vec::from(decoded))
        } else {
            Ok(Vec::from(compiled))
        }
    }
    return Err("Improperly formatted rss file".to_string());
}

pub fn build_rss(config: &Config, rss_file: &Path, cargo_content: &str, rust_content: &str, binary: &[u8]) -> Result<(), String> {
    let mut output_data: Vec<u8> = Vec::new();

    output_data.extend("/*".as_bytes());
    output_data.extend(cargo_content.as_bytes());
    output_data.extend("*/\n".as_bytes());
    output_data.extend(rust_content.as_bytes());
    output_data.extend("\n/*".as_bytes());

    if *config.base64() {
        let b64 = base64::encode(binary);
        output_data.extend(b64.as_bytes());
        output_data.push(58);
        output_data.extend(format!("{}", b64.as_bytes().len()).as_bytes());
    }
    else {
        output_data.extend(binary);
        output_data.extend(&(binary.len() as u32).to_le_bytes());
    }

    if *config.base64() {
        output_data.push(98);
    }
    else {
        output_data.push(114);
    }

    output_data.extend("*/".to_string().as_bytes());

    fs::write(&rss_file, &output_data).map_err(|_| format!("Failed write to [{}]", rss_file.display()))?;

    Ok(())
}