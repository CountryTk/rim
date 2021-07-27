use std::io::{self, stdin, stdout, Read, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct Coordinates {
    pub x: u16,
    pub y: u16,
}

#[derive(Copy, Clone)]
enum EditMode {
    Insert,
    Command,
}

struct Line {
    text: String,
    deleted: bool,
}

impl Line {
    fn new() -> Self {
        Line {
            text: String::new(),
            deleted: false,
        }
    }

    fn set_state(&mut self, state: bool) {
        self.deleted = state;
    }
}

struct Editor {
    buffer: Vec<Line>,
    command_buffer: String,
    cur_pos: Coordinates,
    terminal_size: Coordinates,
    mode: EditMode,
    last_cur_pos: Coordinates, // used for remembering cursor pos when switching modes
}

impl Editor {
    fn set_pos(&mut self, x: u16, y: u16) {
        self.cur_pos.x = x;
        self.cur_pos.y = y;
        print!("{}", termion::cursor::Goto(x, y));
    }
    fn set_last_pos(&mut self, x: u16, y: u16) {
        self.last_cur_pos.x = x;
        self.last_cur_pos.y = y;
    }
    fn clear(&self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    }

    fn show_status(&self) {
        let mut total_length: usize = 0;

        for i in &self.buffer {
            total_length += i.text.len();
        }

        let pos_info = format!("ROW:{}, COL:{}", self.cur_pos.y, self.cur_pos.x);

        print!(
            "{}{}{}L, {}C {} {} {}",
            termion::cursor::Goto(1, self.terminal_size.y),
            termion::clear::CurrentLine,
            self.buffer.len(),
            total_length,
            termion::cursor::Goto(
                (self.terminal_size.x) - 1 - pos_info.len() as u16,
                self.terminal_size.y
            ),
            &pos_info,
            termion::cursor::Goto(self.cur_pos.x, self.cur_pos.y)
        );
    }

    fn set_mode(&mut self, mode: EditMode) {
        self.mode = mode;
    }

    fn render_text(&mut self) {
        for (index, line) in self.buffer.iter().enumerate() {
            print!(
                "{}{}",
                line.text,
                termion::cursor::Goto(1, (index + 1) as u16)
            );
        }
        let last_index = self.buffer.len() - 1;

        self.set_pos(
            (self.buffer[last_index as usize].text.len() + 1) as u16,
            last_index as u16,
        );
    }
}

fn should_delete(line: &Line) -> bool {
    line.deleted
}

fn main() {
    // Get the standard input stream.

    let term_size = termion::terminal_size().unwrap();
    let mut buf = vec![Line {
        text: String::new(),
        deleted: false,
    }];
    let mut editor = Editor {
        cur_pos: Coordinates {
            x: 1 as u16,
            y: 1 as u16,
        },
        buffer: buf,
        command_buffer: String::new(),
        terminal_size: Coordinates {
            x: term_size.0,
            y: term_size.1,
        },
        last_cur_pos: Coordinates { x: 0, y: 0 },
        mode: EditMode::Insert,
    };

    loop {
        let stdin = stdin();
        // Get the standard output stream and go to raw mode.
        let mut stdout = stdout().into_raw_mode().unwrap();
        editor.clear();
        stdout.flush().unwrap();
        match editor.mode {
            EditMode::Insert => {
                print!("{}", termion::cursor::Goto(1, 1));
                if editor.buffer.len() != 1 {
                    editor.render_text();
                }

                if editor.last_cur_pos.x != 0 {
                    editor.set_pos(editor.last_cur_pos.x, editor.last_cur_pos.y);
                }

                for c in stdin.keys() {
                    // Clear the current line.
                    match c.unwrap() {
                        // Exit.
                        Key::Char('\n') => {
                            let cur_x = editor.cur_pos.x;
                            let cur_y = editor.cur_pos.y;
                            editor.buffer.insert(cur_y as usize, Line::new());
                            editor.set_pos(1, cur_y + 1);
                            print!("{}", termion::clear::AfterCursor);
                            for line in &editor.buffer[(cur_y as usize)..] {
                                println!("{}", line.text);
                            }
                            editor.show_status();
                        }
                        Key::Char(c) => {
                            let current_x = editor.cur_pos.x;
                            let current_y = editor.cur_pos.y;
                            let index = current_y - 1;
                            if editor.buffer.len() == 0 {
                                editor.buffer.push(Line {
                                    text: String::from(c),
                                    deleted: false,
                                });
                            } else {
                                if index >= editor.buffer.len() as u16 {
                                    let cur_x = editor.cur_pos.x;
                                    let cur_y = editor.cur_pos.y - 1;
                                    editor.buffer.insert(cur_y as usize, Line::new());
                                    editor.set_pos(1, cur_y + 1);
                                }

                                editor.buffer[index as usize]
                                    .text
                                    .insert((current_x - 1) as usize, c);
                                print!("{}", termion::cursor::Goto(1, current_y));
                                print!("{}", termion::clear::CurrentLine);
                                print!("{}", editor.buffer[index as usize].text);
                                print!(
                                    "{}",
                                    termion::cursor::Goto(
                                        (editor.buffer[index as usize].text.len() + 1) as u16,
                                        current_y
                                    )
                                );
                            }

                            //print!("{}", c);
                            editor.set_pos(current_x + 1, current_y);
                            editor.show_status();
                        }
                        Key::Alt(c) => println!("Alt-{}", c),
                        Key::Ctrl(c) => println!("Ctrl-{}", c),
                        Key::Left => {
                            if editor.cur_pos.x != 1 {
                                editor.set_pos(editor.cur_pos.x - 1, editor.cur_pos.y);
                                editor.show_status();
                            }
                        }
                        Key::Right => {
                            if editor.buffer[(editor.cur_pos.y - 1) as usize].text.len() as u16
                                >= editor.cur_pos.x
                            {
                                editor.set_pos(editor.cur_pos.x + 1, editor.cur_pos.y);
                            }
                             editor.show_status();
                        }
                        Key::Up => {
                            if editor.cur_pos.y != 1 {
                                if editor.buffer[(editor.cur_pos.y - 2) as usize].text.len() == 0 {
                                    editor.set_pos(1, editor.cur_pos.y - 1)
                                } else {
                                    editor.set_pos(editor.cur_pos.x + 1, editor.cur_pos.y - 1);
                                }
                                editor.show_status();
                            }
                        }
                        Key::Down => {
                            if !(editor.cur_pos.y >= editor.buffer.len() as u16) {
                                editor.set_pos(1, editor.cur_pos.y + 1);
                            }

                            //editor.show_status();
                        }
                        Key::Backspace => {
                            let mut buffer_index = (editor.cur_pos.y - 1) as usize;
                            let mut pos_x: u16 = 1;
                            let mut pos_y: u16 = 1;

                            if editor.cur_pos.x != 1 {
                                if editor.cur_pos.x as usize
                                    == editor.buffer[buffer_index].text.len() + 1
                                    || editor.cur_pos.x as usize
                                        == editor.buffer[buffer_index].text.len()
                                {
                                    editor.buffer[buffer_index].text.pop();
                                } else {
                                    if editor.buffer[buffer_index].text.len()
                                        == editor.cur_pos.x as usize - 2
                                    {
                                    } else {
                                        editor.buffer[buffer_index]
                                            .text
                                            .remove(editor.cur_pos.x as usize - 2);
                                    }
                                }

                                pos_x = editor.cur_pos.x - 1;
                                pos_y = editor.cur_pos.y;

                                if editor.buffer[buffer_index].text.len() == 0
                                    && editor.cur_pos.x != 1
                                {
                                    editor.buffer[buffer_index].set_state(true);
                                }
                            } else if editor.cur_pos.y != 1 {
                                let mut index: u16 = 1;
                                if (buffer_index < editor.buffer.len()) {

                                    index = editor.buffer[buffer_index].text.len() as u16; }
                                if buffer_index < editor.buffer.len()
                                    && editor.buffer[buffer_index].text.len() == 0
                                {
                                    editor.buffer[buffer_index].set_state(true);
                                }

                                if editor.cur_pos.x == 1 {
                                    /*
                                     * Check if the cursor is at position 1 and if there are any
                                     * trailing chars, if there are then push them into the line
                                     * before and delete current line
                                     * */
                                    if editor.buffer.len() > buffer_index && editor.buffer[buffer_index].text.len() != 0 {
                                        let current_text = editor.buffer[buffer_index].text.clone();

                                        editor.buffer[buffer_index-1].text.push_str(&current_text);
                                        editor.buffer[buffer_index].set_state(true);
                                        pos_x = index;
                                        pos_y = editor.cur_pos.y - 1
                                    } else {
                                        pos_x = index +1;
                                        pos_y = editor.cur_pos.y - 1;
                                    }

                                } else {


                                buffer_index -= 1;
                                pos_x = index + 1;
                                pos_y = editor.cur_pos.y - 1;
                                }
                            }
                            editor.set_pos(pos_x, pos_y);
                            print!("{}", termion::clear::AfterCursor);
                            for (index, line) in editor.buffer[(editor.cur_pos.y as usize)..]
                                .iter()
                                .enumerate()
                            {
                                let i = editor.buffer[..(editor.cur_pos.y as usize)].len();
                                println!(
                                    "{}{}",
                                    termion::cursor::Goto(1, (i + index) as u16),
                                    line.text
                                );
                            }
                            editor.buffer.retain(|line| !should_delete(line));
                            editor.show_status();
                        }
                        Key::Esc => {
                            editor.set_last_pos(editor.cur_pos.x, editor.cur_pos.y);
                            editor.set_mode(EditMode::Command);
                            editor.clear();
                            break;
                        }

                        _ => println!(""),
                    }

                    stdout.flush().unwrap();
                }
            }
            EditMode::Command => {
                print!("{}", termion::cursor::Goto(1, 1));
                editor.render_text();
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
