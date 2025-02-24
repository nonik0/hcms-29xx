#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Level, Output},
    main,
};
use log::info;

const NUM_CHARS: usize = 8;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    let mut display = hcms_29xx::Hcms29xx::<_, _, _, _, _, _, _, NUM_CHARS>::new(
        Output::new(peripherals.GPIO35, Level::Low),   // Data pin
        Output::new(peripherals.GPIO37, Level::Low),   // RS pin
        Output::new(peripherals.GPIO36, Level::Low),   // Clock pin
        Output::new(peripherals.GPIO34, Level::Low),   // CE pin
        // optional pins logic levels set in hardware
        Some(Output::new(peripherals.GPIO2, Level::Low)),   // CE pin,
        Some(Output::new(peripherals.GPIO3, Level::Low)),   // CE pin,
        Some(Output::new(peripherals.GPIO4, Level::Low)),   // CE pin,
    )
    .unwrap();
    display.begin().unwrap();
    display.display_unblank().unwrap();
    display.set_peak_current(hcms_29xx::PeakCurrent::Max12_8Ma).unwrap();
    display.set_brightness(15).unwrap();

    let mut count: i16 = 1000;
    let delay = Delay::new();
    loop {
        info!("{}", count);
        display.print_i32(count as i32).unwrap();
        count -= 1;
        delay.delay_millis(10);
    }
}
