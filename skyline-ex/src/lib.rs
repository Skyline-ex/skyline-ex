#![no_std]
#![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;

mod alloc;
mod bootstrap;

#[global_allocator]
static ALLOCATOR: alloc::Allocator = alloc::Allocator;

pub unsafe fn main() {
    let _ = nn::fs::mount_sd_card("sd");
    let _ = nn::fs::create_file("sd:/rust-test.txt", 0);
    let file = nn::fs::open_file("sd:/rust-test.txt", nn::fs::OpenMode::WRITE | nn::fs::OpenMode::ALLOW_APPEND).unwrap();
    let _ = nn::fs::write_file(file, 0, "Hello world!".as_ptr() as _, "Hello world!".len(), nn::fs::WriteOptions::FLUSH);
    nn::fs::close_file(file);
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}