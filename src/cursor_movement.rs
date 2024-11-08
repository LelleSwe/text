use crossterm::{cursor, event::KeyEvent};

use crate::{constants::DEFAULT_WORD_SEPARATOR, user_interact::{Cursor, CursorKeybinds, Keybinds, MoveAction}};

pub(crate) fn get_cursor_action(keybinds: &Keybinds, move_command: &KeyEvent) -> MoveAction {
    let CursorKeybinds {MoveUp, MoveDown, MoveLeft, MoveRight, MoveLast, MoveFirst, MoveWordLeft, MoveWordRight, MovePageDown, MovePageUp}
        = keybinds.CursorKeybinds;
    let move_command = *move_command;
    
    //Maybe will make work later, found out iterations don't work on enums.
    /* macro_rules! move_transform {
        (move: tt) => {
            if move_command == move {
                return MoveAction::move;
            }
        };
    }*/
    if move_command == MoveUp {
        return MoveAction::MoveUp
    }
    if move_command == MoveDown {
        return MoveAction::MoveDown
    }
    if move_command == MoveRight {
        return  MoveAction::MoveRight
    }
    if move_command == MoveLeft {
        return MoveAction::MoveLeft;
    }
    if move_command == MoveLast {
        return MoveAction::MoveLast;
    }
    if move_command == MoveFirst {
        return MoveAction::MoveFirst;
    }
    if move_command == MoveWordLeft {
        return MoveAction::MoveWordLeft;
    }
    if move_command == MoveWordRight {
        return MoveAction::MoveWordRight;
    }
    if move_command == MovePageDown {
        return MoveAction::MovePageDown;
    }
    if move_command == MovePageUp {
        return MoveAction::MovePageUp;
    }
    MoveAction::None
}

pub(crate) fn move_up(data: &Vec<Vec<char>>, cursor: &mut Cursor) {
    cursor.pos_y -= 1;
    if cursor.pos_y == -1 {cursor.pos_y = 0;}
    if cursor.pos_x > data[cursor.pos_y as usize].len().try_into().unwrap() {
        cursor.pos_x = data[cursor.pos_y as usize].len() as i16;
    }
}

pub(crate) fn move_down(data: &Vec<Vec<char>>, cursor: &mut Cursor) {
    cursor.pos_y += 1;
    if cursor.pos_y == data.len() as i16 {cursor.pos_y = (data.len()-1) as i16;}
    if cursor.pos_x > data[cursor.pos_y as usize].len().try_into().unwrap() {
        cursor.pos_x = data[cursor.pos_y as usize].len() as i16;
    }
}

pub(crate) fn move_right(data: &Vec<Vec<char>>, cursor: &mut Cursor) {
    cursor.pos_x += 1;
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

pub(crate) fn move_right_noextend(data: &Vec<Vec<char>>, cursor: &mut Cursor) {
    cursor.pos_x += 1;
    if cursor.pos_x >= data[cursor.pos_y as usize].len().try_into().unwrap() {
        if cursor.pos_y == (data.len()-1) as i16 {
            cursor.pos_x = (data[data.len()-1].len() - 1) as i16;
        } else {
            cursor.pos_x = 0;
        }
        cursor.pos_y += 1;
        if cursor.pos_y > (data.len()-1) as i16 {
            cursor.pos_y = (data.len()-1) as i16;
        }
    }
}

pub(crate) fn move_left(data: &Vec<Vec<char>>, cursor: &mut Cursor) {
    cursor.pos_x -= 1;
    if cursor.pos_x < 0 {
        if cursor.pos_y == 0 {
            cursor.pos_x = 0;
        } else {
            cursor.pos_y -= 1;
            cursor.pos_x = data[cursor.pos_y as usize].len() as i16;
        }
    }
}

pub(crate) fn move_left_noextend(data: &Vec<Vec<char>>, cursor: &mut Cursor) {
    cursor.pos_x -= 1;
    if cursor.pos_x < 0 {
        if cursor.pos_y == 0 {
            cursor.pos_x = 0;
        } else {
            cursor.pos_y -= 1;
            cursor.pos_x = (data[cursor.pos_y as usize].len() - 1) as i16;
        }
    }
}

pub(crate) fn move_last(data: &Vec<Vec<char>>, cursor: &mut Cursor) {
    cursor.pos_y = (data.len()-1) as i16;
    //for some reason data[-1] gives compiler error, has to do this instead.
    cursor.pos_x = (data[data.len()-1].len()) as i16;
}

pub(crate) fn move_first(cursor: &mut Cursor) {
    cursor.pos_x = 0;
    cursor.pos_y = 0;
}

pub(crate) fn move_word_left(data: &Vec<Vec<char>>, cursor: &mut Cursor) {
    let goto = find_left_word(data, cursor).0;
    cursor.pos_x = goto.pos_x;
    cursor.pos_y = goto.pos_y;
}

pub(crate) fn move_word_right(data: &Vec<Vec<char>>, cursor: &mut Cursor) {
    let goto = find_right_word(data, cursor).0;
    cursor.pos_x = goto.pos_x;
    cursor.pos_y = goto.pos_y;
}

pub(crate) fn move_page_down(data: &Vec<Vec<char>>, cursor: &mut Cursor) {
    cursor.pos_y += 15;
    if cursor.pos_y > (data.len()-1) as i16 {
        cursor.pos_y = (data.len()-1) as i16;
        cursor.pos_x = data[cursor.pos_y as usize].len() as i16;
    }
    if cursor.pos_x > data[cursor.pos_y as usize].len().try_into().unwrap() {
        cursor.pos_x = data[cursor.pos_y as usize].len() as i16;
    }
}

pub(crate) fn move_page_up(data: &Vec<Vec<char>>, cursor: &mut Cursor) {
    cursor.pos_y -= 15;
    if cursor.pos_y < 0 {
        cursor.pos_y = 0;
        cursor.pos_x = 0;
    }
    if cursor.pos_x > data[cursor.pos_y as usize].len() as i16 {
        cursor.pos_x = data[cursor.pos_y as usize].len() as i16;
    }
}

pub(crate) fn move_cursor(cursor: &mut Cursor, move_action: &MoveAction, data: &Vec<Vec<char>>) {
    match move_action {
        MoveAction::MoveDown => move_down(data, cursor),
        MoveAction::MoveFirst => move_first(cursor),
        MoveAction::MoveLast => move_last(data, cursor),
        MoveAction::MoveLeft => move_left(data, cursor),
        MoveAction::MovePageDown => move_page_down(data, cursor),
        MoveAction::MovePageUp => move_page_up(data, cursor),
        MoveAction::MoveRight => move_right(data, cursor),
        MoveAction::MoveUp => move_up(data, cursor),
        MoveAction::MoveWordLeft => move_word_left(data, cursor),
        MoveAction::MoveWordRight => move_word_right(data, cursor),
        _ => ()
    }
}

//TODO: This shit sort of works, actually.
//Should still be extensively tested, though.
pub(crate) fn find_right_word(data: &Vec<Vec<char>>, cursor: &Cursor) -> (Cursor, i64) {
    let mut finder: Cursor = cursor.clone();
    let mut distance: i64 = 0;
    let mut continued: bool = true;

    if data[finder.pos_y as usize].len() == 0 {
        let _ = move_right(data, &mut finder);
        return (finder, distance + 1)
    }
    if finder.pos_x >= data[finder.pos_y as usize].len() as i16 {
        let _ = move_right_noextend(data, &mut finder);
        return (finder, 1);
    }
    
    while !DEFAULT_WORD_SEPARATOR.contains(&data[finder.pos_y as usize][finder.pos_x as usize]) || continued {
        if data[finder.pos_y as usize].len() == 0 {return (finder, i64::max(distance, 1))}
        let current = data[finder.pos_y as usize][finder.pos_x as usize];

        if finder.pos_x == 0 && finder.pos_y == 0 {break};
        let _ = move_right_noextend(data, &mut finder);
        distance += 1;
        if data[finder.pos_y as usize].len() == 0 {break;}
        if finder.pos_x == 0 && finder.pos_y == 0 {break};

        if finder.pos_y != cursor.pos_y {
            let _ = move_left(data, &mut finder);
            break;
        }
        let next = data[finder.pos_y as usize][finder.pos_x as usize];
        continued = current == next && DEFAULT_WORD_SEPARATOR.contains(&current);
    };

    (finder, i64::max(distance, 1))
}

//TODO: This shit needs polishing to work better.
pub(crate) fn find_left_word(data: &Vec<Vec<char>>, cursor: &Cursor) -> (Cursor, i64) {
    let mut finder: Cursor = cursor.clone();
    let mut distance: i64 = 0;
    let mut continued: bool = true;

    if data[finder.pos_y as usize].len() == 0 {
        let _ = move_left(data, &mut finder);
        return (finder, distance + 1)
    }
    if finder.pos_x >= data[finder.pos_y as usize].len() as i16 {finder.pos_x -= 1;}
    if finder.pos_x == 0 {
        let _ = move_left(data, &mut finder);
        return (finder, 1)
    }
    
    while !DEFAULT_WORD_SEPARATOR.contains(&data[finder.pos_y as usize][finder.pos_x as usize]) || continued {
        if data[finder.pos_y as usize].len() == 0 {return (finder, i64::max(distance, 1))}
        let current = data[finder.pos_y as usize][finder.pos_x as usize];

        if finder.pos_x == 0 && finder.pos_y == 0 {break};
        let _ = move_left_noextend(data, &mut finder);
        distance += 1;
        if data[finder.pos_y as usize].len() == 0 {break;}
        if finder.pos_x == 0 && finder.pos_y == 0 {break};

        if finder.pos_y != cursor.pos_y {
            let _ = move_right_noextend(data, &mut finder);
            break;
        }
        let next = data[finder.pos_y as usize][finder.pos_x as usize];
        continued = current == next && DEFAULT_WORD_SEPARATOR.contains(&current);
    };

    (finder, i64::max(distance, 1))
}