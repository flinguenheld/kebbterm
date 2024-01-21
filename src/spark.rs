use crate::{frame::Drawable, Point};
use rand::Rng;

const MAX_MOVES: u8 = 6;

#[derive(Copy, Clone)]
struct Branch {
    value: char,
    trajectory: u8,
    position: Point,
}

/* Spark contains a group of Branches (9 maxi).
 * It allows to move and display them.
 */
pub struct Spark {
    branches: Vec<Branch>,
    speed_value: usize,
    speed_count: usize,

    nb_moves: u8,
}

impl Spark {
    pub fn new(new_position: Point, chars: Vec<char>) -> Spark {
        let mut spark = Spark {
            branches: Vec::new(),

            speed_value: 10,
            speed_count: 0,

            nb_moves: 0,
        };

        // Branches
        let mut buffer: Vec<u8> = (1..8).collect();
        for c in chars.iter() {
            // Use the center only with 9 branches
            let traj = {
                if buffer.is_empty() {
                    0
                } else {
                    buffer.remove(rand::thread_rng().gen_range(0, buffer.len()))
                }
            };

            spark.branches.push(Branch {
                value: *c,
                trajectory: traj,
                position: new_position,
            });
        }
        spark
    }

    pub fn check_value(&mut self, val: &char) -> Option<char> {
        if let Some(index) = self.branches.iter().position(|b| b.value == *val) {
            Some(self.branches.remove(index).value)
        } else {
            None
        }
    }

    pub fn is_done(&mut self) -> Option<Vec<char>> {
        if self.nb_moves >= MAX_MOVES {
            let chars = Some(self.branches.iter().map(|b| b.value).collect());
            self.branches.clear();
            chars
        } else {
            None
        }
    }

    pub fn run(&mut self) {
        if self.speed_count == self.speed_value || self.speed_count == 0 {
            for branch in self.branches.iter_mut() {
                match branch.trajectory {
                    1 => branch.position.plus_x(),
                    2 => {
                        branch.position.plus_x();
                        branch.position.plus_y();
                    }
                    3 => branch.position.plus_y(),
                    4 => {
                        branch.position.minus_x();
                        branch.position.plus_y();
                    }
                    5 => branch.position.minus_x(),
                    6 => {
                        branch.position.minus_x();
                        branch.position.minus_y();
                    }
                    7 => branch.position.minus_y(),
                    8 => {
                        branch.position.plus_x();
                        branch.position.minus_y();
                    }
                    _ => {}
                }
            }
            self.nb_moves += 1;
            self.speed_count = 1;
        }
        self.speed_count += 1;
    }
}

impl Drawable for Spark {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for branch in self.branches.iter() {
            frame[branch.position.y][branch.position.x] = branch.value;
        }
    }
}
