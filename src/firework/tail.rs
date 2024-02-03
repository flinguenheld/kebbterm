use crate::{geometry::Point, render::frame::Frame};
use crossterm::style;
use std::collections::VecDeque;

/*
 * Display a 'tail' of one char.
 * Give it the next position and use the function 'draw' to update the frame.
 * Tail has the given color vector length.
 */
pub struct Tail {
    pub value: char,
    positions: VecDeque<Point>,
    colors: Vec<u8>,
}

impl Tail {
    pub fn new(new_value: char, first_position: Point, new_colors: Vec<u8>) -> Tail {
        Tail {
            value: new_value,
            positions: VecDeque::from([first_position]),
            colors: new_colors,
        }
    }

    pub fn push(&mut self, point: Point) {
        self.positions.push_front(point);
        if self.positions.len() > self.colors.len() {
            self.positions.pop_back();
        }
    }

    pub fn set_color(&mut self, mut new_colors: Vec<u8>) {
        while new_colors.len() > self.colors.len() {
            new_colors.pop();
        }

        self.colors = new_colors;
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
        for (pt, color) in self.positions.iter().rev().zip(self.colors.iter().rev()) {
            frame[pt.y][pt.x].value = self.value;
            frame[pt.y][pt.x].fore_color = style::Color::AnsiValue(*color);
        }
    }
}
