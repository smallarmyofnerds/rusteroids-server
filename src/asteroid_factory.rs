use std::collections::HashMap;
use std::rc::Rc;

use crate::asteroid::{Asteroid, AsteroidDescriptor, AsteroidSize};
use crate::config::Config;
use crate::power_up_factory::PowerUpFactory;
use crate::vector::Vector2;
use rand::Rng;

pub struct AsteroidFactory {
    asteroid_min_speed: f64,
    asteroid_max_speed: f64,
    asteroid_descriptors: HashMap<AsteroidSize, AsteroidDescriptor>,
    power_up_factory: Rc<PowerUpFactory>,
}

impl AsteroidFactory {
    pub fn new(config: &Config, power_up_factory: Rc<PowerUpFactory>) -> Self {
        Self {
            asteroid_min_speed: config.asteroid_min_speed,
            asteroid_max_speed: config.asteroid_max_speed,
            asteroid_descriptors: config.asteroid_descriptors.clone(),
            power_up_factory,
        }
    }

    pub fn create_at(
        &self,
        size: AsteroidSize,
        position: Vector2,
        asteroid_factory: Rc<AsteroidFactory>,
    ) -> Box<Asteroid> {
        Box::new(Asteroid::new(
            position,
            Vector2::UP.rotate(rand::thread_rng().gen::<f64>() * 360.0)
                * self
                    .asteroid_min_speed
                    .min(rand::thread_rng().gen::<f64>() * self.asteroid_max_speed),
            Vector2::UP.rotate(rand::thread_rng().gen::<f64>() * 360.0),
            rand::thread_rng().gen_range(0..30) as f64,
            *self.asteroid_descriptors.get(&size).unwrap(),
            asteroid_factory.clone(),
            self.power_up_factory.clone(),
        ))
    }
}
