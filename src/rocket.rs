use crate::frame::Drawable;
use crate::tail::Tail;
use crate::{Point, NB_COLS, NB_ROWS};
use rand::Rng;

/*
 * See the yellow !
 */
pub struct Rocket {
    tail: Tail,

    speed_value: usize,
    speed_count: usize,
    speed_m: f32,
    speed_b: f32,

    explosion_row: usize,
}

impl Rocket {
    pub fn new() -> Rocket {
        let mut rocket = Rocket {
            tail: Tail::new(
                '∆',
                5,
                Point {
                    x: rand::thread_rng().gen_range(10, NB_COLS - 10),
                    y: NB_ROWS - 1,
                },
                vec![220, 222, 223, 248, 241],
            ),

            speed_value: 10,
            speed_count: 0,
            speed_m: 0.0, // Speed equation
            speed_b: 0.0,

            explosion_row: rand::thread_rng().gen_range(8, NB_ROWS - 15),
        };

        // Speed --
        let speed_min = rand::thread_rng().gen_range(70.0, 85.0);
        let speed_max = rand::thread_rng().gen_range(35.0, 50.0);
        rocket.speed_m = (speed_max - speed_min) / (NB_ROWS as f32 - 2.0);
        rocket.speed_b = speed_max - rocket.speed_m * 2.0;

        rocket
    }

    pub fn position(&self) -> Option<&Point> {
        self.tail.current_position()
    }

    pub fn exploded(&self) -> bool {
        if let Some(position) = self.tail.current_position() {
            position.y <= self.explosion_row
        } else {
            false
        }
    }

    pub fn run(&mut self) {
        if self.speed_count >= self.speed_value || self.speed_count == 0 {
            if let Some(current) = self.tail.current_position() {
                // Adapt speed --
                self.speed_value = (self.speed_m * current.y as f32 + self.speed_b) as usize;

                let mut new_position = Point {
                    y: current.y - 1,
                    ..*current
                };

                // Set optional x --
                match rand::thread_rng().gen_range(0, 4) {
                    0 => new_position.plus_x(),
                    1 => new_position.minus_x(),
                    _ => {}
                }

                self.tail.push(new_position);
                self.speed_count = 1;
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
