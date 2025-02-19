pub const CHAR_WIDTH: usize = 5;
pub const CHAR_HEIGHT: usize = 7;
pub const DEVICE_CHARS: usize = 4;

pub const CONTROL_WORD_SELECT_BIT: u8 = 0b1000_0000; // low: control word 0, high: control word 1

pub mod control_word_0 {
    pub const BRIGHTNESS_MASK: u8 = 0b0000_1111;
    pub const CURRENT_MASK: u8 = 0b0011_0000;
    pub const WAKE_BIT: u8 = 0b0100_0000;

    pub const MAX_BRIGHTNESS: u8 = 15;
    pub const DEFAULT_BRIGHTNESS: u8 = 12;
    pub const DEFAULT_CURRENT: u8 = current::MAX_4_0MA;

    pub mod current {
        pub const MAX_4_0MA: u8 = 0b0010_0000;
        pub const MAX_6_4MA: u8 = 0b0001_0000;
        pub const MAX_9_3MA: u8 = 0b0000_0000;
        pub const MAX_12_8MA: u8 = 0b0011_0000;
    }
}

pub mod control_word_1 {
    pub const DATA_OUT_BIT: u8 = 0b0000_0001; // low: serial, high: simultaneous
    pub const EXT_OSC_PRESCALER_BIT: u8 = 0b0000_0010; // low: clock/1, clock/8

    pub const DEFAULT_DATA_OUT_MODE: u8 = DATA_OUT_BIT;
}
