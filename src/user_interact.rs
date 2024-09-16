use crossterm::{event::{self, *}, execute, terminal::{disable_raw_mode, LeaveAlternateScreen}};
use core::str;
use std::{sync::MutexGuard, time::Duration};
use std::io::{stdout, Write};
use serde::{Serialize, Deserialize};

use crate::{clear_line, clear_screen, constants::{DEFAULT_CURSOR_KEYBINDS, DEFAULT_WINDOW, KEYBINDS, WINDOW}, draw_line, draw_screen, file_interact::{get_keybinds, write_text_file}};
use crate::terminate_program;
use crate::render::Window;

/// Uses crossterm::event::read()? to search for a key,
/// or returns after 0.5 seconds if no key is pressed.
pub(crate) fn read_key() -> Result<KeyEvent, std::io::Error> {
    loop {
        if poll(Duration::from_millis(500))? {
            if let Event::Key(event) = crossterm::event::read()? {
                    return Ok::<crossterm::event::KeyEvent, std::io::Error>(event);
            };
        }
    }
}

/// Not mouse cursor, but location of 
/// cursor that types stuff.
#[derive(Debug, Copy, Clone)]
pub(crate) struct Cursor {
    pub(crate) pos_x: i16,
    pub(crate) pos_y: i16
}

pub(crate) fn move_cursor(cursor: &mut Cursor, move_command: KeyEvent, data: &Vec<Vec<char>>) {
    let CursorKeybinds {mut MoveUp, mut MoveDown, mut MoveLeft, mut MoveRight, mut MoveLast, mut MoveFirst, mut MoveWordLeft, mut MoveWordRight, mut MovePageDown, mut MovePageUp }
        = DEFAULT_CURSOR_KEYBINDS;
    let mut window = DEFAULT_WINDOW;
    unsafe {
        CursorKeybinds {MoveUp, MoveDown, MoveLeft, MoveRight, MoveLast, MoveFirst, MoveWordLeft, MoveWordRight, MovePageDown, MovePageUp} 
            = KEYBINDS.lock().unwrap().CursorKeybinds;
        window = *WINDOW.lock().unwrap();
    }
    
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

    //Bounding cursor locations (e.g. can't go outside the screen)
    if cursor.pos_y >= window.size_y as i16 {
        cursor.pos_y = (window.size_y - 1) as i16;
    }
    if cursor.pos_x >= window.size_x as i16 {
        cursor.pos_x = (window.size_x - 1) as i16
    }
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

pub(crate) fn process_keypress(data: &mut Vec<Vec<char>>, cursor: &mut Cursor) -> KeyEvent {
    let event = read_key();
    let event = match event {
        Err(_error) => unimplemented!(),
        Ok(event) => event
    };
          
    let code = event.code;

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
    match code {
        KeyCode::Char(code) => {
            unsafe {
                let save_file = KEYBINDS.lock().unwrap().UtilKeybinds.save_file;
                if event == save_file {return event}
            }
            insert_data(data, code, cursor);
        },
        KeyCode::Enter => {
            split_line(data, cursor);
        },
        KeyCode::Backspace => {
            remove_data(data, 1, cursor)
        },
        _ => ()
    };
    event
}

pub(crate) fn update_data() {

}

pub(crate) fn remove_data(data: &mut Vec<Vec<char>>, amount: usize, cursor: &mut Cursor) {
    for _ in 0..amount {
        if cursor.pos_x == 0 {
            merge_line(data, cursor);
            continue;
        }
        data[cursor.pos_y as usize].remove(cursor.pos_x as usize - 1);
        let _ = clear_line();
        let _ = draw_line(data, cursor);
        let keybind = unsafe {KEYBINDS.lock().unwrap().CursorKeybinds.MoveLeft};
        let _ = move_cursor(cursor, keybind, data);
        let _ = clear_line();
        let _ = draw_line(data, cursor);
    }
}

pub(crate) fn merge_line(data: &mut Vec<Vec<char>>, cursor: &mut Cursor) {
    if cursor.pos_y == 0 {return}
    let mut line = data[cursor.pos_y as usize].clone();

    //TODO: This temporary variable thing is super ugly, but it works for now.
    let keybind = unsafe {KEYBINDS.lock().unwrap().CursorKeybinds.MoveLeft};
    let _ = move_cursor(cursor, keybind, data);
    let tmp_x = cursor.pos_x;
    let tmp_y = cursor.pos_y;
    let keybind = unsafe {KEYBINDS.lock().unwrap().CursorKeybinds.MoveRight};
    let _ = move_cursor(cursor, keybind, data);

    data.remove(cursor.pos_y as usize);
    data[cursor.pos_y as usize - 1].append(&mut line);
    cursor.pos_x = tmp_x;
    cursor.pos_y = tmp_y;
    
    let _ = clear_screen();
    let _ = draw_screen(data);
}

pub(crate) fn insert_data(data: &mut Vec<Vec<char>>, insert: char, cursor: &mut Cursor) {
    data[cursor.pos_y as usize].insert(cursor.pos_x as usize, insert);
    let keybind = unsafe {KEYBINDS.lock().unwrap().CursorKeybinds.MoveRight};
    let _ = move_cursor(cursor, keybind, data);
    let _ = clear_line();
    let _ = draw_line(data, cursor);
}

pub(crate) fn split_line(data: &mut Vec<Vec<char>>, cursor: &mut Cursor) {
    let mut data2 = data.clone();
    let (line1, line2) = data2[cursor.pos_y as usize].split_at_mut(cursor.pos_x as usize);
    data[cursor.pos_y as usize] = line1.to_vec();
    data.insert(cursor.pos_y as usize + 1, line2.to_vec());

    let keybind = unsafe {KEYBINDS.lock().unwrap().CursorKeybinds.MoveDown};
    let _ = move_cursor(cursor, keybind, data);
    cursor.pos_x = 0;
    let _ = clear_screen();
    let _ = draw_screen(data);
}

/// Struct containing *all* keybinds for various default actions.
/// Currently contains: CursorKeybinds
#[derive(Serialize, Deserialize, Clone, Copy)]
pub(crate) struct Keybinds {
    pub(crate) CursorKeybinds: CursorKeybinds,
    pub(crate) UtilKeybinds: UtilKeybinds
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
    pub(crate) save_file: KeyEvent
}
