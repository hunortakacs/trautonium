//! Audio configuration constants and hardware abstraction

// --- System & Buffer Settings ---
pub const AUDIO_CORE_STACK_SIZE: usize = 48 * 1024;
pub const AUDIO_SAMPLE_RATE: u32 = 44_100;
pub const AUDIO_BUFFER_SIZE: usize = 256;
pub const WAVETABLE_POWER: u32 = 10;
pub const WAVETABLE_SIZE: usize = 1 << WAVETABLE_POWER;

// --- ADC and Hardware Mapping ---
pub const ADC_MAX_VALUE: f32 = 4095.0;
pub const ADC_SMOOTHING_ALPHA: f32 = 0.5;
pub const MUX_SETTLE_DELAY_US: u32 = 900;

/// Pressure Hysteresis Gate (Converted to Q15)
#[cfg(feature = "envelope")]
pub const PRESSURE_GATE_ON_Q15: i16 = 3932;
#[cfg(feature = "envelope")]
pub const PRESSURE_GATE_OFF_Q15: i16 = 2621;

// --- Voice & Envelope Parameters ---
#[cfg(feature = "envelope")]
pub const ATTACK_MIN: f32 = 0.002;
#[cfg(feature = "envelope")]
pub const ATTACK_MAX: f32 = 1.0;
#[cfg(feature = "envelope")]
pub const RELEASE_MIN: f32 = 0.01;
#[cfg(feature = "envelope")]
pub const RELEASE_MAX: f32 = 2.0;
#[cfg(feature = "envelope")]
pub const ENVELOPE_DEFAULT_SLEW_S: f32 = 0.001;

#[cfg(feature = "lfo")]
pub const LFO_RATE_MIN: f32 = 0.1;
#[cfg(feature = "lfo")]
pub const LFO_RATE_MAX: f32 = 10.0;

pub const DEFAULT_MASTER_VOLUME_Q15: i16 = 26214;

#[cfg(feature = "note_snap")]
pub const NOTE_SNAP_COEFF: f32 = 0.93;

// --- Filter & LFO Modulation Settings ---
#[cfg(feature = "filter")]
pub const FILTER_FREQ_MIN: f32 = 40.0;
#[cfg(feature = "filter")]
pub const FILTER_FREQ_MAX: f32 = 18000.0;
#[cfg(feature = "filter")]
pub const FILTER_RANGE_SEMITONES: f32 = 106.63;
#[cfg(feature = "filter")]
pub const LFO_MAX_SWING_SEMITONES: f32 = FILTER_RANGE_SEMITONES / 2.0;

// --- Delay Effect Settings ---
#[cfg(feature = "delay")]
pub const MAX_DELAY_SAMPLES: usize = 16384;
#[cfg(feature = "delay")]
pub const DELAY_TIME_MIN: f32 = 0.02;
#[cfg(feature = "delay")]
pub const DELAY_TIME_MAX: f32 = (MAX_DELAY_SAMPLES - 1) as f32 / AUDIO_SAMPLE_RATE as f32;
#[cfg(feature = "delay")]
pub const DELAY_FEEDBACK_MAX: f32 = 0.90;
#[cfg(feature = "delay")]
pub const DELAY_DRY_GAIN_Q15: i16 = 22938;
#[cfg(feature = "delay")]
pub const DELAY_WET_GAIN_Q15: i16 = 9830;

// --- Reverb Effect Settings ---
#[cfg(feature = "reverb")]
pub const REVERB_COMB_SIZES: [usize; 4] = [1557, 1617, 1491, 1422];
#[cfg(feature = "reverb")]
pub const REVERB_AP_SIZES: [usize; 2] = [225, 341];
#[cfg(feature = "reverb")]
pub const REVERB_COMB_FEEDBACK_Q15: i16 = 26214;
#[cfg(feature = "reverb")]
pub const REVERB_AP_FEEDBACK_Q15: i16 = 16384;
#[cfg(feature = "reverb")]
pub const REVERB_DEFAULT_AMOUNT_Q15: i16 = 8192;

setup_mux_adc_inputs!(AdcInputs, {
    mux_select: { a: GPIO25, b: GPIO33, c: GPIO32 },
    analog_in: { mux0: GPIO36, mux1: GPIO39 },
    controls: {
        fine_tune: { a: 0, b: 1, c: 0, mux: 0 },
        attack_time: { a: 0, b: 0, c: 0, mux: 0 },
        reverb_amount: { a: 1, b: 1, c: 0, mux: 0 },
        delay_time: { a: 0, b: 1, c: 0, mux: 1 },
        filter_cutoff: { a: 1, b: 0, c: 0, mux: 1 },
        lfo_rate: { a: 0, b: 0, c: 0, mux: 1 },
        pressure: { a: 1, b: 1, c: 0, mux: 1 },
        sub_level: { a: 0, b: 0, c: 1, mux: 0 },
        master_volume: { a: 0, b: 1, c: 1, mux: 0 },
        release_time: { a: 1, b: 1, c: 1, mux: 0 },
        pressure_filter_amt: { a: 1, b: 0, c: 1, mux: 0 },
        delay_feedback: { a: 0, b: 0, c: 1, mux: 1 },
        filter_res: { a: 0, b: 1, c: 1, mux: 1 },
        lfo_depth: { a: 1, b: 0, c: 1, mux: 1 },
        soft_pot: { a: 1, b: 1, c: 1, mux: 1 }
    }
});

setup_binary_switch_inputs!(SwitchInputs, {
    main_waveform: { msb: GPIO18, lsb: GPIO19 },
    octave: { msb: GPIO27, lsb: GPIO26 },
    sub_waveform: { msb: GPIO22, lsb: GPIO23 }
});
