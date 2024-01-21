pub mod draw;
pub mod frame;
pub mod rocket;
pub mod spark;

pub const NB_ROWS: usize = 20;
pub const NB_COLS: usize = 80;

#[derive(Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn plus_x(&mut self) {
        if self.x < NB_COLS - 1 {
            self.x += 1;
        }
    }
    fn minus_x(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    fn plus_y(&mut self) {
        if self.y < NB_ROWS - 1 {
            self.y += 1;
        }
    }
    fn minus_y(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
}
