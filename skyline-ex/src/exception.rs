
/// Register/information context for user-mode exceptions
#[repr(C)]
struct ExceptionContext {
    /// The regular inline context used by skyline hooks
    register_context: skyline::hooks::InlineCtx,

    /// The PC (program counter) where the exception occurred
    pc: skyline::hooks::CpuRegister,

    // I'm not gonna lie I don't really know what these are but they are provided in
    // the kernel's exception info
    pstate: u32,
    afsr0: u32,
    afsr1: u32,
    esr: u32,

    /// Fault address register
    far: u32,
}

/// The size of the exception handler stack
const EXCEPTION_HANDLER_STACK_SIZE: usize = 0x10000;

/// Custom struct for our exception handler stack so that we can align it
#[repr(align(0x1000))]
struct ExceptionHandlerStack([u8; EXCEPTION_HANDLER_STACK_SIZE]);

/// Our stack, initialized to zero in the BSS
static mut EXCEPTION_HANDLER_STACK: ExceptionHandlerStack = ExceptionHandlerStack([0u8; EXCEPTION_HANDLER_STACK_SIZE]);

/// The top of our stack, set during exception handler initialization
#[export_name = "__skex_exception_handler_stack_top"]
static mut EXCEPTION_HANDLER_STACK_TOP: u64 = 0;

// Import the ASM exception handler which gets the register context and also changes our stack pointer
std::arch::global_asm!(include_str!("exception_handler.s"));

/// Our custom exception handler
#[no_mangle]
extern "C" fn __skex_user_exception_handler(ctx: &ExceptionContext) {
    use std::io::Write;
    // TODO: Make this better (lol)
    static mut REPORT_BUFFER: [u8; u16::MAX as usize] = [0u8; u16::MAX as usize];

    let mut cursor = unsafe { std::io::Cursor::new(&mut REPORT_BUFFER[..]) };
    let logger = &crate::logger::LOGGER;

    let _ = logger.print("Entering exception handler...\n");
    std::thread::sleep(std::time::Duration::from_millis(1000));

    let _ = writeln!(&mut cursor, "Exception Ocurred!");
    let _ = write!(&mut cursor, "Current PC: ");
    let _ = skyline::hooks::Backtrace::write_formatted_addr(&mut cursor, ctx.pc.x());
    let _ = writeln!(&mut cursor);
    let _ = match skyline::hooks::Backtrace::new_from_inline_ctx(&ctx.register_context, 32) {
        Ok(bt) => {
            bt.write(&mut cursor).and_then(|_| writeln!(&mut cursor))
        },
        Err(e) => writeln!(&mut cursor, "Invalid backtrace: {:?}", e)
    };

    let len = cursor.position() as usize;
    unsafe {
        let _ = logger.print(std::str::from_utf8(&REPORT_BUFFER[0..len]).unwrap());
    }

    let _ = logger.print("Flushing logger and terminating...\n");
    logger.terminate();
}

extern "C" {
    /// The exception handler in nnSdk, which we replace completely
    #[link_name = "_ZN2nn2os6detail20UserExceptionHandlerEv"]
    fn nn_user_exception_handler();

    /// The name of our exception handler in ASM
    fn skyline_ex_user_exception_handler();

    fn __rtld_resolve_self();
}

pub fn initialize() {
    // Start by setting our exception handler stack top
    unsafe {
        __rtld_resolve_self();
        EXCEPTION_HANDLER_STACK_TOP = &mut EXCEPTION_HANDLER_STACK.0[EXCEPTION_HANDLER_STACK_SIZE - 1] as *mut u8 as u64;
    }

    // We use the FFI directly since we are never going to call the trampoline and symbol resolving means
    // the macro causes another data abort as we write OOB of the provided exception handler stack
    crate::ffi::skex_hooks_install_on_symbol(
        skyline::rtld::get_module_for_self().unwrap() as *const _ as *mut _,
        nn_user_exception_handler as *const (),
        skyline_ex_user_exception_handler as *const (),
        &mut 0u64 as *mut u64 as *mut *mut (),
        skyline::hooks::HookType::Hook
    );
}