use crate::render::frame::Frame;
use crossterm::{cursor, style, QueueableCommand};
use std::io::{self, Write};

pub fn render(frame: &Frame) {
    let mut stdout = io::stdout();
    stdout
        .queue(style::SetAttribute(style::Attribute::Bold))
        .unwrap();
    for row in 1..frame.len() {
        for col in 1..frame[0].len() {
            stdout
                .queue(style::SetBackgroundColor(frame[row][col].back_color))
                .unwrap();
            stdout
                .queue(style::SetForegroundColor(frame[row][col].fore_color))
                .unwrap();
            stdout
                .queue(cursor::MoveTo(col as u16, row as u16))
                .unwrap();
            stdout.queue(style::Print(frame[row][col].value)).unwrap();
        }
    }

    stdout.flush().unwrap();
}
