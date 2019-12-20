//! Blinky example for the CY8CPROTO-063-BLE

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;
extern crate psoc6_hal;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use psoc6_hal::gpio::GpioExt;
use psoc6_hal::hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let p = psoc6_pac::Peripherals::take().unwrap();
    let gpio = p.GPIO.split();

    let mut led3 = gpio.p6_3.into_strong_output();
    let mut led4 = gpio.p7_1.into_strong_output();

    loop {
        hprintln!("red led low").unwrap();
        led3.set_low().unwrap();
        led4.set_high().unwrap();
        for _ in 0..50000 {}

        hprintln!("red led high").unwrap();
        led3.set_high().unwrap();
        led4.set_low().unwrap();
        for _ in 0..50000 {}
    }
}
