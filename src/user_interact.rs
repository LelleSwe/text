use crossterm::event::*;
use std::{time::{Duration}};
use std::io::{stdout, Write};
use serde::Serialize;

use crate::constants::default_cursor_keybinds;
use crate::terminate_program;
use crate::render::Window;

pub(crate) fn read_key() -> Result<KeyEvent, std::io::Error> {
    loop {
        if poll(Duration::from_millis(500))? {
            if let Event::Key(event) = crossterm::event::read()? {
                    return Ok::<crossterm::event::KeyEvent, std::io::Error>(event);
            };
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Cursor {
    pub(crate) pos_x: i16,
    pub(crate) pos_y: i16
}

pub(crate) fn move_cursor(cursor: &mut Cursor, move_command: KeyEvent, window: &Window, data: &Vec<Vec<char>>) -> Cursor {
    let CursorKeybinds {MoveUp, MoveDown, MoveLeft, MoveRight, MoveLast, MoveFirst, MoveWordLeft, MoveWordRight, MovePageDown, MovePageUp} = default_cursor_keybinds;
    //Processing possible cursor movements
    if move_command == MoveDown {cursor.pos_y += 1;}
    if move_command == MoveUp {cursor.pos_y -= 1;}
    if move_command == MoveRight {cursor.pos_x += 1;}
    if move_command == MoveLeft {cursor.pos_x -= 1;}
    if move_command == MoveLast {
        cursor.pos_y = (data.len()-1) as i16;
        //for some reason data[-1] gives compiler error, has to do this instead.
        cursor.pos_x = (data[data.len()-1].len()) as i16;
    }
    if move_command == MoveFirst {
        cursor.pos_x = 0;
        cursor.pos_y = 0;
    }
    if move_command == MoveWordLeft {unimplemented!();}
    if move_command == MoveWordRight {unimplemented!();}
    if move_command == MovePageDown {cursor.pos_y += 15;}
    if move_command == MovePageUp {cursor.pos_y -= 15;}

    //Bounding cursor locations (e.g. can't go outside the screen)
    if cursor.pos_y >= window.size_y as i16 {
        cursor.pos_y = (window.size_y - 1) as i16;
    }
    if cursor.pos_y < 0 {cursor.pos_y = 0;}
    if cursor.pos_x >= window.size_x as i16 {
        cursor.pos_x = (window.size_x - 1) as i16
    }
    if cursor.pos_x < 0 {
        if cursor.pos_y == 0 {
            cursor.pos_x = 0;
        } else {
            cursor.pos_y -= 1;
            cursor.pos_x = data[cursor.pos_y as usize].len() as i16;
        }
    }
    if cursor.pos_y > (data.len()-1) as i16 {
        cursor.pos_y = (data.len()-1) as i16
    }
    if cursor.pos_x > data[cursor.pos_y as usize].len().try_into().unwrap() {
        if cursor.pos_y == (data.len()-1) as i16 {
            cursor.pos_x = data[data.len()-1].len() as i16;
        } else {
            cursor.pos_x = 0;
        }
        cursor.pos_y += 1;
        if cursor.pos_y > (data.len()-1) as i16 {
            cursor.pos_y = (data.len()-1) as i16;
        }
    }
    *cursor
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

        }
    }

    event
}

/*pub(crate) fn insert_data(data: &mut Vec<Vec<char>>, insert: &KeyEvent, cursor: &mut Cursor) -> Vec<Vec<char>> {
    macro_rules! match_keycode {
        ($keycode: tt) => {
            let w = match keycode {
                KeyEvent::$keycode => ("{}", $keycode) as char
            }
            Ok(w)
        };
    }
    let w = insert.code
    let insert_k = match match_keycode!(w) {
        char => insert,
        () => unimplemented!()
    };

    unimplemented!()
}*/

pub(crate) fn split_line(data: &mut Vec<Vec<char>>, cursor: &mut Cursor) -> Vec<Vec<char>> {
    if cursor.pos_x == data[(cursor.pos_y-1) as usize].len().try_into().unwrap() {
        data.push(vec!['\n']);
        cursor.pos_x = 0;
        cursor.pos_y = 0;
    }
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