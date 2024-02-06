use crate::{
    firework::tail::Tail,
    firework::{Check, Run},
    geometry::{Point, Speed, NB_ROWS},
    mode::utils::*,
    render::frame::*,
};
use rand::Rng;

// ----------------------------------------------------------------------------
// ------------------------------------------------------------ GroundFlare ---
/*
 * From a given stock of chars, this struct launches a randomised amount of Flares.
 * All flares are started regulary and each char is recovered at
 * the flare remove to be reused.
 * Once the amount is reached, use the function 'is_done' to get the chars back.
 */
pub struct GroundFlare {
    flares: Vec<Flare>,
    chars: Vec<char>,

    loop_counter: u32,
    loop_counter_step: u32,

    flare_counter: u16,
    flare_nb_max: u16,
    position: Point,
    nb_success: u16,
}

impl GroundFlare {
    pub fn new(position: Point, characters: Vec<char>) -> GroundFlare {
        let ground_flare = GroundFlare {
            flares: Vec::new(),
            chars: characters,

            loop_counter: 0,
            loop_counter_step: rand::thread_rng().gen_range(100, 150),
            position,

            flare_counter: 0,
            flare_nb_max: rand::thread_rng().gen_range(25, 40),

            nb_success: 0,
        };

        ground_flare
    }

    pub fn position(&self) -> Point {
        self.position
    }
}

impl Check for GroundFlare {
    fn check_value(&mut self, val: &char) -> bool {
        for f in self.flares.iter_mut().filter(|f| !f.done) {
            if f.check_value(val) == true {
                self.nb_success += 1;
                return true;
            }
        }
        return false;
    }
}

impl Run for GroundFlare {
    fn is_done(&self) -> Option<(Vec<char>, u16)> {
        if self.flares.is_empty() {
            Some((self.chars.clone(), self.flare_counter - self.nb_success))
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

        // Start a new tail.
        if (self.loop_counter == self.loop_counter_step
            && self.flare_counter < self.flare_nb_max
            && !self.chars.is_empty())
            || self.flare_counter == 0
        {
            self.flares.push(Flare::new(
                self.position,
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
    tail: Tail,
    speed: Speed,

    done: bool,
    max_row: usize,
}

impl Flare {
    fn new(position: Point, value: char) -> Flare {
        Flare {
            tail: Tail::new(value, position, vec![147, 141, 135, 129, 91]),

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

            done: false,
            max_row: rand::thread_rng()
                .gen_range(NB_ROWS / 2 + NB_ROWS / 10, NB_ROWS / 2 + NB_ROWS / 5),
        }
    }

    fn check_value(&mut self, value: &char) -> bool {
        if self.tail.value == *value {
            self.speed.up_by_x((NB_ROWS - 3) as f32);
            self.done = true;
            self.tail.set_color(vec![118, 82, 76, 70, 34]);
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
