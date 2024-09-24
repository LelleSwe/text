use crossterm::event::*;
use core::str;
use std::time::Duration;
use serde::{Serialize, Deserialize};

use crate::terminate_program;

/// Uses crossterm::event::read()? to search for a key,
/// or returns after 0.5 seconds if no key is pressed.
pub(crate) fn read_key() -> Result<KeyEvent, std::io::Error> {
    loop {
        if poll(Duration::from_millis(16))? {
            if let Event::Key(event) = crossterm::event::read()? {
                    return Ok(event);
            };
        }  else {
            return Ok(KeyEvent {
                code: KeyCode::Null,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }
        )}
    }
}

/// Not mouse cursor, but location of 
/// cursor that types stuff.
#[derive(Debug, Copy, Clone)]
pub(crate) struct Cursor {
    pub(crate) pos_x: i16,
    pub(crate) pos_y: i16
}

pub(crate) fn move_cursor(cursor: &mut Cursor, move_command: &KeyEvent, data: &Vec<Vec<char>>, keybinds: &Keybinds) {
    let move_command = *move_command;
    let CursorKeybinds {MoveUp, MoveDown, MoveLeft, MoveRight, MoveLast, MoveFirst, MoveWordLeft, MoveWordRight, MovePageDown, MovePageUp}
        = keybinds.CursorKeybinds;
    
    //Processing possible cursor movements
    if move_command == MoveDown {
        cursor.pos_y += 1;
        if cursor.pos_y == data.len() as i16 {cursor.pos_y = (data.len()-1) as i16;}
        if cursor.pos_x > data[cursor.pos_y as usize].len().try_into().unwrap() {
            cursor.pos_x = data[cursor.pos_y as usize].len() as i16;
        }
    }
    if move_command == MoveUp {
        cursor.pos_y -= 1;
        if cursor.pos_y == -1 {cursor.pos_y = 0;}
        if cursor.pos_x > data[cursor.pos_y as usize].len().try_into().unwrap() {
            cursor.pos_x = data[cursor.pos_y as usize].len() as i16;
        }
    }
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

    if cursor.pos_y < 0 {cursor.pos_y = 0;}
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

}

pub(crate) fn process_keypress(data: &mut Vec<Vec<char>>, cursor: &mut Cursor, event: &KeyEvent, keybinds: &Keybinds) {
    let _ = move_cursor(cursor, event, &data, keybinds);
    let code = event.code;
    let event = *event;

    if event == keybinds.UtilKeybinds.terminate_program {
        terminate_program();
        return
    }
    if event == keybinds.UtilKeybinds.save_file {return}
    if event == keybinds.DataInteractKeybinds.remove_before {
        remove_data_before(data, 1, cursor, keybinds);
    }
    if event == keybinds.DataInteractKeybinds.remove_after {
        remove_data_after(data, 1, cursor, keybinds);
    }
    if event == keybinds.DataInteractKeybinds.new_line {
        split_line(data, cursor, keybinds);
    }

    match code {
        KeyCode::Char(code) => {
            insert_data(data, code, cursor, keybinds);
        },
        _ => ()
    };
}

pub(crate) fn remove_data_after(data: &mut Vec<Vec<char>>, amount: usize, cursor: &mut Cursor, keybinds: &Keybinds) {
    for _ in 0..amount {
        if cursor.pos_x == data[cursor.pos_y as usize].len() as i16 {
            if cursor.pos_y == data.len() as i16 - 1 {
                continue;
            }
            let _ = move_cursor(cursor, &keybinds.CursorKeybinds.MoveRight, data, keybinds);
            merge_line(data, cursor, keybinds);
            continue;
        }

        let _ = data[cursor.pos_y as usize].remove(cursor.pos_x as usize);        
    }
}

pub(crate) fn remove_data_before(data: &mut Vec<Vec<char>>, amount: usize, cursor: &mut Cursor, keybinds: &Keybinds) {
    for _ in 0..amount {
        if cursor.pos_x == 0 {
            merge_line(data, cursor, keybinds);
            continue;
        }
        data[cursor.pos_y as usize].remove(cursor.pos_x as usize - 1);
        let keybind = keybinds.CursorKeybinds.MoveLeft;
        let _ = move_cursor(cursor, &keybind, data, keybinds);
    }
}

pub(crate) fn merge_line(data: &mut Vec<Vec<char>>, cursor: &mut Cursor, keybinds: &Keybinds) {
    if cursor.pos_y == 0 {return}
    let mut line = data[cursor.pos_y as usize].clone();

    //TODO: This temporary variable thing is super ugly, but it works for now.
    let keybind = keybinds.CursorKeybinds.MoveLeft;
    let _ = move_cursor(cursor, &keybind, data, keybinds);
    let tmp_x = cursor.pos_x;
    let tmp_y = cursor.pos_y;
    let keybind = keybinds.CursorKeybinds.MoveRight;
    let _ = move_cursor(cursor, &keybind, data, keybinds);

    data.remove(cursor.pos_y as usize);
    data[cursor.pos_y as usize - 1].append(&mut line);
    cursor.pos_x = tmp_x;
    cursor.pos_y = tmp_y;
}

pub(crate) fn insert_data(data: &mut Vec<Vec<char>>, insert: char, cursor: &mut Cursor, keybinds: &Keybinds) {
    data[cursor.pos_y as usize].insert(cursor.pos_x as usize, insert);
    let keybind = keybinds.CursorKeybinds.MoveRight;
    let _ = move_cursor(cursor, &keybind, data, keybinds);
}

pub(crate) fn split_line(data: &mut Vec<Vec<char>>, cursor: &mut Cursor, keybinds: &Keybinds) {
    let mut data2 = data.clone();
    let (line1, line2) = data2[cursor.pos_y as usize].split_at_mut(cursor.pos_x as usize);
    data[cursor.pos_y as usize] = line1.to_vec();
    data.insert(cursor.pos_y as usize + 1, line2.to_vec());

    let keybind = keybinds.CursorKeybinds.MoveDown;
    let _ = move_cursor(cursor, &keybind, data, keybinds);
    cursor.pos_x = 0;
}


/// Struct containing *all* keybinds for various default actions.
#[derive(Serialize, Deserialize, Clone, Copy)]
pub(crate) struct Keybinds {
    pub(crate) CursorKeybinds: CursorKeybinds,
    pub(crate) UtilKeybinds: UtilKeybinds,
    pub(crate) DataInteractKeybinds: DataInteractKeybinds
}

/// Struct containing 10 Crossterm KeyEvents for cursor behaviour.
/// MoveUp, MoveDown, MoveLeft, MoveRight, MoveLast, MoveFirst, MovePageDown, MovePageUp, MoveWordLeft, MoveWordRight
#[derive(Serialize, Deserialize, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Clone, Copy)]
pub(crate) struct UtilKeybinds {
    pub(crate) save_file: KeyEvent,
    pub(crate) terminate_program: KeyEvent
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub(crate) struct DataInteractKeybinds {
    pub(crate) remove_before: KeyEvent,
    pub(crate) remove_after: KeyEvent,
    pub(crate) new_line: KeyEvent
}