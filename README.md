# HCMS-29xx Driver

[![Crates.io](https://img.shields.io/crates/v/hcms-29xx)](https://crates.io/crates/hcms-29xx)
[![Crates.io](https://img.shields.io/crates/d/hcms-29xx)](https://crates.io/crates/hcms-29xx)
[![docs.rs](https://img.shields.io/docsrs/hcms-29xx)](https://docs.rs/hcms-29xx/latest/hcms-29xx/)

[![lint](https://github.com/nonik0/hcms-29xx/actions/workflows/lint.yml/badge.svg)](https://github.com/nonik0/hcms-29xx/actions/workflows/lint.yml)
[![build](https://github.com/nonik0/hcms-29xx/actions/workflows/build.yml/badge.svg)](https://github.com/nonik0/hcms-29xx/actions/workflows/build.yml)

Driver for [HCMS-29XX](https://docs.broadcom.com/doc/HCMS-29xx-Series-High-Performance-CMOS-5-x-7-Alphanumeric-Displays) and [HCMS-39XX](https://docs.broadcom.com/doc/AV02-0868EN) display ICs.  Many thanks for @Andy4495's existing [HCMS39XX](https://github.com/Andy4495/HCMS39xx) Arduino/C++ library, which I used for a reference implementation as well as the font data.

## Features:
 * Single dependency on embedded-hal v1.0
 * Optional dependency on avr-progmem for AVR targets to store font data in PROGMEM (requires nightly toolchain)
 * Example for Arduino Uno using avr-hal TODO
 * Example for ESP32S3 using esp-hal TODO

## Install
To install this driver in your project add the following line to your `Cargo.toml`'s `dependencies` table:

```toml
hcms-29xx = "0.1.0"
```

```tom;
hcms-29xx { "0.1.0", features=["avr-progmem"] }

## TODO
- [ ] Arduino Uno sample
- [ ] Test on other hardware, add feature flags if needed for specific functionality (e.g. progmem for AVR)
- [ ] Katakana font