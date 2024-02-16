/// Window height.
pub const NB_ROWS: usize = 40;
/// Window width.
pub const NB_COLS: usize = 120;

// ----------------------------------------------------------------------------
// ------------------------------------------------------------------ Point ---
/// Simple point with x, y.  
/// Use methods to stay in the [frame](`crate::render::frame`)
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn plus_x(&mut self) {
        if self.x < NB_COLS - 1 {
            self.x += 1;
        }
    }
    pub fn minus_x(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn plus_y(&mut self) {
        if self.y < NB_ROWS - 1 {
            self.y += 1;
        }
    }
    pub fn minus_y(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
}
// ----------------------------------------------------------------------------
// ------------------------------------------------------------------ Color ---
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl ColorRGB {
    pub fn plus_block(&mut self, letter: char, value: u8) {
        match letter {
            'r' => {
                if self.r as u16 + value as u16 <= 255 {
                    self.r += value
                }
            }
            'g' => {
                if self.g as u16 + value as u16 <= 255 {
                    self.g += value
                }
            }
            _ => {
                if self.b as u16 + value as u16 <= 255 {
                    self.b += value
                }
            }
        }
    }
    pub fn minus_block(&mut self, letter: char, value: u8) {
        match letter {
            'r' => {
                if self.r >= value {
                    self.r -= value
                }
            }
            'g' => {
                if self.g >= value {
                    self.g -= value
                }
            }
            _ => {
                if self.b >= value {
                    self.b -= value
                }
            }
        }
    }
}
// ----------------------------------------------------------------------------
// ------------------------------------------------------------------ Speed ---

/// Speed is measured by ticks. Each game loop counts +1 tick and each element
/// moves according to a sum of ticks.  
///
/// This struct allows you to have a variable speed.  
/// Give two points of a line (slow & fast speed) and update the current speed
/// value by x or y. (y = mx + b)  
///
/// Then use functions 'up_tick' and 'reached' to move your elements.
pub struct Speed {
    m: f32,
    b: f32,

    tick_count: u32,
    current_speed: u32,
}

impl Speed {
    pub fn new(point_a: Point, point_b: Point) -> Speed {
        let mut speed = Speed {
            m: 0.0,
            b: 0.0,
            tick_count: 0,
            current_speed: 0,
        };

        speed.m = (point_b.y as f32 - point_a.y as f32) / (point_b.x as f32 - point_a.x as f32);
        speed.b = point_a.y as f32 - (speed.m * point_a.x as f32);

        speed
    }

    /// Update speed with a value on X.
    pub fn up_by_x(&mut self, y: f32) {
        self.current_speed = ((y - self.b) / self.m) as u32;
    }
    /// Update speed with a value on Y.
    pub fn up_by_y(&mut self, x: f32) {
        self.current_speed = (self.m * x + self.b) as u32;
    }

    /// Tick counter += 1.
    pub fn up_tick(&mut self) {
        self.tick_count += 1;
    }

    /// Check if the tick counter has reached the current speed value.  
    /// If true, reset the tick counter.
    pub fn reached(&mut self) -> bool {
        if self.tick_count >= self.current_speed {
            self.tick_count = 1;
            true
        } else {
            false
        }
    }
}
