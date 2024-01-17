use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal, ExecutableCommand,
};
use kebbterm::{
    draw::{self, border, render},
    frame::{new_frame, Drawable, Frame},
    rocket::Rocket,
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

    let mut rocket = Rocket::new();

    // Better way to print border one time --
    let mut frame = new_frame();
    border(&frame);

    // --
    'gameloop: loop {
        let mut frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => {
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Rocket --
        rocket.run();
        rocket.draw(&mut frame);

        render(&frame);

        // // render(&mut stdout);
        thread::sleep(Duration::from_millis(100));
    }

    // Cleanup
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
