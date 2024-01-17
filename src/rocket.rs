use crate::frame::Drawable;
use crate::{NUM_COLS, NUM_ROWS};

pub struct Rocket {
    x: usize,
    y: usize,
    speed: u8,
}

impl Rocket {
    pub fn new() -> Rocket {
        Rocket {
            x: 10,
            y: NUM_ROWS,
            speed: 3,
        }
    }
    pub fn run(&mut self) {
        // Add speed management

        if self.y > 0 {
            self.y -= 1;
        }
    }
}

impl Drawable for Rocket {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.y][self.x] = 'R';
    }
}
