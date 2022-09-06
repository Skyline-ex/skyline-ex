.macro CODE_BEGIN name
	.section .text.\name, "ax", %progbits
	.global \name
	.type \name, %function
	.align 2
	.cfi_startproc
\name:
.endm

.macro CODE_END
	.cfi_endproc
.endm

# Structures:
# HookCtx:
#   - uintptr_t @ 0x0 - Pointer to HookData
# HookData:
#   - uintptr_t @ 0x00 - Pointer to trampoline
#   - uintptr_t @ 0x08 - Pointer to user callback
#   - uintptr_t @ 0x10 - Pointer to the handler for the hook type
#   - bool @ 0x18 - Hook is enabled

# Backs up the register state of all GP registers
# to the stack
.macro armBackupRegisters
    sub sp, sp, #0x100

    stp  x0,  x1, [sp, #0x00]
    stp  x2,  x3, [sp, #0x10]
    stp  x4,  x5, [sp, #0x20]
    stp  x6,  x7, [sp, #0x30]
    stp  x8,  x9, [sp, #0x40]
    stp x10, x11, [sp, #0x50]
    stp x12, x13, [sp, #0x60]
    stp x14, x15, [sp, #0x70]
    stp x16, x17, [sp, #0x80]
    stp x18, x19, [sp, #0x90]
    stp x20, x21, [sp, #0xA0]
    stp x22, x23, [sp, #0xB0]
    stp x24, x25, [sp, #0xC0]
    stp x26, x27, [sp, #0xD0]
    stp x28, x29, [sp, #0xE0]
    str x30, [sp, #0xF0]
.endm

# Backs up the register state of all GP registers, stack pointer,
# and floating point registers to the stack
.macro armBackupRegistersEx
    sub sp, sp, #0x300
    
    stp  x0,  x1, [sp, #0x00]
    stp  x2,  x3, [sp, #0x10]
    stp  x4,  x5, [sp, #0x20]
    stp  x6,  x7, [sp, #0x30]
    stp  x8,  x9, [sp, #0x40]
    stp x10, x11, [sp, #0x50]
    stp x12, x13, [sp, #0x60]
    stp x14, x15, [sp, #0x70]
    stp x16, x17, [sp, #0x80]
    stp x18, x19, [sp, #0x90]
    stp x20, x21, [sp, #0xA0]
    stp x22, x23, [sp, #0xB0]
    stp x24, x25, [sp, #0xC0]
    stp x26, x27, [sp, #0xD0]
    stp x28, x29, [sp, #0xE0]

    add x0, sp, #0x300
    stp x30, x0, [sp, #0xF0]

    stp  q0,  q1, [sp, #0x100]
    stp  q2,  q3, [sp, #0x120]
    stp  q4,  q5, [sp, #0x140]
    stp  q6,  q7, [sp, #0x160]
    stp  q8,  q9, [sp, #0x180]
    stp q10, q11, [sp, #0x1A0]
    stp q12, q13, [sp, #0x1C0]
    stp q14, q15, [sp, #0x1E0]
    stp q16, q17, [sp, #0x200]
    stp q18, q19, [sp, #0x220]
    stp q20, q21, [sp, #0x240]
    stp q22, q23, [sp, #0x260]
    stp q24, q25, [sp, #0x280]
    stp q26, q27, [sp, #0x2A0]
    stp q28, q29, [sp, #0x2C0]
    stp q30, q31, [sp, #0x2E0]
.endm

# Recovers all of the GP registers from the stack
.macro armRecoverRegisters
    ldp  x0,  x1, [sp, #0x00]
    ldp  x2,  x3, [sp, #0x10]
    ldp  x4,  x5, [sp, #0x20]
    ldp  x6,  x7, [sp, #0x30]
    ldp  x8,  x9, [sp, #0x40]
    ldp x10, x11, [sp, #0x50]
    ldp x12, x13, [sp, #0x60]
    ldp x14, x15, [sp, #0x70]
    ldp x16, x17, [sp, #0x80]
    ldp x18, x19, [sp, #0x90]
    ldp x20, x21, [sp, #0xA0]
    ldp x22, x23, [sp, #0xB0]
    ldp x24, x25, [sp, #0xC0]
    ldp x26, x27, [sp, #0xD0]
    ldp x28, x29, [sp, #0xE0]
    ldr x30, [sp, #0xF0]

    add sp, sp, #0x100
.endm

# Recovers all of the GP and FP registers from the stack
.macro armRecoverRegistersEx
    ldp  x0,  x1, [sp, #0x00]
    ldp  x2,  x3, [sp, #0x10]
    ldp  x4,  x5, [sp, #0x20]
    ldp  x6,  x7, [sp, #0x30]
    ldp  x8,  x9, [sp, #0x40]
    ldp x10, x11, [sp, #0x50]
    ldp x12, x13, [sp, #0x60]
    ldp x14, x15, [sp, #0x70]
    ldp x16, x17, [sp, #0x80]
    ldp x18, x19, [sp, #0x90]
    ldp x20, x21, [sp, #0xA0]
    ldp x22, x23, [sp, #0xB0]
    ldp x24, x25, [sp, #0xC0]
    ldp x26, x27, [sp, #0xD0]
    ldp x28, x29, [sp, #0xE0]
    ldr x30, [sp, #0xF0]

    ldp  q0,  q1, [sp, #0x100]
    ldp  q2,  q3, [sp, #0x120]
    ldp  q4,  q5, [sp, #0x140]
    ldp  q6,  q7, [sp, #0x160]
    ldp  q8,  q9, [sp, #0x180]
    ldp q10, q11, [sp, #0x1A0]
    ldp q12, q13, [sp, #0x1C0]
    ldp q14, q15, [sp, #0x1E0]
    ldp q16, q17, [sp, #0x200]
    ldp q18, q19, [sp, #0x220]
    ldp q20, q21, [sp, #0x240]
    ldp q22, q23, [sp, #0x260]
    ldp q24, q25, [sp, #0x280]
    ldp q26, q27, [sp, #0x2A0]
    ldp q28, q29, [sp, #0x2C0]
    ldp q30, q31, [sp, #0x2E0]

    add sp, sp, #0x300
.endm 

# Input:
#   * x17 - HookData*
# Output:
#   * x16 - Trampoline
#   * x17 - User callback
.macro prepareHookState
    # Load the struct onto the registers
    # x16, x17
    ldrb w16, [x17, 0x18]

    tbnz w16, #0x0, #0xC
    ldr x16, [x17, 0x0]
    br x16

    # RunHook:
    ldp x16, x17, [x17]
.endm

# Detours are similar to instruction hooks except they are
# functionally observers.
#
# Detours are provided the arguments but their return values are unused,
# and any modifications made to the register state are unused.
#
# In cases where you merely want to observe the data, detours are guaranteed
# to be run whereas hooks are not.
CODE_BEGIN DetourHandlerImpl
    prepareHookState

    armBackupRegistersEx

    # Call the user callback
    blr x17

    armRecoverRegistersEx

    # Jump to the trampoline
    br x16
CODE_END

# Extended inline hooks are the same as
# regular inline hooks, except they provide read-only
# access to the stack pointer as well as read/write access
# to the floating point registers as well
CODE_BEGIN InlineExHandlerImpl
    prepareHookState

    armBackupRegistersEx

    mov x0, sp
    blr x17

    armRecoverRegistersEx

    br x16
CODE_END

# Inline hooks are functionally callbacks which can
# mutate the register state when they get run. They
# are provided a pointer to the register context which
# can be modified.
# 
# Any changes made to the register state via operations
# on the context will be passed along to the inline hooks
# and detours which follow them.
CODE_BEGIN InlineHandlerImpl
    prepareHookState

    armBackupRegisters

    mov x0, sp
    blr x17

    armRecoverRegisters

    br x16

CODE_END


# Hooks are the most raw level of redirection, as they have the ability
# to completely change the implementation. For this reason, hooks are run
# last in the chain (compared to inline hooks and detours).
#
# The trampoline register (x16) is provided as the "impl" function to call
# by the replacement. When multiple hooks have been installed on the same location,
# the impl function will point to the next function in the sequence.
CODE_BEGIN HookHandlerImpl
    prepareHookState

    br x17
CODE_END

# Every hook starts the same way, which is by retrieving
# the hook context from the JIT.
# The hook context is used to figure out where to branch to, so
# that the actual memory used in the JIT code space is little.
# The amount of JIT space taken up by every hook is 0xC + sizeof(HookCtx)
# which totals to 0x1C bytes.
# This JIT space is also allocated directly before main so that
# the amount of instructions overwritten is very few
CODE_BEGIN HookHandler
    adr x17, HookHandlerEnd
    ldr x17, [x17]
    ldr x16, [x17, #0x10]
    br x16
CODE_END
HookHandlerEnd:
    .byte 0xDE, 0xAD, 0xBE, 0xEF