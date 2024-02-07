use crate::{
    files::option::*,
    geometry::{NB_COLS, NB_ROWS},
    mode::utils::*,
    mode::*,
    render::frame::*,
};
use crossterm::event::{self, Event, KeyCode};
use std::{io, time::Duration};

pub struct ModeOption {
    menu: Menu,
    options: Options,
}

impl ModeOption {
    pub fn new() -> ModeOption {
        ModeOption {
            menu: Menu::new(Vec::new(), 0),
            options: Options::new(),
        }
    }

    pub fn mode_loop(&mut self, frame: &mut Frame, mode: &mut Mode) -> io::Result<()> {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Enter => {
                        self.options.write()?;
                        *mode = Mode::Welcome;
                        return Ok(());
                    }
                    KeyCode::Up => self.menu.up(),
                    KeyCode::Down => self.menu.down(),
                    KeyCode::Left => match self.menu.current_selection() {
                        0 => match self.options.speed {
                            1 => self.options.speed = 10,
                            _ => self.options.speed -= 1,
                        },
                        1 => self.options.letter = !self.options.letter,
                        2 => self.options.capital = !self.options.capital,
                        3 => self.options.digit = !self.options.digit,
                        4 => self.options.symbol = !self.options.symbol,
                        5 => self.options.french = !self.options.french,
                        _ => self.options.french_cap = !self.options.french_cap,
                    },
                    KeyCode::Right => match self.menu.current_selection() {
                        0 => match self.options.speed {
                            10 => self.options.speed = 1,
                            _ => self.options.speed += 1,
                        },
                        1 => self.options.letter = !self.options.letter,
                        2 => self.options.capital = !self.options.capital,
                        3 => self.options.digit = !self.options.digit,
                        4 => self.options.symbol = !self.options.symbol,
                        5 => self.options.french = !self.options.french,
                        _ => self.options.french_cap = !self.options.french_cap,
                    },
                    _ => {}
                }
            }
        }

        self.menu = up_menu(&mut self.options, self.menu.current_selection());

        // --
        let height = 19;
        let width = 40;
        let y = (NB_ROWS - height) / 2;
        let x = (NB_COLS - width) / 2;

        paint(frame, x, y - 2, height, width, COLOR_BACKGROUND);

        print(frame, y, "OPTIONS", COLOR_TITLE);
        print(frame, y + 2, "━━━━━━━━━━━━━━━━━", COLOR_SEPARATOR);

        let y = print_menu(
            &self.menu,
            frame,
            y + 4,
            COLOR_MENU_TEXT,
            COLOR_MENU_CURRENT,
        );

        print(frame, y + 1, "━━━━━━━━━━━━━━━━━", COLOR_SEPARATOR);
        print(frame, y + 3, "ENTER -> Save & exit", COLOR_KEYS_TEXT);

        Ok(())
    }
}

fn up_menu(options: &mut Options, current_index: usize) -> Menu {
    Menu::new(
        vec![
            format!("Speed: {}", options.speed),
            format!("Letters: {}", options.letter),
            format!("Capitals: {}", options.capital),
            format!("Numbers: {}", options.digit),
            format!("Symbols: {}", options.symbol),
            format!("French: {}", options.french),
            format!("French cap: {}", options.french_cap),
        ],
        current_index,
    )
}
