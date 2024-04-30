use std::io;
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, SetSize}, event::*};
use crate::user_interact::*;
use crate::render::*;

mod user_interact;
mod render;


fn setup() -> Result<(Cursor, Window), std::io::Error> {
    let _ = enable_raw_mode()?;
    let mut cursor = Cursor {
        pos_x: 1,
        pos_y: 0
    };
    let mut window = Window {
        size_x: 150,
        size_y: 40
    };

    let _ = execute!(io::stdout(), SetSize(window.size_x, window.size_y))?;
    let _ = clear_screen()?;
    Ok((cursor, window))
}

/*type Result<T, ReadError> = std::result::Result<T, ReadError>;
#[derive(Debug)]
struct ReadError(std::io::Error);

impl Error for ReadError {}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an error occured (ono)")
    }
}*/

fn process_keypress() -> bool {
        let event = read_key();
        let event = match event {
            Err(_event) => unimplemented!(),
            Ok(event) => event
        };
              
        match event {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => terminate_program(),
            _ => {
                //todo
            }
        }
        true
    }

    fn run(cursor: &Cursor) -> Result<bool, std::io::Error> {

        let _ = update_cursor(cursor);
        let _ = process_keypress();
        Ok(true)
    }

fn main() -> io::Result<()> {
    let Ok((cursor, window)) = setup()
    else {
        unimplemented!()
    };
    //execute!(io::stdout(), SetSize(cols, rows))?;

    while run(&cursor).unwrap() {

    };

    return Ok(());
}

fn terminate_program() {
    let _ = clear_screen();
    let _ = disable_raw_mode();
    let _ = println!("Shutting down program.");
    std::process::exit(1);
}