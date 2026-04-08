//! Audio configuration constants and hardware abstraction

pub const AUDIO_SAMPLE_RATE: u32 = 44_100;
pub const AUDIO_BUFFER_SIZE: usize = 48;
pub const MAX_DELAY_SAMPLES: usize = AUDIO_SAMPLE_RATE as usize / 2;
pub const WAVETABLE_SIZE: usize = 1024;

// ADC reading constants
pub const ADC_MAX_VALUE: f32 = 4095.0; // 12-bit ADC
pub const ADC_SMOOTHING_ALPHA: f32 = 0.3; // Low-pass filter coefficient (0.0-1.0, lower = smoother)
pub const MUX_SETTLE_DELAY_US: u32 = 12;

setup_mux_adc_inputs!(AdcInputs, {
    mux_select: {
        a: GPIO25,
        b: GPIO33,
        c: GPIO32
    },
    analog_in: {
        mux0: GPIO36,
        mux1: GPIO39
    },
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
