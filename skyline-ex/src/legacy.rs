use skyline::hooks::HookType;


#[export_name = "A64HookFunction"]
pub extern "C" fn skyline_compat_hook_function(symbol: *const (), replace: *const (), trampoline: *mut *const ()) {
    unsafe {
        if !trampoline.is_null() {
            *trampoline = crate::ffi::skex_hooks_install(symbol, replace, HookType::Hook);
        } else {
            crate::ffi::skex_hooks_install(symbol, replace, HookType::Hook);
        }
    }
}

#[export_name = "A64InlineHook"]
pub extern "C" fn skyline_compat_inline_hook(symbol: *const (), callback: *const ()) {
    println!("[skyline-ex::legacy] Installing inline hook at {:#x}", symbol as u64 - crate::ffi::skex_memory_get_known_static_module(skyline::memory::StaticModule::Main).text().as_ptr() as u64);
    crate::ffi::skex_hooks_install(symbol, callback, HookType::LegacyInline);
}

#[export_name = "get_plugin_addresses"]
pub extern "C" fn skyline_compat_get_plugin_addresses() {
    unimplemented!()
}

#[export_name = "get_program_id"]
pub extern "C" fn skyline_compat_get_plugin_id() -> u64 {
    skyline::nx::get_program_id()
}

#[export_name = "getRegionAddress"]
pub extern "C" fn skyline_compat_get_region_address(region: u8) -> u64 {
    let info = skyline::memory::get_module(skyline::memory::StaticModule::Main);
    match region {
        0 => info.text().as_ptr() as _,
        1 => info.rodata().as_ptr() as _,
        2 => info.data().as_ptr() as _,
        3 => info.bss().as_ptr() as _,
        4 => {
            skyline::nx::get_heap_region_address() as _
        },
        _ => u64::MAX
    }
}

#[export_name = "skyline_tcp_send_raw"]
pub extern "C" fn skyline_compat_send_raw(msg: *const u8, size: usize) {
    unsafe {
        let _ = crate::logger::LOGGER.print(std::str::from_utf8_unchecked(std::slice::from_raw_parts(
            msg,
            size
        )));
    }
}

#[export_name = "sky_memcpy"]
pub extern "C" fn skyline_compat_protected_memcpy(dst: *mut u8, src: *const u8, size: usize) -> i32 {
    extern "C" {
        fn protected_memcpy(dst: *mut u8, src: *const u8, size: usize) -> i32;
    }
    unsafe {
        protected_memcpy(dst, src, size)
    }
}