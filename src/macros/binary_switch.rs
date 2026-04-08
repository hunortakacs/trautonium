macro_rules! setup_binary_switch_inputs {
    (
        $struct_name:ident,
        {
            $( $name:ident: { msb: $msb_pin:ident, lsb: $lsb_pin:ident } ),* $(,)?
        }
    ) => {
        paste::paste! {
            pub struct $struct_name<'a> {
                $(
                    [<$name _msb>]: esp_hal::gpio::Input<'a>,
                    [<$name _lsb>]: esp_hal::gpio::Input<'a>,
                )*
            }

            impl<'a> $struct_name<'a> {
                pub fn new(
                    $(
                        [<$name _msb_pin>]: esp_hal::peripherals::$msb_pin<'a>,
                        [<$name _lsb_pin>]: esp_hal::peripherals::$lsb_pin<'a>,
                    )*
                ) -> Self {
                    let cfg = esp_hal::gpio::InputConfig::default().with_pull(esp_hal::gpio::Pull::Up);
                    Self {
                        $(
                            [<$name _msb>]: esp_hal::gpio::Input::new([<$name _msb_pin>], cfg),
                            [<$name _lsb>]: esp_hal::gpio::Input::new([<$name _lsb_pin>], cfg),
                        )*
                    }
                }

                fn read_binary(msb: &esp_hal::gpio::Input<'a>, lsb: &esp_hal::gpio::Input<'a>) -> u8 {
                    let msb_bit = msb.is_low();
                    let lsb_bit = lsb.is_low();
                    ((msb_bit as u8) << 1) | (lsb_bit as u8)
                }

                $(
                    pub fn [<$name _state>](&self) -> u8 {
                        Self::read_binary(&self.[<$name _msb>], &self.[<$name _lsb>])
                    }
                )*
            }
        }
    };
}
