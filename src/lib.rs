#![no_std]

#[cfg(feature = "hal-0-2")]
use embedded_hal_0_2 as hal;

#[cfg(feature = "hal-1")]
use embedded_hal_1 as hal;

/// 74HC595 sürücüsü
pub struct ShiftRegister<OE, SER, SRCLR, SRCLK, RCLK> {
    oe: OE,
    ser: SER,
    srclr: SRCLR,
    srclk: SRCLK,
    rclk: RCLK,
}

impl<OE, SER, SRCLR, SRCLK, RCLK> ShiftRegister<OE, SER, SRCLR, SRCLK, RCLK> {
    pub fn new(oe: OE, ser: SER, srclr: SRCLR, srclk: SRCLK, rclk: RCLK) -> Self {
        Self {
            oe,
            ser,
            srclr,
            srclk,
            rclk,
        }
    }
}

//
// --- HAL 0.2.7 için impl ---
//
#[cfg(feature = "hal-0-2")]
impl<OE, SER, SRCLR, SRCLK, RCLK> ShiftRegister<OE, SER, SRCLR, SRCLK, RCLK>
where
    OE: hal::digital::v2::OutputPin<Error = core::convert::Infallible>,
    SER: hal::digital::v2::OutputPin<Error = core::convert::Infallible>,
    SRCLR: hal::digital::v2::OutputPin<Error = core::convert::Infallible>,
    SRCLK: hal::digital::v2::OutputPin<Error = core::convert::Infallible>,
    RCLK: hal::digital::v2::OutputPin<Error = core::convert::Infallible>,
{
    pub fn begin(&mut self) {
        let _ = self.oe.set_low();
        let _ = self.srclr.set_high();
    }

    pub fn load(&mut self, mut data: u8) {
        for _ in 0..8 {
            if (data & 0x80) != 0 {
                let _ = self.ser.set_high();
            } else {
                let _ = self.ser.set_low();
            }
            self.pulse_srclk();
            data <<= 1;
        }
        self.pulse_rclk();
    }

    pub fn enable_output(&mut self) {
        let _ = self.oe.set_low();
    }
    pub fn disable_output(&mut self) {
        let _ = self.oe.set_high();
    }

    pub fn output_clear(&mut self) {
        let _ = self.srclr.set_low();
        self.pulse_rclk();
        let _ = self.srclr.set_high();
    }

    pub fn shift_zero(&mut self) {
        let _ = self.ser.set_low();
        self.pulse_srclk();
        self.pulse_rclk();
    }

    pub fn shift_one(&mut self) {
        let _ = self.ser.set_high();
        self.pulse_srclk();
        self.pulse_rclk();
    }

    pub fn shift_zero_times(&mut self, n: u8) {
        for _ in 0..n {
            self.shift_zero();
        }
    }

    pub fn shift_one_times(&mut self, n: u8) {
        for _ in 0..n {
            self.shift_one();
        }
    }

    fn pulse_srclk(&mut self) {
        let _ = self.srclk.set_high();
        let _ = self.srclk.set_low();
    }

    fn pulse_rclk(&mut self) {
        let _ = self.rclk.set_high();
        let _ = self.rclk.set_low();
    }
}

//
// --- HAL 1.0.0 için impl ---
//
#[cfg(feature = "hal-1")]
impl<OE, SER, SRCLR, SRCLK, RCLK> ShiftRegister<OE, SER, SRCLR, SRCLK, RCLK>
where
    OE: hal::digital::OutputPin,
    SER: hal::digital::OutputPin,
    SRCLR: hal::digital::OutputPin,
    SRCLK: hal::digital::OutputPin,
    RCLK: hal::digital::OutputPin,
{
    pub fn begin(&mut self) {
        self.oe.set_low();
        self.srclr.set_high();
    }

    pub fn load(&mut self, mut data: u8) {
        for _ in 0..8 {
            if (data & 0x80) != 0 {
                self.ser.set_high();
            } else {
                self.ser.set_low();
            }
            self.pulse_srclk();
            data <<= 1;
        }
        self.pulse_rclk();
    }

    pub fn enable_output(&mut self) {
        self.oe.set_low();
    }
    pub fn disable_output(&mut self) {
        self.oe.set_high();
    }

    pub fn output_clear(&mut self) {
        self.srclr.set_low();
        self.pulse_rclk();
        self.srclr.set_high();
    }

    pub fn shift_zero(&mut self) {
        self.ser.set_low();
        self.pulse_srclk();
        self.pulse_rclk();
    }

    pub fn shift_one(&mut self) {
        self.ser.set_high();
        self.pulse_srclk();
        self.pulse_rclk();
    }

    pub fn shift_zero_times(&mut self, n: u8) {
        for _ in 0..n {
            self.shift_zero();
        }
    }

    pub fn shift_one_times(&mut self, n: u8) {
        for _ in 0..n {
            self.shift_one();
        }
    }

    fn pulse_srclk(&mut self) {
        self.srclk.set_high();
        self.srclk.set_low();
    }

    fn pulse_rclk(&mut self) {
        self.rclk.set_high();
        self.rclk.set_low();
    }
}
