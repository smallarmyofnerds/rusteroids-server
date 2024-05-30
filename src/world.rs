use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::asteroid::AsteroidSize;
use crate::asteroid_factory::AsteroidFactory;
use crate::config::Config;
use crate::object::ObjectType;
use crate::object_db::ObjectDb;
use crate::player::Player;
use crate::position_generator::PositionGenerator;
use crate::ship::Ship;
use crate::timer::Timer;
use crate::vector::Vector2;

pub struct World<'a> {
    _width: u32,
    _height: u32,
    config: Config,
    position_generator: &'a PositionGenerator,
    asteroid_factory: Rc<AsteroidFactory>,
    object_db: ObjectDb,
}

impl<'a> World<'a> {
    pub fn new(
        config: Config,
        position_generator: &'a PositionGenerator,
        asteroid_factory: Rc<AsteroidFactory>,
    ) -> Self {
        Self {
            _width: config.world_width,
            _height: config.world_height,
            config: config.clone(),
            position_generator,
            asteroid_factory,
            object_db: ObjectDb::new(),
        }
    }

    fn remove_deleted(&mut self) {
        self.object_db.remove_deleted();
    }

    fn update_all(&mut self, timer: &Timer) {
        let mut new_objects = vec![];
        for o in self.object_db.all() {
            new_objects.append(&mut o.get_object().borrow_mut().update(timer))
        }
        for o in new_objects {
            self.object_db.add(o);
        }
    }

    fn top_up_asteroids(&mut self) {
        let count = self
            .object_db
            .count_where(|o| o.get_object().borrow().get_type() == ObjectType::Asteroid);
        let to_generate = self.config.world_min_obstacles - count;
        if to_generate > 0 {
            println!("Generating {:?} asteroids...", to_generate);
        }
        for _ in 0..to_generate {
            self.object_db.add(self.asteroid_factory.create_at(
                AsteroidSize::Large,
                self.position_generator.random_position(),
                self.asteroid_factory.clone(),
            ));
        }
    }

    fn safe_respawn_position(&self) -> Vector2 {
        loop {
            let position = self.position_generator.random_position();
            let mut objects_checked = 0;
            for o in self.object_db.all_where(|o| {
                let object_type = o.get_object().borrow().get_type();
                object_type == ObjectType::Ship
                    || object_type == ObjectType::Asteroid
                    || object_type == ObjectType::Projectile
            }) {
                objects_checked += 1;
                if !o
                    .get_object()
                    .borrow()
                    .within_range_of(position, self.config.safe_respawn_distance.into())
                {
                    return position;
                }
            }
            if objects_checked == 0 {
                return position;
            }
        }
    }

    pub fn create_ship(&mut self, player: Box<Player>) {
        let position = self.safe_respawn_position();
        self.object_db
            .add(Box::new(Ship::new(&self.config, position, player)))
    }

    fn test_collisions(&mut self) {
        let mut collisions = vec![];
        let mut collisions_by_source: HashMap<u64, HashSet<u64>> = HashMap::new();
        for source in self.object_db.all() {
            for target in self.object_db.all_where(|o| o.get_id() != source.get_id()) {
                let done = match collisions_by_source.get(&source.get_id()) {
                    Some(s) => s.contains(&target.get_id()),
                    None => false,
                };
                if !done
                    && source
                        .get_object()
                        .borrow()
                        .collides_with(&target.get_object().borrow())
                {
                    collisions.push((source.get_id(), target.get_id()));
                    // match collisions_by_source.get_mut(&source.get_id()) {
                    //     Some(s) => {
                    //         s.insert(target.get_id());
                    //     }
                    //     None => {
                    //         collisions_by_source
                    //             .insert(source.get_id(), HashSet::from_iter(vec![target.get_id()]));
                    //     }
                    // }
                    match collisions_by_source.get_mut(&target.get_id()) {
                        Some(s) => {
                            s.insert(source.get_id());
                        }
                        None => {
                            collisions_by_source
                                .insert(target.get_id(), HashSet::from_iter(vec![source.get_id()]));
                        }
                    }
                }
            }
        }
        for (source_id, target_id) in collisions {
            let ship = self.object_db.get(source_id).get_object().borrow();
            let mut other = self.object_db.get(target_id).get_object().borrow_mut();
            ship.collide_with(&mut other);
        }
        for o in self
            .object_db
            .all_where(|o| o.get_object().borrow().is_flagged_for_destruction())
        {
            o.get_object().borrow_mut().destroy();
        }
    }

    pub fn update(&mut self, timer: &Timer) {
        self.remove_deleted();
        self.update_all(timer);
        self.test_collisions();
        self.top_up_asteroids();
    }
}
