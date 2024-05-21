use crate::moveable_object::MoveableObject;
use crate::timer::Timer;
use crate::vector::Vector2;
use crate::world_object::WorldObject;

pub trait Projectile {
    fn update(&mut self, timer: &Timer);
}

pub struct Laser {
    id: u64,
    should_be_destroyed: bool,
    object: MoveableObject,
}

impl Laser {
    pub fn new(id: u64, position: Vector2, velocity: Vector2) -> Self {
        Self {
            id,
            should_be_destroyed: false,
            object: MoveableObject::new(position, velocity, velocity.normalize(), 0.0),
        }
    }
}

impl WorldObject for Laser {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn destroy(&mut self) {
        self.should_be_destroyed = true;
    }

    fn should_be_destroyed(&self) -> bool {
        self.should_be_destroyed
    }
}

impl Projectile for Laser {
    fn update(&mut self, timer: &Timer) {
        self.object.update(timer);
    }
}
