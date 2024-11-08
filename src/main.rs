use std::io::{self, stdout};
use std::env;

use crossterm::{execute, queue, terminal::{disable_raw_mode, enable_raw_mode, SetSize, EnterAlternateScreen, LeaveAlternateScreen, SetTitle}};
use crossterm::cursor::{Show, Hide};
use crossterm::event::KeyEvent;
use file_interact::{get_configs, read_text_file, Config};

use crate::file_interact::get_keybinds;
use crate::user_interact::*;
use crate::render::*;
use crate::constants::DEFAULT_WINDOW;

mod user_interact;
mod render;
mod constants;
mod file_interact;
mod user_prompt;
mod cursor_movement;



fn setup() -> Result<(Cursor, Keybinds, Window), std::io::Error> {
    //Get the keybinds for which the program uses
    //TODO: Replace "./foo.txt" with actual config file.
    let mut window = DEFAULT_WINDOW;
    let keybinds = get_keybinds("./foo.txt");

    //SetTitle doesn't seem to work?
    let _ = queue!(stdout(), EnterAlternateScreen, SetTitle("LElle momento editor"))?;
    let _ = enable_raw_mode()?;
    let cursor: Cursor = Cursor {
        pos_x: 0,
        pos_y: 0
    };

    let _ = execute!(io::stdout(), SetSize(window.size_x.try_into().unwrap(), window.size_y.try_into().unwrap()))?;
    let _ = clear_screen()?;
    Ok((cursor, keybinds, window))
}

fn main() -> io::Result<()> {
    //Forces the program to run terminate_program()
    //on shutdown through the Drop trait.
    //Mainly to stop raw mode escaping into the terminal on crash.
    let _proper_terminate = Terminate;
    
    //Preparing the command line args, data to be edited and save file.
    let args: Vec<String> = env::args().collect();
    let mut data: Vec<Vec<char>> = vec!(vec!()); 
    let mut save_path: String = "".to_string();
    let mut is_saved: bool = true;
    //length is 1 since the command used to run the program also counts.
    if args.len() > 2 {
        println!("Enter one file to open!\n(Command line argument.)");
        return Ok(())
    } else if args.len() == 2 {
        save_path = args[1].to_string();
        data = read_text_file(&save_path);
    }
    let configs: Config = get_configs("the_funny.txt");
    let mut repeat_render = false;
    if configs.funny_config.wiggle_render == true {repeat_render = true}
    
    //Create the cursor, keybinds and window.
    let Ok((mut cursor, keybinds, mut window)) = setup()
    else {
        unimplemented!()
    };

    //Initial screen draw.
    let _ = draw_screen(&data, &cursor, &mut window, &configs);

    //Main runtime loop.
    loop {
        let event: Result<KeyEvent, io::Error> = read_key(repeat_render);
        let event: KeyEvent = match event {
            Err(_) => unimplemented!(),
            Ok(event) => event
        };
        //TODO: Come up with better name for to_print.
        //It prints at the last line of the terminal,
        //so like output for various commands.
        let mut to_print: String = "".to_string();
        
        //Todo: Fix lifetime stuff so don't have to clone
        let mut action = process_keypress(&data, &cursor,&event, &keybinds, save_path.clone());
        
        while match action.clone() {
            Action::None => false,
            Action::UtilAction(UtilAction::GetSavePath(a)) => {save_path = a; true},
            Action::UtilAction(UtilAction::Save) => {action = Action::UtilAction(UtilAction::SaveAs(save_path.clone())); is_saved = true; true}
            Action::ModDataAction(_) => {is_saved = false; true},
            Action::UtilAction(UtilAction::TryKill) => {if is_saved {action = Action::UtilAction(UtilAction::Kill)} else {action = Action::UtilAction(UtilAction::AskSave)}; true}
            Action::PrintResult(s) => {to_print = s; false},
            _ => true
            } {
            action = do_action(&mut cursor, &mut data, action, &keybinds, &window, configs);
        }
        
        let _ = queue!(stdout(), Hide);
        let _ = clear_screen();
        let _ = draw_screen(&data, &cursor, &mut window, &configs);

        let _ = draw_line((0, window.size_y as u16), &to_print);
        let _ = queue!(stdout(), Show);
        let _ = update_cursor(&mut cursor, &window, &configs);
    }
    Ok(())
}

struct Terminate;

impl Drop for Terminate {
    fn drop(&mut self) {
        let _ = terminate_program();
    }
}

pub(crate) fn terminate_program() {
    let _ = clear_screen();
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen);
    let _ = disable_raw_mode();
    let _ = println!("Shutting down program.");
    std::process::exit(0);
}