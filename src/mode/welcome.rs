use crate::{
    geometry::{NB_COLS, NB_ROWS},
    mode::utils::*,
    mode::*,
    render::frame::*,
};
use crossterm::event::{self, Event, KeyCode};
use std::{io, time::Duration};

pub struct ModeWelcome {
    menu: Menu,
}

impl ModeWelcome {
    pub fn new() -> ModeWelcome {
        ModeWelcome {
            menu: Menu::new(
                vec!["PLAY".to_string(), "OPTION".to_string(), "EXIT".to_string()],
                0,
            ),
        }
    }

    pub fn mode_loop(&mut self, frame: &mut Frame, mode: &mut Mode) -> io::Result<()> {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Enter => {
                        match self.menu.current_selection() {
                            0 => *mode = Mode::Game(true),
                            1 => *mode = Mode::Option,
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
        let height = 19;
        let width = 40;
        let y = (NB_ROWS - height) / 2;
        let x = (NB_COLS - width) / 2;

        paint(frame, x, y - 2, height, width, COLOR_BACKGROUND);

        print(frame, y, "KEBB TERM", COLOR_TITLE);
        print(frame, y + 2, "━━━━━━━━━━━━━━━━━", COLOR_SEPARATOR);

        let y = print_menu(
            &self.menu,
            frame,
            y + 4,
            COLOR_MENU_TEXT,
            COLOR_MENU_CURRENT,
        );

        print(frame, y + 1, "━━━━━━━━━━━━━━━━━", COLOR_SEPARATOR);
        print(frame, y + 3, "ENTER -> Throw a rocket", COLOR_KEYS_TEXT);
        print(frame, y + 4, "TAB -> Throw a rocket shape", COLOR_KEYS_TEXT);
        print(
            frame,
            y + 5,
            "SPACE -> Start a ground flare",
            COLOR_KEYS_TEXT,
        );
        print(frame, y + 7, "CTRL + C ->  Exit", COLOR_KEYS_TEXT);

        Ok(())
    }
}
