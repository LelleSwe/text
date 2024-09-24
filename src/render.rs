use crossterm::{cursor::MoveTo, execute, terminal::ClearType};
use std::{cmp, io::{stdout, Write}};
use std::time::SystemTime;

use crate::{file_interact::Config, user_interact::Cursor};

pub(crate) fn update_cursor(cursor: &Cursor, window: &Window, configs: &Config) -> Result<bool, std::io::Error> {
    if !configs.funny_config.wiggle_render {
        let _ = execute!(stdout(), 
        MoveTo(cursor.pos_x as u16 - window.x_offset as u16, cursor.pos_y as u16 - window.y_offset as u16
        ))?;
    } else {
        let x_offset = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64;
        let x_offset = (5.0 * (f64::sin(x_offset/125.0) + 1.1)) as u16;
        let _ = execute!(stdout(), 
        MoveTo(cursor.pos_x as u16 - window.x_offset as u16 + x_offset, cursor.pos_y as u16 - window.y_offset as u16
        ))?;
    }
    Ok(true)
}

fn cursor_inside_window(cursor: &Cursor, window: &mut Window) {
    if cursor.pos_y < window.y_offset as i16 {
        window.y_offset = cursor.pos_y as usize;
    }
    //+-1 because janky logic error otherwise /shrug.
    //I think it's becausse window.size has off-by-one error.
    //+-1 to reduce maximum number of rows.
    //I want to leave the last few rows for printing special stuff.
    if cursor.pos_y >= (window.y_offset + window.size_y) as i16 - 1 - 1 {
        window.y_offset = cursor.pos_y as usize - window.size_y + 1 + 1;
    }
    if cursor.pos_x < (window.x_offset) as i16 {
        window.x_offset = cursor.pos_x as usize;
    } 
    if cursor.pos_x >= (window.x_offset + window.size_x) as i16 - 1 {
        window.x_offset = cursor.pos_x as usize - window.size_x + 1;
    }
    //window.y_offset = cursor.pos_y as usize;
} 

//TODO: Fix word wrapping bugs.
pub(crate) fn draw_screen(data: &Vec<Vec<char>>, cursor: &Cursor, window: &mut Window, configs: &Config) -> Result<bool, std::io::Error> {
    let _ = execute!(stdout(), MoveTo(0, 0))?;
    let mut x_offset = 0;
    if configs.funny_config.wiggle_render {
        let x_pos = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64;
        x_offset = (5.0 * (f64::sin(x_pos/125.0) + 1.1)) as u16;
        let _ = execute!(stdout(), MoveTo(x_offset, 0))?;
    }
    let _ = cursor_inside_window(cursor, window);

    let low_bound = window.y_offset;
    let high_bound = cmp::min(data.len(), window.size_y+window.y_offset-1);
    for i in low_bound..high_bound {
        for (k, j) in data[i].iter().enumerate() {
            if k > window.size_x {break}
            let _ = write!(stdout(), "{}", j)?;
        }
        if i != high_bound {
            let _ = write!(stdout(), "\r\n{}", " ".repeat(x_offset as usize))?;
        }
    }

    let _ = execute!(stdout(), MoveTo(0,0))?;
    let _ = stdout().flush()?;
    Ok(true)
}

pub(crate) fn clear_screen() -> Result<bool, std::io::Error> {
    let _ = execute!(std::io::stdout(), crossterm::terminal::Clear(ClearType::All))?;
    Ok(true)
}

pub(crate) fn draw_line(location: (u16, u16), data: &str) -> Result<(), std::io::Error> {
    let _ = execute!(stdout(), MoveTo(location.0, location.1))?;

    let _ = write!(stdout(), "{}", data)?;
    let _ = stdout().flush()?;
    Ok(())
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Window {
    pub(crate) size_x: usize,
    pub(crate) size_y: usize,
    pub(crate) x_offset: usize,
    pub(crate) y_offset: usize
}