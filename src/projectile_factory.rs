use crate::{id_generator::IdGenerator, projectile::Laser, vector::Vector2};

pub struct ProjectileFactory<'a> {
    id_generator: &'a mut IdGenerator,
}

impl<'a> ProjectileFactory<'a> {
    pub fn new(id_generator: &mut IdGenerator) -> Self {
        Self { id_generator }
    }

    pub fn create_laser(&self, position: Vector2, velocity: Vector2) -> Box<Laser> {
        Box::new(Laser::new(
            self.id_generator.get_next_id(),
            position,
            velocity,
        ))
    }
}
