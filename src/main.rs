use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal, ExecutableCommand,
};
use kebbterm::{
    draw::{border, render},
    frame::{new_frame, Drawable},
    rocket::Rocket,
};

use std::{
    io::{self},
    thread,
    time::Duration,
};

fn main() -> io::Result<()> {
    // Setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let mut rockets: Vec<Rocket> = Vec::new();
    rockets.push(Rocket::new());
    // let mut rocket = Rocket::new();

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
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        rockets.push(Rocket::new());
                    }
                    KeyCode::Char(val) if "0123456789".contains(val) => {
                        println!("hep");

                        rockets
                            .iter_mut()
                            .filter(|r| r.value() == val)
                            .for_each(|r| r.set_done());

                        // Explode
                    }

                    _ => {}
                }
            }
        }

        // Rockets --
        rockets.retain_mut(|r| !r.done());
        rockets.iter_mut().for_each(|r| {
            r.run();
            r.draw(&mut frame)
        });

        // println!("pouet: {}", rockets.len());

        render(&frame);

        // // render(&mut stdout);
        thread::sleep(Duration::from_millis(20));
    }

    // Cleanup
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
