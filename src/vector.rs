use std::fmt;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub const UP: Vector2 = Vector2 { x: 0.0, y: 1.0 };
    pub const DOWN: Vector2 = Vector2 { x: 0.0, y: -1.0 };
    pub const LEFT: Vector2 = Vector2 { x: 1.0, y: 0.0 };
    pub const RIGHT: Vector2 = Vector2 { x: -1.0, y: 0.0 };
    pub const EMPTY: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    pub fn rotate(&self, r: f64) -> Vector2 {
        let sin_r = r.sin();
        let cos_r = r.cos();
        Vector2 {
            x: self.x * cos_r - self.y * sin_r,
            y: self.x * sin_r + self.y * cos_r,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vector2 {
        let length = self.length();
        Vector2 {
            x: self.x / length,
            y: self.y / length,
        }
    }

    pub fn distance_squared_to(&self, position: Vector2) -> f64 {
        (position.x - self.x) * (position.x - self.x)
            + (position.y - self.y) * (position.y - self.y)
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
