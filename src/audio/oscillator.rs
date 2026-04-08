//! Oscillator with phase accumulation and wavetable lookup

use crate::config::{AUDIO_SAMPLE_RATE, WAVETABLE_SIZE};

pub struct Oscillator {
    phase: u32,
    phase_inc: u32,
    wavetable: &'static [f32],
}

impl Oscillator {
    pub fn new(wavetable: &'static [f32]) -> Self {
        Self {
            phase: 0,
            phase_inc: 0,
            wavetable,
        }
    }

    pub fn set_frequency(&mut self, hz: f32) {
        // Calculate phase increment for desired frequency
        // phase_inc = (frequency * 2^32) / sample_rate
        let freq_ratio = hz / AUDIO_SAMPLE_RATE as f32;
        self.phase_inc = (freq_ratio * (u32::MAX as f32)) as u32;
    }

    pub fn set_wavetable(&mut self, wavetable: &'static [f32]) {
        self.wavetable = wavetable;
    }

    pub fn process(&mut self) -> f32 {
        // Get wavetable index from phase
        let index = ((self.phase as u64 * WAVETABLE_SIZE as u64) >> 32) as usize;
        let sample = self.wavetable[index];

        // Advance phase
        self.phase = self.phase.wrapping_add(self.phase_inc);

        sample
    }

    pub fn reset(&mut self) {
        self.phase = 0;
    }
}
