use crate::timer::Timer;

pub struct CoolDown {
    cool_down: f64,
    last_shot: f64,
}

impl CoolDown {
    pub fn new(cool_down: f64) -> Self {
        CoolDown {
            cool_down,
            last_shot: 0.0,
        }
    }

    pub fn shoot(&mut self, timer: &Timer) {
        self.last_shot = timer.last_frame_at.elapsed().as_secs_f64();
    }

    pub fn can_shoot(&self) -> bool {
        self.last_shot >= self.cool_down
    }
}
