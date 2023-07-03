#[derive(Debug, Copy, Clone)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

pub trait Position {
    fn set_buffer_x(&mut self, x: u16);
    fn set_buffer_y(&mut self, y: u16);
    fn set_screen_x(&mut self, x: u16);
    fn set_screen_y(&mut self, y: u16);
}

#[derive(PartialOrd, PartialEq)]
pub enum StatusCodes {
    Exit,
    NoOp,
    Insert,
}

#[derive(Copy, Clone)]
pub enum EditorMode {
    Insert,
    Command,
}

pub fn set_buffer_pos<T>(s: &mut T, x: u16, y: u16)
    where
        T: Position,
{
    s.set_buffer_x(x);
    s.set_buffer_y(y);
    // set_pos(x,y );
}
pub fn set_screen_pos<T>(s: &mut T, x: u16, y: u16)
    where
        T: Position,
{
    s.set_screen_x(x);
    s.set_screen_y(y);
    set_pos(x, y);
}

fn set_pos(x: u16, y: u16) {
    print!(
        "{}{}",
        termion::cursor::SteadyBar,
        termion::cursor::Goto(x, y)
    )
}

