use crate::moveable_object::MoveableObject;
use crate::object::{Object, ObjectType};
use crate::physical_object::PhysicalObject;
use crate::timer::Timer;
use crate::vector::Vector2;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum AsteroidSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Copy)]
pub struct AsteroidDescriptor {
    pub size: AsteroidSize,
    pub radius: f64,
    pub health: u64,
    pub damage: u64,
}

pub struct Asteroid {
    physical_object: PhysicalObject,
}

impl Asteroid {
    pub fn new(
        position: Vector2,
        velocity: Vector2,
        orientation: Vector2,
        rotational_velocity: f64,
        descriptor: AsteroidDescriptor,
    ) -> Self {
        Self {
            physical_object: PhysicalObject::new(
                MoveableObject::new(position, velocity, orientation, rotational_velocity),
                descriptor.radius,
                descriptor.health,
                descriptor.damage,
            ),
        }
    }
}

impl Object for Asteroid {
    fn is_flagged_for_destruction(&self) -> bool {
        self.physical_object.is_flagged_for_destruction()
    }

    fn get_type(&self) -> crate::object::ObjectType {
        ObjectType::Asteroid
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
        self.physical_object.collides_with(other)
    }

    fn collide_with(&self, other: &mut Box<dyn Object>) {
        self.physical_object.collide_with(other);
    }

    fn apply_damage(&mut self, amount: u64) {
        self.physical_object.apply_damage(amount)
    }
}
