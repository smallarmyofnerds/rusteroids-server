use crate::{object::Object, projectile::Laser, vector::Vector2};

pub struct ProjectileFactory {}

impl ProjectileFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_laser(&self, position: Vector2, velocity: Vector2) -> Box<dyn Object> {
        Box::new(Laser::new(position, velocity))
    }
}
