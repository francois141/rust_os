use core::alloc::{GlobalAlloc, Layout};
use core::fmt::Write;
use core::ptr::null_mut;
use crate::page_allocator;
extern crate alloc;

use alloc::boxed::Box;


struct MyAllocator;

static ALLOC_SPACE: usize = 256;

static mut KMALLOC_HEAD: *mut u8 = core::ptr::null_mut();
static mut KMALLOC_END: *mut u8 = core::ptr::null_mut();

unsafe impl GlobalAlloc for MyAllocator {

    // TODO: Add alignment constraints
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if KMALLOC_HEAD.add(layout.size()) > KMALLOC_END {
            println!("No space left, leaving OS!");
            return null_mut()
        }
        
        let output = KMALLOC_HEAD;
        KMALLOC_HEAD = KMALLOC_HEAD.add(layout.size());
        output
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

pub fn init() {
    unsafe {
        KMALLOC_HEAD = page_allocator::alloc(ALLOC_SPACE);
        KMALLOC_END = KMALLOC_HEAD.add(page_allocator::PAGE_SIZE * ALLOC_SPACE);
    }
}

pub fn init_sanity_check() {
	let _test_allocation_heap: Box<u8> = Box::new(5);
}
