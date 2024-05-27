use crate::{
    cool_down::CoolDown, object::Object, projectile_factory::ProjectileFactory, timer::Timer,
    vector::Vector2, weapon::Weapon,
};

pub struct LaserCannon {
    cool_down: CoolDown,
    laser_speed: u32,
}

impl LaserCannon {
    pub fn new(speed: u32, cool_down: f64) -> Self {
        LaserCannon {
            cool_down: CoolDown::new(cool_down),
            laser_speed: speed,
        }
    }
}

impl Weapon for LaserCannon {
    fn shoot_from(
        &mut self,
        timer: &Timer,
        projectile_factory: &ProjectileFactory,
        origin: Vector2,
        orientation: Vector2,
    ) -> Vec<Box<dyn Object>> {
        if self.cool_down.can_shoot() {
            self.cool_down.shoot(timer);
            return vec![
                projectile_factory.create_laser(origin, orientation * self.laser_speed.into())
            ];
        }
        vec![]
    }
}
