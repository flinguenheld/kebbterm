use crate::{
    firework::{flare::GroundFlare, rocket::Rocket, spark::Spark, Check, Run},
    geometry::NB_COLS,
    mode::counter::Counters,
    mode::Mode,
    render::frame::{Drawable, Frame},
};
use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::{io, time::Duration};

pub struct ModeGame {
    rockets: Vec<Rocket>,
    sparks: Vec<Spark>,
    ground_flares: Vec<GroundFlare>,
    chars: Vec<char>,
}

impl ModeGame {
    pub fn new() -> ModeGame {
        ModeGame {
            rockets: Vec::new(),
            sparks: Vec::new(),
            ground_flares: Vec::new(),

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
                        self.rockets.push(Rocket::new());
                    }
                    KeyCode::Char(' ') => {
                        if let Some(selected_chars) = take_chars(&mut self.chars, 10) {
                            loop {
                                let pos = rand::thread_rng().gen_range(10, NB_COLS - 10);
                                if !(self
                                    .ground_flares
                                    .iter()
                                    .any(|f| pos < f.position_x() + 5 && pos > f.position_x() - 5))
                                {
                                    self.ground_flares
                                        .push(GroundFlare::new(selected_chars, pos));
                                    counters.groundflares += 1;
                                    break;
                                }
                            }
                        }
                    }

                    KeyCode::Char(val) => {
                        if check_value(&mut self.sparks, &val, &mut counters.success) == false
                            && check_value(&mut self.ground_flares, &val, &mut counters.success)
                                == false
                        {
                            counters.fails += 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Rockets --
        self.rockets.retain_mut(|r| {
            if r.exploded() {
                if let Some(selected) =
                    take_chars(&mut self.chars, rand::thread_rng().gen_range(3, 10))
                {
                    self.sparks
                        .push(Spark::new(*r.position().unwrap(), selected));
                    counters.sparks += 1;
                };
                false
            } else {
                true
            }
        });
        run_draw(&mut self.rockets, frame);

        // Sparks --
        get_char_back(&mut self.chars, &mut self.sparks);
        run_draw(&mut self.sparks, frame);

        // Flare --
        run_draw(&mut self.ground_flares, frame);
        get_char_back(&mut self.chars, &mut self.ground_flares);

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
fn get_char_back(chars: &mut Vec<char>, elements: &mut Vec<impl Run>) {
    elements.retain_mut(|f| {
        if let Some(mut characters) = f.is_done() {
            chars.append(&mut characters);
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
