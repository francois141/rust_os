use core::mem::size_of;
use core::ptr::null_mut;

use crate::_heap_start;

// TODO: Fix it dynamically
const HEAP_SIZE: usize = 0x1000000;

const EMPTY_PAGE: u8 = 0x0;
const TAKEN_FLAG: u8 = 0x1;
const LAST_FLAG: u8 = 0x2;

pub(crate) const PAGE_SIZE: usize = 4096;

pub struct Page {
    flags: u8,
}

impl Page {
    pub fn free(&self) -> bool {
        self.flags == EMPTY_PAGE
    }

    pub fn taken(&self) -> bool {
        self.flags & TAKEN_FLAG != 0
    }

    pub fn last(&self) -> bool {
        self.flags & LAST_FLAG != 0
    }

    pub fn set_flag(&mut self, flags: u8) {
        self.flags = self.flags | flags
    }

    pub fn clear_flag(&mut self, flags: u8) {
        self.flags = self.flags & !flags
    }

    pub fn clear_all_flags(&mut self) {
        self.flags = 0x0;
    }
}

static mut ALLOC_START: usize = 0;
pub static mut ALLOCATED_PAGE_HEAP_ALLOCATOR: usize = 0;

pub const fn page_align_round_up(val: usize) -> usize {
    let o = 4096 - 1;
    (val + o) & !o
}

pub fn init_allocator() {
    unsafe {
        ALLOCATED_PAGE_HEAP_ALLOCATOR = HEAP_SIZE / PAGE_SIZE;

        let pointer = (&raw const _heap_start as usize) as *mut Page;

        // Reserve some place for the page allocator
        ALLOC_START = page_align_round_up(
            (&raw const _heap_start as usize) + ALLOCATED_PAGE_HEAP_ALLOCATOR * size_of::<Page>(),
        );

        // Clear pages for security reason
        for i in 0..ALLOCATED_PAGE_HEAP_ALLOCATOR {
            (*pointer.add(i)).clear_all_flags();
        }
    }
}

pub fn alloc(pages: usize) -> *mut u8 {
    // Safety assertion
    assert!(pages > 0);

    unsafe {
        let pointer = (&raw const _heap_start as usize) as *mut Page;
        let number_pages: usize = HEAP_SIZE / PAGE_SIZE;

        for i in 0..number_pages {
            if (*pointer.add(i)).free() {
                let mut good = true;

                for j in i..i + pages {
                    if (*pointer.add(j)).taken() {
                        good = false;
                        break;
                    }
                }

                if good {
                    // Set all pages allocated
                    for j in i..i + pages {
                        (*pointer.add(j)).set_flag(TAKEN_FLAG);
                    }

                    (*pointer.add(i + pages - 1)).set_flag(LAST_FLAG);

                    let raw_pointer = (ALLOC_START + PAGE_SIZE * i) as *mut u64;

                    // Clear pages for security reasons
                    for offset in 0..PAGE_SIZE * pages / 8 {
                        (*raw_pointer.add(offset)) = 0;
                    }

                    return raw_pointer as *mut u8;
                }
            }
        }
    }

    // Failure
    null_mut()
}

pub fn dealloc(pointer: *mut u8) {
    // Safety assertion
    assert!(!pointer.is_null());

    unsafe {
        // Convert pointer to page address
        let page_structure_address =
            (&raw const _heap_start as usize) + (pointer as usize - ALLOC_START) / PAGE_SIZE;

        // Safety assertion
        assert!(
            (&raw const _heap_start as usize) <= page_structure_address
                && page_structure_address < (&raw const _heap_start as usize) + HEAP_SIZE
        );

        let mut page_pointer = page_structure_address as *mut Page;

        while (*page_pointer).taken() && !(*page_pointer).last() {
            // Clear page pointer
            (*page_pointer).clear_all_flags();
            // Move to the next one
            page_pointer = page_pointer.add(1);
        }

        // Check for double free
        assert!((*page_pointer).last() == true, "Possible double free here");

        // Clear the last page
        (*page_pointer).clear_all_flags();
    }
}

pub fn init_sanity_check() {
    // Check we allocate in the correct zone
    let first_alloc = alloc(1);
    assert!(first_alloc > 0x80000000 as *mut u8);

    // Check if we deallocate correctly
    dealloc(first_alloc);

    // Assert first allocation is equal second allocation
    let second_alloc = alloc(1);
    assert!(first_alloc == second_alloc);

    // Test bigger allocation
    let third_alloc = alloc(0x100);

    // Make sure we allocate first page again
    dealloc(second_alloc);
    let last_alloc = alloc(1);
    assert!(first_alloc == last_alloc);

    // Free all the memory for the operating systems
    dealloc(third_alloc);
}
