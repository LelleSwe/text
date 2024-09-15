use crossterm::event::*;
use std::sync::Mutex;

use crate::{user_interact::CursorKeybinds, Keybinds};

pub(crate) static mut KEYBINDS: Mutex<Keybinds> = Mutex::new(DEFAULT_KEYBINDS);

pub(crate) const DEFAULT_KEYBINDS: Keybinds = Keybinds {
    CursorKeybinds: DEFAULT_CURSOR_KEYBINDS
};

pub(crate) const DEFAULT_CURSOR_KEYBINDS: CursorKeybinds = CursorKeybinds {
    MoveUp: KeyEvent {
        code: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    },
    MoveDown: KeyEvent {
        code: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    },
    MoveLeft: KeyEvent {
        code: KeyCode::Left,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    },
    MoveRight: KeyEvent {
        code: KeyCode::Right,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    },
    MoveLast: KeyEvent {
        code: KeyCode::End,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    },
    MoveFirst: KeyEvent {
        code: KeyCode::Home,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    },
    MovePageDown: KeyEvent {
        code: KeyCode::PageDown,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    },
    MovePageUp: KeyEvent {
        code: KeyCode::PageUp,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    },
    MoveWordLeft: KeyEvent {
        code: KeyCode::Left,
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    },
    MoveWordRight: KeyEvent {
        code: KeyCode::Right,
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    },
};