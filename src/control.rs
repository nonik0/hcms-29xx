pub const CHAR_WIDTH: usize = 5;
pub const CHAR_HEIGHT: usize = 7;
pub const DEVICE_CHARS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlWord0(u8);

impl ControlWord0 {
    pub const WAKE_BIT: u8 = 0b0100_0000;
    pub const PEAK_CURRENT_MASK: u8 = 0b0011_0000;
    pub const BRIGHTNESS_MASK: u8 = 0b0000_1111;

    pub fn new() -> Self {
        ControlWord0(0b0000_0000) // MSB is always 0
    }

    pub fn set_brightness_bits(&mut self, bits: u8) {
        // just truncate the bits rather than enforce or check for a max value
        self.0 = (self.0 & !Self::BRIGHTNESS_MASK) | (bits & Self::BRIGHTNESS_MASK);
    }

    pub fn set_peak_current_bits(&mut self, bits: PeakCurrent) {
        self.0 = (self.0 & !Self::PEAK_CURRENT_MASK) | bits as u8;
    }

    pub fn set_wake_bit(&mut self, mode: SleepMode) {
        self.0 = (self.0 & !Self::WAKE_BIT) | mode as u8;
    }

    pub fn bits(&self) -> u8 {
        self.0
    }
}

pub enum SleepMode {
    Sleep = 0b0000_0000,
    Normal = 0b0100_0000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeakCurrent {
    Max4_0Ma = 0b0010_0000,
    Max6_4Ma = 0b0001_0000,
    Max9_3Ma = 0b0000_0000,
    Max12_8Ma = 0b0011_0000,
}

impl PeakCurrent {
    pub fn bitmask(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlWord1(u8);

impl ControlWord1 {
    pub const DATA_OUT_BIT: u8 = 0b0000_0001;
    pub const EXT_OSC_PRESCALER_BIT: u8 = 0b0000_0010;

    pub fn new() -> Self {
        ControlWord1(0b1000_0000) // MSB is always 1
    }

    pub fn data_out_mode(&self) -> DataOutModeBit {
        match self.0 & Self::DATA_OUT_BIT {
            0 => DataOutModeBit::Serial,
            _ => DataOutModeBit::Simultaneous,
        }
    }

    pub fn set_data_out_mode_bit(&mut self, bit: DataOutModeBit) {
        self.0 = (self.0 & !Self::DATA_OUT_BIT) | (bit as u8);
    }

    pub fn set_ext_osc_prescaler_bit(&mut self, bit: ExtOscPrescalerBit) {
        self.0 = (self.0 & !Self::EXT_OSC_PRESCALER_BIT) | (bit as u8);
    }

    pub fn bits(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataOutModeBit {
    Serial = 0b0000_0000,
    Simultaneous = 0b0000_0001,
}

impl Default for DataOutModeBit {
    fn default() -> Self {
        DataOutModeBit::Serial
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtOscPrescalerBit {
    Clock1 = 0b0000_0000,
    Clock8 = 0b0000_0001,
}

impl Default for ExtOscPrescalerBit {
    fn default() -> Self {
        ExtOscPrescalerBit::Clock1
    }
}
