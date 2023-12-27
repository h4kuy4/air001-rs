use core::marker::PhantomData;

use self::raw::GpioReg;

mod raw;

pub enum Port {
    GPIOA = 0x5000_0000,
    GPIOB = 0x5000_0400,
    GPIOF = 0x5000_1400,
}

pub struct Input;
pub struct Output;
pub struct AltFunc;
pub struct Analog;

pub struct Gpio<Mode, const PIN_NUM: u8> {
    reg: GpioReg,
    _mode: PhantomData<Mode>
}

pub enum PullMode {
    None     = 0b00,
    PullUp   = 0b01,
    PullDown = 0b11
}

impl<Mode, const PIN_NUM: u8> Gpio<Mode, PIN_NUM> {
    #[inline]
    pub fn set_pull_mode(mut self, mode: PullMode) -> Self {
        let mask = 0b11 << (PIN_NUM * 2);
        let mode = (mode as u32) << (PIN_NUM * 2);

        self.reg.set_pull_mode(mask, mode);

        self
    }
}

impl<const PIN_NUM: u8> Gpio<Input, PIN_NUM> {
    #[inline]
    pub fn new(port: Port) -> Self {
        let mut reg = GpioReg::new(port as u32);

        let mask = 0b11 << (PIN_NUM * 2);
        let mode = 0b00 << (PIN_NUM * 2);

        reg.set_mode(mask, mode);

        Self {
            reg,
            _mode: PhantomData
        }
    } 

    #[inline]
    pub fn read(&mut self) -> bool {
        let val = self.reg.get_input_data();

        if (val >> PIN_NUM) & 0b01 == 0 {
            false
        } else {
            true
        }
    }
}

pub enum OutputSpeed {
    VeryLow  = 0b00,
    Low      = 0b01,
    High     = 0b10,
    VeryHigh = 0b11,
}

pub enum OutputType {
    PushPull  = 0b00,
    OpenDrain = 0b01 
}

impl<const PIN_NUM: u8> Gpio<Output, PIN_NUM> {
    #[inline]
    pub fn new(port: Port) -> Self {
        let mut reg = GpioReg::new(port as u32);

        let mask = 0b11 << (PIN_NUM * 2);
        let mode = 0b01 << (PIN_NUM * 2);

        reg.set_mode(mask, mode);

        Self {
            reg,
            _mode: PhantomData
        }
    } 

    #[inline]
    pub fn set_speed(mut self, speed: OutputSpeed) -> Self {
        let mask = 0b11 << (PIN_NUM * 2);
        let speed = (speed as u32) << (PIN_NUM * 2);

        self.reg.set_output_speed(mask, speed);

        self
    }

    #[inline]
    pub fn set_type(mut self, otype: OutputType) -> Self {
        let mask = 0b01 << PIN_NUM;
        let otype = (otype as u32) << PIN_NUM;

        self.reg.set_output_type(mask, otype);

        self
    }

    #[inline]
    pub fn set(&mut self) {
        let val = 0b01 << PIN_NUM;

        self.reg.bit_set(val);
    }

    #[inline]
    pub fn reset(&mut self) {
        let val = 0b01 << PIN_NUM;

        self.reg.bit_reset(val);
    }

    #[inline]
    pub fn write(&mut self, val: bool) {
        let mask = 0b01 << PIN_NUM;
        let val  = if val { 0b01 << PIN_NUM } else { 0b00 << PIN_NUM };

        self.reg.set_output_data(mask, val);
    }

    #[inline]
    pub fn read(&mut self) -> bool {
        let val = self.reg.get_output_data();

        if (val >> PIN_NUM) & 0b01 == 0 {
            false
        } else {
            true
        }
    }

    #[inline]
    pub fn toggle(&mut self) {
        if self.read() {
            self.reset();
        } else {
            self.set();
        }
    }
}

impl<const PIN_NUM: u8> Gpio<AltFunc, PIN_NUM> {
    #[inline]
    pub fn new(port: Port) -> Self {
        let mut reg = GpioReg::new(port as u32);

        let mask = 0b11 << (PIN_NUM * 2);
        let mode = 0b10 << (PIN_NUM * 2);

        reg.set_mode(mask, mode);

        Self {
            reg,
            _mode: PhantomData
        }
    } 

    #[inline]
    pub fn set_func(mut self, func: u32) -> Self {
        if PIN_NUM > 7 {
            let mask = 0b1111 << (PIN_NUM * 4);
            let func = func << (PIN_NUM * 4);
            self.reg.set_alt_func_l(mask, func);
        } else {
            let mask = 0b1111 << ((PIN_NUM - 8) * 4);
            let func = func << ((PIN_NUM - 8) * 4);
            self.reg.set_alt_func_h(mask, func);
        }

        self
    }
}


impl<const PIN_NUM: u8> Gpio<Analog, PIN_NUM> {
    #[inline]
    pub fn new(port: Port) -> Self {
        let mut reg = GpioReg::new(port as u32);

        let mask = 0b11 << (PIN_NUM * 2);
        let mode = 0b11 << (PIN_NUM * 2);

        reg.set_mode(mask, mode);

        Self {
            reg,
            _mode: PhantomData
        }
    } 
}

