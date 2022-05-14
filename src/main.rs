mod asteroid;
mod asteroid_factory;
mod config;
mod id_generator;
mod vector;
mod world;
mod world_objects;

use asteroid::Asteroid;
use config::Config;
use rand::Rng;
use std::fmt;
use vector::Vector2;
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
    let mut world = World::new(config);
    world.update(0.1);
}
