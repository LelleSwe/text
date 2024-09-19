use std::io::{self, stdout};
use std::env;

use crossterm::{execute, queue, terminal::{disable_raw_mode, enable_raw_mode, SetSize, EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::cursor::{Show, Hide};
use crossterm::event::KeyEvent;
use file_interact::{check_save_file, read_text_file};

use crate::file_interact::get_keybinds;
use crate::user_interact::*;
use crate::render::*;
use crate::constants::DEFAULT_WINDOW;

mod user_interact;
mod render;
mod constants;
mod file_interact;



fn setup() -> Result<(Cursor, Keybinds), std::io::Error> {
    //Get the keybinds for which the program uses
    //TODO: Replace "./foo.txt" with actual config file.
    let mut window = DEFAULT_WINDOW;
    let keybinds = get_keybinds("./foo.txt");

    let _ = execute!(stdout(), EnterAlternateScreen)?;
    let _ = enable_raw_mode()?;
    let cursor: Cursor = Cursor {
        pos_x: 0,
        pos_y: 0
    };


    let _ = execute!(io::stdout(), SetSize(window.size_x.try_into().unwrap(), window.size_y.try_into().unwrap()))?;
    let _ = clear_screen()?;
    Ok((cursor, keybinds))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut data: Vec<Vec<char>> = vec!(vec!()); 
    let mut path: &str = "";
    //length is 1 since the command used to run the program also counts.
    if args.len() > 2 {
        println!("Enter one file to open!\n(Command line argument.)");
        return Ok(())
    } else if args.len() == 2 {
        path = &args[1];
        data = read_text_file(&path);
    }
    
    let Ok((mut cursor, keybinds)) = setup()
    else {
        unimplemented!()
    };

    let _ = draw_screen(&data);
    //execute!(io::stdout(), SetSize(cols, rows))?;
    loop {
        let event = read_key();
        let event = match event {
            Err(_error) => unimplemented!(),
            Ok(event) => event
        };

        let _ = process_keypress(&mut data, &mut cursor, &event, &keybinds);
        let _ = check_save_file(path, &data, &event, &keybinds);

        let _ = queue!(stdout(), Hide);

        let _ = queue!(stdout(), Show);
        let _ = update_cursor(&mut cursor);
    }
    Ok(())
}

pub(crate) fn terminate_program() {
    let _ = clear_screen();
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen);
    let _ = disable_raw_mode();
    let _ = println!("Shutting down program.");
    std::process::exit(1);
}