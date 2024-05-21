use std::collections::HashMap;

use crate::asteroid::{AsteroidDescriptor, AsteroidSize};

pub struct Config {
    pub world_width: u32,
    pub world_height: u32,
    pub world_min_obstacles: usize,
    pub safe_respawn_distance: u32,
    pub asteroid_min_speed: f64,
    pub asteroid_max_speed: f64,
    pub ship_linear_acceleration: u32,
    pub ship_linear_friction: f64,
    pub ship_angular_acceleration: u32,
    pub ship_angular_friction: f64,
    pub laser_canon_cool_down: f64,
    pub laser_canon_laser_speed: u32,
    pub rapid_fire_cannon_laser_speed: u32,
    pub rapid_fire_cannon_cool_down: f64,
    pub double_shot_cannon_speed: u32,
    pub double_shot_cannon_spread: f64,
    pub double_shot_cannon_cool_down: f64,
    pub spread_shot_cannon_laser_speed: u32,
    pub spread_shot_cannon_spread_angle: f64,
    pub spread_shot_cannon_cool_down: f64,
    pub asteroid_descriptors: HashMap<AsteroidSize, AsteroidDescriptor>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            world_width: 5000,
            world_height: 5000,
            world_min_obstacles: 30,
            safe_respawn_distance: 300,
            asteroid_min_speed: 50.0,
            asteroid_max_speed: 100.0,
            ship_linear_acceleration: 20,
            ship_linear_friction: 0.1,
            ship_angular_acceleration: 45,
            ship_angular_friction: 0.1,
            laser_canon_cool_down: 0.5,
            laser_canon_laser_speed: 100,
            rapid_fire_cannon_laser_speed: 300,
            rapid_fire_cannon_cool_down: 0.3,
            double_shot_cannon_speed: 100,
            double_shot_cannon_spread: 50.0,
            double_shot_cannon_cool_down: 0.5,
            spread_shot_cannon_laser_speed: 100,
            spread_shot_cannon_spread_angle: 30.0,
            spread_shot_cannon_cool_down: 0.5,
            asteroid_descriptors: HashMap::from([
                (
                    AsteroidSize::Small,
                    AsteroidDescriptor {
                        size: AsteroidSize::Small,
                        radius: 100.0,
                        health: 1000,
                        damage: 10000,
                    },
                ),
                (
                    AsteroidSize::Medium,
                    AsteroidDescriptor {
                        size: AsteroidSize::Medium,
                        radius: 300.0,
                        health: 2000,
                        damage: 20000,
                    },
                ),
                (
                    AsteroidSize::Large,
                    AsteroidDescriptor {
                        size: AsteroidSize::Large,
                        radius: 500.0,
                        health: 3000,
                        damage: 30000,
                    },
                ),
            ]),
        }
    }
}
