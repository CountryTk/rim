mod editor;

use crate::editor::{EditMode, Editor};
use std::io::{stdin, stdout, Write};
use ropey::Rope;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn run() {
    let mut editor = Editor::new();

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
                editor.render_current_line();
                editor.set_pos(1, editor.terminal_size.y);
                stdout.flush().unwrap();
                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('\n') => {
                            if editor.command_buffer == ":q" {
                                editor.clear();
                                return;
                            } else if editor.command_buffer == "i" {
                                editor.set_mode(EditMode::Insert);
                                editor.command_buffer.clear();
                                editor.set_pos(1, 1);
                                break;
                            }
                        }
                        Key::Char(c) => {
                            print!("{}", c);
                            editor.set_pos(editor.cur_pos.x + 1, editor.terminal_size.y);
                            editor.command_buffer.push(c);
                        }
                        Key::Backspace => {
                            editor.command_buffer.pop();
                            if editor.cur_pos.x != 1 {
                                editor.set_pos(editor.cur_pos.x - 1, editor.terminal_size.y);
                                print!("{}", termion::clear::CurrentLine);
                                print!("{}", editor.command_buffer);
                            }
                        }
                        _ => {}
                    }
                    stdout.flush().unwrap();
                }
            }
        }
    }

}

fn main() {
    // Get the standard input stream.
    let mut rope = Rope::new();
    rope.insert_char(0, 'h');
    rope.insert_char(1, 'e');
    rope.insert_char(2, 'l');
    rope.insert_char(3, 'l');
    rope.insert_char(4, 'o');

    // dbg!(rope.to_string());
    // dbg!(rope.len_chars());
    // rope.remove(4..5);
    //
    // dbg!(rope.to_string());
    run();
}
