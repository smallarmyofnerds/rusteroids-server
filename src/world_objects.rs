use crate::asteroid::Asteroid;
use crate::vector::Vector2;

pub trait Collide {
  fn distance_squared_to(&self, position: Vector2) -> f64;
}

pub struct WorldAsteroid {
  id: u64,
  pub deleted: bool,
  asteroid: Asteroid,
}

impl WorldAsteroid {
  pub fn new(id: u64, asteroid: Asteroid) -> Self {
    Self {
      id,
      deleted: false,
      asteroid,
    }
  }

  pub fn update(&mut self, delta: f64) {
    self.asteroid.update(delta);
  }
}

impl Collide for WorldAsteroid {
  fn distance_squared_to(&self, position: Vector2) -> f64 {
    self.asteroid.position.distance_squared_to(position)
  }
}
