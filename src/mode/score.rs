use crate::{
    geometry::{NB_COLS, NB_ROWS},
    mode::counter::*,
    mode::utils::*,
    mode::*,
    render::frame::*,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
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
        counters: &mut Counters,
    ) -> io::Result<()> {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event {
                    KeyEvent {
                        modifiers: KeyModifiers::CONTROL,
                        code: KeyCode::Char('c'),
                        ..
                    } => {
                        *mode = Mode::Quit;
                        return Ok(());
                    }
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        ..
                    } => {
                        *mode = Mode::Game(false);
                        counters.reset_timer();
                        return Ok(());
                    }
                    KeyEvent {
                        code: KeyCode::Char('n'),
                        ..
                    } => {
                        *mode = Mode::Game(true);
                        return Ok(());
                    }
                    KeyEvent {
                        code: KeyCode::Char('w'),
                        ..
                    } => {
                        *mode = Mode::Welcome;
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
        // --
        let height = 21;
        let width = 40;
        let y = (NB_ROWS - height) / 2;
        let x = (NB_COLS - width) / 2;

        paint(frame, x, y - 2, height, width, COLOR_BACKGROUND);

        let time_str = {
            if counters.elapsed_time / 60 == 0 {
                format!("{} sec", counters.elapsed_time % 60)
            } else {
                format!(
                    "{} min {} sec",
                    counters.elapsed_time / 60,
                    counters.elapsed_time % 60
                )
            }
        };

        print(frame, y, "PAUSE", COLOR_TITLE);
        print(frame, y + 2, "━━━━━━━━━━━━━━━", COLOR_SEPARATOR);
        print(frame, y + 4, &time_str, COLOR_TEXT);

        print(frame, y + 6, &format!("Success: {}", counters.success), 34);
        print(frame, y + 7, &format!("Misses: {}", counters.misses), 215);
        print(frame, y + 8, &format!("Fails: {}", counters.fails), 124);
        // print(
        //     frame,
        //     y + 10,
        //     &format!("Sparks: {}", counters.sparks),
        //     COLOR_TEXT,
        // );
        // print(
        //     frame,
        //     y + 11,
        //     &format!("Shapes: {}", counters.shapes),
        //     COLOR_TEXT,
        // );
        // print(
        //     frame,
        //     y + 12,
        //     &format!("Ground flares: {}", counters.groundflares),
        //     COLOR_TEXT,
        // );
        print(frame, y + 10, "━━━━━━━━━━━━━━━", COLOR_SEPARATOR);

        print(frame, y + 12, "C -> Continue", COLOR_KEYS_TEXT);
        print(frame, y + 13, "N -> New game", COLOR_KEYS_TEXT);
        print(frame, y + 14, "W -> Go to welcome", COLOR_KEYS_TEXT);
        print(frame, y + 16, "CTRL + C ->  Exit", COLOR_KEYS_TEXT);

        Ok(())
    }
}
