use crate::config::{AdcInputs, SwitchInputs};
use crate::wavetables::Waveform;

// Pure ADC mirror — all f32 fields are 0.0-1.0 normalized.
// Mapping to physical units (Hz, seconds, semitones) happens in
// Voice::update_controls and EffectEngine::update_params.
#[derive(Clone, Debug, defmt::Format)]
pub struct Controls {
    pub soft_pot: f32, // 0.0-1.0
    pub pressure: f32, // 0.0-1.0
    pub main_waveform: Waveform,
    pub sub_waveform: Waveform,
    pub octave: i8, // 0, 1, 2, 3, 4
    #[cfg(feature = "sub_osc")]
    pub sub_level: f32, // 0.0-1.0
    #[cfg(feature = "filter")]
    pub filter_cutoff: f32, // 0.0-1.0
    #[cfg(feature = "filter")]
    pub filter_res: f32, // 0.0-1.0
    #[cfg(feature = "filter")]
    pub pressure_filter_amt: f32, // 0.0-1.0
    #[cfg(feature = "envelope")]
    pub attack_time: f32, // 0.0-1.0
    #[cfg(feature = "envelope")]
    pub release_time: f32, // 0.0-1.0
    #[cfg(feature = "lfo")]
    pub lfo_rate: f32, // 0.0-1.0
    #[cfg(feature = "lfo")]
    pub lfo_depth: f32, // 0.0-1.0
    #[cfg(feature = "delay")]
    pub delay_time: f32, // 0.0-1.0
    #[cfg(feature = "delay")]
    pub delay_feedback: f32, // 0.0-1.0
    #[cfg(feature = "reverb")]
    pub reverb_amount: f32, // 0.0-1.0
    pub fine_tune: f32, // 0.0-1.0 (0.5 = center, no detune)
    pub master_volume: f32, // 0.0-1.0
}

impl Controls {
    pub fn new() -> Self {
        Self::new_const()
    }

    pub const fn new_const() -> Self {
        Self {
            soft_pot: 0.5,
            pressure: 0.0,
            main_waveform: Waveform::Sine,
            sub_waveform: Waveform::Sine,
            octave: 1,
            #[cfg(feature = "sub_osc")]
            sub_level: 0.0,
            #[cfg(feature = "filter")]
            filter_cutoff: 1.0,
            #[cfg(feature = "filter")]
            filter_res: 0.1,
            #[cfg(feature = "filter")]
            pressure_filter_amt: 0.0,
            #[cfg(feature = "envelope")]
            attack_time: 0.00,
            #[cfg(feature = "envelope")]
            release_time: 0.00,
            #[cfg(feature = "lfo")]
            lfo_rate: 0.0,
            #[cfg(feature = "lfo")]
            lfo_depth: 0.0,
            #[cfg(feature = "delay")]
            delay_time: 0.0,
            #[cfg(feature = "delay")]
            delay_feedback: 0.0,
            #[cfg(feature = "reverb")]
            reverb_amount: 0.0,
            fine_tune: 0.5,
            master_volume: 0.0,
        }
    }

    pub fn read(&mut self, adc_inputs: &mut AdcInputs, switch_inputs: &SwitchInputs) {
        self.soft_pot = adc_inputs.soft_pot_normalized();
        self.pressure = adc_inputs.pressure_filter_amt_normalized();
        self.main_waveform = Waveform::from_selector(switch_inputs.main_waveform_state());
        self.sub_waveform = Waveform::from_selector(switch_inputs.sub_waveform_state());
        self.octave = switch_inputs.octave_state() as i8;
        #[cfg(feature = "sub_osc")]
        {
            self.sub_level = adc_inputs.sub_level_normalized();
        }
        #[cfg(feature = "filter")]
        {
            self.filter_cutoff = adc_inputs.filter_cutoff_normalized();
            self.filter_res = adc_inputs.filter_res_normalized();
            self.pressure_filter_amt = adc_inputs.pressure_filter_amt_normalized();
        }
        #[cfg(feature = "envelope")]
        {
            self.attack_time = adc_inputs.attack_time_normalized();
            self.release_time = adc_inputs.release_time_normalized();
        }
        #[cfg(feature = "lfo")]
        {
            self.lfo_rate = adc_inputs.lfo_rate_normalized();
            self.lfo_depth = adc_inputs.lfo_depth_normalized();
        }
        #[cfg(feature = "delay")]
        {
            self.delay_time = adc_inputs.delay_time_normalized();
            self.delay_feedback = adc_inputs.delay_feedback_normalized();
        }
        #[cfg(feature = "reverb")]
        {
            self.reverb_amount = adc_inputs.reverb_amount_normalized();
        }
        self.fine_tune = adc_inputs.fine_tune_normalized();
        self.master_volume = adc_inputs.master_volume_normalized();
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self::new()
    }
}
