.text
.global Reset
.thumb_func
Reset:
    /* ldr r0, =_stack_top */
    /* mov sp,  r0 */

    bl main

