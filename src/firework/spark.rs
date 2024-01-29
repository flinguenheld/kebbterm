use crate::{
    firework::tail::Tail,
    firework::{Check, Run},
    geometry::{Point, Speed},
    render::frame::Drawable,
    render::frame::Frame,
};
use rand::Rng;

struct Branch {
    trajectory: u8,
    tail: Tail,
    is_done: bool,
}

/*
 * Spark contains a group of Branches (9 maxi).
 * It allows you to move and display them.
 * A bunch of chars have to be done in the constructor.
 * Use the trait 'is_done' to get them back when all branches are done.
 * Lifetime is counted by 'max_move', moves are counted by 'speed'.
 */
pub struct Spark {
    branches: Vec<Branch>,
    speed: Speed,

    max_moves: u16,
    nb_moves: u16,

    nb_success: u16,
}

impl Spark {
    pub fn new(center: Point, chars: Vec<char>) -> Spark {
        let mut spark = Spark {
            branches: Vec::new(),

            max_moves: 8, // Adapt with speed calculation
            nb_moves: 0,

            nb_success: 0,

            speed: Speed::new(
                Point {
                    // Fast on start
                    x: rand::thread_rng().gen_range(2, 8),
                    y: 2,
                },
                Point {
                    // Slow at the end
                    x: rand::thread_rng().gen_range(70, 90),
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
                // tail: Tail::new(*c, 4, center, vec![160, 196, 202, 208]),
                tail: Tail::new(*c, 4, center, vec![208, 202, 196, 160]),
                is_done: false,
            });
        }
        spark
    }
}

impl Check for Spark {
    fn check_value(&mut self, val: &char) -> bool {
        if self
            .branches
            .iter_mut()
            .filter(|b| b.tail.value == *val)
            .map(|b| {
                b.is_done = true;
                b.tail.set_color(vec![82, 76, 70, 34]);
            })
            .count()
            == 1
        {
            self.nb_success += 1;
            true
        } else {
            false
        }
    }
}

impl Run for Spark {
    fn is_done(&self) -> Option<(Vec<char>, u16)> {
        if self
            .branches
            .iter()
            .all(|b| b.is_done == true && b.tail.is_empty())
        {
            let chars: Vec<char> = self.branches.iter().map(|b| b.tail.value).collect();
            let misses = chars.len() as u16 - self.nb_success;
            Some((chars, misses))
        } else {
            None
        }
    }

    fn run(&mut self) {
        if self.speed.reached() {
            self.speed.up_by_x(self.nb_moves as f32);

            for branch in self.branches.iter_mut() {
                // Too late for the user --
                if self.nb_moves >= self.max_moves && branch.is_done == false {
                    branch.is_done = true;
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
    fn draw(&self, frame: &mut Frame) {
        for branch in self.branches.iter() {
            branch.tail.draw(frame);
        }
    }
}
