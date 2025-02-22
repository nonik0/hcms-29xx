#![no_std]

pub mod control;
mod font5x7;

use crate::control::*;
use core::cell::RefCell;
use embedded_hal::digital::{ErrorType, OutputPin};

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
    DataPin,
    RsPin,
    ClkPin,
    CePin,
    BlankPin,
    OscSelPin,
    ResetPin,
    const NUM_CHARS: usize,
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
    blank: Option<RefCell<BlankPin>>,
    osc_sel: Option<RefCell<OscSelPin>>,
    reset: Option<RefCell<ResetPin>>,
    control_word_0: ControlWord0,
    control_word_1: ControlWord1,
    data_out_mode: DataOutModeBit, // we keep track of just this to avoid more bookkeeping when updating local state of control word before updating
    font_ascii_start_index: u8,
}

impl<
        DataPin,
        RsPin,
        ClkPin,
        CePin,
        BlankPin,
        OscSelPin,
        ResetPin,
        PinErr,
        const NUM_CHARS: usize,
    > Hcms29xx<DataPin, RsPin, ClkPin, CePin, BlankPin, OscSelPin, ResetPin, NUM_CHARS>
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
        blank: Option<BlankPin>,
        osc_sel: Option<OscSelPin>,
        reset: Option<ResetPin>,
    ) -> Result<Self, Hcms29xxError<PinErr>> {
        let data_ref_cell = RefCell::new(data);
        let rs_ref_cell = RefCell::new(rs);
        let clk_ref_cell = RefCell::new(clk);
        let ce_ref_cell = RefCell::new(ce);
        let blank_ref_cell = blank.map(RefCell::new);
        let osc_sel_ref_cell = osc_sel.map(RefCell::new);
        let reset_ref_cell = reset.map(RefCell::new);

        data_ref_cell
            .borrow_mut()
            .set_low()
            .map_err(Hcms29xxError::DataPinError)?;
        ce_ref_cell
            .borrow_mut()
            .set_high()
            .map_err(Hcms29xxError::CePinError)?;
        if let Some(ref blank) = blank_ref_cell {
            blank
                .borrow_mut()
                .set_high()
                .map_err(Hcms29xxError::BlankPinError)?;
        }
        // default to internal oscillator, user can set ext osc if needed
        if let Some(ref osc_sel) = osc_sel_ref_cell {
            osc_sel
                .borrow_mut()
                .set_high()
                .map_err(Hcms29xxError::OscSelPinError)?;
        }
        if let Some(ref reset) = reset_ref_cell {
            reset
                .borrow_mut()
                .set_high()
                .map_err(Hcms29xxError::ResetPinError)?;
        }

        Ok(Hcms29xx {
            data: data_ref_cell,
            rs: rs_ref_cell,
            clk: clk_ref_cell,
            ce: ce_ref_cell,
            blank: blank_ref_cell,
            osc_sel: osc_sel_ref_cell,
            reset: reset_ref_cell,
            control_word_0: ControlWord0::new(),
            control_word_1: ControlWord1::new(),
            data_out_mode: DataOutModeBit::Serial,
            font_ascii_start_index: font5x7::FONT5X7.load_at(0) - 1,
        })
    }

    pub fn begin(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.clear()?;

        self.update_control_word(
            control::control_word_0::WAKE_BIT
                | control::control_word_0::DEFAULT_CURRENT
                | control::control_word_0::DEFAULT_BRIGHTNESS,
        )?;
        self.update_control_word(
            control::CONTROL_WORD_SELECT_BIT | control::control_word_1::DEFAULT_DATA_OUT_MODE,
        )?;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.set_dot_data()?;
        for _ in 0..NUM_CHARS * control::CHAR_WIDTH {
            self.send_byte(0x00)?;
        }
        self.end_transfer()?;
        Ok(())
    }

    pub fn print_c_string(&mut self, c_str: &[u8]) -> Result<(), Hcms29xxError<PinErr>> {
        self.set_dot_data()?;
        for i in 0..NUM_CHARS {
            if i >= c_str.len() || c_str[i] < self.font_ascii_start_index {
                break;
            }
            let char_start_index: usize =
                (c_str[i] - self.font_ascii_start_index) as usize * control::CHAR_WIDTH;
            for j in 0..control::CHAR_WIDTH {
                self.send_byte(font5x7::FONT5X7.load_at(char_start_index + j))?;
            }
        }
        self.end_transfer()?;
        Ok(())
    }

    pub fn print_u32(&mut self, value: u32) -> Result<(), Hcms29xxError<PinErr>> {
        let mut buf = [0; 10]; // u32 max base-10 digits

        let mut value = value;
        for index in (0..NUM_CHARS).rev() {
            buf[index] = if value > 0 {
                (b'0' + (value % 10) as u8) as u8
            } else {
                b' ' as u8
            };
            value /= 10;
        }
        self.print_c_string(&buf[..NUM_CHARS])?;

        Ok(())
    }

    // pub fn print_raw(&mut self, raw: &[u8]) -> Result<(), Hcms29xxError<PinErr>> {
    //     self.set_dot_data()?;
    //     for i in 0..NUM_CHARS {
    //         if i >= raw.len() {
    //             break;
    //         }
    //         self.send_byte(raw[i])?;
    //     }
    //     self.end_transfer()?;
    //     Ok(())
    // }

    //pub fn print_signed_int

    //pub fn print_u16

    pub fn display_blank(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        if let Some(ref blank) = self.blank {
            blank
                .borrow_mut()
                .set_high()
                .map_err(Hcms29xxError::BlankPinError)?;
        } else {
            return Err(Hcms29xxError::PinNotConfigured);
        }
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
        if let Some(ref blank) = self.blank {
            blank
                .borrow_mut()
                .set_low()
                .map_err(Hcms29xxError::BlankPinError)?;
        } else {
            return Err(Hcms29xxError::PinNotConfigured);
        }
        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        if let Some(ref reset) = self.reset {
            reset
                .borrow_mut()
                .set_low()
                .map_err(Hcms29xxError::ResetPinError)?;
            reset
                .borrow_mut()
                .set_high()
                .map_err(Hcms29xxError::ResetPinError)?;
        } else {
            return Err(Hcms29xxError::PinNotConfigured);
        }
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

    pub fn set_ext_osc(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        if let Some(ref osc_sel) = self.osc_sel {
            osc_sel
                .borrow_mut()
                .set_low()
                .map_err(Hcms29xxError::OscSelPinError)?;
        } else {
            return Err(Hcms29xxError::PinNotConfigured);
        }
        Ok(())
    }

    pub fn set_int_osc(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        if let Some(ref osc_sel) = self.osc_sel {
            osc_sel
                .borrow_mut()
                .set_high()
                .map_err(Hcms29xxError::OscSelPinError)?;
        } else {
            return Err(Hcms29xxError::PinNotConfigured);
        }
        Ok(())
    }

    pub fn set_serial_data_out(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.control_word_1.set_data_out_mode_bit(DataOutModeBit::Serial);
        self.update_control_word(self.control_word_1.bits())?;
        Ok(())
    }

    pub fn set_simultaneous_data_out(&mut self) -> Result<(), Hcms29xxError<PinErr>> {
        self.control_word_1.set_data_out_mode_bit(DataOutModeBit::Simultaneous);
        self.update_control_word(self.control_word_1.bits())?;
        Ok(())
    }

    fn update_control_word(&mut self, control_word: u8) -> Result<(), Hcms29xxError<PinErr>> {
        // read current data out mode before potentially changing it
        let times_to_send = if (self.control_word_1 & control::control_word_1::DATA_OUT_BIT) != 0
        {
            1
        } else {
            NUM_CHARS as u8 / control::DEVICE_CHARS as u8
        };

        self.set_control_data()?;
        for _ in 0..times_to_send {
            self.send_byte(control_word)?;
        }
        self.end_transfer()?;

        if (control_word & control::CONTROL_WORD_SELECT_BIT) != 0 {
            self.control_word_1 = control_word;
        } else {
            self.control_word_0 = control_word;
        }

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
