//! Audio processing modules

#[cfg(feature = "envelope")]
pub mod envelope;
#[cfg(feature = "filter")]
pub mod filter;
#[cfg(feature = "lfo")]
pub mod lfo;
pub mod oscillator;

#[cfg(feature = "envelope")]
pub use envelope::Envelope;
#[cfg(feature = "filter")]
pub use filter::Filter;
#[cfg(feature = "lfo")]
pub use lfo::LFO;
pub use oscillator::Oscillator;
