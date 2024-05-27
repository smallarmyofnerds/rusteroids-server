use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;

use crate::asteroid::AsteroidSize;
use crate::asteroid_factory::AsteroidFactory;
use crate::config::Config;
use crate::object::{Object, ObjectType};
use crate::object_db::ObjectDb;
use crate::player::Player;
use crate::position_generator::PositionGenerator;
use crate::ship::Ship;
use crate::timer::Timer;
use crate::vector::Vector2;

pub struct World {
    width: u32,
    height: u32,
    config: Config,
    position_generator: PositionGenerator,
    asteroid_factory: AsteroidFactory,
    object_db: ObjectDb,
}

impl World {
    pub fn new(config: Config) -> Self {
        Self {
            width: config.world_width,
            height: config.world_height,
            config: config.clone(),
            position_generator: PositionGenerator::new(config.world_width, config.world_height),
            asteroid_factory: AsteroidFactory::new(&config),
            object_db: ObjectDb::new(),
        }
    }

    fn remove_deleted(&mut self) {
        self.object_db.remove_deleted();
    }

    fn update_all(&mut self, timer: &Timer) {
        let mut new_objects = vec![];
        for o in self.object_db.all() {
            let object = o.get_object();
            let mut wat = object.borrow_mut().update(timer);
            // let results = wat.update(timer);
            new_objects.append(&mut wat)
        }
        for o in new_objects {
            self.object_db.add(o);
        }
    }

    fn top_up_asteroids(&mut self) {
        let count = self
            .object_db
            .count_where(|o| o.get_object().borrow().get_type() == ObjectType::Asteroid);
        for _ in count..self.config.world_min_obstacles {
            self.object_db.add(self.asteroid_factory.create_at(
                AsteroidSize::Large,
                self.position_generator.random_position(),
            ));
        }
    }

    fn safe_respawn_position(&self) -> Vector2 {
        loop {
            let position = self.position_generator.random_position();
            for o in self.object_db.all_where(|o| {
                let object_type = o.get_object().borrow().get_type();
                object_type == ObjectType::Ship
                    || object_type == ObjectType::Asteroid
                    || object_type == ObjectType::Projectile
            }) {
                if o.get_object()
                    .borrow()
                    .within_range_of(position, self.config.safe_respawn_distance.into())
                {
                    return position;
                }
            }
        }
    }

    pub fn create_ship(&mut self, player: Box<Player>) {
        let position = self.safe_respawn_position();
        self.object_db
            .add(Box::new(Ship::new(&self.config, position, player)))
    }

    fn test_ship_collisions(&mut self) {
        let mut collisions = vec![];
        for ship in self
            .object_db
            .all_where(|o| o.get_object().borrow().get_type() == ObjectType::Ship)
        {
            for other in self.object_db.all_where(|o| {
                o.get_object().borrow().get_type() == ObjectType::Ship
                    && o.get_id() != ship.get_id()
            }) {
                if ship
                    .get_object()
                    .borrow()
                    .collides_with(&other.get_object().borrow())
                {
                    collisions.push((ship.get_id(), other.get_id()));
                    // ship.get_object_mut().collide_with(other.get_object_mut());
                }
            }
        }
        for (shipId, otherId) in collisions {
            let ship = self.object_db.get(shipId).get_object().borrow();
            let mut other = self.object_db.get(otherId).get_object().borrow_mut();
            ship.collide_with(&mut other);
            //     ship.get_object_mut().collide_with(other.get_object_mut());
            //     // other.get_object_mut().collide_with(ship.get_object_mut());
        }
    }

    pub fn update(&mut self, timer: &Timer) {
        self.remove_deleted();
        self.update_all(timer);
        self.test_ship_collisions();
        self.top_up_asteroids();
    }
}
