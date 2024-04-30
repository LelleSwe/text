use crossterm::{execute, cursor::MoveTo, terminal::ClearType};
use std::io::stdout;

use crate::user_interact::Cursor;

pub(crate) fn update_cursor(cursor: &Cursor) {
    let _ = execute!(stdout(), MoveTo(cursor.pos_x, cursor.pos_y));
}

pub(crate) fn draw_line() {

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