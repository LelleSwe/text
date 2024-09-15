use crossterm::event::*;
use crate::render::Window;
use std::sync::Mutex;

use crate::{user_interact::CursorKeybinds, Keybinds};

pub(crate) static mut KEYBINDS: Mutex<Keybinds> = Mutex::new(DEFAULT_KEYBINDS);
pub(crate) static mut WINDOW: Mutex<Window> = Mutex::new(DEFAULT_WINDOW);

pub(crate) const DEFAULT_WINDOW: Window = Window {
    size_x: 120,
    size_y: 30,
    x_offset: 0,
    y_offset: 0};

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