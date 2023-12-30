use core::panic::PanicInfo;

use crate::{rcc::RccBuilder, gpio::{Port, Output, GpioBuilder}};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let _rcc = RccBuilder::new()
        .enable_port(Port::GPIOB)
        .build();

    let mut led = GpioBuilder::<Output, 1>::new(Port::GPIOB)
        .build();

    led.set();

    loop {
        
    }
}

