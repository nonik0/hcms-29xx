#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Level, Output},
    main,
};
use hcms_29xx::UnconfiguredPin;
use log::info;

const MESSAGE: &[u8] = b"Hello from Rust on ESP32-S3! ";
const NUM_CHARS: usize = 8;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();
    let mut display = hcms_29xx::Hcms29xx::<NUM_CHARS, _, _, _, _, _>::new(
        Output::new(peripherals.GPIO35, Level::Low), // Data pin
        Output::new(peripherals.GPIO37, Level::Low), // RS pin
        Output::new(peripherals.GPIO36, Level::Low), // Clock pin
        Output::new(peripherals.GPIO34, Level::Low), // CE pin
        // if optional pins not specified, logic levels should set elsewhere
        UnconfiguredPin, // Optional: Blank pin
        UnconfiguredPin, // Optional: OscSel pin
        UnconfiguredPin, // Optional: Reset pin
    )
    .unwrap();
    display.begin().unwrap();
    display.display_unblank().unwrap();
    display
        .set_peak_current(hcms_29xx::PeakCurrent::Max12_8Ma)
        .unwrap();
    display.set_brightness(15).unwrap();

    info!("Counting down from 1000 to 0!");
    for count in (0..1000).rev() {
        display.print_i32(count).unwrap();
        delay.delay_millis(1);
    }

    info!("Showing scrolling message");
    let mut cursor: usize = 0;
    loop {
        let mut buf = [0; NUM_CHARS];

        for i in 0..NUM_CHARS {
            let index = (cursor + i as usize) % MESSAGE.len();
            buf[i as usize] = MESSAGE[index];
        }

        display.print_ascii_bytes(&buf).unwrap();
        cursor = (cursor + 1) % MESSAGE.len();
        delay.delay_millis(300);
    }
}
