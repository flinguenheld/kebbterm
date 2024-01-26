pub mod counter;
pub mod game;
pub mod score;
pub mod welcome;

pub enum Mode {
    Welcome,
    Game(bool),
    Score,
    Quit,
}
