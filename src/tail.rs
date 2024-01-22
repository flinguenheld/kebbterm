use crate::{frame::Drawable, Point};
use std::collections::VecDeque;

/*
 * Display a 'tail' of one char.
 * Give it the next position and use the trait 'draw' to update the frame.
 */
pub struct Tail {
    pub value: char,
    positions: VecDeque<Point>,
    length: usize,
}

impl Tail {
    pub fn new(new_value: char, new_length: usize, first_position: Point) -> Tail {
        Tail {
            value: new_value,
            positions: VecDeque::from([first_position]),
            length: new_length,
        }
    }

    pub fn push(&mut self, point: Point) {
        self.positions.push_front(point);
        if self.positions.len() > self.length {
            self.positions.pop_back();
        }
    }

    pub fn current_position(&self) -> Option<&Point> {
        self.positions.front()
    }

    pub fn pop(&mut self) {
        self.positions.pop_back();
    }

    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    pub fn clear(&mut self) {
        self.positions.clear();
    }
}

impl Drawable for Tail {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for pos in self.positions.iter() {
            frame[pos.y][pos.x] = self.value;
        }
    }
}
