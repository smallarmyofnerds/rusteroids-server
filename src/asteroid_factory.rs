use std::collections::HashMap;

use crate::asteroid::{Asteroid, AsteroidDescriptor, AsteroidSize};
use crate::config::Config;
use crate::vector::Vector2;
use rand::Rng;

pub struct AsteroidFactory {
    asteroid_min_speed: f64,
    asteroid_max_speed: f64,
    asteroid_descriptors: HashMap<AsteroidSize, AsteroidDescriptor>,
}

impl AsteroidFactory {
    pub fn new(config: &Config) -> Self {
        Self {
            asteroid_min_speed: config.asteroid_min_speed,
            asteroid_max_speed: config.asteroid_max_speed,
            asteroid_descriptors: config.asteroid_descriptors.clone(),
        }
    }

    pub fn create_at(&self, id: u64, size: AsteroidSize, position: Vector2) -> Box<Asteroid> {
        Box::new(Asteroid::new(
            id,
            position,
            Vector2::UP.rotate(rand::thread_rng().gen::<f64>() * 360.0)
                * self
                    .asteroid_min_speed
                    .min(rand::thread_rng().gen::<f64>() * self.asteroid_max_speed),
            Vector2::UP.rotate(rand::thread_rng().gen::<f64>() * 360.0),
            rand::thread_rng().gen_range(0..30) as f64,
            *self.asteroid_descriptors.get(&size).unwrap(),
        ))
    }
}
