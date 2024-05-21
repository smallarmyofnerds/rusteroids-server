use std::cmp::max;

use crate::{
    collides::Collides, moveable_object::MoveableObject, timer::Timer, vector::Vector2,
    world_object::WorldObject,
};

pub struct PhysicalObject {
    moveable_object: MoveableObject,
    pub radius: f64,
    pub health: u64,
    pub damage: u64,
}

impl PhysicalObject {
    pub fn new(moveable_object: MoveableObject, radius: f64, health: u64, damage: u64) -> Self {
        Self {
            moveable_object,
            radius,
            health,
            damage,
        }
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

    pub fn set_position(&mut self, value: Vector2) {
        self.moveable_object.set_position(value);
    }

    pub fn set_velocity(&mut self, value: Vector2) {
        self.moveable_object.set_velocity(value);
    }

    pub fn set_orientation(&mut self, value: Vector2) {
        self.moveable_object.set_orientation(value);
    }

    pub fn set_rotational_velocity(&mut self, value: f64) {
        self.moveable_object.set_rotational_velocity(value);
    }
}

impl Collides for PhysicalObject {
    fn distance_squared_to(&self, position: Vector2) -> f64 {
        self.moveable_object.distance_squared_to(position)
    }

    fn within_range_of(&self, position: Vector2, range: f64) -> bool {
        self.distance_squared_to(position) > ((self.radius + range) * (self.radius + range))
    }

    fn collides_with(&self, other: &dyn Collides) -> bool {
        other.within_range_of(self.moveable_object.get_position(), self.radius)
    }

    fn collide_with(&self, other: &mut dyn Collides, world_object: &mut WorldObject) {
        other.apply_damage(self.damage, world_object);
    }

    fn apply_damage(&mut self, amount: u64, world_object: &mut WorldObject) {
        self.health = max(self.health - amount, 0);
        if self.health == 0 {
            world_object.destroy();
        }
    }
}
