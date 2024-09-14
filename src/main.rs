use std::io::{self, stdout};
use crossterm::{execute, queue, terminal::{disable_raw_mode, enable_raw_mode, SetSize, EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::cursor::{Show, Hide};
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
        size_y: 30,
        x_offset: 0,
        y_offset: 0
    };

    let _ = execute!(io::stdout(), SetSize(window.size_x.try_into().unwrap(), window.size_y.try_into().unwrap()))?;
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

fn main() -> io::Result<()> {
    let Ok((mut cursor, window)) = setup()
    else {
        unimplemented!()
    };
    let mut data = vec!(vec!('e','e'),vec!('w'));
    let _ = draw_screen(&data);
    //execute!(io::stdout(), SetSize(cols, rows))?;
    loop {
        let event = process_keypress();
        let _ = draw_line(&data, &cursor);
        let _ = queue!(stdout(), crossterm::cursor::Show);
        let mut cursor = move_cursor(&mut cursor, event, &window, &data);
        let _ = update_cursor(&mut cursor);
        let _ = queue!(stdout(), crossterm::cursor::Hide);    
    }
    return Ok(());
}

pub(crate) fn terminate_program() {
    let _ = clear_screen();
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen);
    let _ = println!("Shutting down program.");
    std::process::exit(1);
}