use std::{borrow::BorrowMut, collections::HashMap};

use crate::{
    config::Config, double_shot_cannon::DoubleShotCannon, laser_cannon::LaserCannon,
    spread_shot_cannon::SpreadShotCannon, weapon::Weapon,
};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum WeaponType {
    LaserCanon,
    RapidFireCanon,
    DoubleShotCanon,
    SpreadShotCanon,
}

pub struct Armoury {
    weapons: HashMap<WeaponType, Box<dyn Weapon>>,
}

impl Armoury {
    pub fn new(config: &Config) -> Self {
        let mut weapons = HashMap::<WeaponType, Box<dyn Weapon>>::new();
        weapons.insert(
            WeaponType::LaserCanon,
            Box::new(LaserCannon::new(
                config.laser_canon_laser_speed,
                config.laser_canon_cool_down,
            )),
        );
        weapons.insert(
            WeaponType::RapidFireCanon,
            Box::new(LaserCannon::new(
                config.rapid_fire_cannon_laser_speed,
                config.rapid_fire_cannon_cool_down,
            )),
        );
        weapons.insert(
            WeaponType::DoubleShotCanon,
            Box::new(DoubleShotCannon::new(
                config.double_shot_cannon_speed,
                config.double_shot_cannon_spread,
                config.double_shot_cannon_cool_down,
            )),
        );
        weapons.insert(
            WeaponType::SpreadShotCanon,
            Box::new(SpreadShotCannon::new(
                config.spread_shot_cannon_laser_speed,
                config.spread_shot_cannon_spread_angle,
                config.spread_shot_cannon_cool_down,
            )),
        );
        Armoury { weapons }
    }

    pub fn get<'a>(&'a mut self, weapon_type: WeaponType) -> &'a mut dyn Weapon {
        self.weapons.get_mut(&weapon_type).unwrap().borrow_mut()
    }
}
