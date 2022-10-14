#![feature(let_else)]
#![feature(if_let_guard)]

use skyline::memory::StaticModule;

mod bootstrap;
mod exception;
mod ffi;
mod legacy;
mod logger;

extern "C" {
    #[link_name = "_ZN2nn2ro10InitializeEv"]
    fn nn_ro_initialize();

    #[link_name = "_ZN2nn2fs8MountRomEPKcPvm"]
    fn nn_fs_mount_rom(name: *const u8, cache: *mut u8, cache_size: usize) -> u32;
}

#[skyline::shim(replace = nn_ro_initialize)]
fn ro_init_shim() {
    original()
}

#[skyline::shim(replace = nn_fs_mount_rom)]
fn mount_rom_shim(name: *const u8, cache: *mut u8, cache_size: usize) -> u32 {
    original(name, cache, cache_size)
}

#[skyline::main(name = "skyline-ex")]
pub fn main() {
    exception::initialize();
    let _ = nn::fs::mount_sd_card("sd");
    logger::initialize();
    ro_init_shim::install();
    mount_rom_shim::install();

    ro_init_shim();

    let size = match nn::fs::query_rom_cache_size() {
        Ok(size) => size,
        Err(e) => {
            println!("Cannot query rom cache size: {:#x?}", e);
            return;
        },
    };

    let rom_memory = unsafe {
        let layout = std::alloc::Layout::from_size_align(size, 0x1000).unwrap();
        std::alloc::alloc(layout)
    };

    let _ = mount_rom_shim(b"rom\0".as_ptr(), rom_memory, size);

    let loader_results = loader::mount_from_directory(
        skyline::nx::get_program_id(),
        "rom:/skyline/plugins",
        |path| {
            path.extension().is_some() && path.extension().unwrap() == "nro"
        }
    );

    std::thread::sleep(std::time::Duration::from_millis(1000));

    match loader_results {
        Ok(info) => unsafe {
            for module in info.modules.iter() {
                match module.as_ref() {
                    Ok(plugin) => {
                        let mut symbol = 0usize;
                        nnsdk::ro::LookupModuleSymbol(&mut symbol, plugin, b"main\0".as_ptr());
                        let nul = plugin.Name
                            .iter()
                            .enumerate()
                            .find(|(_, byte)| **byte == 0)
                            .map(|(count, _)| count)
                            .unwrap();

                        let name = std::str::from_utf8_unchecked(&plugin.Name[0..nul]);
                        if symbol == 0 {
                            println!("[skyline-ex::loader] Plugin {} does not have main function.", name);
                        } else {
                            let func: extern "C" fn() = std::mem::transmute(symbol);
                            println!("[skyline-ex::loader] Calling function 'main' in plugin {}", name);
                            func();
                            println!("[skyline-ex::loader] Function 'main' has finished in plugin {}", name);
                        }
                    },
                    Err(e) => println!("[skyline-ex::loader] Error mounting module: {}", e)
                }
            }
        },
        Err(e) => println!("Loader failed: {}", e)
    }
}