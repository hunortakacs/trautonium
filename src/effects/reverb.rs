//! Simple reverb effect using a combination of comb and allpass filters

use allocator_api2::vec;
use allocator_api2::vec::Vec;

struct CombFilter {
    buffer: Vec<f32>,
    pos: usize,
    size: usize,
}

impl CombFilter {
    fn new(delay: usize) -> Self {
        Self {
            buffer: vec![0.0; delay],
            pos: 0,
            size: delay,
        }
    }

    fn process(&mut self, input: f32, feedback: f32) -> f32 {
        let output = self.buffer[self.pos];
        self.buffer[self.pos] = input + (output * feedback);
        self.pos = (self.pos + 1) % self.size;
        output
    }
}

struct AllpassFilter {
    buffer: Vec<f32>,
    pos: usize,
    size: usize,
}

impl AllpassFilter {
    fn new(delay: usize) -> Self {
        Self {
            buffer: vec![0.0; delay],
            pos: 0,
            size: delay,
        }
    }

    fn process(&mut self, input: f32, feedback: f32) -> f32 {
        let delayed = self.buffer[self.pos];
        let output = -input + delayed + (feedback * input);
        self.buffer[self.pos] = input + (feedback * delayed);
        self.pos = (self.pos + 1) % self.size;
        output
    }
}

pub struct ReverbEffect {
    combs: Vec<CombFilter>,
    allpass: Vec<AllpassFilter>,
    pub amount: f32,
}

impl ReverbEffect {
    pub fn new() -> Self {
        // Primes for mono channel
        let comb_delays = [1557, 1617, 1491, 1422];
        let allpass_delays = [225, 341];

        Self {
            combs: comb_delays.iter().map(|&d| CombFilter::new(d)).collect(),
            allpass: allpass_delays
                .iter()
                .map(|&d| AllpassFilter::new(d))
                .collect(),
            amount: 0.25,
        }
    }

    /// Process a mono buffer in-place
    pub fn process_buffer(&mut self, buf: &mut [f32]) {
        for sample in buf.iter_mut() {
            let mut wet = 0.0;
            for comb in &mut self.combs {
                wet += comb.process(*sample, 0.84);
            }
            wet *= 0.25;
            for ap in &mut self.allpass {
                wet = ap.process(wet, 0.5);
            }
            // Final Mix
            *sample = *sample * (1.0 - self.amount) + wet * self.amount;
        }
    }
}

impl Default for ReverbEffect {
    fn default() -> Self {
        Self::new()
    }
}
