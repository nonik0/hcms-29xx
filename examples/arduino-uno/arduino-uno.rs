#![no_std]
#![no_main]

use panic_halt as _;

const NUM_CHARS: usize = 8;
const MESSAGE: &[u8] = b"Hello from Rust on Arduino!";

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // modify pin assignments to match wiring, if optional pins are not specified the logic
    // level must set as appropriate externally with pull-up/down resistors or other means
    let mut display = hcms29xx::Hcms29xx::new(
        NUM_CHARS,                               // Number of characters in the display
        pins.d0.into_output().downgrade(),       // Data pin
        pins.d1.into_output().downgrade(),       // Clock pin
        pins.d2.into_output().downgrade(),       // Chip select pin
        pins.d3.into_output().downgrade(),       // Reset pin
        Some(pins.d4.into_output().downgrade()), // Optional: Enable pin
        Some(pins.d5.into_output().downgrade()), // Optional: Write pin
        Some(pins.d6.into_output().downgrade()), // Optional: Read pin
    )
    .unwrap();

    display.begin().unwrap();
    display.display_unblank().unwrap();

    display.set_current(1).unwrap(); // set current (0-3) to 1
    //display.set_brightness(15).unwrap(); // set brightness (0-15) to max
    //display.set_int_osc().unwrap(); // set internal oscillator (default internal)

    // show a scrolling message, wrapping around at the end
    let mut cursor: usize = 0;
    loop {
        let mut buf = [0; NUM_CHARS];

        for i in 0..NUM_CHARS {
            let index = (cursor + i as usize) % MESSAGE.len();
            buf[i as usize] = MESSAGE[index];
        }

        display.print_c_string(&buf).unwrap();
        cursor = (cursor + 1) % MESSAGE.len();
        arduino_hal::delay_ms(30);
    }
}
