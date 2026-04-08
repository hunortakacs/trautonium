//! Voice orchestrator - combines oscillators, filter, and envelope

use allocator_api2::vec::Vec;

use crate::audio::{Envelope, Filter, LFO, Oscillator};
use crate::config::AUDIO_BUFFER_SIZE;
use crate::controls::Controls;
use crate::wavetables::Wavetables;

pub struct Voice {
    main_osc: Oscillator,
    sub_osc: Oscillator,
    filter: Filter,
    envelope: Envelope,
    lfo: LFO,
    sub_level: f32,
    pressure: f32,
}

impl Voice {
    pub fn new(wavetables: &'static Wavetables) -> Self {
        Self {
            main_osc: Oscillator::new(&wavetables.sine),
            sub_osc: Oscillator::new(&wavetables.sine),
            filter: Filter::new(),
            envelope: Envelope::new(),
            lfo: LFO::new(),
            sub_level: 0.0,
            pressure: 0.0,
        }
    }

    pub fn update_controls(&mut self, controls: &Controls, wavetables: &'static Wavetables) {
        // Update oscillator frequencies
        self.main_osc.set_frequency(controls.get_frequency());
        self.sub_osc.set_frequency(controls.get_sub_frequency());

        // Update oscillator wavetables
        self.main_osc
            .set_wavetable(wavetables.get(&controls.main_waveform));
        self.sub_osc
            .set_wavetable(wavetables.get(&controls.sub_waveform));

        // Update envelope timing
        self.envelope.set_attack(controls.attack_time);
        self.envelope.set_release(controls.release_time);

        // Update LFO
        self.lfo.set_rate(controls.lfo_rate);
        self.lfo.set_depth(controls.lfo_depth);

        // Update filter
        self.filter.set_cutoff(controls.filter_cutoff);
        self.filter.set_resonance(controls.filter_res);

        // Update mix levels
        self.sub_level = controls.sub_level;
        self.pressure = controls.pressure;

        // Trigger/release envelope based on pressure
        if controls.pressure > 0.1 {
            self.envelope.trigger();
        } else {
            self.envelope.release();
        }
    }

    pub fn process_buffer(&mut self, buffer: &mut Vec<f32>) {
        for i in 0..AUDIO_BUFFER_SIZE {
            // Generate oscillator signals
            let main = self.main_osc.process();
            let sub = self.sub_osc.process();

            // Mix oscillators
            let mixed = main + sub * self.sub_level;

            // Apply filter with LFO modulation
            let lfo_mod = self.lfo.process();
            let modulated_cutoff = (self.filter.get_cutoff() + lfo_mod).clamp(0.0, 1.0);
            let filtered = self.filter.process_with_cutoff(mixed, modulated_cutoff);

            // Apply envelope and pressure
            let env = self.envelope.process();
            let vol = env * self.pressure;

            // Output to mono buffer
            buffer[i] = filtered * vol;
        }
    }
}
