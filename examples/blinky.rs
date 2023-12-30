#![no_main]
#![no_std]

use air001_rs::{
    entry, 
    gpio::{
        Port, 
        Output, 
        GpioBuilder
    }, 
    rcc::RccBuilder, 
    timer::systick::{
        ClockSource, 
        SysTickBuilder
    }
};

#[entry]
fn main() -> ! {
    let _rcc = RccBuilder::new()
        .enable_port(Port::GPIOB)
        .build();

    let mut sys_tick = SysTickBuilder::new()
        .source(ClockSource::AHBDiv8)
        .disable_int()
        .set_reload(8_000)
        .build();

    let mut led = GpioBuilder::<Output, 0>::new(Port::GPIOB)
        .build();

    led.toggle();

    loop {
        sys_tick.delay(1000);
        led.toggle();
    }
}
