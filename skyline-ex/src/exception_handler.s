.section .text.skyline_ex_user_exception_handler, "ax", %progbits
.global skyline_ex_user_exception_handler
.type skyline_ex_user_exception_handler, %function
.align 2
.cfi_startproc

// Inputs:
// * x0: Exception type
// * x1: ExceptionInfo*
skyline_ex_user_exception_handler:
    // Get our stack pointer
    adrp x8, :got:__skex_exception_handler_stack_top
    ldr x8, [x8, :got_lo12:__skex_exception_handler_stack_top]
    ldr x8, [x8]
    sub x8, x8, #0x320

    // The first 0x48 byets of the ExceptionInfo* struct are for
    // x0-x8, so we copy those into our new exception context
    ldp x0, x2, [x1]
    stp x0, x2, [x8]
    ldp x0, x2, [x1, #0x10]
    stp x0, x2, [x8, #0x10]
    ldp x0, x2, [x1, #0x20]
    stp x0, x2, [x8, #0x20]
    ldp x0, x2, [x1, #0x30]
    stp x0, x2, [x8, #0x30]
    ldr x0, [x8, #0x40]
    mov x2, x9
    stp x0, x2, [x8, #0x40]

    // We now have registers x0-x9 stored, the rest should have been properly saved by AMS so we 
    // are going to just store them in the context
    stp x10, x11, [x8, #0x50]
    stp x12, x13, [x8, #0x60]
    stp x14, x15, [x8, #0x70]
    stp x16, x17, [x8, #0x80]
    stp x18, x19, [x8, #0x90]
    stp x20, x21, [x8, #0xA0]
    stp x22, x23, [x8, #0xB0]
    stp x24, x25, [x8, #0xC0]
    stp x26, x27, [x8, #0xD0]
    stp x28, x29, [x8, #0xE0]

    // The LR and stack pointer are saved in the ExceptionInfo, so we will use those
    ldp x0, x2, [x1, #0x48]
    stp x0, x2, [x8, #0xF0]

    // Now we store the FPU registers
    stp  q0,  q1, [x8, #0x100]
    stp  q2,  q3, [x8, #0x120]
    stp  q4,  q5, [x8, #0x140]
    stp  q6,  q7, [x8, #0x160]
    stp  q8,  q9, [x8, #0x180]
    stp q10, q11, [x8, #0x1A0]
    stp q12, q13, [x8, #0x1C0]
    stp q14, q15, [x8, #0x1E0]
    stp q16, q17, [x8, #0x200]
    stp q18, q19, [x8, #0x220]
    stp q20, q21, [x8, #0x240]
    stp q22, q23, [x8, #0x260]
    stp q24, q25, [x8, #0x280]
    stp q26, q27, [x8, #0x2A0]
    stp q28, q29, [x8, #0x2C0]
    stp q30, q31, [x8, #0x2E0]

    // Move the PC and pstate/afsr0 into the stack
    ldp x0, x2, [x1, #0x58]
    add x3, x8, #0x300
    stp x0, x2, [x3]

    // Move the afsr1/esr into the stack
    ldr x0, [x1, #0x68]
    str x0, [x3, #0x10]
    ldr w0, [x1, #0x70]
    str w0, [x3, #0x18]

    mov x9, sp
    mov x0, x8
    mov sp, x8
    bl __skex_user_exception_handler
    mov sp, x9
    mov w0, #0xf801
    svc 0x28
.cfi_endproc