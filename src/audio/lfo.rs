use crate::config::{AUDIO_SAMPLE_RATE, LFO_RATE_MIN};

pub struct LFO {
    phase: u32,
    phase_inc: u32,
    depth_q15: i16,
}

impl LFO {
    pub fn new() -> Self {
        let mut lfo = Self {
            phase: 0,
            phase_inc: 0,
            depth_q15: 0,
        };
        // Initialize at the minimum rate defined in config
        lfo.set_rate(LFO_RATE_MIN);
        lfo
    }

    pub fn set_rate(&mut self, hz: f32) {
        // Rate is derived from the system sample rate in config
        let ratio = hz as f64 / AUDIO_SAMPLE_RATE as f64;
        self.phase_inc = (ratio * u32::MAX as f64) as u32;
    }

    pub fn set_depth(&mut self, depth: f32) {
        // depth is expected to be 0.0 to 1.0
        self.depth_q15 = (depth.clamp(0.0, 1.0) * 32767.0) as i16;
    }

    #[inline(always)]
    pub fn process(&mut self) -> i16 {
        // Fold 16-bit sawtooth into a symmetric triangle wave
        let saw16 = (self.phase >> 16) as i32;
        let folded = if saw16 < 32768 { saw16 } else { 65535 - saw16 };

        // Output range: [-32768, 32766]
        let triangle = (folded * 2 - 32768) as i16;

        self.phase = self.phase.wrapping_add(self.phase_inc);

        // Scale by depth (Q15 multiplication)
        ((triangle as i32 * self.depth_q15 as i32) >> 15) as i16
    }
}

impl Default for LFO {
    fn default() -> Self {
        Self::new()
    }
}
