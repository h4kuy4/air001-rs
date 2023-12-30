use core::ptr::NonNull;

use volatile::VolatilePtr;

use crate::{gpio::Port, set_bit};

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

impl Rcc {
    pub fn enable_port(&self, port: Port) {
        match port {
            Port::GPIOA => self.iopenr.update(|val| set_bit!(val, 0, 0b01, 0b01)),
            Port::GPIOB => self.iopenr.update(|val| set_bit!(val, 1, 0b01, 0b01)),
            Port::GPIOF => self.iopenr.update(|val| set_bit!(val, 5, 0b01, 0b01)),
        }
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

    pub fn enable_port(self, port: Port) -> Self {
        self.reg.enable_port(port);

        self
    }
}
