use crossterm::{cursor, terminal, ExecutableCommand};
use kebbterm::{
    mode::{counter::Counters, game::ModeGame, score::ModeScore, welcome::ModeWelcome, Mode},
    render::{draw::render, frame::new_frame},
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

    let mut mode = Mode::Welcome;
    let mut mode_welcome = ModeWelcome::new();
    let mut mode_game = ModeGame::new();
    let mut mode_score = ModeScore::new();

    let mut counters = Counters::new();

    // --
    'gameloop: loop {
        let mut frame = new_frame();
        let start_time = std::time::Instant::now();

        match mode {
            Mode::Welcome => mode_welcome.mode_loop(&mut frame, &mut mode)?,
            Mode::Game(new) => {
                if new == true {
                    counters = Counters::new();
                    mode_game = ModeGame::new();
                    mode = Mode::Game(false);
                }
                mode_game.mode_loop(&mut frame, &mut mode, &mut counters)?
            }
            Mode::Score => mode_score.mode_loop(&mut frame, &mut mode, &counters)?,
            _ => break 'gameloop,
        };

        // --
        render(&frame);

        let elapsed_time = start_time.elapsed().as_micros();
        dbg!(elapsed_time);
        if elapsed_time < 5000 {
            thread::sleep(Duration::from_micros(
                (Duration::from_micros(5100).as_micros() - start_time.elapsed().as_micros()) as u64,
            ));
        }
    }

    // Cleanup
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
