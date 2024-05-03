use crossterm::{execute, cursor::MoveTo, terminal::ClearType};
use std::io::{stdout, Write};

use crate::user_interact::Cursor;

pub(crate) fn update_cursor(cursor: &Cursor) -> Result<bool, std::io::Error> {
    let _ = execute!(stdout(), MoveTo(cursor.pos_x, cursor.pos_y))?;
    Ok(true)
}

pub(crate) fn draw_line(row: Vec<char>) -> Result<bool, std::io::Error> {
    for i in row {
        let _ = write!(stdout(), "{}", i)?;
        let _ = stdout().flush()?;
    }
    Ok(true)
}

pub(crate) fn clear_screen() -> Result<bool, std::io::Error> {
    let _ = execute!(std::io::stdout(), crossterm::terminal::Clear(ClearType::All))?;
    Ok(true)
}

pub(crate) fn clear_line() -> Result<bool, std::io::Error> {
    let _ = execute!(std::io::stdout(), crossterm::terminal::Clear(ClearType::UntilNewLine))?;
    Ok(true)
}

pub(crate) struct Window {
    pub(crate) size_x: u16,
    pub(crate) size_y: u16
}