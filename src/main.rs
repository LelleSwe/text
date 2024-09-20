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



fn setup() -> Result<(Cursor, Keybinds, Window), std::io::Error> {
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
    Ok((cursor, keybinds, window))
}

fn main() -> io::Result<()> {
    //Forces the program to run terminate_program()
    //on shutdown through the Drop trait.
    //Stops raw mode escaping the program.
    let _proper_term = terminate;
    
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
    
    let Ok((mut cursor, keybinds, mut window)) = setup()
    else {
        unimplemented!()
    };

    let _ = draw_screen(&data, &cursor, &mut window);
    //execute!(io::stdout(), SetSize(cols, rows))?;

    //Main runtime loop.
    loop {
        let event = read_key();
        let event = match event {
            Err(_error) => unimplemented!(),
            Ok(event) => event
        };
        //TODO: Come up with better name for to_print.
        //It prints at the last line of the terminal,
        //so like output for various commands.
        let mut to_print: String = "".to_string();
        
        let _ = process_keypress(&mut data, &mut cursor, &event, &keybinds);
        let t = check_save_file(path, &data, &event, &keybinds);
        to_print = match t {
            Some(t) => t,
            None => "".to_string()
        };

        let _ = queue!(stdout(), Hide);
        let _ = clear_screen();
        let _ = draw_screen(&data, &cursor, &mut window);

        let _ = draw_line((2, window.size_y as u16), &to_print);
        let _ = queue!(stdout(), Show);
        let _ = update_cursor(&mut cursor, &window);
    }
    Ok(())
}

struct terminate;

impl Drop for terminate {
    fn drop(&mut self) {
        let _ = terminate_program();
    }
}

pub(crate) fn terminate_program() {
    let _ = clear_screen();
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen);
    let _ = disable_raw_mode();
    let _ = println!("Shutting down program.");
    std::process::exit(1);
}