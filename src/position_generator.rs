use rand::Rng;

use crate::vector::Vector2;

pub struct PositionGenerator {
    max_width: u32,
    max_height: u32,
}

impl PositionGenerator {
    pub fn new(max_width: u32, max_height: u32) -> Self {
        PositionGenerator {
            max_width,
            max_height,
        }
    }

    pub fn random_position(&self) -> Vector2 {
        Vector2 {
            x: rand::thread_rng().gen::<f64>() * self.max_width as f64,
            y: rand::thread_rng().gen::<f64>() * self.max_height as f64,
        }
    }
}
