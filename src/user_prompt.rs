use std::io::{stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::event::*;
use crossterm::{execute, queue};
use crossterm::terminal::ClearType;

use crate::constants::DEFAULT_CONFIG;
use crate::{draw_line, terminate_program, update_cursor, Action, UtilAction, Window};
use crate::user_interact::{Cursor,read_key, Keybinds};

pub(crate) fn user_prompt(data: &str, window: &Window, (pos_x, pos_y): (u16, u16), keybinds: &Keybinds) -> String {
    let _ = draw_line((pos_x, pos_y), &data);
    let mut g: Vec<char> = vec!();

    let print_offset: usize = data.len();
    let mut cursor = Cursor {
        pos_x: 0,
        pos_y: 0
    };

    //Clears previous lines. 
    //There's probably a better solution, but can't quite be bothered at the moment.
    let to_print: String = data.to_string() + &" ".repeat(window.size_x - data.len());
    let _ = draw_line((pos_x, pos_y), &to_print);
    let _ = match execute!(stdout(), MoveTo(cursor.pos_x as u16 + print_offset as u16, pos_y)) {
        Ok(_) => (),
        Err(_) => ()
    };

    loop {
        let event = read_key(false);
        let event: KeyEvent = match event {
            Err(_) => unimplemented!(),
            Ok(event) => event
        };

        if event == keybinds.DataInteractKeybinds.new_line {

            return g.into_iter().collect::<String>();
        }
        if event == keybinds.UtilKeybinds.terminate_program {
            let _ = terminate_program();
        }

        let _ = single_line_process_keypress(&mut g, window, &mut cursor, &event, keybinds);
        let _ = update_cursor(&cursor, window, &DEFAULT_CONFIG);
        let _ = draw_data(&g, (pos_x + print_offset as u16, pos_y));
        let _ = draw_line((pos_x, pos_y), data);
        let _ = match execute!(stdout(), MoveTo(cursor.pos_x as u16 + print_offset as u16, pos_y)) {
            Ok(_) => (),
            Err(_) => ()
        };
    }
}

pub(crate) fn parse_prompt(possible_commands: PossibleCommands, command: &str) -> Action {
    if command == possible_commands.save {
        return Action::UtilAction(crate::UtilAction::Save);
    }
    if command == possible_commands.force_quit {
        return Action::UtilAction(UtilAction::Kill);
    }
    if command == possible_commands.quit {
        return Action::UtilAction(UtilAction::TryKill);
    }

    Action::PrintResult("Not a valid command.".to_string())
}

pub(crate) fn single_line_process_keypress(data: &mut Vec<char>, window: &Window, cursor: &mut Cursor, event: &KeyEvent, keybinds: &Keybinds) {
    if *event == keybinds.CursorKeybinds.MoveLeft {
        cursor.pos_x -= 1;
        if cursor.pos_x < 0 {
            cursor.pos_x = 0;
        }
    }
    if *event == keybinds.CursorKeybinds.MoveRight {
        cursor.pos_x += 1;
        if cursor.pos_x > data.len() as i16 {
            cursor.pos_x = data.len() as i16;
        }
    }
    if *event == keybinds.DataInteractKeybinds.remove_before {
        if cursor.pos_x == 0 {
            return
        }
        data.remove(cursor.pos_x as usize - 1);
        let _ = single_line_process_keypress(data, window, cursor, &keybinds.CursorKeybinds.MoveLeft, keybinds);
    }
    if *event == keybinds.DataInteractKeybinds.remove_after {
        if cursor.pos_x as usize == data.len() {
            return
        }
        data.remove(cursor.pos_x as usize);
    }

    let code = event.code;
    let _ = match code {
        KeyCode::Char(code) => {
            data.insert(cursor.pos_x  as usize, code);
            let _ = single_line_process_keypress(data, window, cursor, &keybinds.CursorKeybinds.MoveRight, keybinds);
        },
        _ => ()
    };
}

pub(crate) fn draw_data(data: &Vec<char>, (pos_x, pos_y): (u16, u16)) -> Result<(), std::io::Error> {
    let _ = queue!(stdout(), MoveTo(pos_x, pos_y), crossterm::terminal::Clear(ClearType::CurrentLine))?;
    for i in data {
        let _ = write!(stdout(), "{}", i)?;
    }
    let _ = stdout().flush();
    Ok(())
}

pub(crate) struct PossibleCommands {
    pub(crate) quit: &'static str,
    pub(crate) force_quit: &'static str,
    pub(crate) save: &'static str
}