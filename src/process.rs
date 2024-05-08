use core::ptr::null_mut;

use crate::{page_allocator, paging};


#[repr(C)]
pub struct Process {
    frame: ProcessFrame,
    stack: *mut u8,
    pc: usize,
}


#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcessFrame {
  pub registers:       [usize; 32],
  pub floating_points_registers:      [usize; 32],  
  pub stack_pointer: *mut u8,      
}


impl Process {

  pub fn new_process() -> Self {
    // TODO: Make the number of pages parametrizable
    let process = Process {
      stack: page_allocator::alloc(10),
      pc: 0x80000000,
      frame: ProcessFrame {
        registers: [0;32],
        floating_points_registers: [0;32],
        stack_pointer: null_mut(),
      }, // For now we start at Origin, later we will parse an elf with entry point
    };

    // We don't need to map the stack at this point. We operate under lazy mapping
    // Finally we can return the process
    process
  }

}