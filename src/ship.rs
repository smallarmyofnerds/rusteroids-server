use crate::config::Config;
use crate::inputs::Inputs;
use crate::player::Player;
use crate::projectile::{Laser, Projectile};
use crate::vector::Vector2;

pub struct Ship {
    pub position: Vector2,
    velocity: Vector2,
    orientation: Vector2,
    rotational_velocity: f64,
    player: Box<Player>,
    engine_on: bool,
    acceleration_rate: u32,
    linear_friction: f64,
    rotational_acceleration_rate: u32,
    rotational_velocity_friction: f64,
}

impl Ship {
    pub fn new(config: &Config, position: Vector2, player: Box<Player>) -> Self {
        Ship {
            position,
            velocity: Vector2::EMPTY,
            orientation: Vector2::UP,
            rotational_velocity: 0.0,
            player,
            engine_on: false,
            acceleration_rate: config.ship_linear_acceleration,
            linear_friction: config.ship_linear_friction,
            rotational_acceleration_rate: config.ship_angular_acceleration,
            rotational_velocity_friction: config.ship_angular_friction,
        }
    }

    fn get_rotation(&self, rotating_left: bool, rotating_right: bool) -> f64 {
        if rotating_left {
            if rotating_right {
                -1.0 * self.rotational_velocity_friction * self.rotational_velocity
            } else {
                -1.0 * self.rotational_acceleration_rate as f64
            }
        } else {
            if rotating_right {
                self.rotational_acceleration_rate as f64
            } else {
                -1.0 * self.rotational_velocity_friction * self.rotational_velocity
            }
        }
    }

    fn get_linear(&self, accelerating: bool) -> Vector2 {
        if accelerating {
            self.orientation.normalize() * self.acceleration_rate as f64
        } else {
            -1.0 * self.linear_friction * self.velocity
        }
    }

    fn shoot(&mut self) -> Vec<Box<dyn Projectile>> {
        let action = Laser::new(self.position);
        let boxed_action = Box::new(action);
        vec![boxed_action]
    }

    pub fn update(&mut self, t: f64) -> Vec<Box<dyn Projectile>> {
        let inputs = self.player.get_inputs();

        self.engine_on = inputs.up;

        let acceleration = self.get_linear(inputs.up);
        self.velocity = self.velocity + 0.5 * acceleration;
        if self.velocity.length() < 0.5 {
            self.velocity = Vector2::EMPTY;
        }

        let rotational_acceleration = self.get_rotation(inputs.left, inputs.right);
        self.rotational_velocity += rotational_acceleration;
        if self.rotational_velocity.abs() < 0.001 {
            self.rotational_velocity = 0.0;
        }
        self.rotational_velocity = self.rotational_velocity.clamp(-190.0, 190.0);

        self.position += self.velocity * t;
        self.orientation = self.orientation.rotate(self.rotational_velocity * t);

        if inputs.fire {
            self.shoot()
        } else {
            vec![]
        }
    }
}
