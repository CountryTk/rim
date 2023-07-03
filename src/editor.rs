use crate::util::{Coordinates, EditorMode, Position, set_buffer_pos, set_screen_pos};
use ropey::Rope;

pub struct Editor {
    pub(crate) buffer: Vec<Rope>,
    pub(crate) buffer_position: Coordinates, // Holds the buffer coordinates to know where we are in the data structure
    pub(crate) terminal_size: Coordinates,
    pub(crate) mode: EditorMode,
    pub(crate) last_cur_pos: Coordinates, // used for remembering cursor pos when switching modes
    pub(crate) total_length: u32, // used for remembering cursor pos when switching modes
    pub(crate) window: Coordinates, // sliding window to enable scrolling of text editor content
    pub(crate) screen_position: Coordinates, // Holds the screen coordinates, bounded to terminal length and height
}


impl Position for Editor {
    fn set_buffer_x(&mut self, x: u16) {
        self.buffer_position.x = x;
    }

    fn set_buffer_y(&mut self, y: u16) {
        self.buffer_position.y = y;
    }

    fn set_screen_x(&mut self, x: u16) {
        self.screen_position.x = x;
    }

    fn set_screen_y(&mut self, y: u16) {
        self.screen_position.y = y;
    }
}
impl Editor {
    pub fn new(ropes: Option<Vec<Rope>>) -> Self {
        let term_size = termion::terminal_size().unwrap();
        let buf = Rope::new();
        Editor {
            buffer_position: Coordinates { x: 1, y: 1 },
            buffer: ropes.unwrap_or(vec![buf]),
            terminal_size: Coordinates {
                x: term_size.0,
                y: term_size.1,
            },
            last_cur_pos: Coordinates { x: 1, y: 1 },
            mode: EditorMode::Command,
            total_length: 0,
            window: Coordinates { x: 0, y: term_size.1 },
            screen_position: Coordinates { x: 0, y: 0 },
        }
    }

    pub fn set_last_pos(&mut self, x: u16, y: u16) {
        self.last_cur_pos.x = x;
        self.last_cur_pos.y = y;
    }

    pub fn get_current_buffer_position_y(&self) -> u16 {
        self.buffer_position.y
    }
    pub fn get_current_buffer_position_x(&self) -> u16 {
        self.buffer_position.x
    }

    pub fn get_current_buffer_position_y_usize(&self) -> usize {
        self.buffer_position.y as usize
    }

    pub fn get_current_buffer_position_x_usize(&self) -> usize {
        self.buffer_position.x as usize
    }

    pub fn get_current_screen_position_x(&self) -> u16 {self.screen_position.x}

    pub fn get_current_screen_position_y(&self) -> u16 {self.screen_position.y}

    pub fn clear(&self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    }

    pub fn show_status(&self) {
        let pos_info = format!("ROW:{}, COL:{}", self.buffer_position.y, self.buffer_position.x);

        print!(
            "{}{}{}L, {}C {} {} {}",
            termion::cursor::Goto(1, self.terminal_size.y),
            termion::clear::CurrentLine,
            self.buffer[self.get_current_buffer_position_y_usize() - 1].len_chars(),
            self.total_length,
            termion::cursor::Goto(
                (self.terminal_size.x) - 1 - pos_info.len() as u16,
                self.terminal_size.y
            ),
            &pos_info,
            termion::cursor::Goto(self.buffer_position.x, self.buffer_position.y)
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
    pub fn render_all_lines(&mut self) {
        self.clear();
        let start = self.window.x as usize;
        let end = if self.window.y as usize >= self.buffer.len() {self.buffer.len()} else {self.window.y as usize};
        for (line_nr, rope) in self.buffer[start..end].iter().enumerate() {
            for (index, line) in rope.chars().enumerate()
            {
                print!(
                    "{}{}",
                    termion::cursor::Goto((index + 1) as u16, (line_nr+1) as u16),
                    line,
                );
            }

        }
    }

    pub fn render_all_lines_from(&mut self, index: usize) {
        for (line_nr, rope) in self.buffer[index..].iter().enumerate() {
            for (index, line) in rope.chars().enumerate()
            {
                print!(
                    "{}{}",
                    termion::cursor::Goto((index + 1) as u16, (line_nr+1) as u16),
                    line,
                );
            }

        }
    }


    pub fn render_current_line(&mut self) {
        for (index, line) in self.buffer[(self.buffer_position.y - 1) as usize]
            .chars()
            .enumerate()
        {
            print!(
                "{}{}",
                termion::cursor::Goto((index + 1) as u16, self.buffer_position.y),
                line,
            );
        }
    }
    pub fn handle_delete(&mut self) {
        let x = self.get_current_buffer_position_x_usize();
        let y = self.get_current_buffer_position_y_usize();
        let len = self.buffer[y - 1].len_chars();
        if x <= 1 && y <= 1 {
            self.show_status();
            return;
        }
        else if x == 0 || x == 1 && y != 1 {
            let buffer_length = self.buffer[y-1].len_chars();
            let slice = self.buffer[y-1].slice(0..buffer_length).to_string();
            self.buffer.remove(y-1);
            print!("{}", termion::clear::CurrentLine);
            let new_buffer_length = self.buffer[y-2].len_chars() + 1;
            set_buffer_pos(self, new_buffer_length as u16, (y - 1) as u16);
            self.buffer[y-2].insert(new_buffer_length-1, slice.as_str());
// self.render_current_line();
            set_buffer_pos(self, new_buffer_length as u16, (y - 1) as u16);
            self.show_status();

        } else if len == 0 && y >= 2 {
            let new_x = self.buffer[y - 2].len_chars() + 1;
            self.buffer.remove(y-1);
            set_buffer_pos(self, new_x as u16, (y - 1) as u16);

        } else if len == 1 {
            self.buffer[y - 1] = Rope::new();
            self.total_length -= 1;
            set_buffer_pos(self, (x - 1) as u16, y as u16);
            print!("{}", termion::clear::CurrentLine);
// self.render_current_line();

        } else if len >= 2 {
            if len != x - 1 && x >= 2 && y != 0 {
                let slice = self.buffer[y-1].slice(0..x).len_chars();

                self.buffer[y - 1].remove(x-2..x-1);
                set_buffer_pos(self, (slice-1) as u16, y as u16);

                self.render_current_line();
            } else {
                self.buffer[y - 1].remove(len - 1..len);
                set_buffer_pos(self, (x - 1) as u16, y as u16);
            }
            self.total_length -= 1;
            print!("{}", termion::clear::CurrentLine);
// self.render_current_line();
        }
        self.render_all_lines();
        self.show_status();
    }
    pub fn handle_down(&mut self) {
        if self.get_current_screen_position_y() + 1 >= self.terminal_size.y {
            set_buffer_pos(self, 1, self.get_current_buffer_position_y()+1);
            self.window.x += 1;
            self.window.y += 1;
            set_screen_pos(self, 1, self.get_current_screen_position_y());
            self.render_all_lines();
        }
        else if self.get_current_buffer_position_y_usize() < self.terminal_size.y as usize {
            set_buffer_pos(self, 1, self.buffer_position.y + 1);
            set_screen_pos(self, 1, self.get_current_screen_position_y()+1)
        } self.show_status();
    }
    pub fn handle_up(&mut self) {
        if self.get_current_buffer_position_y_usize() >= 2 {
            let up = self.buffer[self.get_current_buffer_position_y_usize() - 2].len_chars() + 1;
            set_buffer_pos(self, up as u16, (self.get_current_buffer_position_y_usize() - 1) as u16);
            self.show_status();
        }
    }
    pub fn handle_right(&mut self) {
// println!(" {}", self.buffer[self.get_current_y_usize()-1].len_chars());
// println!(" {}", self.buffer[self.get_current_y_usize()-1].to_string());
        if self.buffer[self.get_current_buffer_position_y_usize() - 1].len_chars() as u16 >= self.get_current_buffer_position_x() {
            set_buffer_pos(self, self.get_current_buffer_position_x() + 1, self.get_current_buffer_position_y());
        }
        self.show_status();
    }
    pub fn handle_left(&mut self) {
        if self.get_current_buffer_position_x() != 1 {
            set_buffer_pos(self, self.get_current_buffer_position_x() - 1, self.get_current_buffer_position_y());
            self.show_status();
        }
    }

    pub fn handle_char(&mut self, c: char) {
        let current_x = self.get_current_buffer_position_x_usize();
        let current_y = self.get_current_buffer_position_y_usize();
        let buffer = &self.buffer[current_y - 1];
        if buffer.len_chars() != current_x - 1 {
// we are inserting in the middle of the string
            let slice = buffer.slice(0..current_x);
            let length = slice.len_chars() + 1;

            self.buffer[current_y - 1].insert_char(current_x - 1, c);

            self.render_current_line();
            set_buffer_pos(self, length as u16, current_y as u16);
        } else {
            self.buffer[current_y - 1].insert_char(current_x - 1, c);
            self.render_current_line();
            set_buffer_pos(self, (current_x + 1) as u16, current_y as u16);
        }
        self.total_length += 1;
        self.show_status();
    }
    pub fn handle_newline(&mut self) {
        set_buffer_pos(self, 1, self.get_current_buffer_position_y() + 1);

        match self.buffer.get(self.get_current_buffer_position_y_usize()) {
            None => {
                self.buffer.push(Rope::new());
            }
            Some(_) => {
                self.buffer.insert(self.get_current_buffer_position_y_usize()-1, Rope::new());
            }
        }
// TODO: Don't rerender EVERY line when pressing enter
        self.render_all_lines();
        self.show_status();
    }

    pub fn handle_esc(&mut self) {
        self.set_last_pos(self.get_current_buffer_position_x(), self.get_current_buffer_position_y());
        set_buffer_pos(self, 1, self.terminal_size.y);
        self.set_mode(EditorMode::Command);
    }

    pub fn handle_home(&mut self) {
        set_buffer_pos(self, 1, self.get_current_buffer_position_y());
        self.show_status();
    }
}