use crate::asteroid::AsteroidSize;
use crate::asteroid_factory::AsteroidFactory;
use crate::config::Config;
use crate::id_generator::IdGenerator;
use crate::vector::Vector2;
use crate::world_objects::{Collide, WorldAsteroid};
use rand::Rng;

pub struct World {
    width: u32,
    height: u32,
    config: Config,
    id_generator: IdGenerator,
    position_generator: PositionGenerator,
    asteroid_factory: AsteroidFactory,
    asteroids: Vec<WorldAsteroid>,
}

pub struct PositionGenerator {
    max_width: u32,
    max_height: u32,
}

impl PositionGenerator {
    pub fn new(max_width: u32, max_height: u32) -> Self {
        PositionGenerator {
            max_width,
            max_height,
        }
    }

    pub fn random_position(&self) -> Vector2 {
        Vector2 {
            x: rand::thread_rng().gen::<f64>() * self.max_width as f64,
            y: rand::thread_rng().gen::<f64>() * self.max_height as f64,
        }
    }
}

impl World {
    pub fn new(config: Config) -> Self {
        let mut id_generator = IdGenerator::new();
        let position_generator = PositionGenerator::new(config.world_width, config.world_height);
        let asteroid_factory = AsteroidFactory::new(&config);
        let mut asteroids: Vec<WorldAsteroid> = vec![];
        for _ in 0..config.world_min_obstacles {
            asteroids.push(WorldAsteroid::new(
                id_generator.get_next_id(),
                asteroid_factory
                    .create_at(AsteroidSize::Large, position_generator.random_position()),
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
        }
    }

    fn remove_deleted(&mut self) {
        self.asteroids.retain(|a| !a.deleted);
    }

    fn update_all(&mut self, delta: f64) {
        for asteroid in &mut self.asteroids {
            asteroid.update(delta);
        }
    }

    fn top_up_asteroids(&mut self) {
        let count = self.asteroids.len();
        for _ in count..self.config.world_min_obstacles {
            self.asteroids.push(WorldAsteroid::new(
                self.id_generator.get_next_id(),
                self.asteroid_factory.create_at(
                    AsteroidSize::Large,
                    self.position_generator.random_position(),
                ),
            ));
        }
    }

    fn safe_respawn_position(&self) -> Vector2 {
        let collidable_objects = &self.asteroids;
        loop {
            let position = self.position_generator.random_position();
            for o in collidable_objects {
                if o.distance_squared_to(position)
                    > (self.config.safe_respawn_distance as f64
                        * self.config.safe_respawn_distance as f64)
                {
                    return position;
                }
            }
        }
    }

    pub fn create_ship(&mut self) {
        let position = self.safe_respawn_position();
    }

    pub fn update(&mut self, delta: f64) {
        self.remove_deleted();
        self.update_all(delta);
        self.top_up_asteroids();
    }
}
