use crossterm::style;

use crate::geometry::NB_COLS;
use crate::render::frame::Frame;

// ----------------------------------------------------------------------------
// ------------------------------------------------------------------- Menu ---
pub struct Menu {
    fields: Vec<String>,
    current: usize,
}

impl Menu {
    pub fn new(values: Vec<String>, current_selection: usize) -> Menu {
        Menu {
            fields: values,
            current: current_selection,
        }
    }

    pub fn current_selection(&self) -> usize {
        self.current
    }
    pub fn fields(&self) -> &Vec<String> {
        &self.fields
    }

    pub fn up(&mut self) {
        if self.current == 0 {
            self.current = self.fields.len() - 1;
        } else {
            self.current -= 1;
        }
    }
    pub fn down(&mut self) {
        if self.current == self.fields.len() - 1 {
            self.current = 0;
        } else {
            self.current += 1;
        }
    }
}

// ----------------------------------------------------------------------------
// ------------------------------------------------------------------ Print ---
pub fn print_menu(
    menu: &Menu,
    frame: &mut Frame,
    mut y: usize,
    color_text: u8,
    color_active: u8,
) -> usize {
    for (i, field) in menu.fields().iter().enumerate() {
        if i == menu.current_selection() {
            print(frame, y, &format!("-> {} <-", field), color_active);
        } else {
            print(frame, y, field, color_text);
        }

        y += 1;
    }
    y
}

pub fn print(frame: &mut Frame, y: usize, text: &str, color: u8) {
    let start_x = NB_COLS / 2 - text.chars().count() / 2 - 1;
    for (x, c) in text.chars().enumerate() {
        frame[y][start_x + x].value = c;
        frame[y][start_x + x].fore_color = style::Color::AnsiValue(color);
    }
}

pub fn paint(frame: &mut Frame, x: usize, y: usize, height: usize, width: usize, color: u8) {
    for row in frame.iter_mut().skip(y).take(height) {
        for case in row.iter_mut().skip(x).take(width) {
            case.back_color = style::Color::AnsiValue(color);
        }
    }
}

// ----------------------------------------------------------------------------
// ------------------------------------------------------------------ Trait ---
pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
