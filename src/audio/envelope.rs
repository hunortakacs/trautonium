use crate::config::{
    AUDIO_SAMPLE_RATE, ENVELOPE_DEFAULT_SLEW_S, PRESSURE_GATE_OFF_Q15, PRESSURE_GATE_ON_Q15,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvState {
    Idle,
    Active,
}

pub struct Envelope {
    state: EnvState,
    level_q15: i32,
    attack_inc: i32,
    release_inc: i32,
}

impl Envelope {
    pub fn new() -> Self {
        let default_samples = (ENVELOPE_DEFAULT_SLEW_S * AUDIO_SAMPLE_RATE as f32).max(1.0);
        let default_inc = (32768.0 / default_samples) as i32;

        Self {
            state: EnvState::Idle,
            level_q15: 0,
            attack_inc: default_inc,
            release_inc: default_inc,
        }
    }

    pub fn set_attack(&mut self, time_seconds: f32) {
        let samples = (time_seconds * AUDIO_SAMPLE_RATE as f32).max(1.0);
        self.attack_inc = (32768.0 / samples) as i32;
    }

    pub fn set_release(&mut self, time_seconds: f32) {
        let samples = (time_seconds * AUDIO_SAMPLE_RATE as f32).max(1.0);
        self.release_inc = (32768.0 / samples) as i32;
    }

    #[inline(always)]
    pub fn process(&mut self, pressure_q15: i16) -> i16 {
        let input = pressure_q15 as i32;

        let is_gate_open = match self.state {
            EnvState::Idle => input > PRESSURE_GATE_ON_Q15 as i32,
            EnvState::Active => input > PRESSURE_GATE_OFF_Q15 as i32,
        };

        let target = if is_gate_open { input } else { 0 };

        if target > self.level_q15 {
            self.level_q15 = (self.level_q15 + self.attack_inc).min(target);
        } else if target < self.level_q15 {
            self.level_q15 = (self.level_q15 - self.release_inc).max(target);
        }

        if self.level_q15 <= 0 {
            self.level_q15 = 0;
            self.state = EnvState::Idle;
        } else {
            self.state = EnvState::Active;
        }

        self.level_q15 as i16
    }

    pub fn is_idle(&self) -> bool {
        self.state == EnvState::Idle
    }
}

impl Default for Envelope {
    fn default() -> Self {
        Self::new()
    }
}
