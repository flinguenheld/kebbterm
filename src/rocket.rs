use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

use crate::frame::Drawable;
use crate::{NUM_COLS, NUM_ROWS};

const SPEED_MIN: f32 = 10.0;
const SPEED_MAX: f32 = 30.0;

const M: f32 = (SPEED_MIN - SPEED_MAX) / (NUM_ROWS as f32 - 2.0);
const B: f32 = SPEED_MAX - M * 2.0;

struct Point {
    x: usize,
    y: usize,
}

struct Data {
    value: usize,
    count: usize,
}

pub struct Rocket {
    speed: Data,
    curve: Data,
    positions: VecDeque<Point>,
}

impl Rocket {
    pub fn new() -> Rocket {
        let mut rocket = Rocket {
            speed: Data {
                value: 10,
                count: 0,
            },
            curve: Data { value: 1, count: 0 },
            positions: VecDeque::new(),
        };

        rocket.positions.push_front(Point {
            x: 10,
            y: NUM_ROWS - 1,
        });

        rocket
    }
    pub fn run(&mut self) {
        // Speed management
        let previous = self.positions.front().unwrap();

        self.speed.value = (M * previous.y as f32 + B) as usize;

        // dbg!(NUM_ROWS as f32, M, B, self.speed.value);

        if self.speed.count >= self.speed.value || self.speed.count == 0 {
            let mut new_position = Point { ..*previous };

            if previous.y > 0 {
                new_position.y = previous.y - 1;
            } else {
                new_position.y = NUM_ROWS - 1;
            }

            self.speed.count = 0;

            if self.curve.value == self.curve.count {
                if previous.x < NUM_COLS - 1 {
                    new_position.x += 1;
                }

                self.curve.count = 0;
            } else {
                self.curve.count += 1;
            }

            // --
            self.positions.push_front(new_position);
            if self.positions.len() > 5 {
                self.positions.pop_back();
            }
        }
        self.speed.count += 1;
    }
}

impl Drawable for Rocket {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for pos in self.positions.iter() {
            frame[pos.y][pos.x] = 'R';
        }
        // thread::sleep(Duration::from_millis(10000));
    }
}
