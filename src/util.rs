#[derive(Debug, Copy, Clone)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

pub trait Position {
    fn set_x(&mut self, x: u16);
    fn set_y(&mut self, y: u16);
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

pub fn set_pos<T>(s: &mut T, x: u16, y: u16)
    where
        T: Position,
{
    s.set_x(x);
    s.set_y(y);
    print!(
        "{}{}",
        termion::cursor::SteadyBar,
        termion::cursor::Goto(x, y)
    );
}
