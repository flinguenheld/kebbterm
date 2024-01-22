use crate::{frame::Drawable, tail::Tail, Point};
use rand::Rng;

// #[derive(Copy, Clone)]
struct Branch {
    trajectory: u8,
    tail: Tail,
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
    const MAX_MOVES: u8 = 15;

    pub fn new(center: Point, chars: Vec<char>) -> Spark {
        let mut spark = Spark {
            branches: Vec::new(),

            speed_value: 80,
            speed_count: 0,

            nb_moves: 0,
        };

        // Create branches
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
                trajectory: traj,
                tail: Tail::new(*c, 3, center),
            });
        }
        spark
    }

    pub fn check_value(&mut self, val: &char) -> Option<char> {
        // TODO: Don't remove it but set a 'done' in the tail
        if let Some(index) = self.branches.iter().position(|b| b.tail.value == *val) {
            Some(self.branches.remove(index).tail.value)
        } else {
            None
        }
    }

    pub fn is_done(&mut self) -> Option<Vec<char>> {
        if self.nb_moves >= Spark::MAX_MOVES {
            let chars = Some(self.branches.iter().map(|b| b.tail.value).collect());
            self.branches.clear();
            chars
        } else {
            None
        }
    }

    pub fn run(&mut self) {
        if self.speed_count == self.speed_value || self.speed_count == 0 {
            for branch in self.branches.iter_mut() {
                if let Some(current) = branch.tail.current_position() {
                    let mut new_position = Point { ..*current };

                    match branch.trajectory {
                        1 => new_position.plus_x(),
                        2 => {
                            new_position.plus_x();
                            new_position.plus_y();
                        }
                        3 => new_position.plus_y(),
                        4 => {
                            new_position.minus_x();
                            new_position.plus_y();
                        }
                        5 => new_position.minus_x(),
                        6 => {
                            new_position.minus_x();
                            new_position.minus_y();
                        }
                        7 => new_position.minus_y(),
                        8 => {
                            new_position.plus_x();
                            new_position.minus_y();
                        }
                        _ => {}
                    }

                    // Up the tail --
                    branch.tail.push(new_position);
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
            branch.tail.draw(frame);
        }
    }
}
