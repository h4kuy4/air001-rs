#![no_main]
#![no_std]

use air001_rs::{entry, gpio::{Gpio, Port, Output}, rcc::Rcc};


#[entry]
fn main() -> ! {
    let _rcc = Rcc::new()
        .enable_port(Port::GPIOB);

    let mut led: Gpio<Output, 0> = Gpio::<Output, 0>::new(Port::GPIOB);

    led.set();

    loop {
        
    }
}
