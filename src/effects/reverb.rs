use allocator_api2::vec;
use allocator_api2::vec::Vec;

use crate::config::{
    REVERB_AP_FEEDBACK_Q15, REVERB_AP_SIZES, REVERB_COMB_FEEDBACK_Q15, REVERB_COMB_SIZES,
    REVERB_DEFAULT_AMOUNT_Q15,
};

struct CombFilter {
    buffer: Vec<i16>,
    pos: usize,
    size: usize,
}

impl CombFilter {
    fn new(size: usize) -> Self {
        Self {
            buffer: vec![0i16; size],
            pos: 0,
            size,
        }
    }

    #[inline(always)]
    fn process(&mut self, input: i16, feedback_q15: i16) -> i16 {
        // SAFETY: The pos is manually wrapped by the size, so get_unchecked is safe.
        let output = unsafe { *self.buffer.get_unchecked(self.pos) };

        let fb = ((output as i32 * feedback_q15 as i32) >> 15) as i16;

        unsafe {
            *self.buffer.get_unchecked_mut(self.pos) = input.saturating_add(fb);
        }

        self.pos += 1;
        if self.pos >= self.size {
            self.pos = 0;
        }

        output
    }
}

struct AllpassFilter {
    buffer: Vec<i16>,
    pos: usize,
    size: usize,
}

impl AllpassFilter {
    fn new(size: usize) -> Self {
        Self {
            buffer: vec![0i16; size],
            pos: 0,
            size,
        }
    }

    #[inline(always)]
    fn process(&mut self, input: i16, g_q15: i16) -> i16 {
        let delayed = unsafe { *self.buffer.get_unchecked(self.pos) };
        let g_delayed = ((delayed as i32 * g_q15 as i32) >> 15) as i16;

        // output = -input + delayed + g*delayed
        let output =
            (-(input as i32) + delayed as i32 + g_delayed as i32).clamp(-32768, 32767) as i16;

        unsafe {
            *self.buffer.get_unchecked_mut(self.pos) = input.saturating_add(g_delayed);
        }

        self.pos += 1;
        if self.pos >= self.size {
            self.pos = 0;
        }

        output
    }
}

pub struct ReverbEffect {
    combs: [CombFilter; 4],
    allpasses: [AllpassFilter; 2],
    amount_q15: i16,
}

impl ReverbEffect {
    pub fn new() -> Self {
        Self {
            combs: [
                CombFilter::new(REVERB_COMB_SIZES[0]),
                CombFilter::new(REVERB_COMB_SIZES[1]),
                CombFilter::new(REVERB_COMB_SIZES[2]),
                CombFilter::new(REVERB_COMB_SIZES[3]),
            ],
            allpasses: [
                AllpassFilter::new(REVERB_AP_SIZES[0]),
                AllpassFilter::new(REVERB_AP_SIZES[1]),
            ],
            amount_q15: REVERB_DEFAULT_AMOUNT_Q15,
        }
    }

    pub fn set_amount(&mut self, amount: f32) {
        self.amount_q15 = (amount.clamp(0.0, 1.0) * 32767.0) as i16;
    }

    pub fn process_buffer(&mut self, buffer: &mut [i16]) {
        let amt = self.amount_q15 as i32;
        let inv_amt = 32767 - amt;

        for sample in buffer.iter_mut() {
            let dry = *sample;

            // Parallel combs
            let mut wet_sum: i32 = 0;
            wet_sum += self.combs[0].process(dry, REVERB_COMB_FEEDBACK_Q15) as i32;
            wet_sum += self.combs[1].process(dry, REVERB_COMB_FEEDBACK_Q15) as i32;
            wet_sum += self.combs[2].process(dry, REVERB_COMB_FEEDBACK_Q15) as i32;
            wet_sum += self.combs[3].process(dry, REVERB_COMB_FEEDBACK_Q15) as i32;

            // Average the combs
            let mut wet = (wet_sum >> 2) as i16;

            // Series allpasses
            wet = self.allpasses[0].process(wet, REVERB_AP_FEEDBACK_Q15);
            wet = self.allpasses[1].process(wet, REVERB_AP_FEEDBACK_Q15);

            // Mix
            let dry_part = (dry as i32 * inv_amt) >> 15;
            let wet_part = (wet as i32 * amt) >> 15;
            *sample = (dry_part + wet_part).clamp(-32768, 32767) as i16;
        }
    }
}

impl Default for ReverbEffect {
    fn default() -> Self {
        Self::new()
    }
}
