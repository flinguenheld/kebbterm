pub mod flare;
pub mod rocket;
pub mod spark;
pub mod tail;

pub trait Run {
    fn run(&mut self);
    fn is_done(&self) -> Option<Vec<char>> {
        None
    }
}
pub trait Check {
    fn check_value(&mut self, val: &char) -> bool;
}
