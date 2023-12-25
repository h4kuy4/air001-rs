use crate::register::{RW, RO, WO};

pub struct GpioReg {
    reg: &'static mut Reg
}

#[repr(C)]
pub struct Reg {
    pub moder:   RW<u32>,
    pub otyper:  RW<u32>,
    pub ospeedr: RW<u32>,
    pub pupdr:   RW<u32>,
    pub idr:     RO<u32>,
    pub odr:     RW<u32>,
    pub bsr:     WO<u32>,
    pub bsrr:    WO<u32>,
    pub lckr:    RW<u32>,
    pub afrl:    RW<u32>,
    pub afrh:    RW<u32>,
    pub brr:     RW<u32>
}

impl GpioReg {
    #[inline]
    pub fn new(addr: u32) -> Self {
        Self { 
            reg: unsafe {
                &mut *(addr as *mut Reg)
            } 
        }
    }

    #[inline]
    pub fn set_mode(&mut self, mask: u32, val: u32) {
        let old_val = self.reg.moder.read();

        let val = (old_val ^ (old_val & mask)) | (val & mask);

        self.reg.moder.write(val);
    }

    #[inline]
    pub fn set_port_mode(&mut self, val: u32) {
        let old_val = self.reg.moder.read();

        let val = old_val | val;

        self.reg.moder.write(val);
    }

    pub fn set_output_type(&mut self, val: u32) {
        self.reg.otyper.write(val);
    }

    pub fn set_output_speed(&mut self, val: u32) {
        self.reg.ospeedr.write(val);
    }

    pub fn set_pull_mode(&mut self, val: u32) {
        self.reg.pupdr.write(val);
    }

    #[inline]
    pub fn set_output_data(&mut self, mask: u32, val: u32) {
        let old_val = self.reg.odr.read();

        let val = (old_val ^ (old_val & mask)) | (val & mask);

        self.reg.odr.write(val);
    }

    #[inline]
    pub fn set_port_output_data(&mut self, val: u32) {
        let old_val = self.reg.odr.read();

        let val = old_val | val;

        self.reg.odr.write(val);
    }

    pub fn bit_set_reset(&mut self, val: u32) {
        self.reg.bsrr.write(val);
    }

    pub fn set_lock(&mut self, val: u32) {
        self.reg.lckr.write(val);
    }

    pub fn set_alt_func_l(&mut self, val: u32) {
        self.reg.afrl.write(val);
    }

    pub fn set_alt_func_h(&mut self, val: u32) {
        self.reg.afrh.write(val);
    }

    pub fn get_mode(&self) -> u32 {
        self.reg.moder.read()
    }

    pub fn get_output_type(&self) -> u32 {
        self.reg.otyper.read()
    }

    pub fn get_output_speed(&self) -> u32 {
        self.reg.ospeedr.read()
    }
    
    pub fn get_pull_mode(&self) -> u32 {
        self.reg.pupdr.read()
    }

    pub fn get_input_data(&self) -> u32 {
        self.reg.idr.read()
    }

    pub fn get_lock(&self) -> u32 {
        self.reg.lckr.read()
    }

    pub fn get_alt_func_l(&self) -> u32 {
        self.reg.afrl.read()
    }

    pub fn get_alt_func_h(&self) -> u32 {
        self.reg.afrh.read()
    }
}
