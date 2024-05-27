use crate::{object::Object, projectile_factory::ProjectileFactory, timer::Timer, vector::Vector2};

pub trait Weapon {
    fn shoot_from(
        &mut self,
        timer: &Timer,
        projectile_factory: &ProjectileFactory,
        origin: Vector2,
        orientation: Vector2,
    ) -> Vec<Box<dyn Object>>;
}
