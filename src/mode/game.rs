use crate::{
    firework::{flare::GroundFlare, rocket::Rocket, shape::Shape, spark::Spark, Check, Run},
    geometry::{Point, NB_COLS, NB_ROWS},
    mode::counter::Counters,
    mode::Mode,
    render::frame::{Drawable, Frame},
};
use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::{collections::HashSet, io, time::Duration};

pub struct ModeGame {
    rockets: Vec<Rocket>,

    sparks: Vec<Spark>,
    ground_flares: Vec<GroundFlare>,
    shapes: Vec<Shape>,

    chars: Vec<char>,
}

impl ModeGame {
    pub fn new() -> ModeGame {
        ModeGame {
            rockets: Vec::new(),

            sparks: Vec::new(),
            ground_flares: Vec::new(),
            shapes: Vec::new(),

            chars: "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ^$[]|&~€!{}%~#?@()*_-:;<>+-=`\\/\"'".chars().collect(),
        }
    }

    pub fn mode_loop(
        &mut self,
        frame: &mut Frame,
        mode: &mut Mode,
        counters: &mut Counters,
    ) -> io::Result<()> {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => {
                        counters.elapsed_time += counters.start_time.elapsed().as_secs();
                        *mode = Mode::Score;
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        self.rockets.push(Rocket::new(
                            '∆',
                            vec![220, 222, 223, 248, 241],
                            find_free_position(vec![
                                self.rockets
                                    .iter()
                                    .map(|r| r.position().unwrap_or(&Point { x: 10, y: 10 }).x)
                                    .collect(),
                                // self.sparks.iter().map(|r| r.position().x).collect(),
                                self.shapes.iter().map(|r| r.position().x).collect(),
                            ]),
                        ));
                    }
                    KeyCode::Char(' ') => {
                        if let Some(selected_chars) = take_chars(&mut self.chars, 10) {
                            self.ground_flares.push(GroundFlare::new(
                                find_free_position(vec![self
                                    .ground_flares
                                    .iter()
                                    .map(|r| r.position().x)
                                    .collect()]),
                                selected_chars,
                            ));
                            counters.groundflares += 1;
                        }
                    }
                    KeyCode::Tab => {
                        self.rockets.push(Rocket::new(
                            '⍙',
                            vec![51, 50, 49, 248, 241],
                            find_free_position(vec![
                                self.rockets
                                    .iter()
                                    .map(|r| r.position().unwrap_or(&Point { x: 10, y: 10 }).x)
                                    .collect(),
                                // self.sparks.iter().map(|r| r.position().x).collect(),
                                self.shapes.iter().map(|r| r.position().x).collect(),
                            ]),
                        ));
                    }

                    KeyCode::Char(val) => {
                        if !check_value(&mut self.sparks, &val, &mut counters.success)
                            && !check_value(&mut self.ground_flares, &val, &mut counters.success)
                            && !check_value(&mut self.shapes, &val, &mut counters.success)
                        {
                            counters.fails += 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Rockets (for shapes & sparks)--
        run_draw(&mut self.rockets, frame);
        self.rockets.retain_mut(|rocket| {
            if rocket.exploded() {
                match rocket.symbol() {
                    '∆' => {
                        if let Some(selected) =
                            take_chars(&mut self.chars, rand::thread_rng().gen_range(3, 10))
                        {
                            self.sparks
                                .push(Spark::new(*rocket.position().unwrap(), selected));
                            counters.sparks += 1;
                        };
                    }
                    _ => {
                        if let Some(selected) =
                            take_chars(&mut self.chars, rand::thread_rng().gen_range(1, 4))
                        {
                            self.shapes
                                .push(Shape::new(*rocket.position().unwrap(), selected));
                            counters.shapes += 1;
                        };
                    }
                }
                false
            } else {
                true
            }
        });

        // Shapes --
        get_char_back(&mut self.chars, &mut self.shapes, &mut counters.misses);
        run_draw(&mut self.shapes, frame);

        // Sparks --
        get_char_back(&mut self.chars, &mut self.sparks, &mut counters.misses);
        run_draw(&mut self.sparks, frame);

        // Flares --
        run_draw(&mut self.ground_flares, frame);
        get_char_back(
            &mut self.chars,
            &mut self.ground_flares,
            &mut counters.misses,
        );

        Ok(())
    }
}

// ----------------------------------------------------------------------------
// ------------------------------------------------------------------ Check ---
fn check_value(element: &mut Vec<impl Check>, val: &char, counter: &mut u16) -> bool {
    for e in element.iter_mut() {
        if e.check_value(&val) {
            *counter += 1;
            return true;
        }
    }
    return false;
}

// ----------------------------------------------------------------------------
// --------------------------------------------------------- Run & Drawable ---
fn run_draw(elements: &mut Vec<impl Run + Drawable>, frame: &mut Frame) {
    elements.iter_mut().for_each(|f| {
        f.run();
        f.draw(frame)
    });
}

// Check if all elements are done, if so, put their chars in the buffer and remove them.
// Also get the amount of misses counted by the elements themselves.
fn get_char_back(chars: &mut Vec<char>, elements: &mut Vec<impl Run>, misses: &mut u16) {
    elements.retain_mut(|f| {
        if let Some((mut characters, nb_misses)) = f.is_done() {
            chars.append(&mut characters);
            *misses += nb_misses;
            false
        } else {
            true
        }
    });
}

// Pick an amount up of char in the buffer.
fn take_chars(chars: &mut Vec<char>, amount: usize) -> Option<Vec<char>> {
    if amount <= chars.len() {
        Some(
            (0..amount)
                .map(|_| chars.remove(rand::thread_rng().gen_range(0, chars.len())))
                .collect(),
        )
    } else {
        None
    }
}

// ----------------------------------------------------------------------------
// --------------------------------------------------------------- Position ---
// List all x values which aren't close to all element's x values.
// Then return randomly one point.
fn find_free_position(busy_x: Vec<Vec<usize>>) -> Point {
    let mut slots: HashSet<usize> = (10..(NB_COLS - 10)).collect();

    for tab in busy_x.iter() {
        for val in tab.iter() {
            if *val > 8 {
                for i in (val - 8)..=(val + 8) {
                    slots.remove(&i);
                }
            }
        }
    }

    let mut point = Point {
        x: 0,
        y: NB_ROWS - 1,
    };

    if slots.is_empty() {
        dbg!("Not enough slots");
        point.x = rand::thread_rng().gen_range(10, NB_COLS - 10);
    } else {
        point.x = *slots
            .iter()
            .nth(rand::thread_rng().gen_range(0, slots.len()))
            .unwrap() as usize;
    }
    point
}
