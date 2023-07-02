mod command_line;
mod editor;
mod util;

use std::collections::HashMap;
use crate::command_line::CommandLine;
use crate::editor::Editor;
use crate::util::StatusCodes::Exit;
use crate::util::{EditorMode, set_pos, StatusCodes};
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::terminal_size;

fn run() {
    let mut editor = Editor::new();
    let mut command_line = CommandLine::new();
    let size = terminal_size().unwrap();

    let header = "RIM - VIM clone written in rust";
    let manual_1 = "type :i to enter INSERT mode";
    let manual_2 = "type :q to QUIT";
    let x = size.0 / 2;
    let y = size.1 / 2;

    let mut splash_screen: HashMap<(u16, u16), &str> = HashMap::new();
    splash_screen.insert((x-(header.len() - 17) as u16, y - 2), header);
    splash_screen.insert((x-(header.len() - 10) as u16, y + 2), manual_1);
    splash_screen.insert((x-(header.len() - 10) as u16, y + 3), manual_2);

    editor.clear();
    for (key, value) in &splash_screen {
        print!("{}{}", termion::cursor::Goto(key.0, key.1), value);
    }
    loop {
        let stdin = stdin();
        // Get the standard output stream and go to raw mode.
        let mut stdout = stdout().into_raw_mode().unwrap();

        stdout.flush().unwrap();

        match editor.mode {
            EditorMode::Insert => {
                let x = editor.last_cur_pos.x;
                let y = editor.last_cur_pos.y;
                for (key, value) in &splash_screen {
                    print!("{}{}", termion::cursor::Goto(key.0, key.1), termion::clear::CurrentLine);
                }
                set_pos(&mut editor, x, y);

                stdout.flush().unwrap();

                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('\n') => editor.handle_newline(),
                        Key::Char(c) => editor.handle_char(c),
                        Key::Alt(c) => println!("Alt-{}", c),
                        Key::Ctrl(c) => println!("Ctrl-{}", c),
                        Key::Left => editor.handle_left(),
                        Key::Right => editor.handle_right(),
                        Key::Up => editor.handle_up(),
                        Key::Down => editor.handle_down(),
                        Key::Backspace => editor.handle_delete(),
                        Key::Esc => {
                            editor.handle_esc();
                            break;
                        }

                        _ => println!(),
                    }

                    stdout.flush().unwrap();
                }
            }
            EditorMode::Command => {
                print!("{}{}", termion::cursor::Goto(1, command_line.y), termion::clear::AfterCursor);
                set_pos(&mut editor, 1, command_line.y);
                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('\n') => match command_line.handle_newline(&mut editor) {
                            Exit => {
                                editor.clear();
                                return;
                            }
                            StatusCodes::NoOp => {}
                            StatusCodes::Insert => {
                                break;
                            }
                        },
                        Key::Char(c) => command_line.handle_char(c),
                        Key::Backspace => command_line.handle_delete(),
                        _ => {}
                    }
                    stdout.flush().unwrap();
                }
            }
        }
    }
}

fn main() {
    run();
}
