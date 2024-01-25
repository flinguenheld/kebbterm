use crossterm::{cursor, terminal, ExecutableCommand};
use kebbterm::{
    mode::{game::ModeGame, welcome::ModeWelcome, Mode},
    render::{draw::render, frame::new_frame},
    // render::{self, frame::*},
};

use std::{
    io::{self},
    thread,
    time::{Duration, Instant},
};

fn main() -> io::Result<()> {
    // Setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let mut mode_welcome = ModeWelcome::new();
    let mut mode_game = ModeGame::new();
    let mut mode = Mode::Welcome;

    // --
    'gameloop: loop {
        let mut frame = new_frame();

        match mode {
            Mode::Welcome => mode_welcome.mode_loop(&mut frame, &mut mode)?,
            Mode::Game => mode_game.mode_loop(&mut frame, &mut mode)?,
            _ => break 'gameloop,
        };

        // --
        render(&frame);
        thread::sleep(Duration::from_millis(2));
        // break;
    }

    // Cleanup
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    // Temp scores:
    println!();
    // println!("Elapsed time: {:?}", counters.start_time.elapsed());
    // println!("Success: {}", counters.success);
    // println!("Fails: {}", counters.fails);
    // println!("Rockets: {}", counters.sparks);
    // println!("Ground flares: {}", counters.groundflares);

    Ok(())
}
