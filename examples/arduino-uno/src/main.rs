#![no_std]
#![no_main]

use panic_halt as _;

const NUM_CHARS: usize = 8;
const MESSAGE: &[u8] = b"Hello from Rust on Arduino!";

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut display = hcms_29xx::Hcms29xx::<_, _, _, _, _, _, _, NUM_CHARS>::new(
        pins.d0.into_output(),       // Data pin
        pins.d1.into_output(),       // RS pin
        pins.d2.into_output(),       // Clock pin
        pins.d3.into_output(),       // CE pin
        Some(pins.d4.into_output()), // Optional: Blank pin
        Some(pins.d5.into_output()), // Optional: OscSel pin
        Some(pins.d6.into_output()), // Optional: Reset pin
    )
    .unwrap();

    display.begin().unwrap();
    display.display_unblank().unwrap();


    // show a scrolling message, wrapping around at the end
    let mut cursor: usize = 0;
    loop {
        let mut buf = [0; NUM_CHARS];

        for i in 0..NUM_CHARS {
            let index = (cursor + i as usize) % MESSAGE.len();
            buf[i as usize] = MESSAGE[index];
        }

        display.print_ascii_bytes(&buf).unwrap();
        cursor = (cursor + 1) % MESSAGE.len();
        arduino_hal::delay_ms(30);
    }
}
