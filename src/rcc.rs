use core::ptr::NonNull;

use volatile::VolatilePtr;

use crate::set_bit;

pub struct Rcc {
    cr:       VolatilePtr<'static, u32>,    // 0x00
    icscr:    VolatilePtr<'static, u32>,    // 0x04
    cfgr:     VolatilePtr<'static, u32>,    // 0x08
    pllcfgr:  VolatilePtr<'static, u32>,    // 0x0C
    ecscr:    VolatilePtr<'static, u32>,    // 0x10
    cier:     VolatilePtr<'static, u32>,    // 0x18
    cifr:     VolatilePtr<'static, u32>,    // 0x1C
    cicr:     VolatilePtr<'static, u32>,    // 0x20
    ioprstr:  VolatilePtr<'static, u32>,    // 0x24
    ahbrstr:  VolatilePtr<'static, u32>,    // 0x28
    apbrstr1: VolatilePtr<'static, u32>,    // 0x2C
    apbrstr2: VolatilePtr<'static, u32>,    // 0x30
    iopenr:   VolatilePtr<'static, u32>,    // 0x34
    ahbenr:   VolatilePtr<'static, u32>,    // 0x38
    apbenr1:  VolatilePtr<'static, u32>,    // 0x3C
    apbenr2:  VolatilePtr<'static, u32>,    // 0x40
    ccipr:    VolatilePtr<'static, u32>,    // 0x44
    bdcr:     VolatilePtr<'static, u32>,    // 0x48
    csr:      VolatilePtr<'static, u32>,    // 0x4C
}

pub struct RccBuilder {
    reg: Rcc,
}

pub enum DivCoeff {
    Div1    = 0b000,
    Div2    = 0b001,
    Div4    = 0b010,
    Div8    = 0b011,
    Div16   = 0b100,
    Div32   = 0b101,
    Div64   = 0b110,
    Div128  = 0b111,
}

impl Rcc {
    pub fn enable_port(&self, port: char) {
        match port {
            'A' => self.iopenr.update(|val| set_bit!(val, 0, 0b01, 0b01)),
            'B' => self.iopenr.update(|val| set_bit!(val, 1, 0b01, 0b01)),
            'F' => self.iopenr.update(|val| set_bit!(val, 5, 0b01, 0b01)),
            _   => panic!()
        }
    }

    pub fn hsi_on(&self) {
        self.cr.update(|val| set_bit!(val, 8, 0b01, 0b01));
    }

    pub fn hsi_off(&self) {
        self.cr.update(|val| set_bit!(val, 8, 0b01, 0b00));
    }

    pub fn hse_on(&self) {
        self.cr.update(|val| set_bit!(val, 17, 0b01, 0b01));
    }

    pub fn hse_css_on(&self) {
        self.cr.update(|val| set_bit!(val, 19, 0b01, 0b01));
    }

    pub fn hse_off(&self) {
        self.cr.update(|val| set_bit!(val, 17, 0b01, 0b00));
    }

    pub fn pll_on(&self) {
        self.cr.update(|val| set_bit!(val, 24, 0b01, 0b01));
    }

    pub fn pll_off(&self) {
        self.cr.update(|val| set_bit!(val, 24, 0b01, 0b00));
    }

    pub fn set_hsi_div(&self, coeff: DivCoeff) {
        self.cr.update(|val| set_bit!(val, 11, 0b111, coeff as u32));
    }

    pub fn hsi_ready(&self) -> bool {
        let val = self.cr.read();

        if (val >> 10) & 1 == 1 {
            true
        } else {
            false
        }
    }

    pub fn hse_ready(&self) -> bool {
        let val = self.cr.read();

        if (val >> 17) & 1 == 1 {
            true
        } else {
            false
        }
    }

    pub fn pll_ready(&self) -> bool {
        let val = self.cr.read();

        if (val >> 25) & 1 == 1 {
            true
        } else {
            false
        }
    }
}

impl RccBuilder {
    pub fn new() -> Self {
        Self {
            reg: unsafe {
                Rcc {
                    cr      : VolatilePtr::new(NonNull::new_unchecked(0x4002_1000 as *mut u32)),
                    icscr   : VolatilePtr::new(NonNull::new_unchecked(0x4002_1004 as *mut u32)),
                    cfgr    : VolatilePtr::new(NonNull::new_unchecked(0x4002_1008 as *mut u32)),
                    pllcfgr : VolatilePtr::new(NonNull::new_unchecked(0x4002_100c as *mut u32)),
                    ecscr   : VolatilePtr::new(NonNull::new_unchecked(0x4002_1010 as *mut u32)),
                    cier    : VolatilePtr::new(NonNull::new_unchecked(0x4002_1018 as *mut u32)),
                    cifr    : VolatilePtr::new(NonNull::new_unchecked(0x4002_101C as *mut u32)),
                    cicr    : VolatilePtr::new(NonNull::new_unchecked(0x4002_1020 as *mut u32)),
                    ioprstr : VolatilePtr::new(NonNull::new_unchecked(0x4002_1024 as *mut u32)),
                    ahbrstr : VolatilePtr::new(NonNull::new_unchecked(0x4002_1028 as *mut u32)),
                    apbrstr1: VolatilePtr::new(NonNull::new_unchecked(0x4002_102C as *mut u32)),
                    apbrstr2: VolatilePtr::new(NonNull::new_unchecked(0x4002_1030 as *mut u32)),
                    iopenr  : VolatilePtr::new(NonNull::new_unchecked(0x4002_1034 as *mut u32)),
                    ahbenr  : VolatilePtr::new(NonNull::new_unchecked(0x4002_1038 as *mut u32)),
                    apbenr1 : VolatilePtr::new(NonNull::new_unchecked(0x4002_103C as *mut u32)),
                    apbenr2 : VolatilePtr::new(NonNull::new_unchecked(0x4002_1040 as *mut u32)),
                    ccipr   : VolatilePtr::new(NonNull::new_unchecked(0x4002_1044 as *mut u32)),
                    bdcr    : VolatilePtr::new(NonNull::new_unchecked(0x4002_1048 as *mut u32)),
                    csr     : VolatilePtr::new(NonNull::new_unchecked(0x4002_104C as *mut u32)),
                }
            } 
        }
    }

    pub fn build(self) -> Rcc {
        self.reg
    }

    pub fn enable_port(self, port: char) -> Self {
        self.reg.enable_port(port);

        self
    }

    pub fn hsi_on(self) -> Self {
        self.reg.cr.update(|val| set_bit!(val, 8, 0b01, 0b01));

        self
    }

    pub fn hsi_off(self) -> Self {
        self.reg.cr.update(|val| set_bit!(val, 8, 0b01, 0b00));

        self
    }

    pub fn hse_on(self) -> Self {
        self.reg.cr.update(|val| set_bit!(val, 17, 0b01, 0b01));

        self
    }

    pub fn hse_css_on(self) -> Self {
        self.reg.cr.update(|val| set_bit!(val, 19, 0b01, 0b01));

        self
    }

    pub fn hse_off(self) -> Self {
        self.reg.cr.update(|val| set_bit!(val, 17, 0b01, 0b00));

        self
    }

    pub fn pll_on(self) -> Self {
        self.reg.cr.update(|val| set_bit!(val, 24, 0b01, 0b01));

        self
    }

    pub fn pll_off(self) -> Self {
        self.reg.cr.update(|val| set_bit!(val, 24, 0b01, 0b00));

        self
    }

    pub fn set_hsi_div(self, coeff: DivCoeff) -> Self {
        self.reg.cr.update(|val| set_bit!(val, 11, 0b111, coeff as u32));

        self
    }
}
