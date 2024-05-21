use crate::{
    cool_down::CoolDown, projectile::Projectile, projectile_factory::ProjectileFactory,
    timer::Timer, vector::Vector2, weapon::Weapon,
};

pub struct SpreadShotCannon {
    cool_down: CoolDown,
    laser_speed: u32,
    spread_angle: f64,
}

impl SpreadShotCannon {
    pub fn new(laser_speed: u32, spread_angle: f64, cool_down: f64) -> Self {
        SpreadShotCannon {
            cool_down: CoolDown::new(cool_down),
            laser_speed,
            spread_angle,
        }
    }
}

impl Weapon for SpreadShotCannon {
    fn shoot_from(
        &mut self,
        timer: &Timer,
        projectile_factory: &ProjectileFactory,
        position: Vector2,
        orientation: Vector2,
    ) -> Vec<Box<dyn Projectile>> {
        if self.cool_down.can_shoot() {
            self.cool_down.shoot(timer);
            return vec![
                projectile_factory.create_laser(position, orientation * self.laser_speed.into()),
                projectile_factory.create_laser(
                    position,
                    orientation.rotate(self.spread_angle) * self.laser_speed.into(),
                ),
                projectile_factory.create_laser(
                    position,
                    orientation.rotate(-self.spread_angle) * self.laser_speed.into(),
                ),
            ];
        }
        vec![]
    }
}
