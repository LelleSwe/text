use std::io::{self, Write, Read};
use std::time::{Duration};
use crossterm::{execute, terminal::{ScrollUp, SetSize, size, enable_raw_mode, disable_raw_mode, ClearType}, event::*};
use std::{error::Error, fmt};

/*struct Setup;
impl Setup {
    fn setup() {
        let _ = enable_raw_mode();
        let _ = execute!(std::io::stdout(), );
    }
}*/

/*type Result<T, ReadError> = std::result::Result<T, ReadError>;
#[derive(Debug)]
struct ReadError(std::io::Error);

impl Error for ReadError {}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an error occured (ono)")
    }
}*/

fn read_key() -> Result<KeyEvent, std::io::Error> {
        loop {
            if poll(Duration::from_millis(500))? {
                if let Event::Key(event) = crossterm::event::read()? {
                        return Ok::<crossterm::event::KeyEvent, std::io::Error>(event);
                };
            }
        }
    }

fn process_keypress() -> bool {
        let event = read_key();
        let event = match event {
            Err(event) => unimplemented!(),
            Ok(event) => event
        };
              
        match event {
            KeyEvent {
                code: KeyCode::Char('q'),
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

    fn run() -> Result<bool, std::io::Error> {
        let _ = clear_screen();
        let _ = process_keypress();
        Ok(true)
    }

    fn clear_screen() -> Result<bool, std::io::Error> {
        let _ = execute!(std::io::stdout(), crossterm::terminal::Clear(ClearType::All))?;
        Ok(true)
    }

fn main() -> io::Result<()> {
    
    //execute!(io::stdout(), SetSize(cols, rows))?;

    while run().unwrap() {

    };

    return Ok(());
}

fn terminate_program() {
    let _ = disable_raw_mode();
    print!("\x1b[2j");
    io::stdout().flush();
    let _ = println!("Shutting down program.");
    std::process::exit(1);
}