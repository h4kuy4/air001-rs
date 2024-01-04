#![no_main]
#![no_std]

use air001_rs::{
    entry, 
    gpio::{
        Output, 
        PinBuilder
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
        .enable_port('B')
        .build();

    let mut sys_tick = SysTickBuilder::new()
        .source(ClockSource::AHBDiv8)
        .disable_int()
        .set_reload(8_000)
        .build();

    let mut led = PinBuilder::<Output, 'B', 0>::new()
        .build();

    led.toggle();

    loop {
        sys_tick.delay(1000);
        led.toggle();
    }
}
