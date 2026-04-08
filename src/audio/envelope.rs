//! Attack/Release envelope generator

use crate::config::AUDIO_SAMPLE_RATE;

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

    pub fn trigger(&mut self) {
        self.state = EnvState::Attack;
    }

    pub fn release(&mut self) {
        if self.state != EnvState::Idle {
            self.state = EnvState::Release;
        }
    }

    pub fn process(&mut self) -> f32 {
        match self.state {
            EnvState::Attack => {
                self.level += self.attack_rate;
                if self.level >= 1.0 {
                    self.level = 1.0;
                    self.state = EnvState::Sustain;
                }
            }
            EnvState::Sustain => {
                // Hold at 1.0
            }
            EnvState::Release => {
                self.level -= self.release_rate;
                if self.level <= 0.0 {
                    self.level = 0.0;
                    self.state = EnvState::Idle;
                }
            }
            EnvState::Idle => {
                self.level = 0.0;
            }
        }

        self.level
    }
}

impl Default for Envelope {
    fn default() -> Self {
        Self::new()
    }
}
