macro_rules! setup_mux_adc_inputs {
    (
        $struct_name:ident,
        {
            mux_select: {
                a: $sel_a_pin:ident,
                b: $sel_b_pin:ident,
                c: $sel_c_pin:ident
            },
            analog_in: {
                mux0: $mux0_pin:ident,
                mux1: $mux1_pin:ident
            },
            controls: {
                $( $name:ident: { a: $a:expr, b: $b:expr, c: $c:expr, mux: $mux:expr } ),* $(,)?
            }
        }
    ) => {
        paste::paste! {
            pub struct $struct_name<'a> {
                adc1: esp_hal::analog::adc::Adc<'a, esp_hal::peripherals::ADC1<'a>, esp_hal::Blocking>,
                mux0_analog: esp_hal::analog::adc::AdcPin<esp_hal::peripherals::$mux0_pin<'a>, esp_hal::peripherals::ADC1<'a>>,
                mux1_analog: esp_hal::analog::adc::AdcPin<esp_hal::peripherals::$mux1_pin<'a>, esp_hal::peripherals::ADC1<'a>>,
                sel_c: esp_hal::gpio::Output<'a>,
                sel_b: esp_hal::gpio::Output<'a>,
                sel_a: esp_hal::gpio::Output<'a>,
                delay: esp_hal::delay::Delay,
                $( [<$name _smoothed>]: u32, )*
            }

            impl<'a> $struct_name<'a> {
                pub fn new(
                    adc1_periph: esp_hal::peripherals::ADC1<'a>,
                    sel_a_pin: esp_hal::peripherals::$sel_a_pin<'a>,
                    sel_b_pin: esp_hal::peripherals::$sel_b_pin<'a>,
                    sel_c_pin: esp_hal::peripherals::$sel_c_pin<'a>,
                    mux0_pin: esp_hal::peripherals::$mux0_pin<'a>,
                    mux1_pin: esp_hal::peripherals::$mux1_pin<'a>,
                ) -> Self {
                    let mut adc1_cfg = esp_hal::analog::adc::AdcConfig::new();
                    let mux0_analog = adc1_cfg.enable_pin(mux0_pin, esp_hal::analog::adc::Attenuation::_11dB);
                    let mux1_analog = adc1_cfg.enable_pin(mux1_pin, esp_hal::analog::adc::Attenuation::_11dB);

                    let mut instance = Self {
                        adc1: esp_hal::analog::adc::Adc::new(adc1_periph, adc1_cfg),
                        mux0_analog,
                        mux1_analog,
                        sel_a: esp_hal::gpio::Output::new(sel_a_pin, esp_hal::gpio::Level::Low, esp_hal::gpio::OutputConfig::default()),
                        sel_b: esp_hal::gpio::Output::new(sel_b_pin, esp_hal::gpio::Level::Low, esp_hal::gpio::OutputConfig::default()),
                        sel_c: esp_hal::gpio::Output::new(sel_c_pin, esp_hal::gpio::Level::Low, esp_hal::gpio::OutputConfig::default()),
                        delay: esp_hal::delay::Delay::new(),
                        $( [<$name _smoothed>]: 0, )*
                    };

                    instance.prime_buffers();
                    instance
                }

                fn select_mux_channel(&mut self, a: bool, b: bool, c: bool) {
                    if a {
                        self.sel_a.set_high();
                    } else {
                        self.sel_a.set_low();
                    }
                    if b {
                        self.sel_b.set_high();
                    } else {
                        self.sel_b.set_low();
                    }
                    if c {
                        self.sel_c.set_high();
                    } else {
                        self.sel_c.set_low();
                    }
                    self.delay.delay_micros($crate::config::MUX_SETTLE_DELAY_US);
                }

                fn read_mux_raw(&mut self, mux: usize, a: bool, b: bool, c: bool) -> u16 {
                    self.select_mux_channel(a, b, c);
                    match mux {
                        0 => {
                            let _ = nb::block!(self.adc1.read_oneshot(&mut self.mux0_analog)).unwrap();
                            nb::block!(self.adc1.read_oneshot(&mut self.mux0_analog)).unwrap()
                        }
                        1 => {
                            let _ = nb::block!(self.adc1.read_oneshot(&mut self.mux1_analog)).unwrap();
                            nb::block!(self.adc1.read_oneshot(&mut self.mux1_analog)).unwrap()
                        }
                        _ => panic!("invalid mux index"),
                    }
                }

                fn smooth_and_normalize(prev: &mut u32, raw: u16) -> f32 {
                    let alpha = $crate::config::ADC_SMOOTHING_ALPHA;
                    let smoothed = alpha * (raw as f32) + (1.0 - alpha) * (*prev as f32);
                    *prev = smoothed as u32;

                    1.0 - (smoothed / $crate::config::ADC_MAX_VALUE)
                }

                fn prime_buffers(&mut self) {
                    for _ in 0..16 {
                        $( self.[<$name _normalized>](); )*
                    }
                }

                $(
                    pub fn [<$name _normalized>](&mut self) -> f32 {
                        let raw = self.read_mux_raw($mux, $a != 0, $b != 0, $c != 0);
                        Self::smooth_and_normalize(&mut self.[<$name _smoothed>], raw)
                    }
                )*
            }
        }
    };
}
