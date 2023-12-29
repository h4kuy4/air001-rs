use core::arch::asm;

use crate::register::{RW, RO};

const SYSTICK_LOAD_RELOAD_MSK: u32 = 0xFF_FFFF;

pub struct SysTick {
    reg: &'static mut Reg
}

#[repr(C)]
struct Reg {
    ctrl: RW<u32>,
    load: RW<u32>,
    val: RW<u32>,
    calib: RO<u32>
}

pub enum ClockSource {
    AHBDiv8 = 0,
    AHB     = 1
}

impl SysTick {
    #[inline]
    pub fn new() -> Self {
        Self {
            reg: unsafe {
                &mut *(0xE000_E010 as *mut Reg)
            }
        }     
    }

    #[inline]
    pub fn enable(&mut self) {
        let mut val = self.reg.ctrl.read();

        val |= 1;

        self.reg.ctrl.write(val);
    }

    #[inline]
    pub fn disable(&mut self) {
        let mut val = self.reg.ctrl.read();

        val &= !1;

        self.reg.ctrl.write(val);
    }

    #[inline]
    pub fn enable_int(self) -> Self {
        let mut val = self.reg.ctrl.read();

        val |= 1 << 1;

        self.reg.ctrl.write(val);


        return self;
    }

    #[inline]
    pub fn disable_int(self) -> Self {
        let mut val = self.reg.ctrl.read();

        val &= !(1 << 1);

        self.reg.ctrl.write(val);

        return self;
    }

    #[inline]
    pub fn source(self, source: ClockSource) -> Self {
        let mut val = self.reg.ctrl.read();

        val &= !(1 << 2);
        val |= (1 & source as u32) << 2;

        self.reg.ctrl.write(val);

        return self;
    }
    
    #[inline]
    pub fn get_countflag(&self) -> u32 {
        let val = self.reg.ctrl.read();

        (val >> 16) & 1
    }

    #[inline]
    pub fn set_reload(self, ticks: u32) -> Self {
        if (ticks - 1) > SYSTICK_LOAD_RELOAD_MSK {
            panic!();
        }

        self.reg.load.write(ticks);

        return self;
    }

    #[inline]
    pub fn set_val(&mut self, val: u32) {
        if (val - 1) > SYSTICK_LOAD_RELOAD_MSK {
            panic!();
        }

        self.reg.val.write(val);
    }

    // FIXME: Can't use this in release mode.
    pub fn delay(&mut self, mut ms: u32) {
        self.enable();

        while ms != 0 {
            ms = ms - self.get_countflag();
        }

        self.disable();
    }
    
}
