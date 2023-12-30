use core::ptr::NonNull;

use volatile::VolatilePtr;

const SYSTICK_LOAD_RELOAD_MSK: u32 = 0xFF_FFFF;

pub struct SysTick {
    ctrl: VolatilePtr<'static, u32>,
    load: VolatilePtr<'static, u32>,
    val: VolatilePtr<'static, u32>,
    calib: VolatilePtr<'static, u32>,
}

pub struct SysTickBuilder {
    reg: SysTick,
}

pub enum ClockSource {
    AHBDiv8 = 0,
    AHB     = 1
}

impl SysTick {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            Self {
                ctrl : VolatilePtr::new(NonNull::new_unchecked(0xE000_E010 as *mut  u32)),
                load : VolatilePtr::new(NonNull::new_unchecked(0xE000_E014 as *mut  u32)),
                val  : VolatilePtr::new(NonNull::new_unchecked(0xE000_E018 as *mut  u32)),
                calib: VolatilePtr::new(NonNull::new_unchecked(0xE000_E01C as *mut  u32))
            }     
        }
    }

    #[inline]
    pub fn enable(&self) {
        self.ctrl.update(|val| val | 1);
    }

    #[inline]
    pub fn disable(&self) {
        self.ctrl.update(|val| val & !1);
    }

    #[inline]
    pub fn enable_int(&self) {
        self.ctrl.update(|val| val | (1 << 1));
    }

    #[inline]
    pub fn disable_int(&self) {
        self.ctrl.update(|val| val | !(1 << 1));
    }

    #[inline]
    pub fn source(&self, source: ClockSource) {
        self.ctrl.update(|val| (val & !(1 << 2)) | ((1 & source as u32) << 2));
    }
    
    #[inline]
    pub fn get_countflag(&self) -> u32 {
        let val = self.ctrl.read();

        (val >> 16) & 1
    }

    #[inline]
    pub fn set_reload(&self, ticks: u32) {
        if (ticks - 1) > SYSTICK_LOAD_RELOAD_MSK {
            panic!();
        }

        self.load.write(ticks);
    }

    #[inline]
    pub fn set_val(&mut self, val: u32) {
        if (val - 1) > SYSTICK_LOAD_RELOAD_MSK {
            panic!();
        }

        self.val.write(val);
    }

    pub fn delay(&mut self, mut ticks: u32) {
        self.enable();

        while ticks != 0 {
            ticks = ticks - self.get_countflag();
        }

        self.disable();
    }
    
}

impl SysTickBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            reg: SysTick::new()
        }     
    }

    #[inline]
    pub fn build(self) -> SysTick {
        self.reg
    }

    #[inline]
    pub fn enable(self) -> Self {
        self.reg.ctrl.update(|val| val | 1);

        self
    }

    #[inline]
    pub fn disable(self) -> Self {
        self.reg.ctrl.update(|val| val & !1);

        self
    }

    #[inline]
    pub fn enable_int(self) -> Self {
        self.reg.ctrl.update(|val| val | (1 << 1));

        self
    }

    #[inline]
    pub fn disable_int(self) -> Self {
        self.reg.ctrl.update(|val| val | !(1 << 1));

        self
    }

    #[inline]
    pub fn source(self, source: ClockSource) -> Self {
        self.reg.ctrl.update(|val| (val & !(1 << 2)) | ((1 & source as u32) << 2));

        self
    }

    #[inline]
    pub fn set_reload(self, ticks: u32) -> Self {
        if (ticks - 1) > SYSTICK_LOAD_RELOAD_MSK {
            panic!();
        }

        self.reg.load.write(ticks);

        self
    }

    #[inline]
    pub fn set_val(self, val: u32) -> Self {
        if (val - 1) > SYSTICK_LOAD_RELOAD_MSK {
            panic!();
        }

        self.reg.val.write(val);

        self
    }
}


