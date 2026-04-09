//! EffectEngine: encapsulates delay and reverb processing for clean and efficient audio effect handling

pub mod delay;
pub mod reverb;

use delay::DelayEffect;
use reverb::ReverbEffect;

use crate::controls::Controls;

/// EffectEngine manages delay and reverb effects in a single, efficient unit.
pub struct EffectEngine {
    delay: DelayEffect,
    reverb: ReverbEffect,
}

impl EffectEngine {
    /// Create a new EffectEngine with given max buffer size
    pub fn new() -> Self {
        Self {
            delay: DelayEffect::new(),
            reverb: ReverbEffect::new(),
        }
    }

    /// Set delay and reverb parameters from controls
    pub fn update_params(&mut self, controls: &Controls) {
        self.delay.set_time(controls.delay_time);
        self.delay.set_feedback(controls.delay_feedback);
        self.reverb.amount = controls.reverb_amount;
    }

    /// Process mono buffer in-place through delay and reverb
    pub fn process_buffer(&mut self, buffer: &mut [f32]) {
        self.delay.process_buffer(buffer);
        self.reverb.process_buffer(buffer);
    }

    /// Accessors if needed
    pub fn delay(&self) -> &DelayEffect {
        &self.delay
    }
    pub fn delay_mut(&mut self) -> &mut DelayEffect {
        &mut self.delay
    }
    pub fn reverb(&self) -> &ReverbEffect {
        &self.reverb
    }
    pub fn reverb_mut(&mut self) -> &mut ReverbEffect {
        &mut self.reverb
    }
}

impl Default for EffectEngine {
    fn default() -> Self {
        Self::new()
    }
}
