#![allow(dead_code)]

/// Represents a tickable timer.
#[derive(Debug)]
pub struct Timer {
    /// The cooldown of the timer.
    pub cooldown: f32,
    /// The current time of the timer.
    pub current_time: f32,
}

impl Timer {
    /// Creates a new timer
    pub fn new(cooldown: f32) -> Self {
        Self {
            cooldown,
            current_time: cooldown,
        }
    }

    /// Returns true if the timer is complete.
    pub fn is_complete(&self) -> bool {
        self.current_time >= self.cooldown
    }

    /// Resets the timer.
    pub fn reset(&mut self) {
        self.current_time = 0.0;
    }

    /// Ticks the timer.
    pub fn tick(&mut self, delta_time: f32) {
        self.current_time += delta_time;
    }

    /// Returns the amount of time (in millis) elapsed since the last tick.
    pub fn elapsed(&mut self) -> f32 {
        let elapsed = self.current_time;
        self.current_time = 0.0;

        elapsed
    }
}
