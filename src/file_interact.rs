use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;

use crate::constants::{DEFAULT_CONFIG, DEFAULT_KEYBINDS};
use crate::user_interact::{Cursor, Keybinds, ModDataAction, UtilAction, Action};
use crate::render::Window;
use crate::user_prompt::user_prompt; 


/// Reads the data from file.
pub(crate) fn read_text_file(path: &str) -> Vec<Vec<char>> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Could not open text file for reading.")
    };

    //println!("wat?{}wat?", path);
    let mut data: Vec<Vec<char>> = vec!(vec!());
    let mut buf: String = "".to_string();
    let _ = match file.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("Failed to read file to string")
    };
    let inter_data: Vec<char> = buf.chars().collect();
    
    let mut line_index: usize = 0;
    for i in inter_data {
        if i != '\n' {
            data[line_index].push(i);

        } else {
            data.push(vec!());
            line_index += 1;
        }
    };
    data
}

pub(crate) fn check_save_file(path: &str, data: &Vec<Vec<char>>) -> Action {
    if path == "" {
        return Action::UtilAction(UtilAction::AskSave);
    }

    let mut printed: String = "Failed to save to file!".to_string();
    match write_text_file(&path, &data) {
        Ok(_) => {printed = format!("Saved to file {}", path);},
        Err(_) => ()
    } 
    return Action::PrintResult(printed)
}

/// Saves the data to file
pub(crate) fn write_text_file(path: &str, data: &Vec<Vec<char>>) -> Result<(), std::io::Error> {
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(_) => panic!("Could not open text file for writing.")
    };

    let mut out: String = "".to_string();
    for i in data {
        for j in i {
            out += &j.to_string();
        }
        out += "\n"
    }
    let _ = out.pop();
    file.write_all(&out.as_bytes())?;
    
    Ok(())
}

pub(crate) fn write_configs<T: serde::Serialize>(path: &str, keybinds: T) -> Result<(), std::io::Error>  {
    let mut file = File::create(path)?;
    let buf: String = format_json(serde_json::to_string(&keybinds).unwrap());
    file.write_all(&buf.as_bytes())?;
    Ok(())  
} 

pub(crate) fn read_configs<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, std::io::Error>  {
    let mut file: File = File::open(path)?;
    let mut buf: String = "".to_string();
    let _ = file.read_to_string(&mut buf)?;
    let buf: String = unformat_json(buf);
    
    let out: T = serde_json::from_str(&buf)?;
    Ok(out)
}

fn format_json(buf: String) -> String {
    fn indent(offset: i32, indentation: usize) -> String {
        format!("\n{}", " ".repeat((3*indentation as i32 + 3*offset) as usize))
    }
    let mut out: String = "".to_string();
    let mut indentation: usize = 0;
    let mut buf: std::str::Chars<'_> = buf.chars();
    loop {
        let i = match buf.next() {
            Some(i) => i,
            _ => break
        };

        if i == '{' {
            out += &format!("{}{}{}", indent(0, indentation), "{", indent(1, indentation));
            indentation += 1;    
            continue
        }
        if  i == '}' {
            let j: char = match buf.next() {
                Some(j) => j,
                _ => '\n'
            };
            if j == ',' {
                out += &format!("{}{}{}", indent(-1, indentation), "},", indent(-1, indentation));
            } else {
                out += &format!("{}{}{}", indent(-1, indentation), "}", indent(-1, indentation));  
                out += &j.to_string();
            }
            indentation -= 1;
            continue
        }
        if i == ':' {
            out += ": ";
            continue
        }
        if i == ',' {
            out += ", ";
            continue
        }
        out += &i.to_string();
    }
    out
}

/// Removes spaces, newlines.
/// # is counted as a comment, and everything to the next newline is removed.
fn unformat_json(buf: String) -> String {
    let mut out: String = "".to_string();
    let mut buf = buf.chars();
    loop {
        let i = match buf.next() {
            Some(i) => i,
            _ => break
        };
        if i == ' ' {
            continue;
        }
        if i == '\n' {
            continue;
        }
        if i == '#' {
            loop {
                let j = match buf.next() {
                    Some(j) => j,
                    _ => break
                };
                if j == '\n' {
                    break
                }
            }    
            continue;
        }
        out += &i.to_string();
    }
    out
}

/// Attempts to read the keybinds from given file path.
/// Success => Read keybinds are returned.
/// Fail => Default (hardcoded) keybinds are returned,
///     and defaults are written to file.
///     If writing fails, nothing happens.
pub(crate) fn get_keybinds(path: &str) -> Keybinds {
    //TODO: Figure out bette way to resolve the file path thing
    let keybinds: Keybinds = match read_configs(&format!("/home/lelle/Documents/Rust/text/{}", path)) {
        Ok(keybinds) => keybinds,
        Err(_) => {
            let _ = write_configs(&format!("/home/lelle/Documents/Rust/text/{}", path), DEFAULT_KEYBINDS);
            DEFAULT_KEYBINDS
        }
    };
    keybinds
}

pub(crate) fn get_configs(path: &str) -> Config {
    //TODO: Figure out bette way to resolve the file path thing
    let configs: Config = match read_configs(&format!("/home/lelle/Documents/Rust/text/{}", path)) {
        Ok(configs) => configs,
        Err(_) => {
            let _ = write_configs(&format!("/home/lelle/Documents/Rust/text/{}", path), DEFAULT_CONFIG);
            DEFAULT_CONFIG
        }
    };
    configs
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub(crate) struct FunnyConfig {
    //doesn't work currently
    pub(crate) wave_render: bool,
    pub(crate) wiggle_render: bool
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) funny_config: FunnyConfig
}