//! General purpose input / output

use core::marker::PhantomData;

/// Extension trait to split a GPIO peripheral in independent pins and
/// registers.
pub trait GpioExt {
    /// The parts to split the GPIO into.
    type Parts;

    /// Splits the GPIO block into independent pins and registers.
    fn split(self) -> Self::Parts;
}

/// HSIOM GPIO mode (type state)
pub struct GpioPinMode;

/// High impedance drive mode (type state)
pub struct HighZ;

/// Strong output drive mode
pub struct Strong;

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

// `i` -> port number
// `j` -> pin number
macro_rules! gpio {
    ([
     $($Pi_j:ident: ($pi_j:ident, $prti:ident, $j:expr, $MODE:ty)),+
    ]) => {
        use core::convert::Infallible;

        use embedded_hal::digital::v2::OutputPin;
        use psoc6_pac::GPIO;

        /// GPIO parts
        pub struct Parts {
            $(
                /// Pin
                pub $pi_j: $Pi_j<$MODE>,
            )+
        }

        impl GpioExt for GPIO {
            type Parts = Parts;

            fn split(self) -> Parts {
                Parts {
                    $(
                        $pi_j: $Pi_j { _mode: PhantomData },
                    )+
                }
            }
        }

        $(
            /// Pin
            pub struct $Pi_j<MODE> {
                _mode: PhantomData<MODE>,
            }

            impl<MODE> $Pi_j<MODE> {
                /// Configures the pin to operate as a strong output pin
                pub fn into_strong_output(self) -> $Pi_j<Output<Strong>> {
                    self.set_drive_mode(6);
                    $Pi_j { _mode: PhantomData }
                }

                /// Set the drive mode for the pin
                fn set_drive_mode(&self, bits: u8) {
                    unsafe { (*GPIO::ptr()).$prti.cfg.modify(|_, w| {
                        match $j {
                            0 => w.drive_mode0().bits(bits),
                            1 => w.drive_mode1().bits(bits),
                            2 => w.drive_mode2().bits(bits),
                            3 => w.drive_mode3().bits(bits),
                            4 => w.drive_mode4().bits(bits),
                            5 => w.drive_mode5().bits(bits),
                            6 => w.drive_mode6().bits(bits),
                            7 => w.drive_mode7().bits(bits),
                            _ => panic!(),
                        }
                    })}
                }
            }

            impl<MODE> OutputPin for $Pi_j<Output<MODE>> {
                type Error = Infallible;

                fn set_high(&mut self) -> Result<(), Self::Error> {
                    unsafe { (*GPIO::ptr()).$prti.out_set.write(|w| w.bits(1 << $j)) };
                    Ok(())
                }

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    unsafe { (*GPIO::ptr()).$prti.out_clr.write(|w| w.bits(1 << $j)) };
                    Ok(())
                }
            }
        )+
    };
}

gpio!([
    P0_0: (p0_0, prt0, 0, Input<HighZ>),
    P0_1: (p0_1, prt0, 1, Input<HighZ>),
    P0_2: (p0_2, prt0, 2, Input<HighZ>),
    P0_3: (p0_3, prt0, 3, Input<HighZ>),
    P0_4: (p0_4, prt0, 4, Input<HighZ>),
    P0_5: (p0_5, prt0, 5, Input<HighZ>),

    P1_0: (p1_0, prt1, 0, Input<HighZ>),
    P1_1: (p1_1, prt1, 1, Input<HighZ>),
    P1_2: (p1_2, prt1, 2, Input<HighZ>),
    P1_3: (p1_3, prt1, 3, Input<HighZ>),
    P1_4: (p1_4, prt1, 4, Input<HighZ>),
    P1_5: (p1_5, prt1, 5, Input<HighZ>),

    P5_0: (p5_0, prt5, 0, Input<HighZ>),
    P5_1: (p5_1, prt5, 1, Input<HighZ>),
    P5_2: (p5_2, prt5, 2, Input<HighZ>),
    P5_3: (p5_3, prt5, 3, Input<HighZ>),
    P5_4: (p5_4, prt5, 4, Input<HighZ>),
    P5_5: (p5_5, prt5, 5, Input<HighZ>),
    P5_6: (p5_6, prt5, 6, Input<HighZ>),
    P5_7: (p5_7, prt5, 7, Input<HighZ>),

    P6_0: (p6_0, prt6, 0, Input<HighZ>),
    P6_1: (p6_1, prt6, 1, Input<HighZ>),
    P6_2: (p6_2, prt6, 2, Input<HighZ>),
    P6_3: (p6_3, prt6, 3, Input<HighZ>),
    P6_4: (p6_4, prt6, 4, Input<HighZ>),
    P6_5: (p6_5, prt6, 5, Input<HighZ>),
    P6_6: (p6_6, prt6, 6, Input<HighZ>),
    P6_7: (p6_7, prt6, 7, Input<HighZ>),

    P7_0: (p7_0, prt7, 0, Input<HighZ>),
    P7_1: (p7_1, prt7, 1, Input<HighZ>),
    P7_2: (p7_2, prt7, 2, Input<HighZ>),
    P7_3: (p7_3, prt7, 3, Input<HighZ>),
    P7_4: (p7_4, prt7, 4, Input<HighZ>),
    P7_5: (p7_5, prt7, 5, Input<HighZ>),
    P7_6: (p7_6, prt7, 6, Input<HighZ>),
    P7_7: (p7_7, prt7, 7, Input<HighZ>),

    P8_0: (p8_0, prt8, 0, Input<HighZ>),
    P8_1: (p8_1, prt8, 1, Input<HighZ>),
    P8_2: (p8_2, prt8, 2, Input<HighZ>),
    P8_3: (p8_3, prt8, 3, Input<HighZ>),
    P8_4: (p8_4, prt8, 4, Input<HighZ>),
    P8_5: (p8_5, prt8, 5, Input<HighZ>),
    P8_6: (p8_6, prt8, 6, Input<HighZ>),
    P8_7: (p8_7, prt8, 7, Input<HighZ>),

    P9_0: (p9_0, prt9, 0, Input<HighZ>),
    P9_1: (p9_1, prt9, 1, Input<HighZ>),
    P9_2: (p9_2, prt9, 2, Input<HighZ>),
    P9_3: (p9_3, prt9, 3, Input<HighZ>),
    P9_4: (p9_4, prt9, 4, Input<HighZ>),
    P9_5: (p9_5, prt9, 5, Input<HighZ>),
    P9_6: (p9_6, prt9, 6, Input<HighZ>),
    P9_7: (p9_7, prt9, 7, Input<HighZ>),

    P10_0: (p10_0, prt10, 0, Input<HighZ>),
    P10_1: (p10_1, prt10, 1, Input<HighZ>),
    P10_2: (p10_2, prt10, 2, Input<HighZ>),
    P10_3: (p10_3, prt10, 3, Input<HighZ>),
    P10_4: (p10_4, prt10, 4, Input<HighZ>),
    P10_5: (p10_5, prt10, 5, Input<HighZ>),
    P10_6: (p10_6, prt10, 6, Input<HighZ>),
    P10_7: (p10_7, prt10, 7, Input<HighZ>),

    P11_0: (p11_0, prt11, 0, Input<HighZ>),
    P11_1: (p11_1, prt11, 1, Input<HighZ>),
    P11_2: (p11_2, prt11, 2, Input<HighZ>),
    P11_3: (p11_3, prt11, 3, Input<HighZ>),
    P11_4: (p11_4, prt11, 4, Input<HighZ>),
    P11_5: (p11_5, prt11, 5, Input<HighZ>),
    P11_6: (p11_6, prt11, 6, Input<HighZ>),
    P11_7: (p11_7, prt11, 7, Input<HighZ>),

    P12_0: (p12_0, prt12, 0, Input<HighZ>),
    P12_1: (p12_1, prt12, 1, Input<HighZ>),
    P12_2: (p12_2, prt12, 2, Input<HighZ>),
    P12_3: (p12_3, prt12, 3, Input<HighZ>),
    P12_4: (p12_4, prt12, 4, Input<HighZ>),
    P12_5: (p12_5, prt12, 5, Input<HighZ>),
    P12_6: (p12_6, prt12, 6, Input<HighZ>),
    P12_7: (p12_7, prt12, 7, Input<HighZ>),

    P13_0: (p13_0, prt13, 0, Input<HighZ>),
    P13_1: (p13_1, prt13, 1, Input<HighZ>),
    P13_2: (p13_2, prt13, 2, Input<HighZ>),
    P13_3: (p13_3, prt13, 3, Input<HighZ>),
    P13_4: (p13_4, prt13, 4, Input<HighZ>),
    P13_5: (p13_5, prt13, 5, Input<HighZ>),
    P13_6: (p13_6, prt13, 6, Input<HighZ>),
    P13_7: (p13_7, prt13, 7, Input<HighZ>)
]);
