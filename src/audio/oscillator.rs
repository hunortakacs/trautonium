use crate::config::{AUDIO_SAMPLE_RATE, WAVETABLE_POWER, WAVETABLE_SIZE};

pub struct Oscillator {
    phase: u32,
    phase_inc: u32,
    wavetable: &'static [i16; WAVETABLE_SIZE],
}

impl Oscillator {
    pub fn new(wavetable: &'static [i16; WAVETABLE_SIZE]) -> Self {
        Self {
            phase: 0,
            phase_inc: 0,
            wavetable,
        }
    }

    pub fn set_frequency(&mut self, hz: f32) {
        let ratio = hz as f64 / AUDIO_SAMPLE_RATE as f64;
        self.phase_inc = (ratio * 4294967296.0) as u32;
    }

    pub fn set_wavetable(&mut self, wavetable: &'static [i16; WAVETABLE_SIZE]) {
        self.wavetable = wavetable;
    }

    pub fn reset(&mut self) {
        self.phase = 0;
    }

    #[inline(always)]
    pub fn process(&mut self) -> i16 {
        let index = (self.phase >> (32 - WAVETABLE_POWER)) as usize;

        let sample = self.wavetable[index];

        self.phase = self.phase.wrapping_add(self.phase_inc);

        sample
    }
}
