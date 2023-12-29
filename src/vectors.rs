use crate::{rcc::Rcc, gpio::{Port, Gpio, Output}};

#[repr(C)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

extern "C" {
    fn Reset() -> !;
    fn NMI();
    fn HardFault();
    fn SVCall();
    fn PendSV();
    fn SysTick();
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Vector; 14] = [
    // NMI_Handler
    Vector { handler: NMI },                // 0x08
    // HardFault_Handler
    Vector { handler: HardFault },          // 0x0C
    Vector { reserved: 0 },                 // 0x10
    Vector { reserved: 0 },                 // 0x14
    Vector { reserved: 0 },                 // 0x18
    Vector { reserved: 0 },                 // 0x1C
    Vector { reserved: 0 },                 // 0x20
    Vector { reserved: 0 },                 // 0x24
    Vector { reserved: 0 },                 // 0x28
    // SVCall
    Vector { handler: SVCall },             // 0x2C
    Vector { reserved: 0 },                 // 0x30
    Vector { reserved: 0 },                 // 0x34
    // PendSV
    Vector { handler: PendSV },             // 0x38
    // SysTick
    Vector { handler: SysTick },            // 0x3C
];

extern "C" {
    fn WWDG();
    fn PVD();
    fn RTC();
    fn Flash();
    fn RCC();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_15();
    fn DMA_Channel1();
    fn DMA_Channel2_3();
    fn ADC_COMP();
    fn TIM1_BRK_UP_TRG_COM();
    fn TIM1_CC();
    fn TIM3();
    fn LPTIM1();
    fn TIM14();
    fn TIM16();
    fn TIM17();
    fn I2C1();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn LED();
}

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    // WWDG
    Vector { handler: WWDG },
    // PVD
    Vector { handler: PVD },
    // RTC
    Vector { handler: RTC },
    // Flash
    Vector { handler: Flash },
    // RCC
    Vector { handler: RCC },
    // EXTI0_1
    Vector { handler: EXTI0_1 },
    // EXTI2_3
    Vector { handler: EXTI2_3 },
    // EXTI4_15
    Vector { handler: EXTI4_15 },

    Vector { reserved: 0 },
    // DMA_Channel1
    Vector { handler: DMA_Channel1 },
    // DMA_Channel2_3
    Vector { handler: DMA_Channel2_3 },

    Vector { reserved: 0 },
    // ADC_COMP
    Vector { handler: ADC_COMP },
    // TIM1_BRK_UP_TRG_COM
    Vector { handler: TIM1_BRK_UP_TRG_COM },
    // TIM1_CC
    Vector { handler: TIM1_CC },

    Vector { reserved: 0 },
    // TIM3
    Vector { handler: TIM3 },
    // LPTIM1
    Vector { handler: LPTIM1 },

    Vector { reserved: 0 },
    // TIM14
    Vector { handler: TIM14 },
    
    Vector { reserved: 0 },
    // TIM16
    Vector { handler: TIM16 },
    // TIM17
    Vector { handler: TIM17 },
    // I2C1
    Vector { handler: I2C1 },
    
    Vector { reserved: 0 },
    // SPI1
    Vector { handler: SPI1 },
    // SPI2
    Vector { handler: SPI2 },
    // USART1
    Vector { handler: USART1 },
    // USART2
    Vector { handler: USART2 },

    Vector { reserved: 0 },
    // LED
    Vector { handler: LED },

    Vector { reserved: 0 }
];

#[no_mangle]
pub extern "C" fn HardFault_() {
    let _rcc = Rcc::new()
        .enable_port(Port::GPIOB);

    let mut led = Gpio::<Output, 1>::new(Port::GPIOB);

    led.set();

    loop {
        
    }
}

#[no_mangle]
pub extern "C" fn DefaultHandler() {
    loop {
        
    }
}
