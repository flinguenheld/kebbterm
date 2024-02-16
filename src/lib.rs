//! [KebbTerm](https://github.com/flinguenheld/kebbterm) is a terminal game which allows you to test and improve your typing skills 🚀.  
//! Press **ENTER/SPACE/TAB** to throw a new rocket and try to press all characters **(CTRL+C to exit)**.

/// Physical files management like the options.
pub mod files;
/// Regroup all structs used to display/deal with characters.
pub mod firework;
/// Global structs/methods used to shape elements.
pub mod geometry;
/// Modes used by the game loop (windows).
pub mod mode;
/// Link modes/fireworks to Crossterm.
pub mod render;
