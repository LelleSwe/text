use crossterm::event::*;
use core::str;
use std::time::Duration;
use serde::{Serialize, Deserialize};

use crate::constants::DEFAULT_PROMPTS;
use crate::file_interact::{check_save_file, Config};
use crate::{terminate_program, Window};
use crate::cursor_movement::{find_left_word, find_right_word, get_cursor_action, move_cursor};
use crate::user_prompt::{user_prompt, parse_prompt};

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

pub(crate) fn do_action(cursor: &mut Cursor, data: &mut Vec<Vec<char>>, action: Action, keybinds: &Keybinds, window: &Window, config: Config) -> Action {
    match action {
        //TODO: Change DEFAULT_PROMPTS to actual variable
        Action::UserPrompt(a) => return parse_prompt(DEFAULT_PROMPTS, &user_prompt(&a, window, (0, window.size_y as u16), &keybinds)),
        
        Action::UtilAction(UtilAction::GetSavePath(s)) => return Action::UtilAction(UtilAction::SaveAs(s)),
        Action::UtilAction(UtilAction::AskSavePath) => 
            return Action::UtilAction(UtilAction::GetSavePath(user_prompt("Enter save file path: ", window, (0, window.size_y as u16), keybinds))),
        //TODO: Hack solution, fix proper error handling later!
        Action::UtilAction(UtilAction::SaveAs(path)) => return check_save_file(&path, &data),
        Action::UtilAction(UtilAction::AskSave) => {if user_prompt("Do you want to save? [y] ", window, (0, window.size_y as u16), keybinds) == "y".to_string() {return Action::UtilAction(UtilAction::Save)} else {return Action::UtilAction(UtilAction::Kill)}}
        
        Action::MoveAction(a) => move_cursor(cursor, &a, data),
        Action::UtilAction(UtilAction::Kill) => terminate_program(),

        Action::ModDataAction(ModDataAction::RemoveBefore(a)) => remove_data_before(data, a as usize, cursor),
        Action::ModDataAction(ModDataAction::RemoveAfter(a)) => remove_data_after(data, a as usize, cursor),
        Action::ModDataAction(ModDataAction::Newline) => split_line(data, cursor),
        Action::ModDataAction(ModDataAction::Insert(a)) => insert_data(data, a, cursor),
        Action::ModDataAction(ModDataAction::MakeTab) => insert_tab(data, cursor, &config),
        _ => ()
    };
    Action::None
}

pub(crate) fn process_keypress(data: &Vec<Vec<char>>, cursor: &Cursor, event: &KeyEvent, keybinds: &Keybinds, save_path: String) -> Action {
    let posb_mv_action = get_cursor_action(keybinds, event); 
    match posb_mv_action {
        MoveAction::None => (),
        _ => return Action::MoveAction(posb_mv_action),
    };
    
    let code = event.code;
    let event = *event;
    //dbg!(event);

    if event == keybinds.UtilKeybinds.prompt {
        return Action::UserPrompt("> ".to_string());
    }
    if event == keybinds.UtilKeybinds.save_file {
        return Action::UtilAction(UtilAction::SaveAs(save_path))
    }
    if event == keybinds.DataInteractKeybinds.remove_before {
        return Action::ModDataAction(ModDataAction::RemoveBefore(1));
    }
    if event == keybinds.DataInteractKeybinds.remove_after {
        return Action::ModDataAction(ModDataAction::RemoveAfter(1));
    }
    if event == keybinds.DataInteractKeybinds.remove_word_before {
        return Action::ModDataAction(ModDataAction::RemoveBefore(find_left_word(data, cursor).1));
    }
    if event == keybinds.DataInteractKeybinds.remove_word_after {
        return Action::ModDataAction(ModDataAction::RemoveAfter(find_right_word(data, cursor).1));
    }
    if event == keybinds.DataInteractKeybinds.new_line {
        return Action::ModDataAction(ModDataAction::Newline);
    }
    if event == keybinds.DataInteractKeybinds.tab {
        return Action::ModDataAction(ModDataAction::MakeTab);
    }

    match code {
        KeyCode::Char(code) => return Action::ModDataAction(ModDataAction::Insert(code)),
        _ => ()
    };
    Action::None
}

pub(crate) fn insert_tab(data: &mut Vec<Vec<char>>, cursor: &mut Cursor, config: &Config) {
    if config.tabs_to_spaces {
        for _ in 0..config.tabsize {
            insert_data(data, ' ', cursor)
        }
    } else {
        insert_data(data, '\t', cursor)
    }
}

pub(crate) fn remove_data_after(data: &mut Vec<Vec<char>>, amount: usize, cursor: &mut Cursor) {
    for _ in 0..amount {
        if cursor.pos_x == data[cursor.pos_y as usize].len() as i16 {
            if cursor.pos_y == data.len() as i16 - 1 {
                continue;
            }
            let _ = move_cursor(cursor, &MoveAction::MoveRight, data);
            merge_line(data, cursor);
            continue;
        }

        let _ = data[cursor.pos_y as usize].remove(cursor.pos_x as usize);        
    }
}

pub(crate) fn remove_data_before(data: &mut Vec<Vec<char>>, amount: usize, cursor: &mut Cursor) {
    for _ in 0..amount {
        if cursor.pos_x == 0 {
            merge_line(data, cursor);
            continue;
        }
        data[cursor.pos_y as usize].remove(cursor.pos_x as usize - 1);
        let _ = move_cursor(cursor, &MoveAction::MoveLeft, data);
    }
}

pub(crate) fn merge_line(data: &mut Vec<Vec<char>>, cursor: &mut Cursor) {
    if cursor.pos_y == 0 {return}
    let mut line = data[cursor.pos_y as usize].clone();

    //TODO: This temporary variable thing is super ugly, but it works for now.
    let _ = move_cursor(cursor, &MoveAction::MoveLeft, data);
    let tmp_x = cursor.pos_x;
    let tmp_y = cursor.pos_y;
    let _ = move_cursor(cursor, &MoveAction::MoveRight, data);

    data.remove(cursor.pos_y as usize);
    data[cursor.pos_y as usize - 1].append(&mut line);
    cursor.pos_x = tmp_x;
    cursor.pos_y = tmp_y;
}

pub(crate) fn insert_data(data: &mut Vec<Vec<char>>, insert: char, cursor: &mut Cursor) {
    data[cursor.pos_y as usize].insert(cursor.pos_x as usize, insert);
    let _ = move_cursor(cursor, &MoveAction::MoveRight, data);
}

pub(crate) fn split_line(data: &mut Vec<Vec<char>>, cursor: &mut Cursor) {
    let mut data2 = data.clone();
    let (line1, line2) = data2[cursor.pos_y as usize].split_at_mut(cursor.pos_x as usize);
    data[cursor.pos_y as usize] = line1.to_vec();
    data.insert(cursor.pos_y as usize + 1, line2.to_vec());

    let _ = move_cursor(cursor, &MoveAction::MoveDown, data);
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
    pub(crate) remove_word_before: KeyEvent,
    pub(crate) remove_word_after: KeyEvent,
    pub(crate) new_line: KeyEvent,
    pub(crate) tab: KeyEvent
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
    Save,
    SaveAs(String),
    AskSave,
    AskSavePath,
    GetSavePath(String),
    Kill,
    TryKill,
    None
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ModDataAction {
    Insert(char),
    //Chars to remove
    RemoveBefore(i64),
    RemoveAfter(i64),
    //Data to paste
    PasteMultiLine(Vec<Vec<char>>),
    PasteSingleLine(Vec<char>),
    //Start & stop position
    Cut(Cursor, Cursor),
    Delete(Cursor, Cursor),
    Newline,
    MakeTab,
    None
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum PromptAction {
    Print(String),
    None
}