MEMORY {
    FLASH     (rx)  : ORIGIN = 0x08000000, LENGTH = 0x8000
    RAM       (rwx) : ORIGIN = 0x20000000, LENGTH = 0x1000
}

/* Entry Point of program */
ENTRY(Reset);

EXTERN(DefaultHandler);
EXTERN(__EXCEPTIONS);
PROVIDE(NMI       = DefaultHandler);
PROVIDE(HardFault = HardFault_);
PROVIDE(SVCall    = DefaultHandler);
PROVIDE(PendSV    = DefaultHandler);
PROVIDE(SysTick   = DefaultHandler);

EXTERN(__INTERRUPT);
PROVIDE(WWDG                = DefaultHandler);
PROVIDE(PVD                 = DefaultHandler);
PROVIDE(RTC                 = DefaultHandler);
PROVIDE(Flash               = DefaultHandler);
PROVIDE(RCC                 = DefaultHandler);
PROVIDE(EXTI0_1             = DefaultHandler);
PROVIDE(EXTI2_3             = DefaultHandler);
PROVIDE(EXTI4_15            = DefaultHandler);
PROVIDE(DMA_Channel1        = DefaultHandler);
PROVIDE(DMA_Channel2_3      = DefaultHandler);
PROVIDE(ADC_COMP            = DefaultHandler);
PROVIDE(TIM1_BRK_UP_TRG_COM = DefaultHandler);
PROVIDE(TIM1_CC             = DefaultHandler);
PROVIDE(TIM3                = DefaultHandler);
PROVIDE(LPTIM1              = DefaultHandler);
PROVIDE(TIM14               = DefaultHandler);
PROVIDE(TIM16               = DefaultHandler);
PROVIDE(TIM17               = DefaultHandler);
PROVIDE(I2C1                = DefaultHandler);
PROVIDE(SPI1                = DefaultHandler);
PROVIDE(SPI2                = DefaultHandler);
PROVIDE(USART1              = DefaultHandler);
PROVIDE(USART2              = DefaultHandler);
PROVIDE(LED                 = DefaultHandler);

SECTIONS {

    .vector_table ORIGIN(FLASH) : {
        __vector_table = .;

        PROVIDE(_stack_top = ORIGIN(RAM) + LENGTH(RAM));
        LONG(_stack_top)

        KEEP(*(.vector_table.reset_vector));

        __exceptions = .;
        KEEP(*(.vector_table.exceptions));
        __eexceptions = .;

        KEEP(*(.vector_table.interrupts));

    }

    PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table));

    .text _stext : {
        __stext = .;

        *(.text .text.*);

        . = ALIGN(4);
        __etext = .;
    } > FLASH

    .rodata : ALIGN(4) {
        . = ALIGN(4);
        __srodata = .;

        *(.rodata .rodata*);

        . = ALIGN(4);
        __erodata = .;
    } > RAM
    
    .data : {
        . = ALIGN(4);
        __sdata = .;

        *(.data .data.*);

        . = ALIGN(4);
        __edata = .;
    } > RAM

    _sidata = LOADADDR(.data);

    .bss (NOLOAD) : ALIGN(4) {
        . = ALIGN(4);
        __sbss = .;

        *(.bss .bss.*);
        *(COMMON);

        . = ALIGN(4);
        __ebss = .;
    } > RAM

    /DISCARD/ : {
        *(.ARM.exidx);
        *(.ARM.exidx.*);
        *(.ARM.extab.*);
    }
}
