use crossterm::event::*;
use core::str;
use std::time::Duration;
use serde::{Serialize, Deserialize};

use crate::file_interact::check_save_file;
use crate::{terminate_program, Window};
use crate::cursor_movement::{move_cursor, get_cursor_action};
use crate::user_prompt::user_prompt;

/// Uses crossterm::event::read()? to search for a key,
/// or returns after 0.5 seconds if no key is pressed.
pub(crate) fn read_key(repeat_read: bool) -> Result<KeyEvent, std::io::Error> {
    loop {
        if poll(Duration::from_millis(16))? {
            if let Event::Key(event) = crossterm::event::read()? {
                    return Ok(event);
            };
        }  else if repeat_read {
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
/// Still used for the mouse cursor, though.
#[derive(Debug, Copy, Clone)]
pub(crate) struct Cursor {
    pub(crate) pos_x: i16,
    pub(crate) pos_y: i16
}

pub(crate) fn do_action(cursor: &mut Cursor, data: &mut Vec<Vec<char>>, action: Action, keybinds: &Keybinds, window: &Window) -> Action {
    match action {
        Action::UserPrompt(a) => return Action::PrintResult(user_prompt(&a, window, (0, window.size_y as u16), &keybinds)),
        
        Action::UtilAction(UtilAction::GetSavePath(s)) => return Action::UtilAction(UtilAction::Save(s)),
        Action::UtilAction(UtilAction::AskSave) => 
            return Action::UtilAction(UtilAction::GetSavePath(user_prompt("Enter save file path: ", window, (0, window.size_y as u16), keybinds))),
        //TODO: Hack solution, fix proper error handling later!
        Action::UtilAction(UtilAction::Save(path)) => return check_save_file(&path, &data),
        
        Action::MoveAction(a) => move_cursor(cursor, &a, data, keybinds),
        Action::UtilAction(UtilAction::Kill) => terminate_program(),

        Action::ModDataAction(ModDataAction::RemoveBefore(a)) => remove_data_before(data, a as usize, cursor, keybinds),
        Action::ModDataAction(ModDataAction::RemoveAfter(a)) => remove_data_after(data, a as usize, cursor, keybinds),
        Action::ModDataAction(ModDataAction::Newline) => split_line(data, cursor, keybinds),
        Action::ModDataAction(ModDataAction::Insert(a)) => insert_data(data, a, cursor, keybinds),
        _ => ()
    };
    Action::None
}

pub(crate) fn process_keypress(event: &KeyEvent, keybinds: &Keybinds, save_path: String) -> Action {
    let posb_mv_action = get_cursor_action(keybinds, event); 
    match posb_mv_action {
        MoveAction::None => (),
        _ => return Action::MoveAction(posb_mv_action),
    };
    
    let code = event.code;
    let event = *event;

    if event == keybinds.UtilKeybinds.prompt {
        return Action::UserPrompt("> ".to_string());
    }
    if event == keybinds.UtilKeybinds.terminate_program {
        return Action::UtilAction(UtilAction::Kill)
    }
    if event == keybinds.UtilKeybinds.save_file {
        return Action::UtilAction(UtilAction::Save(save_path))
    }
    if event == keybinds.DataInteractKeybinds.remove_before {
        return Action::ModDataAction(ModDataAction::RemoveBefore(1));
    }
    if event == keybinds.DataInteractKeybinds.remove_after {
        return Action::ModDataAction(ModDataAction::RemoveAfter(1));
    }
    if event == keybinds.DataInteractKeybinds.new_line {
        return Action::ModDataAction(ModDataAction::Newline);
    }

    match code {
        KeyCode::Char(code) => return Action::ModDataAction(ModDataAction::Insert(code)),
        _ => ()
    };
    Action::None
}

pub(crate) fn remove_data_after(data: &mut Vec<Vec<char>>, amount: usize, cursor: &mut Cursor, keybinds: &Keybinds) {
    for _ in 0..amount {
        if cursor.pos_x == data[cursor.pos_y as usize].len() as i16 {
            if cursor.pos_y == data.len() as i16 - 1 {
                continue;
            }
            let _ = move_cursor(cursor, &MoveAction::MoveRight, data, keybinds);
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
        let _ = move_cursor(cursor, &MoveAction::MoveLeft, data, keybinds);
    }
}

pub(crate) fn merge_line(data: &mut Vec<Vec<char>>, cursor: &mut Cursor, keybinds: &Keybinds) {
    if cursor.pos_y == 0 {return}
    let mut line = data[cursor.pos_y as usize].clone();

    //TODO: This temporary variable thing is super ugly, but it works for now.
    let _ = move_cursor(cursor, &MoveAction::MoveLeft, data, keybinds);
    let tmp_x = cursor.pos_x;
    let tmp_y = cursor.pos_y;
    let _ = move_cursor(cursor, &MoveAction::MoveRight, data, keybinds);

    data.remove(cursor.pos_y as usize);
    data[cursor.pos_y as usize - 1].append(&mut line);
    cursor.pos_x = tmp_x;
    cursor.pos_y = tmp_y;
}

pub(crate) fn insert_data(data: &mut Vec<Vec<char>>, insert: char, cursor: &mut Cursor, keybinds: &Keybinds) {
    data[cursor.pos_y as usize].insert(cursor.pos_x as usize, insert);
    let _ = move_cursor(cursor, &MoveAction::MoveRight, data, keybinds);
}

pub(crate) fn split_line(data: &mut Vec<Vec<char>>, cursor: &mut Cursor, keybinds: &Keybinds) {
    let mut data2 = data.clone();
    let (line1, line2) = data2[cursor.pos_y as usize].split_at_mut(cursor.pos_x as usize);
    data[cursor.pos_y as usize] = line1.to_vec();
    data.insert(cursor.pos_y as usize + 1, line2.to_vec());

    let _ = move_cursor(cursor, &MoveAction::MoveDown, data, keybinds);
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
    pub(crate) terminate_program: KeyEvent,
    pub(crate) prompt: KeyEvent
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub(crate) struct DataInteractKeybinds {
    pub(crate) remove_before: KeyEvent,
    pub(crate) remove_after: KeyEvent,
    pub(crate) new_line: KeyEvent
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum MoveAction {
    MoveRight,
    MoveLeft,
    MoveDown,
    MoveUp,
    MoveLast,
    MoveFirst,
    MoveWordLeft,
    MoveWordRight,
    MovePageDown,
    MovePageUp,
    None
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Action {
    MoveAction(MoveAction),
    UtilAction(UtilAction),
    ModDataAction(ModDataAction),
    PrintResult(String),
    UserPrompt(String),
    None
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum UtilAction {
    //Save path
    Save(String),
    AskSave,
    GetSavePath(String),
    Kill,
    None
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ModDataAction {
    Insert(char),
    //Chars to remove
    RemoveBefore(u32),
    RemoveAfter(u32),
    //Data to paste
    PasteMultiLine(Vec<Vec<char>>),
    PasteSingleLine(Vec<char>),
    //Start & stop position
    Cut(Cursor, Cursor),
    Delete(Cursor, Cursor),
    Newline,
    None
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum PromptAction {
    Print(String),
    None
}