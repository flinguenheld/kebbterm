use crate::{geometry::Point, render::frame::Frame};
use crossterm::style;
use std::collections::VecDeque;

/*
 * Display a 'tail' of one char.
 * Give it the next position and use the function 'draw' to update the frame.
 */
pub struct Tail {
    pub value: char,
    positions: VecDeque<Point>,
    length: usize,

    colors: Vec<u8>,
}

impl Tail {
    pub fn new(
        new_value: char,
        new_length: usize,
        first_position: Point,
        new_colors: Vec<u8>,
    ) -> Tail {
        Tail {
            value: new_value,
            positions: VecDeque::from([first_position]),
            length: new_length,
            colors: new_colors,
        }
    }

    pub fn push(&mut self, point: Point) {
        self.positions.push_front(point);
        if self.positions.len() > self.length {
            self.positions.pop_back();
        }
    }

    pub fn set_color(&mut self, new_colors: Vec<u8>) {
        self.colors = new_colors
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

    pub fn draw(&self, frame: &mut Frame) {
        for iter in self.positions.iter().rev().zip(self.colors.iter().rev()) {
            frame[iter.0.y][iter.0.x].value = self.value;
            frame[iter.0.y][iter.0.x].fore_color = style::Color::AnsiValue(*iter.1);
        }
    }
}
