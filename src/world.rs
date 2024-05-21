use crate::asteroid::{Asteroid, AsteroidSize};
use crate::asteroid_factory::AsteroidFactory;
use crate::collides::Collides;
use crate::config::Config;
use crate::id_generator::IdGenerator;
use crate::player::Player;
use crate::position_generator::PositionGenerator;
use crate::projectile::Projectile;
use crate::projectile_factory::ProjectileFactory;
use crate::ship::Ship;
use crate::timer::Timer;
use crate::vector::Vector2;

pub struct World {
    width: u32,
    height: u32,
    config: Config,
    id_generator: IdGenerator,
    position_generator: PositionGenerator,
    asteroid_factory: AsteroidFactory,
    asteroids: Vec<Box<Asteroid>>,
    ships: Vec<Box<Ship>>,
    projectiles: Vec<Box<dyn Projectile>>,
}

impl World {
    pub fn new(config: Config) -> Self {
        let mut id_generator = IdGenerator::new();
        let position_generator = PositionGenerator::new(config.world_width, config.world_height);
        let asteroid_factory = AsteroidFactory::new(&config);
        let mut asteroids: Vec<Box<Asteroid>> = vec![];
        for _ in 0..config.world_min_obstacles {
            asteroids.push(asteroid_factory.create_at(
                id_generator.get_next_id(),
                AsteroidSize::Large,
                position_generator.random_position(),
            ));
        }

        Self {
            width: config.world_width,
            height: config.world_height,
            config,
            id_generator,
            position_generator,
            asteroid_factory,
            asteroids,
            ships: vec![],
            projectiles: vec![],
        }
    }

    fn remove_deleted(&mut self) {
        self.asteroids.retain(|a| !a.is_deleted());
        self.projectiles.retain(|a| !a.is_deleted());
    }

    fn update_all(&mut self, timer: &Timer) {
        self.update_asteroids(timer);
        self.update_projectiles(timer);
        self.update_ships(timer);
    }

    fn update_ships(&mut self, timer: &Timer) {
        let projectile_factory = ProjectileFactory::new(&mut self.id_generator);
        for ship in &mut self.ships {
            let new_projectiles = ship.update(timer, &projectile_factory);
            self.projectiles.append(new_projectiles);
            for projectile in new_projectiles {
                self.projectiles.push(WorldProjectile::new(
                    self.id_generator.get_next_id(),
                    projectile,
                ));
            }
        }
    }

    fn update_projectiles(&mut self, timer: &Timer) {
        for projectile in &mut self.projectiles {
            projectile.update(timer);
        }
    }
    fn update_asteroids(&mut self, timer: &Timer) {
        for asteroid in &mut self.asteroids {
            asteroid.update(timer);
        }
    }

    fn top_up_asteroids(&mut self) {
        let count = self.asteroids.len();
        for _ in count..self.config.world_min_obstacles {
            self.asteroids.push(self.asteroid_factory.create_at(
                self.id_generator.get_next_id(),
                AsteroidSize::Large,
                self.position_generator.random_position(),
            ));
        }
    }

    fn safe_respawn_position(&self) -> Vector2 {
        let collidable_objects = &self.asteroids;
        loop {
            let position = self.position_generator.random_position();
            for o in collidable_objects {
                if o.within_range_of(position, self.config.safe_respawn_distance.into()) {
                    return position;
                }
            }
        }
    }

    pub fn create_ship(&mut self, player: Box<Player>) {
        let position = self.safe_respawn_position();
        self.ships.push(Box::new(Ship::new(
            self.id_generator.get_next_id(),
            &self.config,
            position,
            player,
        )))
    }

    fn test_ship_collisions(&mut self) {
        let mut collisions = vec![];
        for ship in &self.ships {
            for other in &self.ships {
                if other.get_id() != ship.get_id() {
                    if ship.collides_with(other.as_ref()) {
                        collisions.push((ship, other));
                    }
                }
            }
        }
        for (ship, other) in &mut collisions {
            // ship.collide_with(other);
            // other.collide_with(other);
        }
    }

    pub fn update(&mut self, timer: &Timer) {
        self.remove_deleted();
        self.update_all(timer);
        self.test_ship_collisions();
        self.top_up_asteroids();
    }
}
