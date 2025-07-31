#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
};
use hcms_29xx::UnconfiguredPin;
use log::info;

const MESSAGE: &[u8] = b"Hello from Rust on ESP32-C3! ";
const NUM_CHARS: usize = 8;

esp_bootloader_esp_idf::esp_app_desc!();

fn delay(duration_ms: u64) {
    let delay_start = Instant::now();
    let duration = Duration::from_millis(duration_ms);
    while delay_start.elapsed() < duration {}
}

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut display = hcms_29xx::Hcms29xx::<NUM_CHARS, _, _, _, _, _>::new(
        Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default()), // Data pin
        Output::new(peripherals.GPIO6, Level::Low, OutputConfig::default()), // RS pin
        Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default()), // Clock pin
        Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default()), // CE pin
        // if optional pins not specified, logic levels should set elsewhere
        UnconfiguredPin, // Optional: Blank pin
        UnconfiguredPin, // Optional: OscSel pin
        UnconfiguredPin, // Optional: Reset pin
    )
    .unwrap();

    display.begin().unwrap();
    display.display_unblank().unwrap();
    display
        .set_peak_current(hcms_29xx::PeakCurrent::Max6_4Ma)
        .unwrap();
    display.set_brightness(10).unwrap();

    info!("Counting down from 1000 to 0!");
    for count in (0..=1000).rev() {
        display.print_int(count).unwrap();
        delay(1);
    }

    info!("Do these float your boat?");
    let floats = [
        3.1415,
        2.7182,
        -1.234,
        0.0,
        f32::NAN,
        f32::INFINITY,
        -f32::INFINITY,
    ];
    for f in floats {
        display.print_float(f, 3).unwrap();
        delay(300);
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
        delay(300);
    }
}
