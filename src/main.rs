use crossterm::{
    self, cursor,
    event::{self, Event, KeyCode},
    style, terminal, ExecutableCommand, QueueableCommand,
};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() -> io::Result<()> {
    // Setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    'gameloop: loop {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    _ => {
                        // println!("pouet");
                        stdout
                            .queue(style::SetBackgroundColor(style::Color::Green))
                            .unwrap();
                        stdout
                            .queue(style::SetForegroundColor(style::Color::Black))
                            .unwrap();
                        for row in 0..25 {
                            for col in 0..50 {
                                stdout.queue(cursor::MoveTo(col, row)).unwrap();
                                stdout.queue(style::Print("p")).unwrap();
                            }
                        }

                        stdout.flush().unwrap();
                    }
                }
            }
        }

        // render(&mut stdout);
        thread::sleep(Duration::from_millis(100));
    }

    // Cleanup
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
