use core::panic::PanicInfo;

use crate::{rcc::RccBuilder, gpio::{Output, PinBuilder}};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let _rcc = RccBuilder::new()
        .enable_port('B')
        .build();

    let led = PinBuilder::<Output, 'B', 1>::new()
        .build();

    led.set();

    loop {
        
    }
}

