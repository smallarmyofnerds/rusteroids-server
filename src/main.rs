mod armoury;
mod asteroid;
mod asteroid_factory;
mod config;
mod cool_down;
mod double_shot_cannon;
mod id_generator;
mod inputs;
mod laser_cannon;
mod moveable_object;
mod object;
mod object_db;
mod physical_object;
mod player;
mod position_generator;
mod power_up;
mod power_up_factory;
mod projectile;
mod projectile_factory;
mod ship;
mod spread_shot_cannon;
mod timer;
mod vector;
mod weapon;
mod world;

use std::{collections::HashMap, rc::Rc};

use asteroid::AsteroidSize;
use asteroid_factory::AsteroidFactory;
use config::Config;
use player::Player;
use position_generator::PositionGenerator;
use power_up::PowerUpType;
use power_up_factory::PowerUpFactory;
use timer::Timer;
use world::World;

// trait Object {
//     fn init(&self);
//     fn update(&mut self, t: f64);
// }

// struct MoveableObject {
//     position: Vector2,
//     velocity: Vector2,
//     orientation: Vector2,
// }

// impl MoveableObject {
//     fn new() -> MoveableObject {
//         MoveableObject {
//             position: Vector2::EMPTY,
//             velocity: Vector2::RIGHT,
//             orientation: Vector2::UP,
//         }
//     }

//     fn update(&mut self, t: f64) {
//         self.position = self.position + self.velocity * t;
//     }
// }

// impl fmt::Display for MoveableObject {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "p: {} v: {} o: {}", self.position, self.velocity, self.orientation)
//     }
// }

// struct Asteroid {
//     object: MoveableObject,
// }

// impl Asteroid {
//     fn new() -> Asteroid {
//         Asteroid {
//             object: MoveableObject::new(),
//         }
//     }
// }

// impl fmt::Display for Asteroid {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Asteroid: {}", self.object)
//     }
// }

// impl Object for Asteroid {
//     fn init(&self) {}

//     fn update(&mut self, t: f64) {
//         self.object.update(t);
//     }
// }

fn main() {
    let config = Config::new();
    let mut timer = Timer::new(1.0);
    let position_generator = PositionGenerator::new(config.world_width, config.world_height);
    let mut power_up_selections: HashMap<AsteroidSize, Vec<PowerUpType>> = HashMap::new();
    power_up_selections.insert(
        AsteroidSize::Large,
        vec![
            PowerUpType::DoubleShotLaser,
            PowerUpType::SpreadShotLaser,
            PowerUpType::RapidShotLaser,
            PowerUpType::Health(config.power_up_health_amount),
            PowerUpType::Shield(config.power_up_shield_amount),
        ],
    );
    power_up_selections.insert(
        AsteroidSize::Medium,
        vec![
            PowerUpType::DoubleShotLaser,
            PowerUpType::SpreadShotLaser,
            PowerUpType::RapidShotLaser,
            PowerUpType::Health(config.power_up_health_amount),
            PowerUpType::Shield(config.power_up_shield_amount),
            PowerUpType::MegaHealth(config.power_up_mega_health_amount),
            PowerUpType::MegaShield(config.power_up_mega_shield_amount),
        ],
    );
    power_up_selections.insert(
        AsteroidSize::Small,
        vec![
            PowerUpType::RapidShotLaser,
            PowerUpType::MegaHealth(config.power_up_mega_health_amount),
            PowerUpType::MegaShield(config.power_up_mega_shield_amount),
        ],
    );

    let power_up_factory = Rc::new(PowerUpFactory::new(&config, power_up_selections));
    let asteroid_factory = Rc::new(AsteroidFactory::new(&config, power_up_factory.clone()));
    let mut world = World::new(config, &position_generator, asteroid_factory.clone());
    world.create_ship(Box::new(Player::new()));
    for _ in 0..5 {
        println!("Ticking...");
        timer.tick();
        world.update(&timer);
    }
}
