use crate::util::{Coordinates, EditorMode, Position, set_pos};
use ropey::Rope;

pub struct Editor {
    pub(crate) buffer: Vec<Rope>,
    pub(crate) cur_pos: Coordinates,
    pub(crate) terminal_size: Coordinates,
    pub(crate) mode: EditorMode,
    pub(crate) last_cur_pos: Coordinates, // used for remembering cursor pos when switching modes
    pub(crate) total_length: u32,         // used for remembering cursor pos when switching modes
}


impl Position for Editor {
    fn set_x(&mut self, x: u16) {
        self.cur_pos.x = x;
    }

    fn set_y(&mut self, y: u16) {
        self.cur_pos.y = y;
    }
}
impl Editor {
    pub fn new() -> Self {
        let term_size = termion::terminal_size().unwrap();
        let buf = Rope::new();
        Editor {
            cur_pos: Coordinates { x: 1, y: 1 },
            buffer: vec![buf],
            terminal_size: Coordinates {
                x: term_size.0,
                y: term_size.1,
            },
            last_cur_pos: Coordinates { x: 1, y: 1 },
            mode: EditorMode::Command,
            total_length: 0,
        }
    }

    pub fn set_last_pos(&mut self, x: u16, y: u16) {
        self.last_cur_pos.x = x;
        self.last_cur_pos.y = y;
    }

    pub fn get_current_y(&self) -> u16 {
        self.cur_pos.y
    }
    pub fn get_current_x(&self) -> u16 {
        self.cur_pos.x
    }

    pub fn get_current_y_usize(&self) -> usize {
        self.cur_pos.y as usize
    }

    pub fn get_current_x_usize(&self) -> usize {
        self.cur_pos.x as usize
    }

    pub fn clear(&self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    }

    pub fn show_status(&self) {
        let pos_info = format!("ROW:{}, COL:{}", self.cur_pos.y, self.cur_pos.x);

        print!(
            "{}{}{}L, {}C {} {} {}",
            termion::cursor::Goto(1, self.terminal_size.y),
            termion::clear::CurrentLine,
            self.buffer[self.get_current_y_usize() - 1].len_chars(),
            self.total_length,
            termion::cursor::Goto(
                (self.terminal_size.x) - 1 - pos_info.len() as u16,
                self.terminal_size.y
            ),
            &pos_info,
            termion::cursor::Goto(self.cur_pos.x, self.cur_pos.y)
        );
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    pub fn _render_line_text(&self, line_number: u16) {
        for (index, line) in self.buffer[line_number as usize].chars().enumerate() {
            print!("{}{}", termion::cursor::Goto((index + 1) as u16, 1), line,);
        }
    }

    pub fn render_current_line(&mut self) {
        for (index, line) in self.buffer[(self.cur_pos.y - 1) as usize]
            .chars()
            .enumerate()
        {
            print!(
                "{}{}",
                termion::cursor::Goto((index + 1) as u16, self.cur_pos.y),
                line,
            );
        }
    }
    pub fn handle_delete(&mut self) {
        let x = self.get_current_x_usize();
        let y = self.get_current_y_usize();
        let len = self.buffer[y - 1].len_chars();
        if len == 0 && y >= 2 {
            let new_x = self.buffer[y - 2].len_chars() + 1;
            set_pos(self, new_x as u16, (y - 1) as u16);
            self.show_status();
        } else if len == 1 {
            self.buffer[y - 1] = Rope::new();
            self.total_length -= 1;
            set_pos(self, (x - 1) as u16, y as u16);
            print!(" ");
        } else if len >= 2 {
            self.buffer[y - 1].remove(len - 1..len);
            set_pos(self, (x - 1) as u16, y as u16);
            self.total_length -= 1;
            print!(" ");
        }

        self.show_status();
    }
    pub fn handle_down(&mut self) {
        if self.get_current_y() < self.buffer.len() as u16 {
            let down = self.buffer[self.get_current_y_usize()].len_chars() + 1;

            set_pos(self, down as u16, self.cur_pos.y + 1);
        }
        self.show_status();
    }
    pub fn handle_up(&mut self) {
        if self.get_current_y_usize() >= 2 {
            let up = self.buffer[self.get_current_y_usize() - 2].len_chars() + 1;
            set_pos(self, up as u16, (self.get_current_y_usize() - 1) as u16);
            self.show_status();
        }
    }
    pub fn handle_right(&mut self) {
        if self.buffer[self.get_current_y_usize() - 1].len_chars() as u16 >= self.get_current_x() {
            set_pos(self, self.get_current_x() + 1, self.get_current_y());
        }
        self.show_status();
    }
    pub fn handle_left(&mut self) {
        if self.get_current_x() != 1 {
            set_pos(self, self.get_current_x() - 1, self.get_current_y());
            self.show_status();
        }
    }

    pub fn handle_char(&mut self, c: char) {
        let current_x = self.get_current_x_usize();
        let current_y = self.get_current_y_usize();
        let buffer = &self.buffer[current_y - 1];
        if buffer.len_chars() != current_x - 1 {
            // we are inserting in the middle of the string
            let slice = buffer.slice(0..current_x);
            let length = slice.len_chars() + 1;

            self.buffer[current_y - 1].insert_char(current_x - 1, c);

            self.render_current_line();
            set_pos(self, length as u16, current_y as u16);
        } else {
            self.buffer[current_y - 1].insert_char(current_x - 1, c);
            self.render_current_line();
            set_pos(self, (current_x + 1) as u16, current_y as u16);
        }
        self.total_length += 1;
        self.show_status();
    }
    pub fn handle_newline(&mut self) {
        set_pos(self, 1, self.get_current_y() + 1);

        match self.buffer.get(self.get_current_y_usize()) {
            None => {
                self.buffer.push(Rope::new());
            }
            Some(_) => {
                //self.render_line_text(self.get_current_y())
            }
        }
        self.show_status();
    }

    pub fn handle_esc(&mut self) {
        self.set_last_pos(self.get_current_x(), self.get_current_y());
        set_pos(self, 1, self.terminal_size.y);
        self.set_mode(EditorMode::Command);
    }
}
