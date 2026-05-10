use crate::config::WAVETABLE_SIZE;

#[derive(Default, Clone, Copy, PartialEq, Debug, defmt::Format)]
pub enum Waveform {
    #[default]
    Saw,
    Sine,
    Square,
    Triangle,
}

impl Waveform {
    pub fn from_selector(selector: u8) -> Self {
        match selector & 0b11 {
            0 => Self::Sine,
            1 => Self::Square,
            2 => Self::Triangle,
            _ => Self::Saw,
        }
    }
}

const fn const_sin_q15(i: usize) -> i16 {
    let half = WAVETABLE_SIZE / 2;
    let (x, sign): (i64, i64) = if i < half {
        (i as i64, 1)
    } else {
        ((i - half) as i64, -1)
    };

    let h = half as i64;
    let common = x * (h - x);
    let num = 16 * common;
    let den = 5 * h * h - 4 * common;

    if den == 0 {
        return 0;
    }

    let result = (num * 32767) / den;
    (result * sign) as i16
}

const fn generate_sine() -> [i16; WAVETABLE_SIZE] {
    let mut arr = [0i16; WAVETABLE_SIZE];
    let mut i = 0;
    while i < WAVETABLE_SIZE {
        // Sine peak 26754 (matches RMS of full-scale Saw/Triangle)
        let val = (const_sin_q15(i) as i32 * 26754 / 32767) as i16;
        arr[i] = val;
        i += 1;
    }
    arr
}

const fn generate_saw() -> [i16; WAVETABLE_SIZE] {
    let mut arr = [0i16; WAVETABLE_SIZE];
    let mut i = 0;
    while i < WAVETABLE_SIZE {
        // Full Scale Sawtooth
        let val = ((i as i64 * 65535) / (WAVETABLE_SIZE as i64 - 1)) - 32768;
        arr[i] = val as i16;
        i += 1;
    }
    arr
}

const fn generate_square() -> [i16; WAVETABLE_SIZE] {
    let mut arr = [0i16; WAVETABLE_SIZE];
    let mut i = 0;
    while i < WAVETABLE_SIZE {
        // Square peak 18918 (matches RMS of full-scale Saw/Triangle)
        arr[i] = if i < WAVETABLE_SIZE / 2 {
            18918
        } else {
            -18918
        };
        i += 1;
    }
    arr
}

const fn generate_triangle() -> [i16; WAVETABLE_SIZE] {
    let mut arr = [0i16; WAVETABLE_SIZE];
    let mut i = 0;
    let half = WAVETABLE_SIZE as i32 / 2;
    while i < WAVETABLE_SIZE {
        // Full Scale Triangle
        let x = i as i32;
        let val = if x < half {
            -32768 + (x * 65535 / half)
        } else {
            32767 - ((x - half) * 65535 / half)
        };
        arr[i] = val as i16;
        i += 1;
    }
    arr
}

#[repr(C, align(16))]
pub struct Wavetables {
    pub saw: [i16; WAVETABLE_SIZE],
    pub sine: [i16; WAVETABLE_SIZE],
    pub square: [i16; WAVETABLE_SIZE],
    pub triangle: [i16; WAVETABLE_SIZE],
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

    pub fn get(&'static self, waveform: &Waveform) -> &'static [i16; WAVETABLE_SIZE] {
        match waveform {
            Waveform::Saw => &self.saw,
            Waveform::Sine => &self.sine,
            Waveform::Square => &self.square,
            Waveform::Triangle => &self.triangle,
        }
    }
}

impl Default for Wavetables {
    fn default() -> Self {
        Self::new()
    }
}
