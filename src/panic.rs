use core::panic::PanicInfo;

use crate::{rcc::Rcc, gpio::{Port, Gpio, Output}};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let _rcc = Rcc::new()
        .enable_port(Port::GPIOB);

    let mut led = Gpio::<Output, 1>::new(Port::GPIOB);

    led.set();

    loop {
        
    }
}

