use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal, ExecutableCommand,
};
use kebbterm::{
    draw::{border, render},
    frame::{new_frame, Drawable},
    rocket::Rocket,
    spark::Spark,
};
use rand::Rng;

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

    let mut sparks: Vec<Spark> = Vec::new();

    // TODO Create a list of chars which are given to sparks
    // let mut chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    // let mut chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut chars: Vec<char> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ^$[]|&~€!{}%~#?@()*_-:;<>+-=`\\/\"'"
            .chars()
            .collect();

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
                        for rocket in rockets.iter_mut() {
                            if rocket.value() == val {
                                rocket.set_done();

                                // Explode
                                // Take n chars in the buffer and give them to a new spark
                                if let Some(position) = rocket.position() {
                                    if let Some(nb_chars) = rocket.value().to_digit(10) {
                                        if nb_chars <= chars.len() as u32 {
                                            let mut selected_chars = Vec::new();

                                            for _ in 0..nb_chars {
                                                selected_chars.push(chars.remove(
                                                    rand::thread_rng().gen_range(0, chars.len()),
                                                ));
                                            }
                                            sparks.push(Spark::new(*position, selected_chars));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Char(val) => {
                        for spark in sparks.iter_mut() {
                            if let Some(letter) = spark.check_value(&val) {
                                chars.push(letter);
                            }
                            // TODO SCORE -1
                        }
                    }
                    _ => {}
                }
            }
        }

        // Rockets --
        rockets.retain_mut(|r| !r.done());
        rockets.iter_mut().for_each(|r| {
            r.run();
            r.draw(&mut frame);
        });

        // Sparks --
        sparks.retain_mut(|s| {
            if let Some(mut characters) = s.is_done() {
                chars.append(&mut characters);
                false
            } else {
                true
            }
        });

        sparks.iter_mut().for_each(|s| {
            s.run();
            s.draw(&mut frame);
        });

        // --
        render(&frame);

        thread::sleep(Duration::from_millis(50));
    }

    // Cleanup
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
