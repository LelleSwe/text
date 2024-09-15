use std::{io::{self, stdout}, sync::Mutex};
use crossterm::{execute, queue, terminal::{disable_raw_mode, enable_raw_mode, SetSize, EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::cursor::{Show, Hide};
use crossterm::event::KeyEvent;
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
    let Ok(mut cursor) = setup()
    else {
        unimplemented!()
    };
    let mut data: Vec<Vec<char>> = vec!(
        "goddag världen".chars().collect(),
        "det här är en bra dag att leva".chars().collect(),
        "vi tycker om detta".chars().collect(),
        "bruh hittade en bugg".chars().collect()
    );
    let _ = draw_screen(&data);
    //execute!(io::stdout(), SetSize(cols, rows))?;
    loop {
        let event: KeyEvent = process_keypress();
        let _ = queue!(stdout(), Hide);
        let _ = move_cursor(&mut cursor, event, &data);
        let _ = update_data(&mut data, &mut cursor, &event);
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