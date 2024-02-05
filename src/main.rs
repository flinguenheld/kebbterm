use crossbeam::channel::bounded;
use crossterm::{cursor, terminal, ExecutableCommand};
use kebbterm::files::option::Options;
use kebbterm::mode::{counter::*, game::*, score::*, welcome::*, Mode};
use kebbterm::render::{draw::*, frame::*};
use std::{
    io::{self},
    thread,
    time::Duration,
};

fn main() -> io::Result<()> {
    // Setup --
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let mut counters = Counters::new();
    let mut options = Options::new();
    options.read()?;

    let mut mode = Mode::Welcome;
    let mut mode_welcome = ModeWelcome::new();
    let mut mode_game = ModeGame::new(&options);
    let mut mode_score = ModeScore::new();

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
            Mode::Welcome => mode_welcome.mode_loop(&mut frame, &mut mode)?,
            Mode::Game(new) => {
                if new == true {
                    counters = Counters::new();
                    mode_game = ModeGame::new(&options);
                    mode = Mode::Game(false);
                }
                mode_game.mode_loop(&mut frame, &mut mode, &mut counters)?
            }
            Mode::Score => mode_score.mode_loop(&mut frame, &mut mode, &mut counters)?,
            _ => break 'gameloop,
        };

        // --
        let _ = s.send(frame);
        thread::sleep(Duration::from_millis(2));
    }

    // Cleanup --
    drop(s);
    render_thread.join().unwrap();

    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
