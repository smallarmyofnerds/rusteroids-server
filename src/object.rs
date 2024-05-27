use std::cell::RefCell;

use crate::{timer::Timer, vector::Vector2};

#[derive(PartialEq)]
pub enum ObjectType {
    Ship,
    Asteroid,
    Projectile,
}

pub trait Object {
    fn is_flagged_for_destruction(&self) -> bool;
    fn update(&mut self, timer: &Timer) -> Vec<Box<dyn Object>>;
    fn get_type(&self) -> ObjectType;
    fn within_range_of(&self, position: Vector2, range: f64) -> bool;
    fn distance_squared_to(&self, position: Vector2) -> f64;
    fn collides_with(&self, other: &Box<dyn Object>) -> bool;
    fn collide_with(&self, other: &mut Box<dyn Object>);
    fn apply_damage(&mut self, amount: u64);
}
