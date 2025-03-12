# HCMS-29xx Driver

[![Crates.io](https://img.shields.io/crates/v/hcms-29xx)](https://crates.io/crates/hcms-29xx)
[![Crates.io](https://img.shields.io/crates/d/hcms-29xx)](https://crates.io/crates/hcms-29xx)
[![docs.rs](https://img.shields.io/docsrs/hcms-29xx)](https://docs.rs/hcms-29xx/latest/hcms-29xx/)

[![lint](https://github.com/nonik0/hcms-29xx/actions/workflows/lint.yml/badge.svg)](https://github.com/nonik0/hcms-29xx/actions/workflows/lint.yml)
[![build](https://github.com/nonik0/hcms-29xx/actions/workflows/build.yml/badge.svg)](https://github.com/nonik0/hcms-29xx/actions/workflows/build.yml)

A platform agnostic driver for [HCMS-29xx](https://docs.broadcom.com/doc/HCMS-29xx-Series-High-Performance-CMOS-5-x-7-Alphanumeric-Displays) and [HCMS-39xx](https://docs.broadcom.com/doc/AV02-0868EN) display ICs. Many thanks to @Andy4495's existing [HCMS39xx](https://github.com/Andy4495/HCMS39xx) Arduino/C++ library, which I used as a reference implementation as well as for the font data.

## Features:
 * Single dependency on embedded-hal v1.0
 * Optional dependency on avr-progmem for AVR targets to store font data in PROGMEM (requires nightly toolchain)
 * Examples for:
     * [Arduino Uno](examples/arduino-uno/), based on [avr-hal](https://github.com/Rahix/avr-hal/)
     * [ESP32-S3](examples/esp32-s3/), based on [esp-hal](https://github.com/esp-rs/esp-hal)

## Install
To install this driver in your project, add the following line to your `Cargo.toml`'s `dependencies` table:

```toml
hcms-29xx = "0.1.0"
```

For AVR targets:W

```toml
hcms-29xx = { "0.1.0", features=["avr-progmem"] }
```

## How to Use

The HCMS-29xx/HCMS-39xx displays require a minimum of four pins to control: Data (Din), Register Select (RS), Clock (CLK), and Chip Enable (CE). The other pins, Blank (BL), Oscillator Select (SEL), and Reset (RST), are optional. If not given, the optional pins' logic levels must be set appropriately, typically BL low, SEL high, and RST high.

Specifying only required pins:

```rust
const NUM_CHARS: usize = 8;

let mut display = hcms_29xx::Hcms29xx::<NUM_CHARS, _, _, _, _>::new(
    HalOutputPin1,   // Data pin
    HalOutputPin2,   // RS pin
    HalOutputPin3,   // Clock pin
    HalOutputPin4,   // CE pin
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
display.print_ascii_bytes(b"hello!").unwrap();
```

Specifying all pins:

```rust
const NUM_CHARS: usize = 8;

let mut display = hcms_29xx::Hcms29xx::<NUM_CHARS, _, _, _, _, _, _, _>::new(
    HalOutputPin1, // Data pin
    HalOutputPin2, // RS pin
    HalOutputPin3, // Clock pin
    HalOutputPin4, // CE pin
    HalOutputPin5, // Optional: Blank pin
    HalOutputPin6, // Optional: OscSel pin
    HalOutputPin7, // Optional: Reset pin
)
.unwrap();

display.begin().unwrap();
display.display_unblank().unwrap();
display.print_ascii_bytes(b"goodbye!").unwrap();
```

## TODO
- [ ] Improve generic type interface, e.g. UnconfiguredPin improvements, better constructor, etc.
- [ ] Improve function signatures, e.g. generic implementation for integer print functions
- [ ] Katakana font