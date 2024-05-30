use std::f64::consts::FRAC_PI_2;

use crate::{
    cool_down::CoolDown, object::Object, projectile_factory::ProjectileFactory, timer::Timer,
    vector::Vector2, weapon::Weapon,
};

pub struct DoubleShotCannon {
    cool_down: CoolDown,
    laser_speed: u32,
    spread: f64,
}

impl DoubleShotCannon {
    pub fn new(speed: u32, spread: f64, cool_down: f64) -> Self {
        DoubleShotCannon {
            cool_down: CoolDown::new(cool_down),
            laser_speed: speed,
            spread,
        }
    }
}

impl Weapon for DoubleShotCannon {
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
                projectile_factory.create_laser(
                    origin + orientation.rotate(FRAC_PI_2) * self.spread,
                    orientation * self.laser_speed.into(),
                ),
                projectile_factory.create_laser(
                    origin + orientation.rotate(-FRAC_PI_2) * self.spread,
                    orientation * self.laser_speed.into(),
                ),
            ];
        }
        vec![]
    }
}
