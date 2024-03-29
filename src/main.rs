use crossbeam::channel::bounded;
use crossterm::{cursor, terminal, ExecutableCommand};
use kebbterm::mode::{counter::*, game::*, option::*, score::*, welcome::*, Mode};
use kebbterm::render::{draw::*, frame::*};
use std::{
    io::{self},
    thread,
    time::Duration,
};

/// Launch [Crossterm](https://docs.rs/crossterm/latest/crossterm/) and manage the game loop.
fn main() -> io::Result<()> {
    // Setup --
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let mut counters = Counters::new();
    let mut mode = Mode::Welcome;
    let mut mode_game = ModeGame::new();
    let mut mode_option = ModeOption::new();
    let mut mode_score = ModeScore::new();
    let mut mode_welcome = ModeWelcome::new();

    // Render --
    render_init();
    let (s, r) = bounded::<Frame>(1);
    let render_thread = thread::spawn(move || {
        let mut previous_frame = new_frame();
        while let Ok(frame) = r.recv() {
            render(&frame, &previous_frame);
            previous_frame = frame;
        }
    });

    // --
    'gameloop: loop {
        let mut frame = new_frame();

        match mode {
            Mode::Game(new) => {
                if new {
                    counters = Counters::new();
                    mode_game = ModeGame::new();
                    mode = Mode::Game(false);
                }
                mode_game.mode_loop(&mut frame, &mut mode, &mut counters)?
            }
            Mode::Option => mode_option.mode_loop(&mut frame, &mut mode)?,
            Mode::Score => mode_score.mode_loop(&mut frame, &mut mode, &mut counters)?,
            Mode::Welcome => mode_welcome.mode_loop(&mut frame, &mut mode)?,
            _ => break 'gameloop,
        };

        // --
        let _ = s.send(frame);
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup --
    drop(s);
    render_thread.join().unwrap();

    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
