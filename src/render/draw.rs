use crate::render::frame::Frame;
use crossterm::{
    cursor,
    style::{Attribute, Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, Write};

pub fn render_init() {
    let mut stdout = io::stdout();

    stdout.queue(SetAttribute(Attribute::Bold)).unwrap();
    stdout
        .queue(SetBackgroundColor(Color::AnsiValue(235)))
        .unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();
}

pub fn render(frame: &Frame, previous_frame: &Frame) {
    let mut stdout = io::stdout();
    for row in 0..frame.len() {
        for col in 0..frame[0].len() {
            let case = &frame[row][col];
            let then_case = &previous_frame[row][col];

            if case.value != then_case.value
                || case.fore_color != then_case.fore_color
                || case.back_color != then_case.back_color
            {
                stdout
                    .queue(cursor::MoveTo(col as u16, row as u16))
                    .unwrap();
                stdout.queue(SetBackgroundColor(case.back_color)).unwrap();
                stdout.queue(SetForegroundColor(case.fore_color)).unwrap();
                stdout.queue(Print(case.value)).unwrap();
            }
        }
    }

    stdout.flush().unwrap();
}
