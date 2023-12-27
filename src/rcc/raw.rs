use crate::register::{RW, RO, WO};

pub struct RccReg {
    reg: &'static mut Reg
}

#[repr(C)]
struct Reg {
    cr: RW<u32>,        // 0x00
    icscr: RW<u32>,     // 0x04
    cfgr: RW<u32>,      // 0x08
    pllcfgr: RW<u32>,   // 0x0C
    ecscr: RW<u32>,     // 0x10
    _gap1: RO<u32>,     // 0x14
    cier: RW<u32>,      // 0x18
    cifr: RO<u32>,      // 0x1C
    cicr: WO<u32>,      // 0x20
    ioprstr: RW<u32>,   // 0x24
    ahbrstr: RW<u32>,   // 0x28
    apbrstr1: RW<u32>,  // 0x2C
    apbrstr2: RW<u32>,  // 0x30
    iopenr: RW<u32>,    // 0x34
    ahbenr: RW<u32>,
    apbenr1: RW<u32>,
    apbenr2: RW<u32>,
    ccipr: RW<u32>,
    bdcr: RW<u32>,
    csr: RW<u32>,
}

impl RccReg {
    #[inline]
    pub fn new(addr: u32) -> Self {
        Self { 
            reg: unsafe {
                &mut *(addr as *mut Reg)
            } 
        }
    }

    #[inline]
    pub fn set_hsi(&mut self) {
        let mut val = self.reg.cr.read();

        val |= 0b01 << 8;

        self.reg.cr.write(val);
    }

    #[inline]
    pub fn reset_hsi(&mut self) {
        let mut val = self.reg.cr.read();

        val = (val ^ (0b01 << 8)) & val;

        self.reg.cr.write(val);
    }

    #[inline]
    pub fn hsi_ready(&self) -> bool {
        let val = self.reg.cr.read();

        if (val >> 9) & 1 == 1 {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn hsi_div(&mut self, div: u32) {
        let mut val = self.reg.cr.read();

        val = ((val ^ (0b111 << 11)) & val) | ((div & 0b111) << 11);

        self.reg.cr.write(val);
    }

    #[inline]
    pub fn set_hse(&mut self) {
        let mut val = self.reg.cr.read();

        val |= 0b01 << 16;

        self.reg.cr.write(val);
    }

    #[inline]
    pub fn reset_hse(&mut self) {
        let mut val = self.reg.cr.read();

        val = (val ^ (0b01 << 16)) & val;

        self.reg.cr.write(val);
    }

    #[inline]
    pub fn hse_ready(&self) -> bool {
        let val = self.reg.cr.read();

        if (val >> 17) & 1 == 1 {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn set_hse_css(&mut self) {
        let mut val = self.reg.cr.read();

        val |= 0b01 << 19;

        self.reg.cr.write(val);
    }

    #[inline]
    pub fn set_pll(&mut self) {
        let mut val = self.reg.cr.read();

        val |= 0b01 << 24;

        self.reg.cr.write(val);
    }

    #[inline]
    pub fn reset_pll(&mut self) {
        let mut val = self.reg.cr.read();

        val = (val ^ (0b01 << 24)) & val;

        self.reg.cr.write(val);
    }

    #[inline]
    pub fn pll_ready(&self) -> bool {
        let val = self.reg.cr.read();

        if (val >> 25) & 1 == 1 {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn hsi_trim(&mut self, v: u32) {
        let mut val = self.reg.icscr.read();

        val = ((val ^ 0b1_1111_1111_1111) & val) | (v & 0b1_1111_1111_1111);

        self.reg.icscr.write(val);
    }

    #[inline]
    pub fn hsi_speed(&mut self, v: u32) {
        let mut val = self.reg.icscr.read();

        val = ((val ^ (0b111 << 13)) & val) | ((v & 0b111) << 13);

        self.reg.icscr.write(val);
    }

    #[inline]
    pub fn lsi_trim(&mut self, v: u32) {
        let mut val = self.reg.icscr.read();

        val = ((val ^ (0b1_1111_1111 << 16)) & val) | ((v & 0b1_1111_1111) << 16);

        self.reg.icscr.write(val);
    }

    #[inline]
    pub fn lsi_startup(&mut self, v: u32) {
        let mut val = self.reg.icscr.read();

        val = ((val ^ (0b11 << 26)) & val) | ((v & 0b11) << 26);

        self.reg.icscr.write(val);
    }

    #[inline]
    pub fn enable_port_a(&mut self) {
        let mut val = self.reg.iopenr.read();

        val |= 0b01;

        self.reg.iopenr.write(val);
    } 

    #[inline]
    pub fn disable_port_a(&mut self) {
        let mut val = self.reg.iopenr.read();

        val = (val ^ 0b01) & val;

        self.reg.iopenr.write(val);
    }

    #[inline]
    pub fn enable_port_b(&mut self) {
        let mut val = self.reg.iopenr.read();

        val |= 0b01 << 1;

        self.reg.iopenr.write(val);
    } 

    #[inline]
    pub fn disable_port_b(&mut self) {
        let mut val = self.reg.iopenr.read();

        val = (val ^ (0b01 << 1)) & val;

        self.reg.iopenr.write(val);
    }

    #[inline]
    pub fn enable_port_f(&mut self) {
        let mut val = self.reg.iopenr.read();

        val |= 0b01 << 5;

        self.reg.iopenr.write(val);
    } 

    #[inline]
    pub fn disable_port_f(&mut self) {
        let mut val = self.reg.iopenr.read();

        val = (val ^ (0b01 << 5)) & val;

        self.reg.iopenr.write(val);
    }
}
