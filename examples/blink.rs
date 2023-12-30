#![no_main]
#![no_std]

use air001_rs::{entry, exception, gpio::{Gpio, Port, Output}, rcc::Rcc, timer::systick::{SysTick, ClockSource, SysTickBuilder}};

#[exception("SysTick")]
fn systick() -> ! {
    let _rcc = Rcc::new()
        .enable_port(Port::GPIOB);

    let mut led: Gpio<Output, 3> = Gpio::<Output, 3>::new(Port::GPIOB);

    led.toggle();
}


#[entry]
fn main() -> ! {
    let mut sys_tick = SysTickBuilder::new()
        .source(ClockSource::AHBDiv8)
        .disable_int()
        .set_reload(8_000)
        .build();

    sys_tick.enable();

    let _rcc = Rcc::new()
        .enable_port(Port::GPIOB);

    let mut led: Gpio<Output, 0> = Gpio::<Output, 0>::new(Port::GPIOB);

    led.toggle();

    loop {
        sys_tick.delay(1000);
        led.toggle();
    }
}
