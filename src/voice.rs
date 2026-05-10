use crate::controls::Controls;
use crate::wavetables::Wavetables;

use crate::config::DEFAULT_MASTER_VOLUME_Q15;

#[cfg(feature = "envelope")]
use crate::config::{ATTACK_MAX, ATTACK_MIN, RELEASE_MAX, RELEASE_MIN};

#[cfg(feature = "lfo")]
use crate::config::{LFO_RATE_MAX, LFO_RATE_MIN};

#[cfg(feature = "note_snap")]
use crate::config::NOTE_SNAP_COEFF;

#[cfg(feature = "envelope")]
use crate::audio::Envelope;
#[cfg(feature = "filter")]
use crate::audio::Filter;
#[cfg(feature = "lfo")]
use crate::audio::LFO;
use crate::audio::Oscillator;

#[cfg(any(feature = "envelope", feature = "lfo", feature = "filter"))]
#[inline(always)]
fn map_range(value: f32, min: f32, max: f32) -> f32 {
    min + value * (max - min)
}

/// Snap a frequency to the nearest equal-temperament semitone.
/// Returns the frequency of the nearest MIDI-style note.
#[cfg(feature = "note_snap")]
#[inline(always)]
fn snap_to_nearest_note(hz: f32) -> f32 {
    // A4 = 440 Hz is the reference (MIDI note 69)
    const A4_HZ: f32 = 440.0;

    // Convert to fractional semitone offset from A4
    let semitones = 12.0 * libm::log2f(hz / A4_HZ);

    // Round to nearest semitone
    let rounded = libm::roundf(semitones);

    // Convert back to Hz
    A4_HZ * libm::exp2f(rounded / 12.0)
}

pub struct Voice {
    main_osc: Oscillator,

    #[cfg(feature = "sub_osc")]
    sub_osc: Oscillator,

    #[cfg(feature = "filter")]
    filter: Filter,

    #[cfg(feature = "envelope")]
    envelope: Envelope,

    #[cfg(feature = "lfo")]
    lfo: LFO,

    #[cfg(feature = "sub_osc")]
    sub_level_q15: i16,
    #[cfg(feature = "sub_osc")]
    main_level_q15: i16,

    #[cfg(feature = "filter")]
    pressure_filter_amt_q15: i16,

    master_volume_q15: i16,

    /// Smoothed frequency used when note_snap is active.
    /// Tracks the target (snapped) frequency with a one-pole lowpass.
    #[cfg(feature = "note_snap")]
    smoothed_freq: f32,
}

impl Voice {
    pub fn new(wt: &'static Wavetables) -> Self {
        Self {
            main_osc: Oscillator::new(&wt.saw),

            #[cfg(feature = "sub_osc")]
            sub_osc: Oscillator::new(&wt.sine),

            #[cfg(feature = "filter")]
            filter: Filter::new(),

            #[cfg(feature = "envelope")]
            envelope: Envelope::new(),

            #[cfg(feature = "lfo")]
            lfo: LFO::new(),

            #[cfg(feature = "sub_osc")]
            sub_level_q15: 0,
            #[cfg(feature = "sub_osc")]
            main_level_q15: 32767,

            #[cfg(feature = "filter")]
            pressure_filter_amt_q15: 0,

            master_volume_q15: DEFAULT_MASTER_VOLUME_Q15,

            #[cfg(feature = "note_snap")]
            smoothed_freq: 0.0,
        }
    }

    pub fn update_controls(&mut self, c: &Controls, wt: &'static Wavetables) {
        let freq = self.get_frequency(c);

        // With note_snap: steer smoothed_freq toward the nearest valid note each
        // update. The oscillator is driven by smoothed_freq so the pitch slides
        // in rather than jumping.
        #[cfg(feature = "note_snap")]
        let freq = {
            let target = snap_to_nearest_note(freq);
            if self.smoothed_freq == 0.0 {
                // First call — seed with exact target so there's no initial glide from 0 Hz.
                self.smoothed_freq = target;
            } else {
                // One-pole lowpass: smoothed = coeff * smoothed + (1 - coeff) * target
                self.smoothed_freq =
                    NOTE_SNAP_COEFF * self.smoothed_freq + (1.0 - NOTE_SNAP_COEFF) * target;
            }
            self.smoothed_freq
        };

        self.main_osc.set_frequency(freq);
        self.main_osc.set_wavetable(wt.get(&c.main_waveform));

        #[cfg(feature = "sub_osc")]
        {
            self.sub_osc.set_frequency(freq * 0.5);
            self.sub_osc.set_wavetable(wt.get(&c.sub_waveform));
            self.sub_level_q15 = (c.sub_level.clamp(0.0, 1.0) * 32767.0) as i16;
            self.main_level_q15 = 32767 - self.sub_level_q15;
        }

        #[cfg(feature = "envelope")]
        {
            self.envelope
                .set_attack(map_range(c.attack_time, ATTACK_MIN, ATTACK_MAX));
            self.envelope
                .set_release(map_range(c.release_time, RELEASE_MIN, RELEASE_MAX));
        }

        #[cfg(feature = "lfo")]
        {
            self.lfo
                .set_rate(map_range(c.lfo_rate, LFO_RATE_MIN, LFO_RATE_MAX));
            self.lfo.set_depth(c.lfo_depth);
        }

        #[cfg(feature = "filter")]
        {
            self.filter.set_params(c.filter_cutoff, c.filter_res);
            self.pressure_filter_amt_q15 = (c.pressure_filter_amt * 32767.0) as i16;
        }

        let master_vol_curved = c.master_volume * c.master_volume;
        self.master_volume_q15 = (master_vol_curved * 32767.0) as i16;
    }

    pub fn process_buffer(&mut self, buffer: &mut [i16], controls: &Controls) {
        let curved_pressure = controls.pressure * controls.pressure;
        let pressure_q15 = (curved_pressure * 32767.0) as i32;
        let master_vol = self.master_volume_q15 as i32;

        for sample in buffer.iter_mut() {
            #[cfg(feature = "envelope")]
            let env = self.envelope.process(pressure_q15 as i16) as i32;
            #[cfg(not(feature = "envelope"))]
            let env = pressure_q15;

            let main = self.main_osc.process() as i32;

            #[cfg(feature = "sub_osc")]
            let osc_mix = {
                let sub = self.sub_osc.process() as i32;
                let main_scaled = (main * self.main_level_q15 as i32) >> 15;
                let sub_scaled = (sub * self.sub_level_q15 as i32) >> 15;
                (main_scaled + sub_scaled).clamp(-32768, 32767) as i16
            };
            #[cfg(not(feature = "sub_osc"))]
            let osc_mix = main as i16;

            #[cfg(feature = "filter")]
            let filtered = {
                #[cfg(feature = "lfo")]
                let lfo_mod = self.lfo.process() as i32;
                #[cfg(not(feature = "lfo"))]
                let lfo_mod = 0i32;

                let env_mod = (env * self.pressure_filter_amt_q15 as i32) >> 15;

                let total_mod = (lfo_mod + env_mod).clamp(-32768, 32767) as i16;

                self.filter.process_lp(osc_mix, total_mod)
            };
            #[cfg(not(feature = "filter"))]
            let filtered = osc_mix;

            let enveloped = (filtered as i32 * env) >> 15;
            let final_val = (enveloped * master_vol) >> 15;

            *sample = final_val.clamp(-32768, 32767) as i16;
        }
    }

    pub fn get_frequency(&self, c: &Controls) -> f32 {
        let start_freq = 65.4;
        let fine_tune_semitones = (c.fine_tune - 0.5) * 2.0;
        let pot_semitones = c.soft_pot * 20.0;

        let total_octaves = (pot_semitones + fine_tune_semitones) / 12.0 + (c.octave as f32);

        start_freq * libm::exp2f(total_octaves)
    }
}
