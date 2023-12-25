use self::raw::GpioReg;

pub use self::builder::GpioBuilder;

pub mod raw;

pub mod builder;

#[derive(Clone, Copy)]
pub enum Port {
    GPIOA = 0x5000_0000,
    GPIOB = 0x5000_0400,
    GPIOF = 0x5000_1400,
}

#[derive(Clone, Copy)]
pub enum Pin {
    Pin0  = 0,
    Pin1  = 1,
    Pin2  = 2,
    Pin3  = 3,
    Pin4  = 4,
    Pin5  = 5,
    Pin6  = 6,
    Pin7  = 7,
    Pin8  = 8,
    Pin9  = 9,
    Pin10 = 10,
    Pin11 = 11,
    Pin12 = 12,
    Pin13 = 13,
    Pin14 = 14,
    Pin15 = 15,
}

pub enum PinLevel {
    Low  = 0,
    High = 1,
}

pub enum PinMode {
    Input       = 0b00,
    Output      = 0b01,
    Alternate   = 0b10,
    Analog      = 0b11
}

pub struct Gpio {
    reg: GpioReg,
    pin: u32
}

impl Gpio {
    #[inline]
    pub fn new(port: Port, pin: Pin) -> Self {
        Self {
            reg: GpioReg::new(port as u32),
            pin: pin as u32
        }
    } 

    #[inline]
    pub fn set_mode(&mut self, mode: PinMode) {
        let mask = 0b11 << (self.pin * 2);
        let val  = (mode as u32) << (self.pin * 2);

        self.reg.set_mode(mask, val);
    }

    #[inline]
    pub fn set_level(&mut self, level: PinLevel) {
        let mask = 0b1 << self.pin;
        let val  = (level as u32) << self.pin;

        self.reg.set_output_data(mask, val);
    }
}
