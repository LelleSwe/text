use crossterm::{cursor::MoveTo, event::KeyEvent, execute};
use serde_json;
use std::{fs::File, io::stdout};
use std::io::prelude::*;

use crate::{constants::DEFAULT_KEYBINDS, Keybinds};


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


pub(crate) fn check_save_file(path: &str, data: &Vec<Vec<char>>, event: &KeyEvent, keybinds: &Keybinds) {
    if *event == keybinds.UtilKeybinds.save_file {
        let _ = write_text_file(path, &data);
        let _ = execute!(stdout(), MoveTo(0, 28)); 
        println!("wrote to file");
    }
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

//TODO: Make CursorKeybinds generic when I learn how to
pub(crate) fn write_keybinds(path: &str, keybinds: Keybinds) -> Result<(), std::io::Error>  {
    let mut file = File::create(path)?;
    let buf: String = format_json(serde_json::to_string(&keybinds).unwrap());
    file.write_all(&buf.as_bytes())?;
    Ok(())  
} 

//TODO: Make CursorKeybinds generic when I learn how to
pub(crate) fn read_keybinds(path: &str) -> Result<Keybinds, std::io::Error>  {
    let mut file: File = File::open(path)?;
    let mut buf: String = "".to_string();
    let _ = file.read_to_string(&mut buf)?;
    let buf: String = unformat_json(buf);
    let out: Keybinds = serde_json::from_str(&buf)?;
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
            out += " : ";
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
    let keybinds: Keybinds = match read_keybinds(path) {
        Ok(keybinds) => keybinds,
        Err(_) => {
            let _ = write_keybinds(path, DEFAULT_KEYBINDS);
            DEFAULT_KEYBINDS
        }
    };
    keybinds
}