pub mod delay;
pub mod reverb;

#[cfg(feature = "delay")]
use crate::config::{DELAY_TIME_MAX, DELAY_TIME_MIN};
use crate::controls::Controls;
#[cfg(feature = "delay")]
use delay::DelayEffect;
#[cfg(feature = "reverb")]
use reverb::ReverbEffect;

#[inline(always)]
fn map_range(value: f32, min: f32, max: f32) -> f32 {
    min + value * (max - min)
}

pub struct EffectEngine {
    #[cfg(feature = "delay")]
    delay: DelayEffect,

    #[cfg(feature = "reverb")]
    reverb: ReverbEffect,
}

impl EffectEngine {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "delay")]
            delay: DelayEffect::new(),

            #[cfg(feature = "reverb")]
            reverb: ReverbEffect::new(),
        }
    }

    pub fn update_params(&mut self, controls: &Controls) {
        #[cfg(feature = "delay")]
        {
            self.delay.set_time(map_range(
                controls.delay_time,
                DELAY_TIME_MIN,
                DELAY_TIME_MAX,
            ));
            self.delay.set_feedback(controls.delay_feedback);
        }

        #[cfg(feature = "reverb")]
        self.reverb.set_amount(controls.reverb_amount);
    }

    pub fn process_buffer(&mut self, buffer: &mut [i16]) {
        #[cfg(feature = "delay")]
        self.delay.process_buffer(buffer);

        #[cfg(feature = "reverb")]
        self.reverb.process_buffer(buffer);

        for sample in buffer.iter_mut() {
            *sample = (*sample).clamp(-32768, 32767);
        }
    }
}

impl Default for EffectEngine {
    fn default() -> Self {
        Self::new()
    }
}
