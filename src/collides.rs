use crate::{vector::Vector2, world_object::WorldObject};

pub trait Collides {
    fn within_range_of(&self, position: Vector2, range: f64) -> bool;
    fn distance_squared_to(&self, position: Vector2) -> f64;
    fn collides_with(&self, other: &dyn Collides) -> bool;
    fn collide_with(&self, other: &mut dyn Collides, world_object: &mut WorldObject);
    fn apply_damage(&mut self, amount: u64, world_object: &mut WorldObject);
}
