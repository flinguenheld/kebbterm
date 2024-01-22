use crate::frame::Drawable;
use crate::tail::Tail;
use crate::{Point, NB_COLS, NB_ROWS};
use rand::Rng;

/*
 *
 */
pub struct Rocket {
    tail: Tail,

    speed_value: usize,
    speed_count: usize,
    speed_m: f32,
    speed_b: f32,

    end_row: usize,
}

impl Rocket {
    pub fn new() -> Rocket {
        let mut rocket = Rocket {
            tail: Tail::new(
                '0',
                5,
                Point {
                    // TODO Adapt min/man ?
                    x: rand::thread_rng().gen_range(7, NB_COLS - 8),
                    y: NB_ROWS - 1,
                },
                vec![220, 222, 223, 248, 241],
            ),

            speed_value: 10,
            speed_count: 0,
            speed_m: 0.0, // Speed equation
            speed_b: 0.0,

            end_row: rand::thread_rng().gen_range(3, 7),
        };

        // Speed --
        let speed_min = rand::thread_rng().gen_range(70.0, 80.0);
        let speed_max = rand::thread_rng().gen_range(40.0, 50.0);
        // let speed_min = rand::thread_rng().gen_range(30.0, 35.0);
        // let speed_max = rand::thread_rng().gen_range(18.0, 20.0);
        rocket.speed_m = (speed_max - speed_min) / (NB_ROWS as f32 - 2.0);
        rocket.speed_b = speed_max - rocket.speed_m * 2.0;

        rocket
    }

    pub fn position(&self) -> Option<&Point> {
        self.tail.current_position()
    }
    pub fn value(&self) -> char {
        self.tail.value
    }
    pub fn done(&self) -> bool {
        self.tail.is_empty()
    }
    pub fn set_done(&mut self) {
        self.tail.clear();
    }

    pub fn run(&mut self) {
        if self.speed_count >= self.speed_value || self.speed_count == 0 {
            if let Some(current) = self.tail.current_position() {
                // Adapt speed --
                self.speed_value = (self.speed_m * current.y as f32 + self.speed_b) as usize;

                // Is done ? (will be 'done' when tail is empty)
                if current.y == self.end_row {
                    self.tail.pop();
                    self.speed_count = 1;
                } else {
                    let mut new_position = Point {
                        y: current.y - 1,
                        ..*current
                    };

                    // Up tail's value --
                    let m: f32 = (self.end_row as f32 - NB_ROWS as f32) / 10.0;
                    let b: f32 = self.end_row as f32 - m * 10.0;
                    self.tail.value =
                        char::from_digit(((current.y as f32 - b) / m) as u32, 10).unwrap_or('9');

                    // Set next position --
                    match rand::thread_rng().gen_range(0, 4) {
                        0 => new_position.plus_x(),
                        1 => new_position.minus_x(),
                        _ => {}
                    }

                    self.tail.push(new_position);
                    self.speed_count = 1;
                }
            }
        }
        self.speed_count += 1;
    }
}

impl Drawable for Rocket {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        self.tail.draw(frame);
    }
}
