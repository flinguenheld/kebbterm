use super::{shape_skeletons::skeleton, Check, Run};
use crate::{geometry::*, render::frame::*};
use crossterm::style::{self};
use rand::Rng;

pub struct Shape {
    current_skeleton: Vec<(Point, char, bool)>,
    characters: Vec<char>,
    center: Point,

    explosion_step: usize, // Max steps -> colors length

    colors: Vec<u8>,
    colors_checked: Vec<u8>,

    speed: Speed,
}

impl Shape {
    pub fn new(explosion_center: Point, chars: Vec<char>) -> Shape {
        Shape {
            current_skeleton: Vec::new(),
            characters: chars,
            center: explosion_center,

            explosion_step: 0,
            colors: vec![
                124, 160, 196, 197, 202, 203, 204, 208, 209, 210, 214, 215, 216, 220, 221, 222,
                223, 226, 227, 228, 229, 230, 231,
            ],
            colors_checked: vec![34, 35, 40, 41, 46, 47, 48, 76, 77, 82, 83, 84, 118, 119],

            speed: Speed::new(
                Point {
                    // Fast on start
                    x: rand::thread_rng().gen_range(10, 20),
                    y: 20,
                },
                Point {
                    // Slow at the end
                    x: rand::thread_rng().gen_range(170, 180),
                    y: 2,
                },
            ),
        }
    }

    fn upload_skeleton(&mut self, skeleton_index: usize) {
        let mut skeleton = skeleton(skeleton_index);

        let center_shape = skeleton.pop().unwrap();
        let (plus_x, plus_y) = (
            center_shape.x - self.center.x,
            center_shape.y - self.center.y,
        );

        self.current_skeleton = skeleton
            .iter()
            .map(|skeleton| {
                (
                    Point {
                        x: skeleton.x - plus_x,
                        y: skeleton.y - plus_y,
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
        let mut ok = false;

        if let Some((_, _, done)) = self
            .current_skeleton
            .iter_mut()
            .find(|(_, value, is_done)| value == val && !is_done)
        {
            *done = true;
            ok = true;
        }

        // If all done, shorten shape's life
        if self.current_skeleton.iter_mut().all(|(_, _, done)| *done) {
            while self.colors.len() > 2 {
                self.colors.pop();
            }
            while self.colors_checked.len() > 2 {
                self.colors_checked.pop();
            }
        }

        return ok;
    }
}

impl Run for Shape {
    fn is_done(&self) -> Option<(Vec<char>, u16)> {
        if self.colors.is_empty() {
            let misses = self.current_skeleton.iter().count()
                - self
                    .current_skeleton
                    .iter()
                    .filter(|(_, _, done)| *done == true)
                    .count();
            Some((self.characters.clone(), misses as u16))
        } else {
            None
        }
    }

    fn run(&mut self) {
        if self.speed.reached() {
            self.speed.up_by_x(self.colors.len() as f32);

            match self.explosion_step {
                s if s < 4 => {
                    self.upload_skeleton(self.explosion_step as usize);
                    // self.explosion_step += 1;
                }
                4 => {
                    self.upload_skeleton(rand::thread_rng().gen_range(4, 9));
                    // self.explosion_step = 255;
                }
                // s if s % 2 == 0 => {
                //     self.current_skeleton
                //         .iter_mut()
                //         // .filter(|(pt, _, _)| pt.x % 2 == 0)
                //         .for_each(|(pt, _, _)| {
                //             pt.plus_y();
                //             pt.plus_x()
                //         });
                // }
                _ => {}
            }
            self.explosion_step += 1;

            // --
            self.colors.pop();
            if self.colors_checked.len() > self.colors.len() {
                self.colors_checked.pop();
            }
        }
        self.speed.up_tick();
    }
}

impl Drawable for Shape {
    fn draw(&self, frame: &mut Frame) {
        for &(pt, value, done) in self.current_skeleton.iter() {
            frame[pt.y][pt.x].value = value;
            if !done {
                frame[pt.y][pt.x].fore_color =
                    style::Color::AnsiValue(*self.colors.last().unwrap_or(&88));
            } else {
                frame[pt.y][pt.x].fore_color =
                    style::Color::AnsiValue(*self.colors_checked.last().unwrap_or(&34));
            }
        }
    }
}
