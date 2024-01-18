use rand::Rng;
use std::collections::VecDeque;

use crate::frame::Drawable;
use crate::{NUM_COLS, NUM_ROWS};

const SPEED_MIN: f32 = 10.0; // Randomise ?
const SPEED_MAX: f32 = 30.0;

const M_SPEED: f32 = (SPEED_MIN - SPEED_MAX) / (NUM_ROWS as f32 - 2.0);
const B_SPEED: f32 = SPEED_MAX - M_SPEED * 2.0;

// #[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

pub struct Rocket {
    speed_value: usize,
    speed_count: usize,
    positions: VecDeque<Point>,
    end: usize,
    done: bool,

    value: u32,
}

impl Rocket {
    pub fn new() -> Rocket {
        let mut rocket = Rocket {
            speed_value: 10,
            speed_count: 0,
            positions: VecDeque::new(),
            end: rand::thread_rng().gen_range(3, 7),
            done: false,

            value: 0,
        };

        rocket.positions.push_front(Point {
            x: rand::thread_rng().gen_range(7, NUM_COLS - 8),
            y: NUM_ROWS - 1,
        });

        rocket
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn run(&mut self) {
        if self.speed_count >= self.speed_value || self.speed_count == 0 {
            if let Some(current) = self.positions.front() {
                // Adapt speed
                self.speed_value = (M_SPEED * current.y as f32 + B_SPEED) as usize;

                // Is done ?
                if current.y == self.end {
                    self.positions.pop_back();
                    self.speed_count = 1;
                    self.done = self.positions.is_empty();
                } else {
                    let mut new_position = Point {
                        y: current.y - 1,
                        ..*current
                    };

                    // Up value --
                    let m: f32 = (self.end as f32 - NUM_ROWS as f32) / 10.0;
                    let b: f32 = self.end as f32 - m * 10.0;
                    self.value = ((current.y as f32 - b) / m) as u32;

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

                    // --
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
            frame[pos.y][pos.x] = char::from_digit(self.value, 10).unwrap_or('9');
        }
    }
}
