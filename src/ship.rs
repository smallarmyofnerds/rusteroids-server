use crate::armoury::{Armoury, WeaponType};
use crate::config::Config;
use crate::moveable_object::MoveableObject;
use crate::object::{Object, ObjectType};
use crate::physical_object::PhysicalObject;
use crate::player::Player;
use crate::projectile_factory::ProjectileFactory;
use crate::timer::Timer;
use crate::vector::Vector2;

pub struct Ship {
    is_flagged_for_destruction: bool,
    physical_object: PhysicalObject,
    player: Box<Player>,
    projectile_factory: ProjectileFactory,
    engine_on: bool,
    acceleration_rate: u32,
    linear_friction: f64,
    rotational_acceleration_rate: u32,
    rotational_velocity_friction: f64,
    armoury: Box<Armoury>,
    active_weapon_type: WeaponType,
}

impl Ship {
    pub fn new(config: &Config, position: Vector2, player: Box<Player>) -> Self {
        Ship {
            is_flagged_for_destruction: false,
            physical_object: PhysicalObject::new(
                MoveableObject::new(position, Vector2::EMPTY, Vector2::UP, 0.0),
                40.0,
                1000,
                1000,
            ),
            player,
            projectile_factory: ProjectileFactory::new(),
            engine_on: false,
            acceleration_rate: config.ship_linear_acceleration,
            linear_friction: config.ship_linear_friction,
            rotational_acceleration_rate: config.ship_angular_acceleration,
            rotational_velocity_friction: config.ship_angular_friction,
            armoury: Box::new(Armoury::new(config)),
            active_weapon_type: WeaponType::LaserCanon,
        }
    }

    fn set_active_weapon(&mut self, weapon_type: WeaponType) {
        self.active_weapon_type = weapon_type;
    }

    fn get_rotation(&self, rotating_left: bool, rotating_right: bool) -> f64 {
        if rotating_left {
            if rotating_right {
                -1.0 * self.rotational_velocity_friction
                    * self.physical_object.get_rotational_velocity()
            } else {
                -1.0 * self.rotational_acceleration_rate as f64
            }
        } else {
            if rotating_right {
                self.rotational_acceleration_rate as f64
            } else {
                -1.0 * self.rotational_velocity_friction
                    * self.physical_object.get_rotational_velocity()
            }
        }
    }

    fn get_linear(&self, accelerating: bool) -> Vector2 {
        if accelerating {
            self.physical_object.get_orientation().normalize() * self.acceleration_rate as f64
        } else {
            -1.0 * self.linear_friction * self.physical_object.get_velocity()
        }
    }

    // pub fn get_position(&self) -> Vector2 {
    //     self.object.position
    // }

    // pub fn get_orientation(&self) -> Vector2 {
    //     self.object.orientation
    // }

    fn shoot(&mut self, timer: &Timer) -> Vec<Box<dyn Object>> {
        self.armoury.get(self.active_weapon_type).shoot_from(
            timer,
            &self.projectile_factory,
            self.physical_object.get_position(),
            self.physical_object.get_orientation(),
        )
    }
}

impl Object for Ship {
    fn is_flagged_for_destruction(&self) -> bool {
        self.physical_object.is_flagged_for_destruction()
    }

    fn update(&mut self, timer: &Timer) -> Vec<Box<dyn Object>> {
        let inputs = self.player.get_inputs();

        self.engine_on = inputs.up;

        let acceleration = self.get_linear(inputs.up);
        self.physical_object
            .set_velocity(self.physical_object.get_velocity() + 0.5 * acceleration);
        if self.physical_object.get_velocity().length() < 0.5 {
            self.physical_object.set_velocity(Vector2::EMPTY);
        }

        let rotational_acceleration = self.get_rotation(inputs.left, inputs.right);
        self.physical_object.set_rotational_velocity(
            self.physical_object.get_rotational_velocity() + rotational_acceleration,
        );
        if self.physical_object.get_rotational_velocity().abs() < 0.001 {
            self.physical_object.set_rotational_velocity(0.0);
        }
        self.physical_object.set_rotational_velocity(
            self.physical_object
                .get_rotational_velocity()
                .clamp(-190.0, 190.0),
        );

        self.physical_object.update(timer);

        if inputs.fire {
            self.shoot(timer)
        } else {
            vec![]
        }
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Ship
    }

    fn within_range_of(&self, position: Vector2, range: f64) -> bool {
        self.physical_object.within_range_of(position, range)
    }

    fn distance_squared_to(&self, position: Vector2) -> f64 {
        self.physical_object.distance_squared_to(position)
    }

    fn collides_with(&self, other: &Box<dyn Object>) -> bool {
        self.physical_object.collides_with(other)
    }

    fn collide_with(&self, other: &mut Box<dyn Object>) {
        self.physical_object.collide_with(other);
    }

    fn apply_damage(&mut self, amount: u64) {
        self.physical_object.apply_damage(amount);
    }
}
