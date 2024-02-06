use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;


pub struct DummyAllocator;


unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should never be called")
    }
}


#[global_allocator]
static ALLOCATOR: DummyAllocator = DummyAllocator;
