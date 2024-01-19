use rand::Rng;
use std::collections::VecDeque;

use crate::frame::Drawable;
use crate::{Point, NUM_COLS, NUM_ROWS};

pub struct Rocket {
    speed_value: usize,
    speed_count: usize,
    speed_m: f32,
    speed_b: f32,

    positions: VecDeque<Point>,
    end_row: usize,
    done: bool,

    value: char,
}

impl Rocket {
    pub fn new() -> Rocket {
        let mut rocket = Rocket {
            speed_value: 10,
            speed_count: 0,
            speed_m: 0.0, // Speed equation
            speed_b: 0.0,

            positions: VecDeque::new(),
            end_row: rand::thread_rng().gen_range(3, 7),
            done: false,

            value: '0',
        };

        rocket.positions.push_front(Point {
            x: rand::thread_rng().gen_range(7, NUM_COLS - 8), // TODO Adapt min/man ?
            y: NUM_ROWS - 1,
        });

        // Speed --
        let speed_min = rand::thread_rng().gen_range(30.0, 35.0);
        let speed_max = rand::thread_rng().gen_range(18.0, 20.0);
        rocket.speed_m = (speed_max - speed_min) / (NUM_ROWS as f32 - 2.0);
        rocket.speed_b = speed_max - rocket.speed_m * 2.0;

        rocket
    }

    pub fn value(&self) -> char {
        self.value
    }
    pub fn done(&self) -> bool {
        self.done
    }

    pub fn run(&mut self) {
        if self.speed_count >= self.speed_value || self.speed_count == 0 {
            if let Some(current) = self.positions.front() {
                // Adapt speed --
                self.speed_value = (self.speed_m * current.y as f32 + self.speed_b) as usize;

                // Is done ?
                if current.y == self.end_row {
                    self.positions.pop_back();
                    self.speed_count = 1;
                    self.done = self.positions.is_empty();
                } else {
                    let mut new_position = Point {
                        y: current.y - 1,
                        ..*current
                    };

                    // Up value --
                    let m: f32 = (self.end_row as f32 - NUM_ROWS as f32) / 10.0;
                    let b: f32 = self.end_row as f32 - m * 10.0;
                    self.value =
                        char::from_digit(((current.y as f32 - b) / m) as u32, 10).unwrap_or('9');

                    // Move --
                    match rand::thread_rng().gen_range(0, 4) {
                        0 => {
                            if new_position.x < NUM_COLS - 1 {
                                new_position.x += 1
                            }
                        }
                        1 => {
                            if new_position.x > 0 {
                                new_position.x -= 1
                            }
                        }
                        _ => {}
                    }

                    self.speed_count = 0;

                    // Up the tail --
                    self.positions.push_front(new_position);
                    if self.positions.len() > 3 {
                        self.positions.pop_back();
                    }
                }
            }
        }
        self.speed_count += 1;
    }
}

impl Drawable for Rocket {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for pos in self.positions.iter() {
            frame[pos.y][pos.x] = self.value;
        }
    }
}
