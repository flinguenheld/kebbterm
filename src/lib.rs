pub mod draw;
pub mod frame;
pub mod rocket;

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 80;

struct Point {
    x: usize,
    y: usize,
}
