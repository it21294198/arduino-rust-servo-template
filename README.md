arduino-rust-pwm-template
=========================
[`avr-hal`](https://github.com/Rahix/avr-hal#readme)

[`ravedude`](https://crates.io/crates/ravedude)

Rust project for the _Arduino Uno_.

## About the Project

1. This is example usage of PWM singles on different Hardware components.
2. Here Digital pin 6 is used as the example. To see the effect use LED bulb with 220 ohms resistor to Pin 6 and ground.
3. To run,
   ```bash
   cagro run
   ```

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.


