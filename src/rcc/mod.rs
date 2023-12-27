use crate::gpio::Port;

use self::raw::RccReg;

mod raw;

pub struct Rcc {
    reg: RccReg,
}

impl Rcc {
    #[inline]
    pub fn new() -> Self {
        Self {
            reg: RccReg::new(0x4002_1000) 
        }
    }

    #[inline]
    pub fn enable_port(mut self, port: Port) -> Self {
        match port {
            Port::GPIOA => self.reg.enable_port_a(),
            Port::GPIOB => self.reg.enable_port_b(),
            Port::GPIOF => self.reg.enable_port_f()
        }

        self
    }
}
