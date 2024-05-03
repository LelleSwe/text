use crossterm::event::*;
use std::{time::{Duration}};
use std::io::{stdout, Write};
use serde::Serialize;

use crate::terminate_program;

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

pub(crate) fn move_cursor(cursor: &mut Cursor, move_command: KeyEvent) -> &mut Cursor {    
    match move_command {
        default_cursor_keybinds@MoveUp => cursor.pos_y += 1,
        default_cursor_keybinds@MoveDown => cursor.pos_y -= 1,
        default_cursor_keybinds@MoveLeft => cursor.pos_x -= 1,
        default_cursor_keybinds@MoveRight => cursor.pos_x += 1,
        default_cursor_keybinds@MoveLast => unimplemented!(),
        default_cursor_keybinds@MoveFirst => {let mut cursor = Cursor { pos_x: 0, pos_y: 0};
    let mut cursor = &cursor;},
        default_cursor_keybinds@MoveWordLeft => unimplemented!(),
        default_cursor_keybinds@MoveWordRight => unimplemented!(),
        default_cursor_keybinds@PageDown => unimplemented!(),
        default_cursor_keybinds@PageUp => unimplemented!(),
        _ => ()
    };
    cursor
}

pub(crate) fn process_keypress() -> KeyEvent {
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
            write!(stdout(), "{:?}\r", event.code);
            std::io::stdout().flush();
        }
    }

    event
}

pub(crate) struct Keybinds {
    pub(crate) CursorKeybinds: CursorKeybinds
}

//#[derive(Serialize)]
pub(crate) struct CursorKeybinds {
    pub(crate) MoveUp: KeyEvent,
    pub(crate) MoveDown: KeyEvent,
    pub(crate) MoveLeft: KeyEvent,
    pub(crate) MoveRight: KeyEvent,
    pub(crate) MoveLast: KeyEvent,
    pub(crate) MoveFirst: KeyEvent,
    pub(crate) MovePageDown: KeyEvent,
    pub(crate) MovePageUp: KeyEvent,
    pub(crate) MoveWordLeft: KeyEvent,
    pub(crate) MoveWordRight: KeyEvent,
}