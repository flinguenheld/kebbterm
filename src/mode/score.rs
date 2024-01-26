use crate::{
    geometry::{NB_COLS, NB_ROWS},
    mode::{counter::Counters, Mode},
    render::frame::{paint, print, Frame},
};
use crossterm::event::{self, Event, KeyCode};
use std::{io, time::Duration};

pub struct ModeScore {}

impl ModeScore {
    pub fn new() -> ModeScore {
        ModeScore {}
    }

    pub fn mode_loop(
        &mut self,
        frame: &mut Frame,
        mode: &mut Mode,
        counters: &Counters,
    ) -> io::Result<()> {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => {
                        *mode = Mode::Quit;
                        return Ok(());
                    }
                    KeyCode::Char('C') | KeyCode::Char('c') => {
                        *mode = Mode::Game(false);
                        return Ok(());
                    }
                    KeyCode::Char('N') | KeyCode::Char('n') => {
                        *mode = Mode::Game(true);
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }

        paint(frame, NB_COLS / 2 - 20, NB_ROWS / 2 - 12, 20, 40, 236);

        let y = NB_ROWS / 2 - 10;
        let fore_color = 250;

        print(frame, y, "PAUSE", fore_color);
        print(frame, y + 2, "━━━━━━━━━━━━━━━", 235);

        print(frame, y + 4, &format!("Success: {}", counters.success), 34);
        print(frame, y + 5, &format!("Fails: {}", counters.fails), 172);
        print(
            frame,
            y + 7,
            &format!("Sparks: {}", counters.sparks),
            fore_color,
        );
        print(
            frame,
            y + 8,
            &format!("Ground flares: {}", counters.groundflares),
            fore_color,
        );
        print(frame, y + 10, "━━━━━━━━━━━━━━━", 235);

        print(frame, y + 12, "C -> Continue", fore_color);
        print(frame, y + 13, "N -> New game", fore_color);
        print(frame, y + 15, "ESC -> Exit", fore_color);

        // frame[10][10].value = 'A';

        Ok(())
    }
}
