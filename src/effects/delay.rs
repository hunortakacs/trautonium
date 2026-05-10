use crate::config::{
    AUDIO_SAMPLE_RATE, DELAY_DRY_GAIN_Q15, DELAY_FEEDBACK_MAX, DELAY_WET_GAIN_Q15,
    MAX_DELAY_SAMPLES,
};
use allocator_api2::vec;
use allocator_api2::vec::Vec;

pub struct DelayEffect {
    buffer: Vec<i16>,
    write_pos: usize,
    delay_samples: usize,
    feedback_q15: i16,
    mask: usize,
}

impl DelayEffect {
    pub fn new() -> Self {
        Self {
            buffer: vec![0i16; MAX_DELAY_SAMPLES],
            write_pos: 0,
            delay_samples: MAX_DELAY_SAMPLES / 2,
            feedback_q15: 0,
            mask: MAX_DELAY_SAMPLES - 1, 
        }
    }

    pub fn set_time(&mut self, time_seconds: f32) {
        let samples = (time_seconds * AUDIO_SAMPLE_RATE as f32) as usize;
        self.delay_samples = samples.clamp(1, MAX_DELAY_SAMPLES - 1);
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback_q15 = (feedback.clamp(0.0, DELAY_FEEDBACK_MAX) * 32767.0) as i16;
    }

    pub fn process_buffer(&mut self, buffer: &mut [i16]) {
        let mask = self.mask;
        let fb_q15 = self.feedback_q15 as i32;
        let dry_gain = DELAY_DRY_GAIN_Q15 as i32;
        let wet_gain = DELAY_WET_GAIN_Q15 as i32;

        for sample in buffer.iter_mut() {
            let read_pos = (self.write_pos.wrapping_sub(self.delay_samples)) & mask;

            let delayed = unsafe { *self.buffer.get_unchecked(read_pos) };

            let fb = (delayed as i32 * fb_q15) >> 15;
            unsafe {
                *self.buffer.get_unchecked_mut(self.write_pos) =
                    (*sample).saturating_add(fb as i16);
            }

            let dry = (*sample as i32 * dry_gain) >> 15;
            let wet = (delayed as i32 * wet_gain) >> 15;

            *sample = (dry + wet).clamp(-32768, 32767) as i16;

            self.write_pos = (self.write_pos + 1) & mask;
        }
    }
}

impl Default for DelayEffect {
    fn default() -> Self {
        Self::new()
    }
}
