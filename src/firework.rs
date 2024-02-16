pub mod flare;
pub mod rocket;
pub mod shape;
pub mod shape_skeletons;
pub mod spark;
pub mod tail;

/// Oil the wheels of [`crate::mode::game`] to update all fireworks.
pub trait Run {
    /// Used by each game loop cycle to update firework private values.
    fn run(&mut self);
    /// Check if the firework is over.
    /// Free all its chars for the global char buffer
    /// and the amount of misses to update counters.
    fn is_done(&self) -> Option<(Vec<char>, u16)> {
        None
    }
}

/// Allow [`crate::mode::game`] to check all firework values.
pub trait Check {
    fn check_value(&mut self, val: &char) -> bool;
}
