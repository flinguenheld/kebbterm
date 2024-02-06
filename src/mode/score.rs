use crate::{
    geometry::{NB_COLS, NB_ROWS},
    mode::counter::*,
    mode::utils::*,
    mode::*,
    render::frame::*,
};
use crossterm::event::{self, Event, KeyCode};
use std::{io, time::Duration};

pub struct ModeScore {
    menu: Menu,
}

impl ModeScore {
    pub fn new() -> ModeScore {
        ModeScore {
            menu: Menu::new(
                vec![
                    "RESUME".to_string(),
                    "NEW GAME".to_string(),
                    "HOME".to_string(),
                    "EXIT".to_string(),
                ],
                0,
            ),
        }
    }

    pub fn mode_loop(
        &mut self,
        frame: &mut Frame,
        mode: &mut Mode,
        counters: &mut Counters,
    ) -> io::Result<()> {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Enter => {
                        match self.menu.current_selection() {
                            0 => {
                                *mode = Mode::Game(false);
                                counters.reset_timer();
                            }
                            1 => *mode = Mode::Game(true),
                            2 => *mode = Mode::Welcome,
                            _ => *mode = Mode::Quit,
                        }
                        return Ok(());
                    }
                    KeyCode::Up => self.menu.up(),
                    KeyCode::Down => self.menu.down(),
                    _ => {}
                }
            }
        }

        // --
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

        // --
        let height = 20;
        let width = 40;
        let y = (NB_ROWS - height) / 2;
        let x = (NB_COLS - width) / 2;

        paint(frame, x, y - 2, height, width, COLOR_BACKGROUND);

        print(frame, y, "PAUSE", COLOR_TITLE);
        print(frame, y + 2, "━━━━━━━━━━━━━━━", COLOR_SEPARATOR);
        print(frame, y + 4, &time_str, COLOR_TEXT);

        print(frame, y + 6, &format!("Success: {}", counters.success), 34);
        print(frame, y + 7, &format!("Misses: {}", counters.misses), 215);
        print(frame, y + 8, &format!("Fails: {}", counters.fails), 124);
        print(frame, y + 10, "━━━━━━━━━━━━━━━", COLOR_SEPARATOR);

        print_menu(
            &self.menu,
            frame,
            y + 12,
            COLOR_MENU_TEXT,
            COLOR_MENU_CURRENT,
        );

        Ok(())
    }
}
