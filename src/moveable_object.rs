use crate::{timer::Timer, vector::Vector2};

pub struct MoveableObject {
    position: Vector2,
    velocity: Vector2,
    orientation: Vector2,
    rotational_velocity: f64,
}

impl MoveableObject {
    pub fn new(
        position: Vector2,
        velocity: Vector2,
        orientation: Vector2,
        rotational_velocity: f64,
    ) -> Self {
        Self {
            position,
            velocity,
            orientation,
            rotational_velocity,
        }
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn get_velocity(&self) -> Vector2 {
        self.velocity
    }

    pub fn get_orientation(&self) -> Vector2 {
        self.orientation
    }

    pub fn get_rotational_velocity(&self) -> f64 {
        self.rotational_velocity
    }

    pub fn set_position(&mut self, value: Vector2) {
        self.position = value;
    }

    pub fn set_velocity(&mut self, value: Vector2) {
        self.velocity = value;
    }

    pub fn set_orientation(&mut self, value: Vector2) {
        self.orientation = value
    }

    pub fn set_rotational_velocity(&mut self, value: f64) {
        self.rotational_velocity = value
    }

    pub fn update(&mut self, timer: &Timer) {
        self.position += timer.delta.as_secs_f64() * self.velocity;
        self.orientation = self
            .orientation
            .rotate(timer.delta.as_secs_f64() * self.rotational_velocity);
    }

    pub fn distance_squared_to(&self, position: Vector2) -> f64 {
        self.position.distance_squared_to(position)
    }
}
