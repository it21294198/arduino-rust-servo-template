#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut _pin6 = pins.d6.into_output();  // Configure pin 6 as output

    // Configure Timer0 for Fast PWM on pin 6 (OC0A)
    let tc0 = dp.TC0;
    // Set Fast PWM mode (WGM02:0 = 011), Clear OC0A on Compare Match (COM0A1:0 = 10)
    tc0.tccr0a.write(|w| {
        w.wgm0().bits(0b11)   // Fast PWM, WGM00 and WGM01 set
         .com0a().match_clear()  // Clear OC0A on Compare Match
    });
    tc0.tccr0b.write(|w| {
        w.wgm02().bit(false)  // Fast PWM, WGM02 cleared
         .cs0().prescale_64()  // Prescaler set to 64
    });

    // Calculate delay for 3-second fade
    const STEP_DELAY: u16 = 12; // 12ms per step for smooth fading

    loop {
        // Fade in over 3 seconds
        for duty in 0u8..=255u8 {
            tc0.ocr0a.write(|w| { w.bits(duty) }); // Set PWM duty cycle
            arduino_hal::delay_ms(STEP_DELAY);
        }

        // Fade out over 3 seconds
        for duty in (0u8..=255u8).rev() {
            tc0.ocr0a.write(|w| { w.bits(duty) }); // Set PWM duty cycle
            arduino_hal::delay_ms(STEP_DELAY);
        }
    }
}

