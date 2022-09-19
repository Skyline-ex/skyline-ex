// #![no_std]
// #![feature(default_alloc_error_handler)]

mod alloc;
mod bootstrap;

#[global_allocator]
static ALLOCATOR: alloc::Allocator = alloc::Allocator;

std::arch::global_asm!(
    r#"
    .section .nro_header
    .global __nro_header_start
    .global __module_start
    __module_start:
    .word 0
    .word _mod_header
    .word 0
    .word 0
    
    .section .rodata.mod0
    .global _mod_header
    _mod_header:
        .ascii "MOD0"
        .word __dynamic_start - _mod_header
        .word __bss_start - _mod_header
        .word __bss_end - _mod_header
        .word __eh_frame_hdr_start - _mod_header
        .word __eh_frame_hdr_end - _mod_header
        .word __nx_module_runtime - _mod_header // runtime-generated module object offset
    .global IS_NRO
    IS_NRO:
        .word 1
    
    .section .bss.module_runtime
    .space 0xD0
    "#
);

pub unsafe fn main() {
    let _ = nn::fs::mount_sd_card("sd");
    let _ = nn::fs::create_file("sd:/rust-test.txt", 0);
    let file = nn::fs::open_file("sd:/rust-test.txt", nn::fs::OpenMode::WRITE | nn::fs::OpenMode::ALLOW_APPEND).unwrap();
    let _ = nn::fs::write_file(file, 0, &nn::fs::fs_impl::OpenFile as *const _ as *const () as _, "Hello world!".len(), nn::fs::WriteOptions::FLUSH);
    nn::fs::close_file(file);
}