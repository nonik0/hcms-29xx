#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use hcms_29xx::UnconfiguredPin;
use panic_halt as _;

const MESSAGE: &[u8] = b"Hello from Rust on Arduino!";
const NUM_CHARS: usize = 8;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut display = hcms_29xx::Hcms29xx::<NUM_CHARS, _, _, _, _>::new(
        pins.d2.into_output(), // Data pin
        pins.d3.into_output(), // RS pin
        pins.d4.into_output(), // Clock pin
        pins.d5.into_output(), // CE pin
        // if optional pins not specified, logic levels should set elsewhere
        UnconfiguredPin,       // Optional: Blank pin
        UnconfiguredPin,       // Optional: OscSel pin
        UnconfiguredPin,       // Optional: Reset pin
    )
    .unwrap();

    display.begin().unwrap();
    display.display_unblank().unwrap();

    ufmt::uwriteln!(&mut serial, "Counting down from 1000 to 0!").unwrap_infallible();
    for count in (0..1000).rev() {
        display.print_i32(count).unwrap();
        arduino_hal::delay_ms(1);
    }

    ufmt::uwriteln!(&mut serial, "Showing scrolling message").unwrap_infallible();
    let mut cursor: usize = 0;
    loop {
        let mut buf = [0; NUM_CHARS];

        for i in 0..NUM_CHARS {
            let index = (cursor + i as usize) % MESSAGE.len();
            buf[i as usize] = MESSAGE[index];
        }

        display.print_ascii_bytes(&buf).unwrap();
        cursor = (cursor + 1) % MESSAGE.len();
        arduino_hal::delay_ms(300);
    }
}
