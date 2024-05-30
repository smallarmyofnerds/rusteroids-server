use crate::armoury::WeaponType;
use crate::moveable_object::MoveableObject;
use crate::object::{Object, ObjectType};
use crate::physical_object::PhysicalObject;
use crate::timer::Timer;
use crate::vector::Vector2;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum PowerUpType {
    Health(u64),
    MegaHealth(u64),
    Shield(u64),
    MegaShield(u64),
    DoubleShotLaser,
    SpreadShotLaser,
    RapidShotLaser,
}

pub struct PowerUp {
    power_up_type: PowerUpType,
    physical_object: PhysicalObject,
}

impl PowerUp {
    pub fn new(
        position: Vector2,
        velocity: Vector2,
        power_up_type: PowerUpType,
        radius: f64,
    ) -> Self {
        Self {
            power_up_type,
            physical_object: PhysicalObject::new(
                MoveableObject::new(position, velocity, Vector2::UP, 0.0),
                radius,
                0,
                0,
                0,
            ),
        }
    }
}

impl Object for PowerUp {
    fn is_flagged_for_destruction(&self) -> bool {
        self.physical_object.is_flagged_for_destruction()
    }

    fn get_type(&self) -> crate::object::ObjectType {
        ObjectType::PowerUp
    }

    fn update(&mut self, timer: &Timer) -> Vec<Box<dyn Object>> {
        self.physical_object.update(timer);
        vec![]
    }

    fn _distance_squared_to(&self, position: Vector2) -> f64 {
        self.physical_object.distance_squared_to(position)
    }

    fn within_range_of(&self, position: Vector2, range: f64) -> bool {
        self.physical_object.within_range_of(position, range)
    }

    fn collides_with(&self, other: &Box<dyn Object>) -> bool {
        other.get_type() == ObjectType::Ship && self.physical_object.collides_with(other)
    }

    fn collide_with(&self, other: &mut Box<dyn Object>) {
        if other.get_type() == ObjectType::Ship {
            let ship = other.as_ship_mut().unwrap();
            match self.power_up_type {
                PowerUpType::DoubleShotLaser => ship.set_active_weapon(WeaponType::DoubleShotCanon),
                PowerUpType::SpreadShotLaser => ship.set_active_weapon(WeaponType::SpreadShotCanon),
                PowerUpType::RapidShotLaser => ship.set_active_weapon(WeaponType::RapidFireCanon),
                PowerUpType::Health(amount) => ship.heal(amount),
                PowerUpType::MegaHealth(amount) => ship.heal(amount),
                PowerUpType::Shield(amount) => ship.charge_shield(amount),
                PowerUpType::MegaShield(amount) => ship.charge_shield(amount),
            }
        }
    }

    fn apply_damage(&mut self, _amount: u64) {}

    fn destroy(&self) -> Vec<Box<dyn Object>> {
        vec![]
    }

    fn as_ship(&self) -> Result<&crate::ship::Ship, &'static str> {
        Err("PowerUp!")
    }

    fn as_ship_mut(&mut self) -> Result<&mut crate::ship::Ship, &'static str> {
        Err("PowerUp!")
    }
}
