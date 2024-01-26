use crate::{
    geometry::{NB_COLS, NB_ROWS},
    mode::Mode,
    render::frame::{paint, print, Frame},
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
                    }
                    KeyCode::Enter => {
                        *mode = Mode::Game(false);
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }

        paint(frame, NB_COLS / 2 - 20, NB_ROWS / 2 - 7, 12, 40, 236);

        let y = NB_ROWS / 2 - 5;
        let fore_color = 250;

        print(frame, y, "KEBB TERM", 214);
        print(frame, y + 2, "━━━━━━━━━━━━━━━━━", 235);
        print(frame, y + 4, "ENTER -> Throw a rocket", fore_color);
        print(frame, y + 5, "SPACE -> Start a ground flare", fore_color);
        print(frame, y + 7, "ESC ->  Exit", fore_color);

        Ok(())
    }
}
