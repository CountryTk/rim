use std::io::{Stdout, stdout, Write};
use crate::editor::Editor;
use crate::util::{set_pos, Coordinates, EditMode, Position, StatusCodes};
use std::process;
use std::process::exit;
use termion::raw::RawTerminal;

pub struct CommandLine {
    pub(crate) buffer: String,
    pub(crate) cur_pos: Coordinates,
}

impl Position for CommandLine {
    fn cur_pos(&self) -> Coordinates {
        todo!()
    }

    fn set_x(&mut self, x: u16) {
        self.cur_pos.x = x;
    }

    fn set_y(&mut self, y: u16) {
        self.cur_pos.y = y;
    }
}

impl CommandLine {
    pub fn new() -> CommandLine {
        let term_size = termion::terminal_size().unwrap();

        CommandLine {
            buffer: String::from(""),
            cur_pos: Coordinates {
                x: 1,
                y: term_size.1,
            },
        }
    }

    pub fn handle_newline(&mut self, editor: &mut Editor ) -> StatusCodes {
        if self.buffer == ":q" {
            StatusCodes::Exit
        } else if self.buffer == "i" {
            editor.set_mode(EditMode::Insert);
            self.buffer.clear();
            set_pos(self, 1, 1);
            StatusCodes::NoOp
        } else {
            StatusCodes::NoOp
        }
    }

    pub fn handle_char(&mut self, c: char) {
        print!("{}", c);
        let x = self.cur_pos.x;
        let y = self.cur_pos.y;
        set_pos(self, x + 1, y);
        self.buffer.push(c);
    }

    pub fn handle_delete(&mut self) {
        if self.cur_pos.x != 1 {
            self.buffer.pop();
            let x = self.cur_pos.x;
            let y = self.cur_pos.y;
            set_pos(self, x - 1, y);
            print!("{}", termion::clear::AfterCursor);
        }
    }
}
