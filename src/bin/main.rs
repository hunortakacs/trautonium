#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types."
)]
#![deny(clippy::large_stack_frames)]

use core::ptr::addr_of_mut;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::i2s::master::{Channels, Config as I2sConfig, I2s};
use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::main;
use esp_hal::system::Stack;
use esp_hal::timer::timg::TimerGroup;
use trautonium::config::{AUDIO_BUFFER_SIZE, AUDIO_CORE_STACK_SIZE, AdcInputs, SwitchInputs};
use trautonium::controls::Controls;
#[cfg(any(feature = "delay", feature = "reverb"))]
use trautonium::effects::EffectEngine;
use trautonium::voice::Voice;
use trautonium::wavetables::Wavetables;
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[unsafe(link_section = ".data")]
static WAVETABLES: Wavetables = Wavetables::new();
static mut AUDIO_CORE_STACK: Stack<AUDIO_CORE_STACK_SIZE> = Stack::new();
static CONTROLS_SIGNAL: Signal<CriticalSectionRawMutex, Controls> = Signal::new();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 80 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_ints = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_ints.software_interrupt0);

    let i2s0 = peripherals.I2S0;
    let dma_i2s0 = peripherals.DMA_I2S0;
    let bclk = peripherals.GPIO14;
    let ws = peripherals.GPIO13;
    let dout = peripherals.GPIO12;

    esp_rtos::start_second_core(
        peripherals.CPU_CTRL,
        sw_ints.software_interrupt1,
        unsafe { &mut *addr_of_mut!(AUDIO_CORE_STACK) },
        move || {
            const DMA_BUFFER_SIZE: usize = AUDIO_BUFFER_SIZE * 4 * 2;
            let (_, tx_descriptors) = esp_hal::dma_circular_descriptors!(0, DMA_BUFFER_SIZE);

            let i2s = I2s::new(
                i2s0,
                dma_i2s0,
                I2sConfig::new_tdm_philips()
                    .with_channels(Channels::STEREO)
                    .with_data_format(esp_hal::i2s::master::DataFormat::Data16Channel16),
            )
            .expect("Failed to initialize I2S");

            let mut i2s_tx = i2s
                .i2s_tx
                .with_bclk(bclk)
                .with_ws(ws)
                .with_dout(dout)
                .build(tx_descriptors);

            #[repr(align(4))]
            struct AlignedBuffer([u8; DMA_BUFFER_SIZE]);
            let frame_bytes = AlignedBuffer([0u8; DMA_BUFFER_SIZE]);
            let mut transfer = i2s_tx
                .write_dma_circular(&frame_bytes.0)
                .expect("Failed to start circular DMA");

            let mut voice = Voice::new(&WAVETABLES);
            #[cfg(any(feature = "delay", feature = "reverb"))]
            let mut effects = EffectEngine::new();
            let mut pipeline_buf = [0i16; AUDIO_BUFFER_SIZE];
            let mut controls = Controls::new_const();

            esp_println::println!("Audio core: pipeline active");

            loop {
                if let Some(new_controls) = CONTROLS_SIGNAL.try_take() {
                    controls = new_controls;
                    voice.update_controls(&controls, &WAVETABLES);
                    #[cfg(any(feature = "delay", feature = "reverb"))]
                    effects.update_params(&controls);
                }

                pipeline_buf.fill(0);
                voice.process_buffer(&mut pipeline_buf, &controls);
                #[cfg(any(feature = "delay", feature = "reverb"))]
                effects.process_buffer(&mut pipeline_buf);

                let mut samples_pushed = 0;
                while samples_pushed < AUDIO_BUFFER_SIZE {
                    transfer
                        .push_with(|buf| {
                            let space_in_dma = buf.len() / 4;
                            let remaining = AUDIO_BUFFER_SIZE - samples_pushed;
                            let to_copy = core::cmp::min(space_in_dma, remaining);

                            for i in 0..to_copy {
                                let sample = pipeline_buf[samples_pushed + i];
                                let bytes = sample.to_le_bytes();
                                let base = i << 2;
                                buf[base] = bytes[0];
                                buf[base + 1] = bytes[1];
                                buf[base + 2] = bytes[0];
                                buf[base + 3] = bytes[1];
                            }

                            samples_pushed += to_copy;
                            to_copy * 4
                        })
                        .expect("push_with failed");
                }
            }
        },
    );

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

    let mut local_controls = Controls::new_const();
    let delay = Delay::new();

    local_controls.read(&mut adc_inputs, &switch_inputs);
    loop {
        local_controls.read(&mut adc_inputs, &switch_inputs);
        CONTROLS_SIGNAL.signal(local_controls.clone());
        delay.delay_millis(1);
    }
}
