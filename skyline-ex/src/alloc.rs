use core::alloc::{GlobalAlloc, Layout};

// these symbols should be exported by nnsdk and smash
extern "C" {
    fn aligned_alloc(alignment: usize, size: usize) -> *mut u8;
    fn free(pointer: *mut u8);
}

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();
        aligned_alloc(align, size)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr)
    }
}