//! Low-frequency oscillator

use crate::config::AUDIO_SAMPLE_RATE;

pub struct LFO {
    phase: u32,
    phase_inc: u32,
    depth: f32,
}

impl LFO {
    pub fn new() -> Self {
        Self {
            phase: 0,
            phase_inc: 0,
            depth: 0.0,
        }
    }

    pub fn set_rate(&mut self, hz: f32) {
        let freq_ratio = hz / AUDIO_SAMPLE_RATE as f32;
        self.phase_inc = (freq_ratio * (u32::MAX as f32)) as u32;
    }

    pub fn set_depth(&mut self, depth: f32) {
        self.depth = depth.clamp(0.0, 1.0);
    }

    pub fn process(&mut self) -> f32 {
        // Simple sine LFO using phase
        let normalized_phase = self.phase as f32 / u32::MAX as f32;
        let sine_value = libm::sinf(normalized_phase * core::f32::consts::TAU);

        // Advance phase
        self.phase = self.phase.wrapping_add(self.phase_inc);

        // Return bipolar modulation: -depth to +depth
        sine_value * self.depth
    }
}

impl Default for LFO {
    fn default() -> Self {
        Self::new()
    }
}
