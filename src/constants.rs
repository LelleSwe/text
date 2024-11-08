use crossterm::event::*;
use crate::file_interact::{Config, FunnyConfig};
use crate::render::Window;

use crate::user_prompt::PossibleCommands;
use crate::DataInteractKeybinds;
use crate::{user_interact::{CursorKeybinds, UtilKeybinds}, Keybinds};

pub(crate) const DEFAULT_WORD_SEPARATOR: [char; 9] = [' ', '.', ',', '[', ']', '{', '}', '(', ')'];

pub(crate) const DEFAULT_WINDOW: Window = Window {
    size_x: 120,
    size_y: 30,
    x_offset: 0,
    y_offset: 0};

pub(crate) const DEFAULT_KEYBINDS: Keybinds = Keybinds {
    CursorKeybinds: DEFAULT_CURSOR_KEYBINDS,
    UtilKeybinds: DEFAULT_UTIL_KEYBINDS,
    DataInteractKeybinds: DEFAULT_DATA_INTERACT_KEYBINDS
};

pub(crate) const DEFAULT_DATA_INTERACT_KEYBINDS: DataInteractKeybinds = DataInteractKeybinds {
    remove_before: KeyEvent {
        code: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE

    },
    remove_after: KeyEvent {
        code: KeyCode::Delete,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE

    },
    //Replaced 'CTRL + BACKSPACE' with 'CTRL + h',
    //see https://github.com/crossterm-rs/crossterm/issues/504 or https://github.com/crossterm-rs/crossterm/issues/685.
    remove_word_before: KeyEvent {
        code: KeyCode::Char('h'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE

    },
    remove_word_after: KeyEvent {
        code: KeyCode::Delete,
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE

    },
    new_line: KeyEvent {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE

    },
    tab: KeyEvent {
        code: KeyCode::Tab,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    }
    
};

pub(crate) const DEFAULT_UTIL_KEYBINDS: UtilKeybinds = UtilKeybinds {
    save_file: KeyEvent {
        code: KeyCode::Char('s'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    },
    terminate_program: KeyEvent {
        code: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    } ,
    prompt: KeyEvent {
        code: KeyCode::Char('.'),
        modifiers: KeyModifiers::ALT,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
    }
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

pub(crate) const DEFAULT_FUNNY_CONFIG: FunnyConfig = FunnyConfig {
    //wave_render currently isn't implemented
    wave_render: false,
    wiggle_render: false
};


pub(crate) const DEFAULT_CONFIG: Config = Config {
    funny_config: DEFAULT_FUNNY_CONFIG,
    tabs_to_spaces: true,
    tabsize: 3
};

pub(crate) const DEFAULT_PROMPTS: PossibleCommands = PossibleCommands {
    quit: "q",
    force_quit: "q!",
    save: "sv"
};