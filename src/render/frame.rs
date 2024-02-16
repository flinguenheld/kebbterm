use crate::geometry::{NB_COLS, NB_ROWS};
use crossterm::style;

/// Regroup information for one case.
#[derive(Copy, Clone)]
pub struct FrameCase {
    pub value: char,
    pub fore_color: style::Color,
    pub back_color: style::Color,
}

/// Represent the mode (window) area.  
/// Size is fixed in [`crate::geometry`].
pub type Frame = [[FrameCase; NB_COLS]; NB_ROWS];

pub fn new_frame() -> Frame {
    [[FrameCase {
        value: ' ',
        fore_color: style::Color::White,
        back_color: style::Color::AnsiValue(235),
    }; NB_COLS]; NB_ROWS]
}
