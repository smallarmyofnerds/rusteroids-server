use crate::asteroid::{Asteroid, AsteroidSize};
use crate::config::Config;
use crate::vector::Vector2;
use rand::Rng;

pub struct AsteroidFactory {
    asteroid_min_speed: f64,
    asteroid_max_speed: f64,
}

impl AsteroidFactory {
    pub fn new(config: &Config) -> Self {
        Self {
            asteroid_min_speed: config.asteroid_min_speed,
            asteroid_max_speed: config.asteroid_max_speed,
        }
    }

    pub fn create_at(&self, size: AsteroidSize, position: Vector2) -> Asteroid {
        Asteroid::new(
            position,
            Vector2::UP.rotate(rand::thread_rng().gen::<f64>() * 360.0)
                * self
                    .asteroid_min_speed
                    .min(rand::thread_rng().gen::<f64>() * self.asteroid_max_speed),
            Vector2::UP.rotate(rand::thread_rng().gen::<f64>() * 360.0),
            rand::thread_rng().gen_range(0..30) as f64,
            size,
        )
    }
}
