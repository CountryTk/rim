mod command_line;
mod editor;
mod util;

use crate::command_line::CommandLine;
use crate::editor::Editor;
use crate::util::{set_pos, EditMode};
use ropey::Rope;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use crate::util::StatusCodes::Exit;

fn run() {
    let mut editor = Editor::new();
    let mut command_line = CommandLine::new();

    loop {
        let stdin = stdin();
        // Get the standard output stream and go to raw mode.
        let mut stdout = stdout().into_raw_mode().unwrap();
        editor.clear();
        stdout.flush().unwrap();

        match editor.mode {
            EditMode::Insert => {
                print!("{}", termion::cursor::Goto(1, 1));
                if editor.last_cur_pos.x != 0 {
                    editor.set_pos(editor.last_cur_pos.x, editor.last_cur_pos.y);
                }

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
                            editor.set_last_pos(editor.cur_pos.x, editor.cur_pos.y);
                            editor.set_mode(EditMode::Command);
                            editor.clear();
                            break;
                        }

                        _ => println!(),
                    }

                    stdout.flush().unwrap();
                }
            }
            EditMode::Command => {
                print!("{}", termion::cursor::Goto(1, 1));
                editor.set_pos(1, command_line.cur_pos.y);
                stdout.flush().unwrap();
                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('\n') => if command_line.handle_newline(&mut editor) == Exit {
                            editor.clear();
                            return;
                        } else {},
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
