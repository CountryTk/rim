use crate::editor::Editor;
use crate::util::{set_buffer_pos, Coordinates, EditorMode, Position, StatusCodes};

pub struct CommandLine {
    pub(crate) buffer: String,
    pub(crate) cur_pos: Coordinates,
    pub(crate) y: u16,
}

impl Position for CommandLine {
    fn set_buffer_x(&mut self, x: u16) {
        self.cur_pos.x = x;
    }

    fn set_buffer_y(&mut self, y: u16) {
        self.cur_pos.y = y;
    }

    fn set_screen_x(&mut self, x: u16) {
        todo!()
    }

    fn set_screen_y(&mut self, y: u16) {
        todo!()
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
            y: term_size.1,
        }
    }

    pub fn handle_newline(&mut self, editor: &mut Editor) -> StatusCodes {
        if self.buffer == ":q" {
            StatusCodes::Exit
        } else if self.buffer == ":i" {
            editor.set_mode(EditorMode::Insert);
            self.buffer.clear();
            set_buffer_pos(self, editor.last_cur_pos.x, editor.last_cur_pos.y);
            StatusCodes::Insert
        } else {
            StatusCodes::NoOp
        }
    }

    pub fn handle_char(&mut self, c: char) {
        print!("{}", c);
        let x = self.cur_pos.x;
        set_buffer_pos(self, x + 1, self.y);
        self.buffer.push(c);
    }

    pub fn handle_delete(&mut self) {
        if self.cur_pos.x != 1 {
            self.buffer.pop();
            let x = self.cur_pos.x;
            let y = self.cur_pos.y;
            set_buffer_pos(self, x - 1, y);
            print!("{}", termion::clear::AfterCursor);
        }
    }
}
