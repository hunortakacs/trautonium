//! Control input reading and mapping

use crate::config::{AdcInputs, SwitchInputs};
use crate::wavetables::Waveform;

#[derive(Default)]
pub struct Controls {
    pub soft_pot: f32,            // 0.0-1.0 (pitch)
    pub pressure: f32,            // 0.0-1.0 (volume)
    pub main_waveform: Waveform,  // main oscillator
    pub sub_level: f32,           // 0.0-1.0
    pub sub_waveform: Waveform,   // sub oscillator
    pub filter_cutoff: f32,       // 0.0-1.0
    pub filter_res: f32,          // 0.0-1.0
    pub pressure_filter_amt: f32, // 0.0-1.0
    pub attack_time: f32,         // 0.005-0.5 seconds
    pub release_time: f32,        // 0.01-0.5 seconds
    pub lfo_rate: f32,            // 0.1-10 Hz
    pub lfo_depth: f32,           // 0.0-1.0
    pub delay_time: f32,          // 0.05-1.0 seconds
    pub delay_feedback: f32,      // 0.0-1.0
    pub reverb_amount: f32,       // 0.0-1.0
    pub fine_tune: f32,           // -1.0 to +1.0 semitones
    pub octave: i8,               // -1, 0, +1, +2
    pub master_volume: f32,       // 0.0-1.0
}

impl Controls {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(&mut self, adc_inputs: &mut AdcInputs, switch_inputs: &SwitchInputs) {
        self.soft_pot = adc_inputs.soft_pot_normalized();
        self.pressure = adc_inputs.pressure_normalized();
        self.main_waveform = Waveform::from_selector(switch_inputs.main_waveform_state());
        self.sub_waveform = Waveform::from_selector(switch_inputs.sub_waveform_state());
        self.octave = switch_inputs.octave_state() as i8 - 1;
        self.sub_level = adc_inputs.sub_level_normalized();
        self.filter_cutoff = adc_inputs.filter_cutoff_normalized();
        self.filter_res = adc_inputs.filter_res_normalized();
        self.pressure_filter_amt = adc_inputs.pressure_filter_amt_normalized();
        self.attack_time = map_range(adc_inputs.attack_time_normalized(), 0.005, 0.5);
        self.release_time = map_range(adc_inputs.release_time_normalized(), 0.01, 0.5);
        self.lfo_rate = map_range(adc_inputs.lfo_rate_normalized(), 0.1, 1.0);
        self.lfo_depth = adc_inputs.lfo_depth_normalized();
        self.delay_time = map_range(adc_inputs.delay_time_normalized(), 0.05, 1.0);
        self.delay_feedback = adc_inputs.delay_feedback_normalized();
        self.reverb_amount = adc_inputs.reverb_amount_normalized();
        self.fine_tune = map_range(adc_inputs.fine_tune_normalized(), -1.0, 1.0);
        self.master_volume = adc_inputs.master_volume_normalized();
    }

    pub fn get_frequency(&self) -> f32 {
        // Map soft_pot (0-1) to base frequency range (C2 to C7: 65.4Hz to 2093Hz)
        let base_freq = 65.4 + self.soft_pot * (2093.0 - 65.4);

        // Apply octave shift (multiply by 2^octave)
        let octave_mult = libm::powf(2.0, self.octave as f32);

        // Apply fine tune (-1 to +1 semitones = multiply by 2^(cents/12))
        let fine_tune_mult = libm::powf(2.0, self.fine_tune / 12.0);

        base_freq * octave_mult * fine_tune_mult
    }

    pub fn get_sub_frequency(&self) -> f32 {
        // Sub oscillator is typically one octave below
        self.get_frequency() * 0.5
    }
}

/// Map normalized value to specific range
fn map_range(value: f32, min: f32, max: f32) -> f32 {
    min + value * (max - min)
}
