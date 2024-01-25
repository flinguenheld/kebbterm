use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal, ExecutableCommand,
};
use kebbterm::{
    firework::flare::*, firework::rocket::*, firework::spark::*, firework::*, geometry::NB_COLS,
    render::draw::*, render::frame::*,
};
use rand::Rng;

use std::{
    io::{self},
    thread,
    time::{Duration, Instant},
};

struct Counters {
    success: u16,
    fails: u16,
    sparks: u16,
    groundflares: u16,
    start_time: std::time::Instant,
}

fn main() -> io::Result<()> {
    // Setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let mut rockets: Vec<Rocket> = Vec::new();
    let mut sparks: Vec<Spark> = Vec::new();
    let mut ground_flares: Vec<GroundFlare> = Vec::new();

    let mut chars: Vec<char> =
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ^$[]|&~€!{}%~#?@()*_-:;<>+-=`\\/\"'"
        // "0123456789"
            .chars()
            .collect();

    let mut counters = Counters {
        success: 0,
        fails: 0,
        sparks: 0,
        groundflares: 0,
        start_time: Instant::now(),
    };

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
                                if !(ground_flares
                                    .iter()
                                    .any(|f| pos < f.position_x() + 5 && pos > f.position_x() - 5))
                                {
                                    ground_flares.push(GroundFlare::new(selected_chars, pos));
                                    counters.groundflares += 1;
                                    break;
                                }
                            }
                        }
                    }

                    KeyCode::Char(val) => {
                        if check_value(&mut sparks, &val, &mut counters.success) == false
                            && check_value(&mut ground_flares, &val, &mut counters.success) == false
                        {
                            counters.fails += 1;
                        }
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
                    counters.sparks += 1;
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
        run_draw(&mut ground_flares, &mut frame);
        get_char_back(&mut chars, &mut ground_flares);

        // --
        render(&frame);
        thread::sleep(Duration::from_millis(2));
    }

    // Cleanup
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    // Temp scores:
    println!();
    println!("Elapsed time: {:?}", counters.start_time.elapsed());
    println!("Success: {}", counters.success);
    println!("Fails: {}", counters.fails);
    println!("Rockets: {}", counters.sparks);
    println!("Ground flares: {}", counters.groundflares);

    Ok(())
}

// ----------------------------------------------------------------------------
// ------------------------------------------------------------------ Check ---
fn check_value(element: &mut Vec<impl Check>, val: &char, counter: &mut u16) -> bool {
    for e in element.iter_mut() {
        if e.check_value(&val) {
            *counter += 1;
            return true;
        }
    }
    return false;
}

// ----------------------------------------------------------------------------
// --------------------------------------------------------- Run & Drawable ---
fn run_draw(elements: &mut Vec<impl Run + Drawable>, frame: &mut Frame) {
    elements.iter_mut().for_each(|f| {
        f.run();
        f.draw(frame)
    });
}

// Check if all elements are done, if so, put their chars in the buffer and remove them.
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
