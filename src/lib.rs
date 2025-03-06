#![no_std]

mod control_word;
mod font5x7;

pub use control_word::PeakCurrent;
use control_word::*;
use core::cell::RefCell;
use embedded_hal::digital::{ErrorType, OutputPin};

pub const CHAR_HEIGHT: usize = 7;
pub const CHAR_WIDTH: usize = 5;
const DEVICE_CHARS: u8 = 4;

pub struct UnconfiguredPin;

impl OutputPin for UnconfiguredPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl ErrorType for UnconfiguredPin {
    type Error = core::convert::Infallible;
}

#[derive(Debug)]
pub enum Hcms29xxError<PinErr> {
    PinNotConfigured,
    DataPinError(PinErr),
    RsPinError(PinErr),
    ClkPinError(PinErr),
    CePinError(PinErr),
    BlankPinError(PinErr),
    OscSelPinError(PinErr),
    ResetPinError(PinErr),
}

pub struct Hcms29xx<
    const NUM_CHARS: usize,
    DataPin,
    RsPin,
    ClkPin,
    CePin,
    BlankPin = UnconfiguredPin,
    OscSelPin = UnconfiguredPin,
    ResetPin = UnconfiguredPin,
> where
    DataPin: OutputPin,
    RsPin: OutputPin,
    ClkPin: OutputPin,
    CePin: OutputPin,
    BlankPin: OutputPin,
    OscSelPin: OutputPin,
    ResetPin: OutputPin,
{
    data: RefCell<DataPin>,
    rs: RefCell<RsPin>,
    clk: RefCell<ClkPin>,
    ce: RefCell<CePin>,
    blank: RefCell<BlankPin>,
    osc_sel: RefCell<OscSelPin>,
    reset: RefCell<ResetPin>,
    control_word_0: ControlWord0,
    control_word_1: ControlWord1,
    // state kept locally to simplify/reduce overall code size
    data_out_mode: DataOutMode,
    font_ascii_start_index: u8,
}

impl<
        const NUM_CHARS: usize,
        DataPin,
        RsPin,
        ClkPin,
        CePin,
        BlankPin,
        OscSelPin,
        ResetPin,
        PinErr,
    > Hcms29xx<NUM_CHARS, DataPin, RsPin, ClkPin, CePin, BlankPin, OscSelPin, ResetPin>
where
    DataPin: OutputPin + ErrorType<Error = PinErr>,
    RsPin: OutputPin + ErrorType<Error = PinErr>,
    ClkPin: OutputPin + ErrorType<Error = PinErr>,
    CePin: OutputPin + ErrorType<Error = PinErr>,
    BlankPin: OutputPin + ErrorType<Error = PinErr>,
    OscSelPin: OutputPin + ErrorType<Error = PinErr>,
    ResetPin: OutputPin + ErrorType<Error = PinErr>,
{
    pub fn new(
        data: DataPin,
        rs: RsPin,
        clk: ClkPin,
        ce: CePin,
        blank: BlankPin,
        osc_sel: OscSelPin,
        reset: ResetPin,
    ) -> Result<Self, Hcms29xxError<PinErr>> {
        let data_ref_cell = RefCell::new(data);
        let rs_ref_cell = RefCell::new(rs);
        let clk_ref_cell = RefCell::new(clk);
        let ce_ref_cell = RefCell::new(ce);
        let blank_ref_cell = RefCell::new(blank);
        let osc_sel_ref_cell = RefCell::new(osc_sel);
        let reset_ref_cell = RefCell::new(reset);

        data_ref_cell
            .borrow_mut()
            .set_low()
            .map_err(Hcms29xxError::DataPinError)?;
        ce_ref_cell
            .borrow_mut()
            .set_high()
            .map_err(Hcms29xxError::CePinError)?;
        blank_ref_cell
            .borrow_mut()
            .set_high()
            .map_err(Hcms29xxError::BlankPinError)?;
        // default to internal oscillator, user can set ext osc if needed
        osc_sel_ref_cell
            .borrow_mut()
            .set_high()
            .map_err(Hcms29xxError::OscSelPinError)?;
        reset_ref_cell
            .borrow_mut()
            .set_high()
            .map_err(Hcms29xxError::ResetPinError)?;

        Ok(Hcms29xx {
            data: data_ref_cell,
            rs: rs_ref_cell,
            clk: clk_ref_cell,
            ce: ce_ref_cell,
            blank: blank_ref_cell,
            osc_sel: osc_sel_ref_cell,
            reset: reset_ref_cell,
            control_word_0: ControlWord0::default(),
            control_word_1: ControlWord1::default(),
            data_out_mode: DataOutMode::Serial,
            #[cfg(feature = "avr-progmem")]
            font_ascii_start_index: font5x7::FONT5X7.load_at(0) - 1,
            #[cfg(not(feature = "avr-progmem"))]
            font_ascii_start_index: font5x7::FONT5X7[0] - 1,
        })
    }

    pub fn destroy(self) -> (DataPin, RsPin, ClkPin, CePin, BlankPin, OscSelPin, ResetPin) {
        (
            self.data.into_inner(),
            self.rs.into_inner(),
            self.clk.into_inner(),
            self.ce.into_inner(),
            self.blank.into_inner(),
            self.osc_sel.into_inner(),
            self.reset.into_inner(),
        )
    }

    pub fn begin(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.clear()?;

        self.update_control_word(self.control_word_0.bits())?;
        self.update_control_word(self.control_word_1.bits())?;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.set_dot_data()?;
        for _ in 0..NUM_CHARS * CHAR_WIDTH {
            self.send_byte(0x00)?;
        }
        self.end_transfer()?;
        Ok(())
    }

    pub fn print_ascii_bytes(&mut self, bytes: &[u8]) -> Result<(), Hcms29xxError<PinErr>> {
        self.set_dot_data()?;
        for i in 0..NUM_CHARS {
            if i >= bytes.len() || bytes[i] < self.font_ascii_start_index {
                break;
            }
            let char_index: usize = (bytes[i] - self.font_ascii_start_index) as usize * CHAR_WIDTH;
            for col in 0..CHAR_WIDTH {
                #[cfg(feature = "avr-progmem")]
                self.send_byte(font5x7::FONT5X7.load_at(char_index + col))?;
                #[cfg(not(feature = "avr-progmem"))]
                self.send_byte(font5x7::FONT5X7[char_index + col])?;
            }
        }
        self.end_transfer()?;
        Ok(())
    }

    pub fn print_cols(&mut self, cols: &[u8]) -> Result<(), Hcms29xxError<PinErr>> {
        self.set_dot_data()?;
        for &byte in cols {
            self.send_byte(byte)?;
        }
        self.end_transfer()?;
        Ok(())
    }

    pub fn print_i32(&mut self, value: i32) -> Result<(), Hcms29xxError<PinErr>> {
        let mut buf = [0; 11]; // i32 max 11 base-10 digits

        let mut minus = value < 0;
        let mut value = value;
        if minus {
            value = -value;
        }
        for index in (0..NUM_CHARS).rev() {
            buf[index] = if value > 0 {
                let digit = b'0' + (value % 10) as u8;
                value /= 10;
                digit
            } else if minus {
                minus = false;
                b'-'
            } else {
                b' '
            };
        }
        self.print_ascii_bytes(&buf[..NUM_CHARS])?;

        Ok(())
    }

    pub fn print_u32(&mut self, value: u32) -> Result<(), Hcms29xxError<PinErr>> {
        let mut buf = [0; 10]; // u32 max 10 base-10 digits

        let mut value = value;
        for index in (0..NUM_CHARS).rev() {
            buf[index] = if value > 0 {
                let digit = b'0' + (value % 10) as u8;
                value /= 10;
                digit
            } else {
                b' '
            };
        }
        self.print_ascii_bytes(&buf[..NUM_CHARS])?;

        Ok(())
    }

    pub fn display_blank(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.blank
            .borrow_mut()
            .set_high()
            .map_err(Hcms29xxError::BlankPinError)?;
        Ok(())
    }

    pub fn display_sleep(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.control_word_0.set_wake_bit(SleepMode::Sleep);
        self.update_control_word(self.control_word_0.bits())?;
        Ok(())
    }

    pub fn display_wake(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.control_word_0.set_wake_bit(SleepMode::Normal);
        self.update_control_word(self.control_word_0.bits())?;
        Ok(())
    }

    pub fn display_unblank(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.blank
            .borrow_mut()
            .set_low()
            .map_err(Hcms29xxError::BlankPinError)?;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.reset
            .borrow_mut()
            .set_low()
            .map_err(Hcms29xxError::ResetPinError)?;
        self.reset
            .borrow_mut()
            .set_high()
            .map_err(Hcms29xxError::ResetPinError)?;
        Ok(())
    }

    pub fn set_brightness(&mut self, brightness: u8) -> Result<(), Hcms29xxError<PinErr>> {
        self.control_word_0.set_brightness_bits(brightness);
        self.update_control_word(self.control_word_0.bits())?;
        Ok(())
    }

    pub fn set_peak_current(&mut self, current: PeakCurrent) -> Result<(), Hcms29xxError<PinErr>> {
        self.control_word_0.set_peak_current_bits(current);
        self.update_control_word(self.control_word_0.bits())?;
        Ok(())
    }

    pub fn set_ext_osc_prescale_direct(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.control_word_1
            .set_ext_osc_prescaler_bit(ExtOscPrescaler::Direct);
        self.update_control_word(self.control_word_1.bits())?;
        Ok(())
    }

    pub fn set_ext_osc_prescale_div8(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.control_word_1
            .set_ext_osc_prescaler_bit(ExtOscPrescaler::Div8);
        self.update_control_word(self.control_word_1.bits())?;
        Ok(())
    }

    pub fn set_ext_osc(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.osc_sel
            .borrow_mut()
            .set_low()
            .map_err(Hcms29xxError::OscSelPinError)?;
        Ok(())
    }

    pub fn set_int_osc(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.osc_sel
            .borrow_mut()
            .set_high()
            .map_err(Hcms29xxError::OscSelPinError)?;
        Ok(())
    }

    pub fn set_serial_data_out(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.control_word_1
            .set_data_out_mode_bit(DataOutMode::Serial);
        self.update_control_word(self.control_word_1.bits())?;

        // update local state once change is sent to device
        self.data_out_mode = DataOutMode::Serial;

        Ok(())
    }

    pub fn set_simultaneous_data_out(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.control_word_1
            .set_data_out_mode_bit(DataOutMode::Simultaneous);
        self.update_control_word(self.control_word_1.bits())?;

        // update local state once change is sent to device
        self.data_out_mode = DataOutMode::Simultaneous;

        Ok(())
    }

    fn update_control_word(&mut self, control_word: u8) -> Result<(), Hcms29xxError<PinErr>> {
        let times_to_send = if self.data_out_mode == DataOutMode::Serial {
            NUM_CHARS as u8 / DEVICE_CHARS as u8
        } else {
            1
        };

        self.set_control_data()?;
        for _ in 0..times_to_send {
            self.send_byte(control_word)?;
        }
        self.end_transfer()?;

        Ok(())
    }

    fn set_dot_data(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.clk
            .borrow_mut()
            .set_high()
            .map_err(Hcms29xxError::ClkPinError)?;
        self.rs
            .borrow_mut()
            .set_low()
            .map_err(Hcms29xxError::RsPinError)?;
        self.ce
            .borrow_mut()
            .set_low()
            .map_err(Hcms29xxError::CePinError)?;
        Ok(())
    }

    fn set_control_data(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.clk
            .borrow_mut()
            .set_high()
            .map_err(Hcms29xxError::ClkPinError)?;
        self.rs
            .borrow_mut()
            .set_high()
            .map_err(Hcms29xxError::RsPinError)?;
        self.ce
            .borrow_mut()
            .set_low()
            .map_err(Hcms29xxError::CePinError)?;
        Ok(())
    }

    fn send_byte(&mut self, byte: u8) -> Result<(), Hcms29xxError<PinErr>> {
        for i in 0..8 {
            self.clk
                .borrow_mut()
                .set_low()
                .map_err(Hcms29xxError::ClkPinError)?;
            if (byte & (1 << (7 - i))) != 0 {
                self.data
                    .borrow_mut()
                    .set_high()
                    .map_err(Hcms29xxError::DataPinError)?;
            } else {
                self.data
                    .borrow_mut()
                    .set_low()
                    .map_err(Hcms29xxError::DataPinError)?;
            }
            self.clk
                .borrow_mut()
                .set_high()
                .map_err(Hcms29xxError::ClkPinError)?;
        }
        Ok(())
    }

    fn end_transfer(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.ce
            .borrow_mut()
            .set_high()
            .map_err(Hcms29xxError::CePinError)?;
        self.clk
            .borrow_mut()
            .set_low()
            .map_err(Hcms29xxError::ClkPinError)?;
        Ok(())
    }
}
