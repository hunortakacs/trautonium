//! Audio processing modules

pub mod envelope;
pub mod filter;
pub mod lfo;
pub mod oscillator;

pub use envelope::Envelope;
pub use filter::Filter;
pub use lfo::LFO;
pub use oscillator::Oscillator;
