use crate::Point;

/*
 * Speed is measured by ticks. Each game loop counts +1 tick and each element moves according to a sum of ticks.
 * This struct allows you to have a variable speed.
 * Give two points of a line (slow & fast speed) and update the current speed value by x or y. (y = mx + b)
 * Then use functions 'up_tick' and 'reached' to move your elements.
 */
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

    pub fn up_by_x(&mut self, y: f32) {
        self.current_speed = ((y - self.b) / self.m) as u32;
    }
    pub fn up_by_y(&mut self, x: f32) {
        self.current_speed = (self.m * x + self.b) as u32;
    }

    pub fn up_tick(&mut self) {
        self.tick_count += 1;
    }
    pub fn reached(&mut self) -> bool {
        if self.tick_count >= self.current_speed {
            self.tick_count = 1;
            true
        } else {
            false
        }
    }
}
