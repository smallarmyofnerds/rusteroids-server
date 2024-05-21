use crate::collides::Collides;
use crate::moveable_object::MoveableObject;
use crate::physical_object::PhysicalObject;
use crate::timer::Timer;
use crate::vector::Vector2;
use crate::world_object::WorldObject;

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
    world_object: WorldObject,
    physical_object: PhysicalObject,
}

impl Asteroid {
    pub fn new(
        id: u64,
        position: Vector2,
        velocity: Vector2,
        orientation: Vector2,
        rotational_velocity: f64,
        descriptor: AsteroidDescriptor,
    ) -> Self {
        Self {
            world_object: WorldObject::new(id),
            physical_object: PhysicalObject::new(
                MoveableObject::new(position, velocity, orientation, rotational_velocity),
                descriptor.radius,
                descriptor.health,
                descriptor.damage,
            ),
        }
    }

    pub fn is_deleted(&self) -> bool {
        self.world_object.deleted
    }
    pub fn update(&mut self, timer: &Timer) {
        self.physical_object.update(timer);
    }
}

impl Collides for Asteroid {
    fn distance_squared_to(&self, position: Vector2) -> f64 {
        self.physical_object.distance_squared_to(position)
    }

    fn within_range_of(&self, position: Vector2, range: f64) -> bool {
        self.physical_object.within_range_of(position, range)
    }

    fn collides_with(&self, other: &dyn Collides) -> bool {
        self.physical_object.collides_with(other)
    }

    fn collide_with(&self, other: &mut dyn Collides, world_object: &mut WorldObject) {
        self.physical_object.collide_with(other, world_object);
    }

    fn apply_damage(&mut self, amount: u64, world_object: &mut WorldObject) {
        self.physical_object.apply_damage(amount, world_object)
    }
}
