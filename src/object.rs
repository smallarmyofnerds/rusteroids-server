use crate::{ship::Ship, timer::Timer, vector::Vector2};

#[derive(PartialEq)]
pub enum ObjectType {
    Ship,
    Asteroid,
    Projectile,
    PowerUp,
}

pub trait Object {
    fn is_flagged_for_destruction(&self) -> bool;
    fn update(&mut self, timer: &Timer) -> Vec<Box<dyn Object>>;
    fn get_type(&self) -> ObjectType;
    fn within_range_of(&self, position: Vector2, range: f64) -> bool;
    fn _distance_squared_to(&self, position: Vector2) -> f64;
    fn collides_with(&self, other: &Box<dyn Object>) -> bool;
    fn collide_with(&self, other: &mut Box<dyn Object>);
    fn apply_damage(&mut self, amount: u64);
    fn destroy(&self) -> Vec<Box<dyn Object>>;
    fn as_ship(&self) -> Result<&Ship, &'static str>;
    fn as_ship_mut(&mut self) -> Result<&mut Ship, &'static str>;
}
