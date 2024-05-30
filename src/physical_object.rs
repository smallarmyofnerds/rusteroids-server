use std::cmp::{max, min};

use crate::{moveable_object::MoveableObject, object::Object, timer::Timer, vector::Vector2};

pub struct PhysicalObject {
    is_flagged_for_destruction: bool,
    moveable_object: MoveableObject,
    pub radius: f64,
    pub max_health: u64,
    pub health: u64,
    pub damage: u64,
}

impl PhysicalObject {
    pub fn new(
        moveable_object: MoveableObject,
        radius: f64,
        max_health: u64,
        health: u64,
        damage: u64,
    ) -> Self {
        Self {
            is_flagged_for_destruction: false,
            moveable_object,
            radius,
            max_health,
            health,
            damage,
        }
    }

    pub fn is_flagged_for_destruction(&self) -> bool {
        self.is_flagged_for_destruction
    }

    pub fn update(&mut self, timer: &Timer) {
        self.moveable_object.update(timer);
    }

    pub fn get_position(&self) -> Vector2 {
        self.moveable_object.get_position()
    }

    pub fn get_velocity(&self) -> Vector2 {
        self.moveable_object.get_velocity()
    }

    pub fn get_orientation(&self) -> Vector2 {
        self.moveable_object.get_orientation()
    }

    pub fn get_rotational_velocity(&self) -> f64 {
        self.moveable_object.get_rotational_velocity()
    }

    // pub fn set_position(&mut self, value: Vector2) {
    //     self.moveable_object.set_position(value);
    // }

    pub fn set_velocity(&mut self, value: Vector2) {
        self.moveable_object.set_velocity(value);
    }

    // pub fn set_orientation(&mut self, value: Vector2) {
    //     self.moveable_object.set_orientation(value);
    // }

    pub fn set_rotational_velocity(&mut self, value: f64) {
        self.moveable_object.set_rotational_velocity(value);
    }

    pub fn distance_squared_to(&self, position: Vector2) -> f64 {
        self.moveable_object.distance_squared_to(position)
    }

    pub fn within_range_of(&self, position: Vector2, range: f64) -> bool {
        self.distance_squared_to(position) > ((self.radius + range) * (self.radius + range))
    }

    pub fn collides_with(&self, other: &Box<dyn Object>) -> bool {
        other.within_range_of(self.moveable_object.get_position(), self.radius)
    }

    pub fn collide_with(&self, other: &mut Box<dyn Object>) {
        other.apply_damage(self.damage);
    }

    pub fn apply_damage(&mut self, amount: u64) {
        self.health = max(self.health - amount, 0);
        if self.health == 0 {
            self.is_flagged_for_destruction = true
        }
    }

    pub fn heal(&mut self, amount: u64) {
        self.health = min(self.health + amount, self.max_health);
    }
}
