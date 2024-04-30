use crossterm::event::*;
use std::{time::{Duration}, io::stdout};

pub(crate) fn read_key() -> Result<KeyEvent, std::io::Error> {
    loop {
        if poll(Duration::from_millis(500))? {
            if let Event::Key(event) = crossterm::event::read()? {
                    return Ok::<crossterm::event::KeyEvent, std::io::Error>(event);
            };
        }
    }
}

pub(crate) struct Cursor {
    pub(crate) pos_x: u16,
    pub(crate) pos_y: u16
}

pub(crate) fn move_cursor(cursor: &Cursor, direction: u8) -> Cursor {
    unimplemented!();
}


