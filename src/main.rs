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
mod projectile;
mod projectile_factory;
mod ship;
mod spread_shot_cannon;
mod timer;
mod vector;
mod weapon;
mod world;

use config::Config;
use player::Player;
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
    let mut world = World::new(config);
    let player = Box::new(Player::new());
    world.create_ship(player);
    for _ in 0..5 {
        timer.tick();
        world.update(&timer);
    }
}
