use crossterm::{cursor, style, QueueableCommand};
use std::io::{self, Write};

use crate::frame::Frame;

pub fn render(frame: &Frame) {
    let mut stdout = io::stdout();
    stdout
        .queue(style::SetBackgroundColor(style::Color::Green))
        .unwrap();
    stdout
        .queue(style::SetForegroundColor(style::Color::Black))
        .unwrap();
    for row in 1..frame.len() {
        for col in 1..frame[0].len() {
            stdout
                .queue(cursor::MoveTo(col as u16, row as u16))
                .unwrap();
            stdout.queue(style::Print(frame[row][col])).unwrap();
        }
    }

    stdout.flush().unwrap();
}

pub fn border(frame: &Frame) {
    let mut stdout = io::stdout();
    stdout
        .queue(style::SetBackgroundColor(style::Color::DarkGrey))
        .unwrap();

    for col in 0..=frame[0].len() as u16 {
        stdout.queue(cursor::MoveTo(col, 0)).unwrap();
        stdout.queue(style::Print(' ')).unwrap();
        stdout
            .queue(cursor::MoveTo(col, frame.len() as u16))
            .unwrap();
        stdout.queue(style::Print(' ')).unwrap();
    }

    for row in 0..frame.len() as u16 {
        stdout.queue(cursor::MoveTo(0, row)).unwrap();
        stdout.queue(style::Print(' ')).unwrap();
        stdout
            .queue(cursor::MoveTo(frame[0].len() as u16, row))
            .unwrap();
        stdout.queue(style::Print(' ')).unwrap();
    }

    // stdout.flush().unwrap();
}
