use crate::{
    firework::tail::Tail,
    firework::Run,
    geometry::{Point, Speed, NB_ROWS},
    render::frame::{Drawable, Frame},
};
use rand::Rng;

// ----------------------------------------------------------------------------
// ------------------------------------------------------------ GroundFlare ---
/*
 * From a given stock of chars, this struct launches a randomised amount of Flares.
 * Each flares are started regulary and each char is recovered at the flare remove.
 * Once the amount is reached, use the function 'is_done' to get the chars back.
 */
pub struct GroundFlare {
    flares: Vec<Flare>,
    chars: Vec<char>,

    loop_counter: u32,
    loop_counter_step: u32,

    flare_counter: u8,
    flare_total: u8,
    position_x: usize,
}

impl GroundFlare {
    pub fn new(characters: Vec<char>, position: usize) -> GroundFlare {
        let ground_flare = GroundFlare {
            flares: Vec::new(),
            chars: characters,

            loop_counter: 0,
            loop_counter_step: rand::thread_rng().gen_range(100, 150),
            position_x: position,

            flare_counter: 0,
            flare_total: rand::thread_rng().gen_range(25, 40),
        };

        ground_flare
    }

    pub fn check_value(&mut self, val: &char) -> bool {
        for f in self.flares.iter_mut() {
            if f.check_value(val) == true {
                return true;
            }
        }
        return false;
    }

    // Useful in main to take GroundFlares away.
    pub fn position_x(&self) -> usize {
        self.position_x
    }
}

impl Run for GroundFlare {
    fn is_done(&self) -> Option<Vec<char>> {
        if self.flares.is_empty() {
            Some(self.chars.clone())
        } else {
            None
        }
    }

    fn run(&mut self) {
        // Remove ended tails and get their char back.
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
        if (self.loop_counter == self.loop_counter_step
            && self.flare_counter < self.flare_total
            && !self.chars.is_empty())
            || self.flare_counter == 0
        {
            self.flares.push(Flare::new(
                self.position_x,
                self.chars
                    .remove(rand::thread_rng().gen_range(0, self.chars.len())),
            ));

            self.loop_counter = 1;
            self.flare_counter += 1;
        }
        self.loop_counter += 1;

        // --
        for f in &mut self.flares {
            f.run();
        }
    }
}

impl Drawable for GroundFlare {
    fn draw(&self, frame: &mut Frame) {
        // frame[NB_ROWS - 1][self.position_x].value = '🭩';
        // frame[NB_ROWS - 1][self.position_x].fore_color = style::Color::AnsiValue(240);

        for f in &self.flares {
            f.draw(frame);
        }
    }
}

// ----------------------------------------------------------------------------
// ------------------------------------------------------------------ Flare ---
/*
 * A Flare is a tail with specifics parameters (speed, direction ...).
 * Only useful with GroundFlare.
 */
struct Flare {
    done: bool,
    tail: Tail,
    speed: Speed,

    max_row: usize,
}

impl Flare {
    fn new(new_position: usize, value: char) -> Flare {
        Flare {
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

            max_row: rand::thread_rng()
                .gen_range(NB_ROWS / 2 + NB_ROWS / 10, NB_ROWS / 2 + NB_ROWS / 5),

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

    fn check_value(&mut self, value: &char) -> bool {
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
    fn draw(&self, frame: &mut Frame) {
        self.tail.draw(frame);
    }
}
