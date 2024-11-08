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

// or

// #![no_std]
// #![no_main]

// use arduino_hal::prelude::*;
// use arduino_hal::Delay;
// use panic_halt as _;

// #[arduino_hal::entry]
// fn main() -> ! {
//     let dp = arduino_hal::Peripherals::take().unwrap();
//     let pins = arduino_hal::pins!(dp);
//     let mut pin6 = pins.d6.into_output();
//     let pin7 = pins.d7.into_floating_input(); // Set pin 7 as an input
//     let mut delay = Delay::new();

//     // Set up serial communication
//     let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

//     // Define initial position pulse width (in microseconds)
//     let initial_position = 1500u16; // 1.5ms pulse width (middle position)
//     let target_position = 3000u16;  // Target position pulse width (e.g., 2ms)

//     loop {
//         // Move from initial position to target position
//         let mut current_target_position = target_position; // Variable to track current target position

//         // Move to the target position
//         for pulse_width in (initial_position..=target_position).step_by(10) {
//             // Set servo high
//             pin6.set_high();
//             delay.delay_us(pulse_width);  // Hold high for current pulse width
//             pin6.set_low();
//             delay.delay_ms(20 - (pulse_width / 1000) as u16);  // Rest of the 20ms period
//             delay.delay_ms(10u16);  // Small delay for smooth motion

//             // Print pulse_width to the console
//             ufmt::uwriteln!(&mut serial, "Pulse Width: {} us, Pin 7 State: {}", pulse_width, pin7.is_high()).unwrap();

//             // Check if pin 7 is high
//             if pin7.is_high() {
//                 // Stop at current pulse width
//                 current_target_position = pulse_width; 
//                 ufmt::uwriteln!(&mut serial, "Pin 7 HIGH detected. Stopping and waiting.").unwrap();
                
//                 // Wait for 3 seconds (3000ms)
//                 delay.delay_ms(3000u16);
                
//                 // Move back to the initial position after waiting
//                 break;
//             }
//         }

//         // Move back from the current target position to initial position
//         for pulse_width in (current_target_position..=initial_position).rev().step_by(10) {
//             // Set servo high
//             pin6.set_high();
//             delay.delay_us(pulse_width);  // Hold high for current pulse width
//             pin6.set_low();
//             delay.delay_ms(20 - (pulse_width / 1000) as u16);  // Rest of the 20ms period
//             delay.delay_ms(10u16);  // Small delay for smooth motion

//             // Print pulse_width to the console
//             ufmt::uwriteln!(&mut serial, "Pulse Width: {} us, Pin 7 State: {}", pulse_width, pin7.is_high()).unwrap();
//         }
//     }
// }

