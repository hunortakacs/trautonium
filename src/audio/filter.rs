use crate::config::{AUDIO_SAMPLE_RATE, FILTER_FREQ_MAX, FILTER_FREQ_MIN, LFO_MAX_SWING_SEMITONES};
use core::f32::consts::PI;

pub struct Filter {
    ic1eq: f32,
    ic2eq: f32,
    g: f32,
    k: f32,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            ic1eq: 0.0,
            ic2eq: 0.0,
            g: 0.05,
            k: 2.0,
        }
    }

    pub fn set_params(&mut self, cutoff: f32, resonance: f32) {
        // Calculate the frequency range ratio once
        let range_ratio = FILTER_FREQ_MAX / FILTER_FREQ_MIN;

        // Exponential mapping from 0.0-1.0 to Hz
        let fc_hz = FILTER_FREQ_MIN * libm::powf(range_ratio, cutoff.clamp(0.0, 1.0));

        // Normalize frequency and calculate the 'g' parameter (TPT SVF)
        let fc_norm = (fc_hz / AUDIO_SAMPLE_RATE as f32).min(0.499);
        self.g = libm::tanf(PI * fc_norm);

        // Resonance mapping: 2.0 (flat) to 0.04 (self-oscillation)
        self.k = 2.0 - resonance.clamp(0.0, 1.0) * 1.96;
    }

    #[inline(always)]
    pub fn process_lp(&mut self, input: i16, mod_q15: i16) -> i16 {
        let semitones = (mod_q15 as f32 / 32767.0) * LFO_MAX_SWING_SEMITONES;

        let g = (self.g * libm::powf(2.0, semitones / 12.0)).clamp(0.0005, 0.999);

        let a1 = 1.0 / (1.0 + g * (g + self.k));
        let a2 = g * a1;
        let a3 = g * a2;

        let v0 = input as f32 / 32768.0;
        let v3 = v0 - self.ic2eq;
        let v1 = a1 * self.ic1eq + a2 * v3;
        let v2 = self.ic2eq + a2 * self.ic1eq + a3 * v3;

        self.ic1eq = 2.0 * v1 - self.ic1eq;
        self.ic2eq = 2.0 * v2 - self.ic2eq;

        (v2 * 32767.0).clamp(-32768.0, 32767.0) as i16
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}
