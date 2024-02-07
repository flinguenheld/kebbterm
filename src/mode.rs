pub mod counter;
pub mod game;
pub mod option;
pub mod score;
pub mod utils;
pub mod welcome;

pub enum Mode {
    Game(bool),
    Option,
    Quit,
    Score,
    Welcome,
}

const COLOR_TITLE: u8 = 214;
const COLOR_TEXT: u8 = 250;
const COLOR_BACKGROUND: u8 = 236;
const COLOR_SEPARATOR: u8 = 235;
const COLOR_KEYS_TEXT: u8 = 242;
const COLOR_MENU_TEXT: u8 = 250;
// const COLOR_MENU_CURRENT: u8 = 39;
const COLOR_MENU_CURRENT: u8 = 214;
