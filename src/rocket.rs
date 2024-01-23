use crate::frame::Drawable;
use crate::speed::Speed;
use crate::tail::Tail;
use crate::{Point, NB_COLS, NB_ROWS};
use rand::Rng;

/*
 * See the yellow !
 */
pub struct Rocket {
    tail: Tail,
    speed: Speed,

    explosion_row: usize,
}

impl Rocket {
    pub fn new() -> Rocket {
        let rocket = Rocket {
            tail: Tail::new(
                '∆',
                5,
                Point {
                    x: rand::thread_rng().gen_range(10, NB_COLS - 10),
                    y: NB_ROWS - 1,
                },
                vec![220, 222, 223, 248, 241],
            ),

            speed: Speed::new(
                Point {
                    // Fast at the bottom
                    x: rand::thread_rng().gen_range(12, 20),
                    y: NB_ROWS - 1,
                },
                Point {
                    // Slow at the top
                    x: rand::thread_rng().gen_range(70, 85),
                    y: 2,
                },
            ),

            explosion_row: rand::thread_rng().gen_range(8, NB_ROWS - 15),
        };

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
        if self.speed.reached() {
            if let Some(current) = self.tail.current_position() {
                // Adapt speed --
                self.speed.up_by_x(current.y as f32);

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
            }
        }
        self.speed.up_tick();
    }
}

impl Drawable for Rocket {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        self.tail.draw(frame);
    }
}
