use crossterm::{execute, cursor::MoveTo, terminal::ClearType};
use std::io::{stdout, Write};

use crate::user_interact::Cursor;

pub(crate) fn update_cursor(cursor: &Cursor) -> Result<bool, std::io::Error> {
    let _ = execute!(stdout(), MoveTo(cursor.pos_x.try_into().unwrap(), cursor.pos_y.try_into().unwrap()))?;
    Ok(true)
}

pub(crate) fn draw_line(data: &Vec<Vec<char>>, cursor: &Cursor) -> Result<bool, std::io::Error> {
    if cursor.pos_y > (data.len() - 1).try_into().unwrap() {return Ok(true)}
    let _ = execute!(stdout(), MoveTo(0, cursor.pos_y.try_into().unwrap()))?;
    for i in &data[cursor.pos_y as usize] {
        let _ = write!(stdout(), "{}", i)?;
    }
    let _ = stdout().flush()?;
    Ok(true)
}

pub(crate) fn draw_screen(data: &Vec<Vec<char>>) -> Result<bool, std::io::Error> {
    let _ = execute!(stdout(), MoveTo(0,0))?;
    for i in 0..data.len() {
        for j in &data[i] {
            let _ = write!(stdout(), "{}", j)?;
        }
        let _ = write!(stdout(), "\r\n")?;
    }
    let _ = execute!(stdout(), MoveTo(0,0))?;
    let _ = stdout().flush()?;
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

#[derive(Debug, Copy, Clone)]
pub(crate) struct Window {
    pub(crate) size_x: usize,
    pub(crate) size_y: usize,
    pub(crate) x_offset: usize,
    pub(crate) y_offset: usize
}