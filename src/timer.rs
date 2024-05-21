use std::{thread::sleep, time::Duration};

pub struct Timer {
    frame_duration: std::time::Duration,
    pub last_frame_at: std::time::Instant,
    pub delta: std::time::Duration,
}

impl Timer {
    pub fn new(fps: f64) -> Self {
        Self {
            frame_duration: Duration::from_secs_f64(1.0 / fps),
            last_frame_at: std::time::Instant::now(),
            delta: Duration::from_secs(0),
        }
    }

    pub fn tick(&mut self) {
        let sleep_until = self.last_frame_at + self.frame_duration;
        let mut now = std::time::Instant::now();
        while now < sleep_until {
            sleep(Duration::from_secs(0));
            now = std::time::Instant::now();
        }
        self.delta = now - self.last_frame_at;
        self.last_frame_at = now;
    }
}
