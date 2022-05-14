pub struct Config {
    pub world_width: u32,
    pub world_height: u32,
    pub world_min_obstacles: usize,
    pub safe_respawn_distance: u32,
    pub asteroid_min_speed: f64,
    pub asteroid_max_speed: f64,
}

impl Config {
    pub fn new() -> Self {
        Self {
            world_width: 5000,
            world_height: 5000,
            world_min_obstacles: 30,
            safe_respawn_distance: 300,
            asteroid_min_speed: 50.0,
            asteroid_max_speed: 100.0,
        }
    }
}
