use crossterm::event::*;
use std::{time::{Duration}};
use std::io::{stdout, Write};
use serde::Serialize;

use crate::constants::default_cursor_keybinds;
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

#[derive(Debug)]
pub(crate) struct Cursor {
    pub(crate) pos_x: u16,
    pub(crate) pos_y: u16
}

pub(crate) fn move_cursor(cursor: &mut Cursor, move_command: KeyEvent) -> &mut Cursor {
    let CursorKeybinds {MoveUp, MoveDown, MoveLeft, MoveRight, MoveLast, MoveFirst, MoveWordLeft, MoveWordRight, MovePageDown, MovePageUp} = default_cursor_keybinds;
    //assert_eq!(MoveUp, move_command);
    if move_command == MoveDown {write!(stdout(), "down\n");}
    if move_command == MoveUp {write!(stdout(), "up\n");}
    if move_command == MoveRight {write!(stdout(), "right\n");}
    if move_command == MoveLeft {write!(stdout(), "left\n");}
    match move_command {
        
        MoveDown => cursor.pos_y -= 1,
        MoveUp => {cursor.pos_y += 1;},
        MoveLeft => cursor.pos_x -= 1,
        MoveRight => cursor.pos_x += 1,
        MoveLast => unimplemented!(),
        MoveFirst => {
            let mut cursor = Cursor { pos_x: 0, pos_y: 0};
            let mut cursor = &cursor;
        },
        MoveWordLeft => unimplemented!(),
        MoveWordRight => unimplemented!(),
        PageDown => unimplemented!(),
        PageUp => unimplemented!(),
        _ => {write!(stdout(), "\nfail");}
    };
    let tttt = format!("{:?}", cursor);
    write!(stdout(), "{}", tttt);
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

//#[derive(std::fmt::Display)]
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