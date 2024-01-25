use super::Mode;
use crate::{
    geometry::{NB_COLS, NB_ROWS},
    render::frame::Frame,
};
use crossterm::event::{self, Event, KeyCode};
use std::{io, time::Duration};

pub struct ModeWelcome {}

impl ModeWelcome {
    pub fn new() -> ModeWelcome {
        ModeWelcome {}
    }

    pub fn mode_loop(&mut self, frame: &mut Frame, mode: &mut Mode) -> io::Result<()> {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => {
                        *mode = Mode::Quit;
                        return Ok(());
                        // break 'gameloop;
                    }
                    KeyCode::Enter => {
                        *mode = Mode::Game;
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }

        let y = NB_ROWS / 2 - 2;

        print(frame, y, "KEBBTERM");
        print(frame, y + 2, "ESCAPE -> Exit");
        print(frame, y + 3, "ENTER -> Throw a rocket");
        print(frame, y + 4, "SPACE -> Start a ground flare");
        // frame[10][10].value = 'A';

        Ok(())
    }
}

fn print(frame: &mut Frame, y: usize, text: &str) {
    // let y = 20;
    let start_x = NB_COLS / 2 - text.len() / 2 - 1;
    for (x, c) in text.chars().enumerate() {
        frame[y][start_x + x].value = c;
    }
}
