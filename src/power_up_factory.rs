use std::collections::HashMap;

use crate::asteroid::AsteroidSize;
use crate::config::Config;
use crate::power_up::{PowerUp, PowerUpType};
use crate::vector::Vector2;
use rand::Rng;

pub struct PowerUpFactory {
    power_up_min_speed: f64,
    power_up_max_speed: f64,
    power_up_selections: HashMap<AsteroidSize, Vec<PowerUpType>>,
}

impl PowerUpFactory {
    pub fn new(
        config: &Config,
        power_up_selections: HashMap<AsteroidSize, Vec<PowerUpType>>,
    ) -> Self {
        Self {
            power_up_min_speed: config.power_up_min_speed,
            power_up_max_speed: config.power_up_max_speed,
            power_up_selections,
        }
    }

    pub fn create_at(&self, size: AsteroidSize, position: Vector2) -> Box<PowerUp> {
        let possible_power_ups = self.power_up_selections.get(&size).unwrap();
        let random_power_up = possible_power_ups
            .get(rand::thread_rng().gen_range(0..possible_power_ups.len()))
            .unwrap();
        Box::new(PowerUp::new(
            position,
            Vector2::UP.rotate(rand::thread_rng().gen::<f64>() * 360.0)
                * self
                    .power_up_min_speed
                    .min(rand::thread_rng().gen::<f64>() * self.power_up_max_speed),
            *random_power_up,
            100.0,
        ))
    }
}
