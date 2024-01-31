use crossterm::style::{self};
use rand::Rng;

use crate::{
    geometry::{Point, Speed},
    render::frame::{Drawable, Frame},
};

use super::{Check, Run};

pub struct Shape {
    curent_skeleton: Vec<(Point, char, bool)>,

    characters: Vec<char>,
    center: Point,

    current_step: u16, // Max steps = colors.len() <= NO !
    max_step: u16,
    explosion_done: bool,

    speed: Speed,
    colors: Vec<u8>,
}

impl Shape {
    pub fn new(explosion_center: Point, chars: Vec<char>) -> Shape {
        Shape {
            curent_skeleton: Vec::new(),
            characters: chars,
            center: explosion_center,

            current_step: 0,
            max_step: 20,

            explosion_done: false,

            colors: vec![87, 51, 50, 45, 39, 33, 32, 31, 30, 24, 240, 239, 238],

            speed: Speed::new(
                Point {
                    // Fast on start
                    x: rand::thread_rng().gen_range(20, 35),
                    y: 2,
                },
                Point {
                    // Slow at the end
                    x: rand::thread_rng().gen_range(70, 90),
                    y: 8,
                },
            ),
        }
    }

    fn upload_skeleton(&mut self, skeleton_index: u16) {
        let mut skeleton = skeleton(skeleton_index);

        let center_shape = skeleton.pop().unwrap();
        let (plus_x, plus_y) = (
            center_shape.x - self.center.x,
            center_shape.y - self.center.y,
        );

        self.curent_skeleton = skeleton
            .iter()
            .map(|s| {
                (
                    Point {
                        x: s.x - plus_x,
                        y: s.y - plus_y,
                    },
                    self.characters[rand::thread_rng().gen_range(0, self.characters.len())],
                    false,
                )
            })
            .collect();
    }
}

impl Check for Shape {
    fn check_value(&mut self, val: &char) -> bool {
        if let Some((_, _, done)) = self
            .curent_skeleton
            .iter_mut()
            .find(|s| s.1 == *val && !s.2)
        {
            *done = true;
            true
        } else {
            false
        }
    }
}

impl Run for Shape {
    fn is_done(&self) -> Option<(Vec<char>, u16)> {
        if self.explosion_done && self.curent_skeleton.iter().all(|&(_, _, done)| done) {
            Some((self.characters.clone(), 0))
        } else if self.current_step >= self.max_step {
            let misses = self.curent_skeleton.iter().count()
                - self
                    .curent_skeleton
                    .iter()
                    .filter(|(_, _, done)| *done == true)
                    .count();
            Some((self.characters.clone(), misses as u16))
        } else {
            None
        }
    }

    fn run(&mut self) {
        if self.current_step <= self.max_step {
            if self.speed.reached() {
                self.speed.up_by_x(self.current_step as f32);

                match self.current_step {
                    s if s < 3 => {
                        self.upload_skeleton(s);
                    }
                    3 => {
                        self.upload_skeleton(rand::thread_rng().gen_range(3, 4));
                        self.explosion_done = true;
                    }
                    4 | 5 => {}
                    _ => {
                        self.curent_skeleton.iter_mut().for_each(|s| s.0.x += 1);
                    }
                }

                self.current_step += 1;
            }
            self.speed.up_tick();
        }
    }
}

impl Drawable for Shape {
    fn draw(&self, frame: &mut Frame) {
        for &(pt, value, done) in self.curent_skeleton.iter() {
            frame[pt.y][pt.x].value = value;
            if !done {
                frame[pt.y][pt.x].fore_color = style::Color::AnsiValue(
                    *self.colors.get(self.current_step as usize).unwrap_or(&52),
                );
            } else {
                frame[pt.y][pt.x].fore_color = style::Color::AnsiValue(76); // TODO: up colors
            }
        }
    }
}

fn skeleton(nb: u16) -> Vec<Point> {
    match nb {
        0 => vec![
            // ###
            // #.#
            // ###
            Point { x: 1_000, y: 1_000 },
            Point { x: 1_001, y: 1_000 },
            Point { x: 1_002, y: 1_000 },
            Point { x: 1_002, y: 1_001 },
            Point { x: 1_002, y: 1_002 },
            Point { x: 1_001, y: 1_002 },
            Point { x: 1_000, y: 1_002 },
            Point { x: 1_000, y: 1_001 },
            // --
            Point { x: 1_001, y: 1_001 },
        ],
        1 => vec![
            //  ###
            // #   #
            // # . #
            // #   #
            //  ###
            Point { x: 1_001, y: 1_000 },
            Point { x: 1_002, y: 1_000 },
            Point { x: 1_003, y: 1_000 },
            // --
            Point { x: 1_001, y: 1_004 },
            Point { x: 1_002, y: 1_004 },
            Point { x: 1_003, y: 1_004 },
            // --
            Point { x: 1_000, y: 1_001 },
            Point { x: 1_000, y: 1_002 },
            Point { x: 1_000, y: 1_003 },
            // --
            Point { x: 1_004, y: 1_001 },
            Point { x: 1_004, y: 1_002 },
            Point { x: 1_004, y: 1_003 },
            // --
            Point { x: 1_002, y: 1_002 },
        ],
        2 => vec![
            // TODO: Bof bof bof
            //    #
            //   # #
            //  #   #
            // #  .  #
            //  #   #
            //   # #
            //    #
            Point { x: 1_003, y: 1_000 },
            Point { x: 1_002, y: 1_001 },
            Point { x: 1_004, y: 1_001 },
            Point { x: 1_001, y: 1_002 },
            Point { x: 1_005, y: 1_002 },
            // --
            Point { x: 1_000, y: 1_003 },
            Point { x: 1_006, y: 1_003 },
            // --
            Point { x: 1_001, y: 1_004 },
            Point { x: 1_005, y: 1_004 },
            Point { x: 1_002, y: 1_005 },
            Point { x: 1_004, y: 1_005 },
            Point { x: 1_003, y: 1_006 },
            // --
            Point { x: 1_003, y: 1_003 },
        ],

        3 => vec![
            // #######
            // #     #
            // ###.###
            // #     #
            // #     #
            Point { x: 1_000, y: 1_000 },
            Point { x: 1_000, y: 1_001 },
            Point { x: 1_000, y: 1_002 },
            Point { x: 1_000, y: 1_003 },
            Point { x: 1_000, y: 1_004 },
            // --
            Point { x: 1_006, y: 1_000 },
            Point { x: 1_006, y: 1_001 },
            Point { x: 1_006, y: 1_002 },
            Point { x: 1_006, y: 1_003 },
            Point { x: 1_006, y: 1_004 },
            // --
            Point { x: 1_001, y: 1_000 },
            Point { x: 1_002, y: 1_000 },
            Point { x: 1_003, y: 1_000 },
            Point { x: 1_004, y: 1_000 },
            Point { x: 1_005, y: 1_000 },
            // --
            Point { x: 1_001, y: 1_002 },
            Point { x: 1_002, y: 1_002 },
            Point { x: 1_003, y: 1_002 },
            Point { x: 1_004, y: 1_002 },
            Point { x: 1_005, y: 1_002 },
            // --
            Point { x: 1_003, y: 1_003 },
        ],

        _ => vec![Point { x: 3, y: 3 }],
    }
}
