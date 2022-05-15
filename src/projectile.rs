use crate::vector::Vector2;

pub trait Projectile {
    fn update(&mut self, delta: f64);
}

pub struct Laser {
    position: Vector2,
}

impl Laser {
    pub fn new(position: Vector2) -> Self {
        Self { position }
    }
}

impl Projectile for Laser {
    fn update(&mut self, delta: f64) {
        self.position += Vector2::UP;
    }
}
