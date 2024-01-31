use std::time::Instant;

pub struct Counters {
    pub success: u16,
    pub fails: u16,
    pub misses: u16,
    pub sparks: u16,
    pub shapes: u16,
    pub groundflares: u16,
    pub start_time: std::time::Instant,
    pub elapsed_time: u64,
}

impl Counters {
    pub fn new() -> Counters {
        Counters {
            success: 0,
            fails: 0,
            misses: 0,
            sparks: 0,
            shapes: 0,
            groundflares: 0,
            start_time: Instant::now(),
            elapsed_time: 0,
        }
    }
    pub fn reset_timer(&mut self) {
        self.start_time = Instant::now()
    }
}
