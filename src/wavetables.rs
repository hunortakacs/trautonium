//! Pre-calculated waveform tables

use crate::config::WAVETABLE_SIZE;

#[derive(Default)]
pub enum Waveform {
    #[default]
    Saw,
    Sine,
    Square,
    Triangle,
}

const fn generate_saw() -> [f32; WAVETABLE_SIZE] {
    let mut arr = [0.0; WAVETABLE_SIZE];
    let mut i = 0;
    while i < WAVETABLE_SIZE {
        arr[i] = 2.0 * i as f32 / WAVETABLE_SIZE as f32 - 1.0;
        i += 1;
    }
    arr
}

const fn sin_approx(x: f32) -> f32 {
    let x2 = x * x;
    let x3 = x2 * x;
    let x5 = x3 * x2;
    let x7 = x5 * x2;
    x - x3 / 6.0 + x5 / 120.0 - x7 / 5040.0
}

const fn generate_sine() -> [f32; WAVETABLE_SIZE] {
    let mut arr = [0.0; WAVETABLE_SIZE];
    let mut i = 0;
    while i < WAVETABLE_SIZE {
        arr[i] = sin_approx(2.0 * core::f32::consts::PI * i as f32 / WAVETABLE_SIZE as f32);
        i += 1;
    }
    arr
}

const fn generate_square() -> [f32; WAVETABLE_SIZE] {
    let mut arr = [0.0; WAVETABLE_SIZE];
    let mut i = 0;
    while i < WAVETABLE_SIZE {
        arr[i] = if i < WAVETABLE_SIZE / 2 { 1.0 } else { -1.0 };
        i += 1;
    }
    arr
}

const fn generate_triangle() -> [f32; WAVETABLE_SIZE] {
    let mut arr = [0.0; WAVETABLE_SIZE];
    let mut i = 0;
    while i < WAVETABLE_SIZE {
        arr[i] = 2.0 * (i as f32 / WAVETABLE_SIZE as f32 - 0.5).abs() - 1.0;
        i += 1;
    }
    arr
}

pub struct Wavetables {
    pub saw: [f32; WAVETABLE_SIZE],
    pub sine: [f32; WAVETABLE_SIZE],
    pub square: [f32; WAVETABLE_SIZE],
    pub triangle: [f32; WAVETABLE_SIZE],
}

impl Wavetables {
    pub const fn new() -> Self {
        Self {
            saw: generate_saw(),
            sine: generate_sine(),
            square: generate_square(),
            triangle: generate_triangle(),
        }
    }

    pub fn get(&self, waveform: &Waveform) -> &[f32] {
        match waveform {
            Waveform::Saw => &self.saw,
            Waveform::Sine => &self.sine,
            Waveform::Square => &self.square,
            Waveform::Triangle => &self.triangle,
        }
    }
}

impl Waveform {
    pub fn from_selector(selector: u8) -> Self {
        match selector & 0b11 {
            0 => Self::Saw,
            1 => Self::Sine,
            2 => Self::Square,
            _ => Self::Triangle,
        }
    }
}

impl Default for Wavetables {
    fn default() -> Self {
        Self::new()
    }
}
