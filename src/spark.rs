use crate::geometry::{Point, Speed};
use crate::{frame::Drawable, tail::Tail};
use rand::Rng;

// #[derive(Copy, Clone)]
struct Branch {
    trajectory: u8,
    tail: Tail,
    is_done: bool,
}

/*
 * Spark contains a group of Branches (9 maxi).
 * It allows to move and display them.
 */
pub struct Spark {
    branches: Vec<Branch>,
    speed: Speed,

    max_moves: u8,
    nb_moves: u8,
}

impl Spark {
    pub fn new(center: Point, chars: Vec<char>) -> Spark {
        let mut spark = Spark {
            branches: Vec::new(),

            max_moves: 8, // Adapt with speed
            nb_moves: 0,

            speed: Speed::new(
                Point {
                    // Fast on start
                    x: rand::thread_rng().gen_range(10, 15),
                    y: 2,
                },
                Point {
                    // Slow at the end
                    x: rand::thread_rng().gen_range(110, 130),
                    y: 8,
                },
            ),
        };

        // Create branches
        let mut buffer: Vec<u8> = (1..9).collect();
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
                // tail: Tail::new(*c, 3, center, vec![202, 208, 242]),
                tail: Tail::new(*c, 3, center, vec![208, 202, 242]),
                is_done: false,
            });
        }
        spark
    }

    pub fn check_value(&mut self, val: &char) -> bool {
        // TODO: Don't remove it but set a 'done' in the tail

        self.branches
            .iter_mut()
            .filter(|b| b.tail.value == *val)
            .map(|b| {
                b.is_done = true;
                b.tail.set_color(vec![76, 70, 28]);
            })
            .count()
            == 1
    }

    /*
     * Check if all tails are done
     */
    pub fn is_done(&mut self) -> Option<Vec<char>> {
        if self
            .branches
            .iter()
            .all(|b| b.is_done == true && b.tail.is_empty())
        {
            Some(self.branches.iter().map(|b| b.tail.value).collect())
        } else {
            None
        }
    }

    pub fn run(&mut self) {
        if self.speed.reached() {
            self.speed.up_by_x(self.nb_moves as f32);

            for branch in self.branches.iter_mut() {
                // End of time --
                if self.nb_moves >= self.max_moves && branch.is_done == false {
                    branch.is_done = true;
                    // TODO up colours ?
                }

                if branch.is_done == true {
                    branch.tail.pop();
                } else {
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

                        branch.tail.push(new_position);
                    }
                }
            }

            self.nb_moves += 1;
        }
        self.speed.up_tick();
    }
}

impl Drawable for Spark {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for branch in self.branches.iter() {
            branch.tail.draw(frame);
        }
    }
}
