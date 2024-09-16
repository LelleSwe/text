use std::{io::{self, stdout}, sync::{Arc, Mutex}};
use std::env;

use crossterm::{execute, queue, terminal::{disable_raw_mode, enable_raw_mode, SetSize, EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::cursor::{Show, Hide};
use crossterm::event::KeyEvent;
use file_interact::{check_save_file, read_text_file, write_text_file};

use crate::file_interact::get_keybinds;
use crate::user_interact::*;
use crate::render::*;
use crate::constants::{KEYBINDS, WINDOW, DEFAULT_WINDOW};

mod user_interact;
mod render;
mod constants;
mod file_interact;



fn setup() -> Result<Cursor, std::io::Error> {
    //Get the keybinds for which the program uses
    //TODO: Replace "./foo.txt" with actual config file.
    let mut window = DEFAULT_WINDOW;
    unsafe {
        KEYBINDS = Mutex::new(get_keybinds("./foo.txt"));
        let window = WINDOW.lock().unwrap();
    }

    let _ = enable_raw_mode()?;
    let _ = execute!(stdout(), EnterAlternateScreen)?;
    let cursor: Cursor = Cursor {
        pos_x: 0,
        pos_y: 0
    };


    let _ = execute!(io::stdout(), SetSize(window.size_x.try_into().unwrap(), window.size_y.try_into().unwrap()))?;
    let _ = clear_screen()?;
    Ok(cursor)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut data: Vec<Vec<char>> = vec!(); 
    let path: &str = &args[1];
    //length is 1 since the command used to run the program also counts.
    if args.len() > 2 {
        println!("Enter one file to open!\n(Command line argument.)");
        return Ok(())
    } else if args.len() == 2 {
        data = read_text_file(&path);
    }
    
    let Ok(mut cursor) = setup()
    else {
        unimplemented!()
    };
    /*let mut data: Vec<Vec<char>> = vec!(
        "goddag världen".chars().collect(),
        "det här är en bra dag att leva".chars().collect(),
        "vi tycker om detta".chars().collect(),
        "bruh hittade en bugg".chars().collect()
    );*/
    let _ = draw_screen(&data);
    //execute!(io::stdout(), SetSize(cols, rows))?;
    loop {
        let event: KeyEvent = process_keypress(&mut data, &mut cursor);
        let _ = check_save_file(path, &data, &event);

        let _ = queue!(stdout(), Hide);
        let _ = move_cursor(&mut cursor, event, &data);
        let _ = queue!(stdout(), Show);
        let _ = update_cursor(&mut cursor);
    }
    Ok(())
}

pub(crate) fn terminate_program() {
    let _ = clear_screen();
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen);
    let _ = println!("Shutting down program.");
    std::process::exit(1);
}