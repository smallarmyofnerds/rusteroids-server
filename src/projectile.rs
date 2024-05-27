use crate::moveable_object::MoveableObject;
use crate::object::{Object, ObjectType};
use crate::physical_object::PhysicalObject;
use crate::timer::Timer;
use crate::vector::Vector2;

pub struct Laser {
    physical_object: PhysicalObject,
}

impl Laser {
    pub fn new(position: Vector2, velocity: Vector2) -> Self {
        Self {
            physical_object: PhysicalObject::new(
                MoveableObject::new(position, velocity, velocity.normalize(), 0.0),
                100.0,
                100,
                100,
            ),
        }
    }
}

impl Object for Laser {
    fn is_flagged_for_destruction(&self) -> bool {
        self.physical_object.is_flagged_for_destruction()
    }

    fn update(&mut self, timer: &Timer) -> Vec<Box<dyn Object>> {
        self.physical_object.update(timer);
        vec![]
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Projectile
    }

    fn within_range_of(&self, position: Vector2, range: f64) -> bool {
        self.physical_object.within_range_of(position, range)
    }

    fn distance_squared_to(&self, position: Vector2) -> f64 {
        self.physical_object.distance_squared_to(position)
    }

    fn collides_with(&self, other: &Box<dyn Object>) -> bool {
        self.physical_object.collides_with(other)
    }

    fn collide_with(&self, other: &mut Box<dyn Object>) {
        self.physical_object.collide_with(other)
    }

    fn apply_damage(&mut self, amount: u64) {
        self.physical_object.apply_damage(amount)
    }
}
