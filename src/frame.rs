use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = [[char; NUM_COLS]; NUM_ROWS];

pub fn new_frame() -> Frame {
    [[' '; NUM_COLS]; NUM_ROWS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
