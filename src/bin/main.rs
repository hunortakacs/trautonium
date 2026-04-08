#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use allocator_api2::vec;
use esp_hal::clock::CpuClock;
use esp_hal::dma_descriptors;
use esp_hal::i2s::master::{Channels, Config as I2sConfig, DataFormat, I2s};
use esp_hal::main;
use esp_hal::time::Rate;
use trautonium_2::config::{AUDIO_BUFFER_SIZE, AUDIO_SAMPLE_RATE, AdcInputs, SwitchInputs};
use trautonium_2::controls::Controls;
use trautonium_2::effects::EffectEngine;
use trautonium_2::voice::Voice;
use trautonium_2::wavetables::Wavetables;
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.2.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 96 * 1024);

    static WAVETABLES: Wavetables = Wavetables::new();
    let mut voice = Voice::new(&WAVETABLES);
    let mut effects = EffectEngine::new();
    let mut controls = Controls::new();
    let mut adc_inputs = AdcInputs::new(
        peripherals.ADC1,
        peripherals.GPIO25,
        peripherals.GPIO33,
        peripherals.GPIO32,
        peripherals.GPIO36,
        peripherals.GPIO39,
    );
    let switch_inputs = SwitchInputs::new(
        peripherals.GPIO18,
        peripherals.GPIO19,
        peripherals.GPIO27,
        peripherals.GPIO26,
        peripherals.GPIO22,
        peripherals.GPIO23,
    );

    let (_, tx_descriptors) = dma_descriptors!(0, AUDIO_BUFFER_SIZE * 4);
    let i2s = I2s::new(
        peripherals.I2S0,
        peripherals.DMA_I2S0,
        I2sConfig::new_tdm_philips()
            .with_sample_rate(Rate::from_hz(AUDIO_SAMPLE_RATE))
            .with_data_format(DataFormat::Data16Channel16)
            .with_channels(Channels::STEREO),
    )
    .expect("Failed to initialize I2S");

    let mut i2s_tx = i2s
        .i2s_tx
        .with_bclk(peripherals.GPIO14)
        .with_ws(peripherals.GPIO13)
        .with_dout(peripherals.GPIO12)
        .build(tx_descriptors);

    let mut buffer = vec![0.0; AUDIO_BUFFER_SIZE];
    let mut i2s_frames = [0i16; AUDIO_BUFFER_SIZE * 2];

    // Main audio loop
    loop {
        controls.read(&mut adc_inputs, &switch_inputs);
        voice.update_controls(&controls, &WAVETABLES);
        effects.update_params(&controls);

        // Generate mono audio
        voice.process_buffer(&mut buffer);

        // Apply effects in mono
        effects.process_buffer(&mut buffer);

        // Apply master volume
        for sample in buffer.iter_mut() {
            *sample *= controls.master_volume;
        }

        for (i, sample) in buffer.iter().enumerate() {
            let pcm = (*sample).clamp(-1.0, 1.0) * 32_767.0;
            let pcm = pcm as i16;
            let idx = i * 2;
            i2s_frames[idx] = pcm;
            i2s_frames[idx + 1] = pcm;
        }

        i2s_tx
            .write_words(&i2s_frames)
            .expect("Failed to write I2S audio frame");
    }
}
