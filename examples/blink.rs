#![no_main]
#![no_std]

use air001_rs::{entry, gpio::{Gpio, Port, Output}};


#[entry]
fn main() -> ! {
    let rcc_iopenr = 0x4002_1034 as *mut u32;

    unsafe {
        (*rcc_iopenr) = 0b00_00_00_11;
    }

    let mut led: Gpio<Output, 0> = Gpio::<Output, 0>::new(Port::GPIOB);

    led.set();

    loop {
        
    }
}
