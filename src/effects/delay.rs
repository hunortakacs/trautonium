//! Echo/delay effect with feedback

use allocator_api2::vec;
use allocator_api2::vec::Vec;

use crate::config::{AUDIO_SAMPLE_RATE, MAX_DELAY_SAMPLES};

pub struct DelayEffect {
    // Using Vec moves data to the Heap, preventing Stack Overflow
    buffer: Vec<f32>,
    write_pos: usize,
    max_samples: usize,
    delay_samples: f32, // now fractional
    feedback: f32,
}

impl DelayEffect {
    pub fn new() -> Self {
        Self {
            // Allocate exactly the memory needed on the heap
            buffer: vec![0.0; MAX_DELAY_SAMPLES],
            write_pos: 0,
            max_samples: MAX_DELAY_SAMPLES,
            delay_samples: (MAX_DELAY_SAMPLES / 2) as f32,
            feedback: 0.0,
        }
    }

    pub fn set_time(&mut self, time_seconds: f32) {
        let delay = time_seconds * AUDIO_SAMPLE_RATE as f32;
        // Clamp to valid range
        self.delay_samples = delay.clamp(1.0, (self.max_samples - 1) as f32);
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback.clamp(0.0, 0.95);
    }

    pub fn process_buffer(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            // Calculate fractional read position
            let delay = self.delay_samples;
            let write_pos = self.write_pos as isize;
            let max_samples = self.max_samples as isize;
            let read_pos_f = write_pos as f32 - delay;
            // Wrap negative positions
            let read_pos_f = if read_pos_f < 0.0 {
                read_pos_f + max_samples as f32
            } else {
                read_pos_f
            };
            let idx0 = libm::floorf(read_pos_f) as usize % self.max_samples;
            let idx1 = (idx0 + 1) % self.max_samples;
            let frac = read_pos_f - libm::floorf(read_pos_f);
            let delayed = self.buffer[idx0] * (1.0 - frac) + self.buffer[idx1] * frac;

            // Feedback loop
            self.buffer[self.write_pos] = *sample + delayed * self.feedback;

            // Simple 30/70 mix
            *sample = *sample * 0.7 + delayed * 0.3;
            self.write_pos = (self.write_pos + 1) % self.max_samples;
        }
    }
}

impl Default for DelayEffect {
    fn default() -> Self {
        Self::new()
    }
}
