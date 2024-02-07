use crate::{
    firework::tail::Tail,
    firework::Run,
    geometry::{Point, Speed, NB_ROWS},
    mode::utils::*,
    render::frame::*,
};
use rand::Rng;

/*
 * See the yellow !
 * Rocket is a tail which aims to explode at a randomised row.
 * The explosion and the spark creation are managed by main.
 */
pub struct Rocket {
    tail: Tail,
    speed: Speed,
    explosion_row: usize,
}

impl Rocket {
    pub fn new(symbol: char, colors: Vec<u8>, position: Point, speed_option: usize) -> Rocket {
        let rocket = Rocket {
            tail: Tail::new(symbol, position, colors),
            speed: Speed::new(
                Point {
                    // Fast at the bottom
                    x: rand::thread_rng().gen_range(4, 8),
                    y: NB_ROWS - 1,
                },
                Point {
                    // Slow at the top
                    x: rand::thread_rng()
                        .gen_range(40 + (speed_option / 10), 50 + (speed_option / 10)),
                    y: 2,
                },
            ),
            explosion_row: rand::thread_rng().gen_range(8, NB_ROWS / 2 + 2),
        };

        rocket
    }

    pub fn symbol(&self) -> char {
        self.tail.value
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
}

impl Run for Rocket {
    fn run(&mut self) {
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
    fn draw(&self, frame: &mut Frame) {
        self.tail.draw(frame);
    }
}
