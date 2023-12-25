__ROM_BASE = 0x08000000;
__ROM_SIZE = 0x00008000;

__RAM_BASE = 0x20000000;
__RAM_SIZE = 0x00001000;

__STACK_SIZE = 0x00000400;
__HEAP_SIZE  = 0x00000000;

MEMORY {
    FLASH (rx)  : ORIGIN = __ROM_BASE, LENGTH = __ROM_SIZE
    RAM   (rwx) : ORIGIN = __RAM_BASE, LENGTH = __RAM_SIZE
}

/* Entry Point of program */
ENTRY(Reset_Handler);

SECTIONS {
    .text : {
        _stack_top = ORIGIN(RAM) + LENGTH(RAM);

        LONG(_stack_top)
        KEEP(* (.isr_vec_table))
        . = ALIGN(4);

        *(.text)
        . = ALIGN(4);

        *(.rodata*)
        . = ALIGN(4);
        
    } > FLASH  /* Put this in the flash memory region */

}
