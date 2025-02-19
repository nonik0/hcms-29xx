#![no_std]

use constants::control_word_0::current;
use core::cell::RefCell;
use embedded_hal::digital::{self, ErrorType, OutputPin};

type Hcms29xxErr<Pin> = Hcms29xxError<<Pin as ErrorType>::Error>;

pub mod constants;
mod font5x7;

#[derive(Clone, Copy, Debug)]
pub enum Hcms29xxError<E> {
    OutputPinError(E),
    PinNotConfigured,
}

impl<PinError> From<PinError> for Hcms29xxError<PinError> {
    fn from(error: PinError) -> Self {
        Hcms29xxError::OutputPinError(error)
    }
}

impl<E> digital::Error for Hcms29xxError<E>
where
    E: core::fmt::Debug,
{
    fn kind(&self) -> digital::ErrorKind {
        digital::ErrorKind::Other
    }
}

// TODO: each pin has own generic type
pub struct Hcms29xx<Pin>
where
    Pin: OutputPin,
{
    num_chars: u8,
    data: RefCell<Pin>,
    rs: RefCell<Pin>,
    clk: RefCell<Pin>,
    ce: RefCell<Pin>,
    blank: Option<RefCell<Pin>>,
    osc_sel: Option<RefCell<Pin>>,
    reset: Option<RefCell<Pin>>,
    control_word_0: u8,
    control_word_1: u8,
    font_ascii_start_index: u8,
}

impl<Pin> Hcms29xx<Pin>
where
    Pin: OutputPin,
{
    pub fn new(
        num_chars: usize,
        data: Pin,
        rs: Pin,
        clk: Pin,
        ce: Pin,
        blank: Option<Pin>,
        osc_sel: Option<Pin>,
        reset: Option<Pin>,
    ) -> Result<Self, Hcms29xxErr<Pin>> {
        let data_ref_cell = RefCell::new(data);
        let rs_ref_cell = RefCell::new(rs);
        let clk_ref_cell = RefCell::new(clk);
        let ce_ref_cell = RefCell::new(ce);
        let blank_ref_cell = blank.map(RefCell::new);
        let osc_sel_ref_cell = osc_sel.map(RefCell::new);
        let reset_ref_cell = reset.map(RefCell::new);

        data_ref_cell.borrow_mut().set_low()?;
        ce_ref_cell.borrow_mut().set_high()?;
        if let Some(ref blank) = blank_ref_cell {
            blank.borrow_mut().set_high()?;
        }
        // default to internal oscillator, user can set ext osc if needed
        if let Some(ref osc_sel) = osc_sel_ref_cell {
            osc_sel.borrow_mut().set_high()?;
        }
        if let Some(ref reset) = reset_ref_cell {
            reset.borrow_mut().set_high()?;
        }

        Ok(Hcms29xx {
            num_chars: num_chars as u8,
            data: data_ref_cell,
            rs: rs_ref_cell,
            clk: clk_ref_cell,
            ce: ce_ref_cell,
            blank: blank_ref_cell,
            osc_sel: osc_sel_ref_cell,
            reset: reset_ref_cell,
            control_word_0: 0,
            control_word_1: 0,
            font_ascii_start_index: font5x7::FONT5X7.load_at(0) - 1,
        })
    }

    pub fn begin(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        self.clear()?;

        self.update_control_word(
            constants::control_word_0::WAKE_BIT
                | constants::control_word_0::DEFAULT_CURRENT
                | constants::control_word_0::DEFAULT_BRIGHTNESS,
        )?;
        self.update_control_word(
            constants::CONTROL_WORD_SELECT_BIT | constants::control_word_1::DEFAULT_DATA_OUT_MODE,
        )?;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        self.set_dot_data()?;
        for _ in 0..self.num_chars * constants::CHAR_WIDTH as u8 {
            self.send_byte(0x00)?;
        }
        self.end_transfer()?;
        Ok(())
    }

    pub fn print_c_string(&mut self, c_str: &[u8]) -> Result<(), Hcms29xxErr<Pin>> {
        self.set_dot_data()?;
        for i in 0..self.num_chars {
            if i >= c_str.len() as u8 || c_str[i as usize] < self.font_ascii_start_index {
                break;
            }
            let char_start_index: usize =
                (c_str[i as usize] - self.font_ascii_start_index) as usize * constants::CHAR_WIDTH;
            for j in 0..constants::CHAR_WIDTH {
                self.send_byte(font5x7::FONT5X7.load_at(char_start_index + j as usize))?;
            }
        }
        self.end_transfer()?;
        Ok(())
    }

    pub fn print_u32(&mut self, value: u32) -> Result<(), Hcms29xxErr<Pin>> {
        let mut buf = [0; 10]; // u32 max base-10 digits

        let mut value = value;
        for index in (0..self.num_chars).rev() {
            buf[index as usize] = if value > 0 {
                (b'0' + (value % 10) as u8) as u8
            } else {
                b' ' as u8
            };
            value /= 10;
        }
        self.print_c_string(&buf[..self.num_chars as usize])?;

        Ok(())
    }

    pub fn display_blank(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        if let Some(ref blank) = self.blank {
            blank.borrow_mut().set_high()?;
        } else {
            return Err(Hcms29xxError::PinNotConfigured);
        }
        Ok(())
    }

    pub fn display_sleep(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        self.update_control_word(self.control_word_0 & !constants::control_word_0::WAKE_BIT)?;
        Ok(())
    }

    pub fn display_wake(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        self.update_control_word(self.control_word_0 | constants::control_word_0::WAKE_BIT)?;
        Ok(())
    }

    pub fn display_unblank(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        if let Some(ref blank) = self.blank {
            blank.borrow_mut().set_low()?;
        } else {
            return Err(Hcms29xxError::PinNotConfigured);
        }
        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        if let Some(ref reset) = self.reset {
            reset.borrow_mut().set_low()?;
            reset.borrow_mut().set_high()?;
        } else {
            return Err(Hcms29xxError::PinNotConfigured);
        }
        Ok(())
    }

    pub fn set_brightness(&mut self, brightness: u8) -> Result<(), Hcms29xxErr<Pin>> {
        self.update_control_word(
            (self.control_word_0 & !constants::control_word_0::BRIGHTNESS_MASK)
                | (brightness & constants::control_word_0::BRIGHTNESS_MASK),
        )?;
        Ok(())
    }

    pub fn set_current(&mut self, current: u8) -> Result<(), Hcms29xxErr<Pin>> {
        let current = match current {
            0 => current::MAX_4_0MA,
            1 => current::MAX_6_4MA,
            2 => current::MAX_9_3MA,
            _ => current::MAX_12_8MA,
        };

        self.update_control_word(
            (self.control_word_0 & !constants::control_word_0::CURRENT_MASK)
                | (current & constants::control_word_0::CURRENT_MASK),
        )?;
        Ok(())
    }

    pub fn set_ext_osc(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        if let Some(ref osc_sel) = self.osc_sel {
            osc_sel.borrow_mut().set_low()?;
        } else {
            return Err(Hcms29xxError::PinNotConfigured);
        }
        Ok(())
    }

    pub fn set_int_osc(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        if let Some(ref osc_sel) = self.osc_sel {
            osc_sel.borrow_mut().set_high()?;
        } else {
            return Err(Hcms29xxError::PinNotConfigured);
        }
        Ok(())
    }

    pub fn set_serial_data_out(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        self.update_control_word(self.control_word_1 & !constants::control_word_1::DATA_OUT_BIT)?;
        Ok(())
    }

    pub fn set_simultaneous_data_out(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        self.update_control_word(self.control_word_1 | constants::control_word_1::DATA_OUT_BIT)?;
        Ok(())
    }

    fn update_control_word(&mut self, control_word: u8) -> Result<(), Hcms29xxErr<Pin>> {
        // read current data out mode before potentially changing it
        let times_to_send = if (self.control_word_1 & constants::control_word_1::DATA_OUT_BIT) != 0
        {
            1
        } else {
            self.num_chars / constants::DEVICE_CHARS as u8
        };

        self.set_control_data()?;
        for _ in 0..times_to_send {
            self.send_byte(control_word)?;
        }
        self.end_transfer()?;

        if (control_word & constants::CONTROL_WORD_SELECT_BIT) != 0 {
            self.control_word_1 = control_word;
        } else {
            self.control_word_0 = control_word;
        }

        Ok(())
    }

    fn set_dot_data(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        self.clk.borrow_mut().set_high()?;
        self.rs.borrow_mut().set_low()?;
        self.ce.borrow_mut().set_low()?;
        Ok(())
    }

    fn set_control_data(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        self.clk.borrow_mut().set_high()?;
        self.rs.borrow_mut().set_high()?;
        self.ce.borrow_mut().set_low()?;
        Ok(())
    }

    fn send_byte(&mut self, byte: u8) -> Result<(), Hcms29xxErr<Pin>> {
        for i in 0..8 {
            self.clk.borrow_mut().set_low()?;
            if (byte & (1 << (7 - i))) != 0 {
                self.data.borrow_mut().set_high()?;
            } else {
                self.data.borrow_mut().set_low()?;
            }
            self.clk.borrow_mut().set_high()?;
        }
        Ok(())
    }

    fn end_transfer(&mut self) -> Result<(), Hcms29xxErr<Pin>> {
        self.ce.borrow_mut().set_high()?;
        self.clk.borrow_mut().set_low()?;
        Ok(())
    }
}
