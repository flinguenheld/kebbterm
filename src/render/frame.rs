use crate::geometry::{NB_COLS, NB_ROWS};
use crossterm::style;

#[derive(Copy, Clone)]
pub struct FrameCase {
    pub value: char,
    pub fore_color: style::Color,
    pub back_color: style::Color,
}

pub type Frame = [[FrameCase; NB_COLS]; NB_ROWS];

pub fn new_frame() -> Frame {
    [[FrameCase {
        value: ' ',
        fore_color: style::Color::White,
        back_color: style::Color::AnsiValue(235),
    }; NB_COLS]; NB_ROWS]
}

// TODO: Factorise with Welcome
pub fn print(frame: &mut Frame, y: usize, text: &str, color: u8) {
    let start_x = NB_COLS / 2 - text.chars().count() / 2 - 1;
    for (x, c) in text.chars().enumerate() {
        frame[y][start_x + x].value = c;
        frame[y][start_x + x].fore_color = style::Color::AnsiValue(color);
    }
}

pub fn paint(frame: &mut Frame, x: usize, y: usize, height: usize, width: usize, color: u8) {
    for row in y..(y + height) {
        for col in x..(x + width) {
            frame[row][col].back_color = style::Color::AnsiValue(color);
        }
    }
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
