use crate::vector::Vector2;
use rand::Rng;

pub enum AsteroidSize {
    Small,
    Medium,
    Large,
}

pub struct Asteroid {
    pub position: Vector2,
    velocity: Vector2,
    orientation: Vector2,
    rotational_velocity: f64,
    size: AsteroidSize,
}

impl Asteroid {
    pub fn new(
        position: Vector2,
        velocity: Vector2,
        orientation: Vector2,
        rotational_velocity: f64,
        size: AsteroidSize,
    ) -> Self {
        Self {
            position,
            velocity,
            orientation,
            rotational_velocity,
            size,
        }
    }

    pub fn update(&mut self, t: f64) {
        self.position += self.velocity * t;
        self.orientation = self.orientation.rotate(self.rotational_velocity * t);
    }
}
