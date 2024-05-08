use std::io::{self, stdout};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, SetSize, EnterAlternateScreen, LeaveAlternateScreen}};
use crate::user_interact::*;
use crate::render::*;

mod user_interact;
mod render;
mod constants;

fn setup() -> Result<(Cursor, Window), std::io::Error> {
    let _ = enable_raw_mode()?;
    let _ = execute!(stdout(), EnterAlternateScreen)?;
    let mut cursor = Cursor {
        pos_x: 1,
        pos_y: 0
    };
    let mut window = Window {
        size_x: 120,
        size_y: 30
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

fn run(cursor: &mut Cursor) -> Result<&mut Cursor, std::io::Error> {
    let _ = update_cursor(cursor);
    let event = process_keypress();
    let mut cursor = move_cursor(cursor, event);
    Ok(cursor)
}

fn main() -> io::Result<()> {
    let Ok((mut cursor, window)) = setup()
    else {
        unimplemented!()
    };
    //execute!(io::stdout(), SetSize(cols, rows))?;

    while let mut cursor = run(&mut cursor).unwrap() {

    };

    return Ok(());
}

pub(crate) fn terminate_program() {
    let _ = clear_screen();
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen);
    let _ = println!("Shutting down program.");
    std::process::exit(1);
}