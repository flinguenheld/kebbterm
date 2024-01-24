use crate::{
    frame::Drawable,
    geometry::{Point, Speed, NB_ROWS},
    tail::Tail,
};
use rand::Rng;

// ----------------------------------------------------------------------------
// ------------------------------------------------------------ GroundFlare ---
pub struct GroundFlare {
    flares: Vec<Flare>,
    chars: Vec<char>,

    count: u32,
    count_step: u32,

    total_count: u8,
    total_flares: u8,
    position_x: usize,
}

impl GroundFlare {
    pub fn new(characters: Vec<char>, position: usize) -> GroundFlare {
        let ground_flare = GroundFlare {
            flares: Vec::new(),
            chars: characters,

            count: 0,
            count_step: rand::thread_rng().gen_range(120, 160),
            position_x: position,

            total_count: 0,
            total_flares: rand::thread_rng().gen_range(25, 40),
        };

        ground_flare
    }
    pub fn check_value(&mut self, val: &char) -> bool {
        for f in self.flares.iter_mut() {
            if f.valid(val) == true {
                return true;
            }
        }
        return false;
    }

    pub fn position_x(&self) -> usize {
        self.position_x
    }

    pub fn is_done(&mut self) -> Option<Vec<char>> {
        if self.flares.is_empty() {
            dbg!(self.chars.len());
            Some(self.chars.clone())
            // Some(self.flares.iter().map(|b| b.tail.value).collect())
        } else {
            None
        }
    }

    pub fn run(&mut self) {
        // Launch flares one by one

        // Get back char from its tail
        self.flares.retain(|f| {
            if f.is_done() {
                self.chars.push(f.tail.value);
                false
            } else {
                true
            }
        });

        // Put char in tail
        // Force the first
        if (self.count == self.count_step
            && self.total_count < self.total_flares
            && !self.chars.is_empty())
            || self.count == 0
        {
            self.flares.push(Flare::new(
                self.position_x,
                self.chars
                    .remove(rand::thread_rng().gen_range(0, self.chars.len())),
            ));

            self.count = 1;
            self.total_count += 1;
        }
        self.count += 1;

        // --
        for f in &mut self.flares {
            f.run();
        }
    }
}

impl Drawable for GroundFlare {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for f in &self.flares {
            f.draw(frame);
        }
    }
}

// ----------------------------------------------------------------------------
// ------------------------------------------------------------------ Flare ---
struct Flare {
    position: usize,
    done: bool,
    tail: Tail,
    speed: Speed,

    max_row: usize,
}

impl Flare {
    fn new(new_position: usize, value: char) -> Flare {
        Flare {
            position: new_position,
            done: false,
            tail: Tail::new(
                value,
                5,
                Point {
                    x: new_position,
                    y: NB_ROWS - 2,
                },
                vec![207, 212, 219, 248, 241],
            ),

            max_row: rand::thread_rng().gen_range(NB_ROWS / 3, NB_ROWS / 2),

            speed: Speed::new(
                Point {
                    // Fast at the bottom
                    x: rand::thread_rng().gen_range(12, 20),
                    y: NB_ROWS - 1,
                },
                Point {
                    // Slow at the top
                    x: rand::thread_rng().gen_range(70, 85),
                    y: 10,
                },
            ),
        }
    }

    fn valid(&mut self, value: &char) -> bool {
        if self.tail.value == *value {
            self.speed.up_by_x((NB_ROWS - 3) as f32);
            self.done = true;
            self.tail.set_color(vec![76, 70, 72, 28, 241]);
            true
        } else {
            false
        }
    }

    fn is_done(&self) -> bool {
        self.tail.is_empty()
    }

    fn run(&mut self) {
        if self.speed.reached() {
            if let Some(current) = self.tail.current_position() {
                self.speed.up_by_x(current.y as f32);

                if current.y <= self.max_row || self.done == true {
                    self.tail.pop();
                } else {
                    let mut new_position = Point {
                        y: current.y - 1,
                        ..*current
                    };

                    // Set optional x --
                    match rand::thread_rng().gen_range(0, 6) {
                        0 => new_position.plus_x(),
                        1 => new_position.minus_x(),
                        _ => {}
                    }
                    self.tail.push(new_position);
                }
            }
        }
        self.speed.up_tick();
    }
}

impl Drawable for Flare {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[NB_ROWS - 1][self.position].value = 'F';

        self.tail.draw(frame);
    }
}
