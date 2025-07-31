#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlWord0(u8);

impl ControlWord0 {
    pub const WORD_SELECT_BIT: u8 = 0b0000_0000; // MSB is 1 for control word 1
    pub const WAKE_BIT: u8 = 0b0100_0000;
    pub const PEAK_CURRENT_MASK: u8 = 0b0011_0000;
    pub const BRIGHTNESS_MASK: u8 = 0b0000_1111;
    pub const BRIGHTNESS_DEFAULT: u8 = 0b0000_1100;

    pub fn set_brightness_bits(&mut self, brightness: u8) {
        // just truncate the bits rather than enforce or check for a max value
        self.0 = (self.0 & !Self::BRIGHTNESS_MASK) | (brightness & Self::BRIGHTNESS_MASK);
    }

    pub fn set_peak_current_bits(&mut self, current: PeakCurrent) {
        self.0 = (self.0 & !Self::PEAK_CURRENT_MASK) | current as u8;
    }

    pub fn set_wake_bit(&mut self, mode: SleepMode) {
        self.0 = (self.0 & !Self::WAKE_BIT) | mode as u8;
    }

    pub fn bits(&self) -> u8 {
        self.0
    }
}

impl Default for ControlWord0 {
    fn default() -> Self {
        let mut control_word = ControlWord0(Self::WORD_SELECT_BIT); // MSB is always 0
        control_word.set_brightness_bits(Self::BRIGHTNESS_DEFAULT);
        control_word.set_peak_current_bits(PeakCurrent::default());
        control_word.set_wake_bit(SleepMode::default());
        control_word
    }
}

#[derive(Default)]
pub enum SleepMode {
    Sleep = 0b0000_0000,
    #[default]
    Normal = 0b0100_0000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PeakCurrent {
    Max4_0Ma = 0b0010_0000,
    Max6_4Ma = 0b0001_0000,
    Max9_3Ma = 0b0000_0000,
    #[default]
    Max12_8Ma = 0b0011_0000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlWord1(u8);

impl ControlWord1 {
    pub const WORD_SELECT_BIT: u8 = 0b1000_0000; // MSB is 1 for control word 1
    pub const DATA_OUT_BIT: u8 = 0b0000_0001;
    pub const EXT_OSC_PRESCALER_BIT: u8 = 0b0000_0010;

    pub fn set_data_out_mode_bit(&mut self, bit: DataOutMode) {
        self.0 = (self.0 & !Self::DATA_OUT_BIT) | (bit as u8);
    }

    pub fn set_ext_osc_prescaler_bit(&mut self, bit: ExtOscPrescaler) {
        self.0 = (self.0 & !Self::EXT_OSC_PRESCALER_BIT) | (bit as u8);
    }

    pub fn bits(&self) -> u8 {
        self.0
    }
}

impl Default for ControlWord1 {
    fn default() -> Self {
        let mut control_word = ControlWord1(Self::WORD_SELECT_BIT);
        control_word.set_data_out_mode_bit(DataOutMode::default());
        control_word.set_ext_osc_prescaler_bit(ExtOscPrescaler::default());
        control_word
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DataOutMode {
    #[default]
    Serial = 0b0000_0000,
    Simultaneous = 0b0000_0001,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExtOscPrescaler {
    #[default]
    Direct = 0b0000_0000,
    Div8 = 0b0000_0001,
}
