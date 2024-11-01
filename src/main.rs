#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use arduino_hal::Delay;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut pin6 = pins.d6.into_output();
    let mut delay = Delay::new();

    loop {
        // Gradually move from minimum (1ms) to maximum (2ms) position
        for pulse_width in (1000u16..=2000u16).step_by(10) {
            pin6.set_high();
            delay.delay_us(pulse_width);  // Hold high for current pulse width
            pin6.set_low();
            delay.delay_ms(20 - (pulse_width / 1000) as u16);  // Rest of the 20ms period
            delay.delay_ms(10u16);  // Small delay for smooth motion
        }

        // Gradually move back from maximum (2ms) to minimum (1ms) position
        for pulse_width in (1000u16..=2000u16).rev().step_by(10) {
            pin6.set_high();
            delay.delay_us(pulse_width);  // Hold high for current pulse width
            pin6.set_low();
            delay.delay_ms(20 - (pulse_width / 1000) as u16);  // Rest of the 20ms period
            delay.delay_ms(10u16);  // Small delay for smooth motion
        }
    }
}

