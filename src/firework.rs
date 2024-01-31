pub mod flare;
pub mod rocket;
pub mod shape;
pub mod spark;
pub mod tail;

pub trait Run {
    fn run(&mut self);
    // Chars to get in the buffer back and amount of misses.
    fn is_done(&self) -> Option<(Vec<char>, u16)> {
        None
    }
}
pub trait Check {
    fn check_value(&mut self, val: &char) -> bool;
}
