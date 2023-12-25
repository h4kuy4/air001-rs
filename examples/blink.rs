#![no_main]
#![no_std]

use core::{panic::PanicInfo, arch::global_asm};

use air001_rs::gpio::{GpioBuilder, Port, Pin, PinMode, PinLevel, raw::Reg};

global_asm!(include_str!("startup.S"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() {
    let rcc_iopenr = 0x4002_1034 as *mut u32;

    unsafe {
        (*rcc_iopenr) = 0b00_00_00_11;
    }

    let mut led = GpioBuilder::new(Port::GPIOB, Pin::Pin1)
        .mode(PinMode::Output)
        .build();

    led.set_level(PinLevel::High);

    loop {
        
    }
}

#[no_mangle]
pub extern "C" fn fault() {
    let rcc_iopenr = 0x4002_1034 as *mut u32;

    unsafe {
        (*rcc_iopenr) = 0b00_00_00_11;
    }

    let mut led = GpioBuilder::new(Port::GPIOB, Pin::Pin1)
        .mode(PinMode::Output)
        .build();

    led.set_level(PinLevel::High);

    loop {
        
    }
}
