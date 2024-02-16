use crate::{
    firework::shape_skeletons::*,
    firework::{Check, Run},
    geometry::{Point, Speed},
    mode::utils::*,
    render::frame::*,
};
use crossterm::style::{self};
use rand::Rng;

/// Shape is a kind of spark with a skeleton with two steps:
/// - Explosion (a succession of shapes).  
/// - Fade until its disapearance.  
///
/// It takes a group of letters and uses them several times to fill a
/// [shape skeleton](`crate::firework::shape_skeletons`).  
/// The user has to press all of them to complete the shape.
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
    pub fn new(explosion_center: Point, chars: Vec<char>, speed_option: usize) -> Shape {
        Shape {
            current_skeleton: Vec::new(),
            characters: chars,
            center: explosion_center,

            explosion_step: 0,
            colors: vec![
                237, 240, 245, 216, 217, 218, 219, 225, 224, 223, 222, 221, 220, 226, 227, 228,
                229, 230, 231, 240, 220, 214, 208, 202, 196,
            ],
            colors_checked: vec![34, 35, 40, 41, 46, 47, 48, 76, 77, 82, 83, 84, 118, 119],

            speed: Speed::new(
                Point {
                    // Fast on start
                    x: rand::thread_rng().gen_range(40, 55),
                    y: 20,
                },
                Point {
                    // Slow at the end
                    x: rand::thread_rng().gen_range(180 + speed_option, 190 + speed_option),
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

    /// Shape's center.
    pub fn position(&self) -> Point {
        self.center
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

        ok
    }
}

impl Run for Shape {
    fn is_done(&self) -> Option<(Vec<char>, u16)> {
        if self.colors.is_empty() {
            let misses = self.current_skeleton.len()
                - self
                    .current_skeleton
                    .iter()
                    .filter(|(_, _, done)| *done)
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
                s if s < 5 => {
                    self.upload_skeleton(self.explosion_step);
                }
                5 => {
                    self.upload_skeleton(rand::thread_rng().gen_range(5, 12));
                }
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
