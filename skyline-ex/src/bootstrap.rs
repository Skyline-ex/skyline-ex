
// Extern block to link into exlaunch's initialization requirements
extern "C" {
    #[link_name = "_ZN3exl4util11proc_handle3GetEv"]
    fn exl_util_proc_handle_get() -> u32;
    
    #[link_name = "_ZN3exl4util4Hook10InitializeEv"]
    fn exl_util_hook_initialize();

    #[link_name = "envSetOwnProcessHandle"]
    fn env_set_own_process_handle(handle: u32);

    fn exl_module_init();
}

// cargo skyline does this really awesome thing which means that we have to link *back* into
// exl in order to eventually get into exl_main

#[no_mangle]
unsafe extern "C" fn __custom_init() {
    exl_module_init()
}

#[no_mangle]
pub unsafe extern "C" fn exl_main(_: u64, _: u64) {
    let proc_handle = exl_util_proc_handle_get();
    env_set_own_process_handle(proc_handle);

    exl_util_hook_initialize();

    crate::main();
}