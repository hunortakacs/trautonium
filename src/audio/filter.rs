//! Resonant low-pass filter

pub struct Filter {
    state: [f32; 4],
    cutoff: f32,
    resonance: f32,
    // Filter coefficients
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            state: [0.0; 4],
            cutoff: 1.0,
            resonance: 0.0,
            a1: 0.0,
            a2: 0.0,
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
        }
    }

    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff.clamp(0.0, 1.0);
        self.update_coefficients();
    }

    pub fn set_resonance(&mut self, resonance: f32) {
        self.resonance = resonance.clamp(0.0, 1.0);
        self.update_coefficients();
    }

    pub fn get_cutoff(&self) -> f32 {
        self.cutoff
    }

    fn update_coefficients(&mut self) {
        // Calculate biquad low-pass filter coefficients
        let sample_rate = crate::config::AUDIO_SAMPLE_RATE as f32;
        let cutoff_freq = self.cutoff * (sample_rate * 0.5); // cutoff is 0-1, map to 0-Nyquist
        let omega = 2.0 * core::f32::consts::PI * cutoff_freq / sample_rate;

        // Q factor from resonance (0-1), higher resonance = lower Q = more resonance
        let q = 1.0 / (2.0 * (self.resonance * 0.9 + 0.1)); // avoid division by zero

        let sin_omega = libm::sinf(omega);
        let cos_omega = libm::cosf(omega);
        let alpha = sin_omega / (2.0 * q);

        // Low-pass filter coefficients
        let b0 = (1.0 - cos_omega) / 2.0;
        let b1 = 1.0 - cos_omega;
        let b2 = (1.0 - cos_omega) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha;

        // Normalize by a0
        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
    }

    pub fn process(&mut self, input: f32) -> f32 {
        // Biquad filter difference equation:
        // y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2] - a1*y[n-1] - a2*y[n-2]
        let output = self.b0 * input + self.b1 * self.state[0] + self.b2 * self.state[1]
            - self.a1 * self.state[2]
            - self.a2 * self.state[3];

        // Update state: shift x[n-1] -> x[n-2], y[n-1] -> y[n-2]
        self.state[1] = self.state[0]; // x[n-2] = x[n-1]
        self.state[0] = input; // x[n-1] = x[n]
        self.state[3] = self.state[2]; // y[n-2] = y[n-1]
        self.state[2] = output; // y[n-1] = y[n]

        output
    }

    pub fn process_with_cutoff(&mut self, input: f32, cutoff: f32) -> f32 {
        // Calculate coefficients for this sample's modulated cutoff
        let sample_rate = crate::config::AUDIO_SAMPLE_RATE as f32;
        let cutoff_freq = cutoff * (sample_rate * 0.5);
        let omega = 2.0 * core::f32::consts::PI * cutoff_freq / sample_rate;

        let q = 1.0 / (2.0 * (self.resonance * 0.9 + 0.1));
        let sin_omega = libm::sinf(omega);
        let cos_omega = libm::cosf(omega);
        let alpha = sin_omega / (2.0 * q);

        let b0 = (1.0 - cos_omega) / 2.0;
        let b1 = 1.0 - cos_omega;
        let b2 = (1.0 - cos_omega) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha;

        // Apply filter with these coefficients
        let output = (b0 * input + b1 * self.state[0] + b2 * self.state[1]
            - a1 * self.state[2]
            - a2 * self.state[3])
            / a0;

        // Update state
        self.state[1] = self.state[0];
        self.state[0] = input;
        self.state[3] = self.state[2];
        self.state[2] = output;

        output
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}
