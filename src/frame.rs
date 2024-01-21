use crate::{NB_COLS, NB_ROWS};

pub type Frame = [[char; NB_COLS]; NB_ROWS];

pub fn new_frame() -> Frame {
    [[' '; NB_COLS]; NB_ROWS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
