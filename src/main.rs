use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal, ExecutableCommand,
};
use kebbterm::{
    draw::{border, render},
    flare::GroundFlare,
    frame::{new_frame, Drawable},
    geometry::NB_COLS,
    rocket::Rocket,
    spark::Spark,
};
use rand::Rng;

use std::{
    io::{self},
    thread,
    time::Duration,
};

fn take_chars(chars: &mut Vec<char>, amount: usize) -> Option<Vec<char>> {
    if amount <= chars.len() {
        Some(
            (0..amount)
                .map(|_| chars.remove(rand::thread_rng().gen_range(0, chars.len())))
                .collect(),
        )
    } else {
        None
    }
}

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
    let mut flares: Vec<GroundFlare> = Vec::new();

    // TODO Create a list of chars which are given to sparks
    // let mut chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    // let mut chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut chars: Vec<char> =
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ^$[]|&~€!{}%~#?@()*_-:;<>+-=`\\/\"'"
        // "0123456789"
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
                    KeyCode::Enter => {
                        rockets.push(Rocket::new());
                    }
                    KeyCode::Char(' ') => {
                        // Take x chars

                        if let Some(selected_chars) = take_chars(&mut chars, 10) {
                            loop {
                                let pos = rand::thread_rng().gen_range(10, NB_COLS - 10);
                                if !flares
                                    .iter()
                                    .any(|f| pos <= f.position_x() + 3 && pos >= f.position_x() - 3)
                                {
                                    flares.push(GroundFlare::new(selected_chars, pos));
                                    break;
                                }
                            }
                        }
                    }

                    KeyCode::Char(val) => {
                        for spark in sparks.iter_mut() {
                            if spark.check_value(&val) {

                                // TODO SCORE +1
                            }
                        }

                        for flare in flares.iter_mut() {
                            if flare.check_value(&val) {

                                // TODO SCORE +1
                            }
                        }

                        // TODO SCORE -1
                    }
                    _ => {}
                }
            }
        }

        println!("longueur avant run et compagnie: {} ", chars.len());

        // Rockets --
        for rocket in rockets.iter_mut() {
            if rocket.exploded() {
                if let Some(selected) = take_chars(&mut chars, rand::thread_rng().gen_range(3, 10))
                {
                    sparks.push(Spark::new(*rocket.position().unwrap(), selected));
                }
            }
        }

        rockets.retain_mut(|r| !r.exploded());
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

        // TODO: combine with sparks !!!!
        // Flare --

        flares.iter_mut().for_each(|f| {
            f.run();
            f.draw(&mut frame)
        });

        flares.retain_mut(|f| {
            if let Some(mut characters) = f.is_done() {
                dbg!(&characters);
                chars.append(&mut characters);
                false
            } else {
                true
            }
        });

        println!("longueur APRES run et compagnie: {} ", chars.len());
        // --
        render(&frame);

        thread::sleep(Duration::from_millis(2));
    }

    // Cleanup
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
