use skyline::hooks::HookType;
use skyline::memory::ModuleMemory;
use skyline::memory::StaticModule;
use skyline::rtld;

extern "C" {
    fn install_hook(symbol: *const (), replace: *const (), hook_ty: HookType) -> *const ();
    fn install_hook_in_plt(
        host_module_object: *mut rtld::ModuleObject,
        function: *const (), 
        replace: *const (),
        out_trampoline: *mut *mut (),
        hook_ty: HookType
    );
    fn install_future_hook_in_plt(
        host_module_object: *mut rtld::ModuleObject,
        name: *const u8,
        replace: *const (),
        out_trampoline: *mut *mut (),
        hook_ty: HookType
    );
    fn set_hook_enable(user: *const (), enable: bool);

    #[link_name = "_ZN3exl4util4impl10mem_layout13s_ModuleInfosE"]
    static MODULE_INFOS: [ModuleMemory; 13];

    #[link_name = "_ZN3exl4util10mem_layout13s_ModuleCountE"]
    static mut MODULE_COUNT: i32;

    #[link_name = "_ZN3exl4util10mem_layout15s_SelfModuleIdxE"]
    static mut SELF_MODULE_INDEX: i32;
}

// HOOKING FFI

#[no_mangle]
pub extern "C" fn skex_hooks_install_on_symbol(
    host_object: *mut rtld::ModuleObject,
    function: *const (),
    replace: *const (),
    out_trampoline: *mut *mut (),
    hook_ty: HookType
) {
    unsafe {
        install_hook_in_plt(host_object, function, replace, out_trampoline, hook_ty);
    }
}

#[no_mangle]
pub extern "C" fn skex_hooks_install(
    symbol: *const (),
    replace: *const (), 
    hook_ty: HookType
) -> *const () {
    unsafe {
        install_hook(symbol, replace, hook_ty)
    }
}

#[no_mangle]
pub extern "C" fn skex_hooks_install_on_symbol_future(
    host_object: *mut rtld::ModuleObject,
    name: *const u8,
    replace: *const (),
    out_trampoline: *mut *mut (),
    hook_ty: HookType
) {
    unsafe {
        install_future_hook_in_plt(host_object, name, replace, out_trampoline, hook_ty)
    }
}

extern "C" {
    #[link_name = "_ZN2nn2ro6detail8RoModule10InitializeEmmPKNS1_3Elf5Elf643DynEb"]
    fn ro_module_init(module: *mut skyline::rtld::ModuleObject, arg: usize, arg2: usize, dyn_section: *mut u64, arg3: bool);
}

static mut HOOK_LIST: std::sync::RwLock<Vec<(usize, usize, usize, String, HookType)>> = std::sync::RwLock::new(Vec::new());

#[skyline::hook(replace = ro_module_init)]
unsafe fn ro_module_init_hook(module: *mut skyline::rtld::ModuleObject, arg: usize, arg2: usize, dyn_section: *mut u64, arg3: bool) {
    original(module, arg, arg2, dyn_section, arg3);

    let list = HOOK_LIST.read().unwrap();
    for (symbol_offset, replace, out_trampoline, name, ty) in list.iter() {
        if (*module).get_module_name().unwrap_or("__invalid_name") != name.as_str() {
            continue;
        }

        *(*out_trampoline as *mut *const ()) = skex_hooks_install((*module).module_base.add(*symbol_offset) as *const u8 as _, *replace as *const (), *ty);
    }
}

#[no_mangle]
pub extern "C" fn skex_hooks_install_on_dynamic_load(symbol_offset: usize, replace: *const (), out_trampoline: *mut *mut (), name: *const u8, hook_ty: HookType) {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| ro_module_init_hook::install());

    unsafe {
        let mut current = name;
        while *current != 0 { current = current.add(1) };
        let str_len = current.offset_from(name) as usize;
        let str_slice = std::slice::from_raw_parts(name, str_len);
        let name = std::str::from_utf8_unchecked(str_slice);
        HOOK_LIST.write().unwrap().push((symbol_offset, replace as usize, out_trampoline as usize, name.to_string(), hook_ty));
    }
}

#[no_mangle]
pub extern "C" fn skex_hooks_set_enable(user: *const (), enable: bool) {
    unsafe {
        set_hook_enable(user, enable)
    }
}

#[no_mangle]
pub extern "C" fn skex_hooks_uninstall(user: *const ()) {

}

#[no_mangle]
pub extern "C" fn skex_hooks_uninstall_from_symbol(user: *const ()) {

}

// STATIC MEMORY FFI

#[no_mangle]
pub extern "C" fn skex_memory_get_known_static_module(module: StaticModule) -> &'static ModuleMemory {
    unsafe {
        if MODULE_COUNT == -1 { panic!("Module information is not initialized!") };

        match module {
            StaticModule::Rtld => &MODULE_INFOS[0],
            StaticModule::Main => &MODULE_INFOS[1],
            StaticModule::SkylineEx => &MODULE_INFOS[SELF_MODULE_INDEX as usize],
            StaticModule::Sdk => &MODULE_INFOS[MODULE_COUNT as usize - 1]
        }
    }
}

#[no_mangle]
pub extern "C" fn skex_memory_get_static_module_by_name(name: *const u8) -> Option<&'static ModuleMemory> {
    if name.is_null() { return None; }
    if unsafe { MODULE_COUNT } == -1 { panic!("Module information is not initialized!") };

    let mut len = 0usize;
    let mut current = name;
    unsafe {
        while *current != 0 {
            len += 1;
            current = current.add(1);
        }
    }

    let name = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(name, len))
    };

    let mut infos = unsafe { MODULE_INFOS[0..(MODULE_COUNT as usize)].iter() };

    infos.find(|info| Some(name) == info.module_object().get_module_name())
}