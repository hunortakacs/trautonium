//! Attack/Release envelope generator

use crate::config::AUDIO_SAMPLE_RATE;
use crate::config::{PRESSURE_GATE_OFF, PRESSURE_GATE_ON};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvState {
    Attack,
    Sustain,
    Release,
    Idle,
}

pub struct Envelope {
    attack_rate: f32,
    release_rate: f32,
    level: f32,
    state: EnvState,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            attack_rate: 0.01,  // Default 10ms attack
            release_rate: 0.01, // Default 10ms release
            level: 0.0,
            state: EnvState::Idle,
        }
    }

    pub fn set_attack(&mut self, time_seconds: f32) {
        // Calculate increment per sample to reach 1.0 in given time
        self.attack_rate = 1.0 / (time_seconds * AUDIO_SAMPLE_RATE as f32);
    }

    pub fn set_release(&mut self, time_seconds: f32) {
        self.release_rate = 1.0 / (time_seconds * AUDIO_SAMPLE_RATE as f32);
    }

    pub fn process(&mut self, pressure: f32) -> f32 {
        let pressure = pressure.clamp(0.0, 1.0);
        let was_open = !matches!(self.state, EnvState::Idle);

        let gate_open = if pressure >= PRESSURE_GATE_ON {
            true
        } else if pressure <= PRESSURE_GATE_OFF {
            false
        } else {
            was_open
        };

        let target = if gate_open { pressure } else { 0.0 };

        if target > self.level {
            self.level = (self.level + self.attack_rate).min(target);
            self.state = if (self.level - target).abs() <= f32::EPSILON {
                EnvState::Sustain
            } else {
                EnvState::Attack
            };
        } else if target < self.level {
            self.level = (self.level - self.release_rate).max(target);
            self.state = if self.level <= 0.0 {
                EnvState::Idle
            } else {
                EnvState::Release
            };
        } else {
            self.state = if self.level <= 0.0 {
                EnvState::Idle
            } else {
                EnvState::Sustain
            };
        }

        self.level
    }
}

impl Default for Envelope {
    fn default() -> Self {
        Self::new()
    }
}
