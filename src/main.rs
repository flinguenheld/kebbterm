use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal, ExecutableCommand,
};
use kebbterm::{
    firework::flare::GroundFlare,
    firework::rocket::Rocket,
    firework::spark::Spark,
    firework::Run,
    geometry::NB_COLS,
    render::draw::{border, render},
    render::frame::{new_frame, Drawable, Frame},
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

    let mut sparks: Vec<Spark> = Vec::new();
    let mut flares: Vec<GroundFlare> = Vec::new();

    let mut chars: Vec<char> =
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ^$[]|&~€!{}%~#?@()*_-:;<>+-=`\\/\"'"
        // "0123456789"
            .chars()
            .collect();

    // Better way to print border one time --
    let frame = new_frame();
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
                                if !(flares
                                    .iter()
                                    .any(|f| pos < f.position_x() + 5 && pos > f.position_x() - 5))
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

        // Rockets --
        rockets.retain_mut(|r| {
            if r.exploded() {
                if let Some(selected) = take_chars(&mut chars, rand::thread_rng().gen_range(3, 10))
                {
                    sparks.push(Spark::new(*r.position().unwrap(), selected));
                };
                false
            } else {
                true
            }
        });
        run_draw(&mut rockets, &mut frame);

        // Sparks --
        get_char_back(&mut chars, &mut sparks);
        run_draw(&mut sparks, &mut frame);

        // Flare --
        run_draw(&mut flares, &mut frame);
        get_char_back(&mut chars, &mut flares);

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

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
fn run_draw(elements: &mut Vec<impl Run + Drawable>, frame: &mut Frame) {
    elements.iter_mut().for_each(|f| {
        f.run();
        f.draw(frame)
    });
}

// Check if all elements are done, if so put their char in the buffer and remove them.
fn get_char_back(chars: &mut Vec<char>, elements: &mut Vec<impl Run>) {
    elements.retain_mut(|f| {
        if let Some(mut characters) = f.is_done() {
            chars.append(&mut characters);
            false
        } else {
            true
        }
    });
}

// Pick an amount up of char in the buffer.
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
