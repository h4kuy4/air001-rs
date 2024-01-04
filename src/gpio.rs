use core::{marker::PhantomData, ptr::NonNull};

use volatile::VolatilePtr;

use crate::set_bit;

pub struct Input;
pub struct Output;
pub struct AltFunc;
pub struct Analog;

pub struct Pin<Mode, const PORT_NUM: char, const PIN_NUM: u8> {
    moder:   VolatilePtr<'static, u32>,
    otyper:  VolatilePtr<'static, u32>,
    ospeedr: VolatilePtr<'static, u32>,
    pupdr:   VolatilePtr<'static, u32>,
    idr:     VolatilePtr<'static, u32>,
    odr:     VolatilePtr<'static, u32>,
    bsrr:    VolatilePtr<'static, u32>,
    lckr:    VolatilePtr<'static, u32>,
    afrl:    VolatilePtr<'static, u32>,
    afrh:    VolatilePtr<'static, u32>,
    brr:     VolatilePtr<'static, u32>,
    _mode: PhantomData<Mode>
}

pub struct PinBuilder<Mode, const PORT_NUM: char, const PIN_NUM: u8> {
    reg: Pin<Mode, PORT_NUM, PIN_NUM>
}

pub enum PullMode {
    None     = 0b00,
    PullUp   = 0b01,
    PullDown = 0b11
}

impl<Mode, const PORT_NUM: char, const PIN_NUM: u8> Pin<Mode, PORT_NUM, PIN_NUM> {
    pub fn set_pull_mode(&self, mode: PullMode) {
    }
}

impl<const PORT_NUM: char, const PIN_NUM: u8> Pin<Input, PORT_NUM, PIN_NUM> {
    pub fn read(&mut self) -> bool {
        let val = self.idr.read();

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

impl<const PORT_NUM: char, const PIN_NUM: u8> Pin<Output, PORT_NUM, PIN_NUM> {
    pub fn set_speed(&self, speed: OutputSpeed) {
        self.ospeedr.update(|val| set_bit!(val, PIN_NUM * 2, 0b11, speed as u32));
    }

    pub fn set_type(&self, otype: OutputType) {
        self.otyper.update(|val| set_bit!(val, PIN_NUM, 0b01, otype as u32));
    }

    pub fn set(&self) {
        self.odr.update(|val| set_bit!(val, PIN_NUM, 0b01, 0b01));
    }

    pub fn reset(&mut self) {
        self.odr.update(|val| set_bit!(val, PIN_NUM, 0b01, 0b00));
    }

    pub fn write(&mut self, level: bool) {
        self.odr.update(|val| set_bit!(val, PIN_NUM, 0b01, if level { 0b01 } else { 0b00 }));
    }

    pub fn read(&mut self) -> bool {
        let val = self.odr.read();

        if (val >> PIN_NUM) & 0b01 == 0 {
            false
        } else {
            true
        }
    }

    pub fn toggle(&mut self) {
        if self.read() {
            self.reset();
        } else {
            self.set();
        }
    }
}

pub enum AltFuncNum {
    AF0  = 0b0000,
    AF1  = 0b0001,
    AF2  = 0b0010,
    AF3  = 0b0011,
    AF4  = 0b0100,
    AF5  = 0b0101,
    AF6  = 0b0110,
    AF7  = 0b0111,
    AF8  = 0b1000,
    AF9  = 0b1001,
    AF10 = 0b1010,
    AF11 = 0b1011,
    AF12 = 0b1100,
    AF13 = 0b1101,
    AF14 = 0b1110,
    AF15 = 0b1111,
}

impl<const PORT_NUM: char, const PIN_NUM: u8> Pin<AltFunc, PORT_NUM, PIN_NUM> {
    pub fn set_func(&mut self, func: AltFuncNum) {
        if PIN_NUM > 7 {
            self.afrl.update(|val| set_bit!(val, PIN_NUM * 4, 0b1111, func as u32)); 
        } else {
            self.afrh.update(|val| set_bit!(val, PIN_NUM * 4, 0b1111, func as u32)); 
        }
    }
}

impl<const PORT_NUM: char, const PIN_NUM: u8> Pin<Analog, PORT_NUM, PIN_NUM> {
}

impl<Mode, const PORT_NUM: char, const PIN_NUM: u8> PinBuilder<Mode, PORT_NUM, PIN_NUM> {
    pub fn build(self) -> Pin<Mode, PORT_NUM, PIN_NUM> {
        self.reg
    }

    pub fn pull_mode(self, mode: PullMode) -> Self {
        self.reg.pupdr.update(|val| set_bit!(val, PIN_NUM * 2, 0b11, mode as u32));

        self
    }
}

impl<const PORT_NUM: char, const PIN_NUM: u8> PinBuilder<Input, PORT_NUM, PIN_NUM> {
    pub fn new() -> Self {
        let base = match PORT_NUM {
            'A' => 0x5000_0000,
            'B' => 0x5000_0400,
            'F' => 0x5000_1400,
            _   => panic!()
        };

        let builder = Self { 
            reg: unsafe {
                Pin {
                    moder  : VolatilePtr::new(NonNull::new_unchecked((base + 0x00) as *mut u32)),
                    otyper : VolatilePtr::new(NonNull::new_unchecked((base + 0x04) as *mut u32)),
                    ospeedr: VolatilePtr::new(NonNull::new_unchecked((base + 0x08) as *mut u32)),
                    pupdr  : VolatilePtr::new(NonNull::new_unchecked((base + 0x0C) as *mut u32)),
                    idr    : VolatilePtr::new(NonNull::new_unchecked((base + 0x10) as *mut u32)),
                    odr    : VolatilePtr::new(NonNull::new_unchecked((base + 0x14) as *mut u32)),
                    bsrr   : VolatilePtr::new(NonNull::new_unchecked((base + 0x18) as *mut u32)),
                    lckr   : VolatilePtr::new(NonNull::new_unchecked((base + 0x1C) as *mut u32)),
                    afrl   : VolatilePtr::new(NonNull::new_unchecked((base + 0x20) as *mut u32)),
                    afrh   : VolatilePtr::new(NonNull::new_unchecked((base + 0x24) as *mut u32)),
                    brr    : VolatilePtr::new(NonNull::new_unchecked((base + 0x28) as *mut u32)),

                    _mode: PhantomData
                }
            }
        };

        builder.reg.moder.update(|val| set_bit!(val, PIN_NUM * 2, 0b11, 0b00));

        builder
    }
}

impl<const PORT_NUM: char, const PIN_NUM: u8> PinBuilder<Output, PORT_NUM, PIN_NUM> {
    pub fn new() -> Self {
        let base = match PORT_NUM {
            'A' => 0x5000_0000,
            'B' => 0x5000_0400,
            'F' => 0x5000_1400,
            _   => panic!()
        };

        let builder = Self { 
            reg: unsafe {
                Pin {
                    moder  : VolatilePtr::new(NonNull::new_unchecked((base + 0x00) as *mut u32)),
                    otyper : VolatilePtr::new(NonNull::new_unchecked((base + 0x04) as *mut u32)),
                    ospeedr: VolatilePtr::new(NonNull::new_unchecked((base + 0x08) as *mut u32)),
                    pupdr  : VolatilePtr::new(NonNull::new_unchecked((base + 0x0C) as *mut u32)),
                    idr    : VolatilePtr::new(NonNull::new_unchecked((base + 0x10) as *mut u32)),
                    odr    : VolatilePtr::new(NonNull::new_unchecked((base + 0x14) as *mut u32)),
                    bsrr   : VolatilePtr::new(NonNull::new_unchecked((base + 0x18) as *mut u32)),
                    lckr   : VolatilePtr::new(NonNull::new_unchecked((base + 0x1C) as *mut u32)),
                    afrl   : VolatilePtr::new(NonNull::new_unchecked((base + 0x20) as *mut u32)),
                    afrh   : VolatilePtr::new(NonNull::new_unchecked((base + 0x24) as *mut u32)),
                    brr    : VolatilePtr::new(NonNull::new_unchecked((base + 0x28) as *mut u32)),

                    _mode: PhantomData
                }
            }
        };

        builder.reg.moder.update(|val| set_bit!(val, PIN_NUM * 2, 0b11, 0b01));

        builder
    }

    pub fn output_type(self, otype: OutputType) -> Self {
        self.reg.otyper.update(|val| set_bit!(val, PIN_NUM, 0b01, otype as u32));
        
        self
    }

    pub fn speed(self, speed: OutputSpeed) -> Self {
        self.reg.ospeedr.update(|val| set_bit!(val, PIN_NUM * 2, 0b11, speed as u32));

        self
    }
}

impl<const PORT_NUM: char, const PIN_NUM: u8> PinBuilder<AltFunc, PORT_NUM, PIN_NUM> {
    pub fn new() -> Self {
        let base = match PORT_NUM {
            'A' => 0x5000_0000,
            'B' => 0x5000_0400,
            'F' => 0x5000_1400,
            _   => panic!()
        };

        let builder = Self { 
            reg: unsafe {
                Pin {
                    moder  : VolatilePtr::new(NonNull::new_unchecked((base + 0x00) as *mut u32)),
                    otyper : VolatilePtr::new(NonNull::new_unchecked((base + 0x04) as *mut u32)),
                    ospeedr: VolatilePtr::new(NonNull::new_unchecked((base + 0x08) as *mut u32)),
                    pupdr  : VolatilePtr::new(NonNull::new_unchecked((base + 0x0C) as *mut u32)),
                    idr    : VolatilePtr::new(NonNull::new_unchecked((base + 0x10) as *mut u32)),
                    odr    : VolatilePtr::new(NonNull::new_unchecked((base + 0x14) as *mut u32)),
                    bsrr   : VolatilePtr::new(NonNull::new_unchecked((base + 0x18) as *mut u32)),
                    lckr   : VolatilePtr::new(NonNull::new_unchecked((base + 0x1C) as *mut u32)),
                    afrl   : VolatilePtr::new(NonNull::new_unchecked((base + 0x20) as *mut u32)),
                    afrh   : VolatilePtr::new(NonNull::new_unchecked((base + 0x24) as *mut u32)),
                    brr    : VolatilePtr::new(NonNull::new_unchecked((base + 0x28) as *mut u32)),

                    _mode: PhantomData
                }
            }
        };

        builder.reg.moder.update(|val| set_bit!(val, PIN_NUM * 2, 0b11, 0b10));

        builder
    }

    pub fn func(self, func: AltFuncNum) -> Self {
        if PIN_NUM > 7 {
            self.reg.afrl.update(|val| set_bit!(val, PIN_NUM * 4, 0b1111, func as u32)); 
        } else {
            self.reg.afrh.update(|val| set_bit!(val, PIN_NUM * 4, 0b1111, func as u32)); 
        }

        self
    }
}

impl<const PORT_NUM: char, const PIN_NUM: u8> PinBuilder<Analog, PORT_NUM, PIN_NUM> {
    pub fn new() -> Self {
        let base = match PORT_NUM {
            'A' => 0x5000_0000,
            'B' => 0x5000_0400,
            'F' => 0x5000_1400,
            _   => panic!()
        };

        let builder = Self { 
            reg: unsafe {
                Pin {
                    moder  : VolatilePtr::new(NonNull::new_unchecked((base + 0x00) as *mut u32)),
                    otyper : VolatilePtr::new(NonNull::new_unchecked((base + 0x04) as *mut u32)),
                    ospeedr: VolatilePtr::new(NonNull::new_unchecked((base + 0x08) as *mut u32)),
                    pupdr  : VolatilePtr::new(NonNull::new_unchecked((base + 0x0C) as *mut u32)),
                    idr    : VolatilePtr::new(NonNull::new_unchecked((base + 0x10) as *mut u32)),
                    odr    : VolatilePtr::new(NonNull::new_unchecked((base + 0x14) as *mut u32)),
                    bsrr   : VolatilePtr::new(NonNull::new_unchecked((base + 0x18) as *mut u32)),
                    lckr   : VolatilePtr::new(NonNull::new_unchecked((base + 0x1C) as *mut u32)),
                    afrl   : VolatilePtr::new(NonNull::new_unchecked((base + 0x20) as *mut u32)),
                    afrh   : VolatilePtr::new(NonNull::new_unchecked((base + 0x24) as *mut u32)),
                    brr    : VolatilePtr::new(NonNull::new_unchecked((base + 0x28) as *mut u32)),

                    _mode: PhantomData
                }
            }
        };

        builder.reg.moder.update(|val| set_bit!(val, PIN_NUM * 2, 0b11, 0b11));

        builder
    }
}
