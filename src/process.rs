use core::ptr::{null, null_mut};

use crate::{page_allocator, paging};
use core::fmt::Write;

#[repr(C)]
pub struct Process {
    frame: ProcessFrame,
    stack: *mut u8,
    pub start_pc: usize,
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

  pub fn new_process(start_pc: usize) -> Self {
    // TODO: Make the number of pages parametrizable
    let process = Process {
      stack: page_allocator::alloc(10),
      start_pc: start_pc,
      pc: start_pc,
      frame: ProcessFrame {
        registers: [0;32],
        floating_points_registers: [0;32],
        stack_pointer: null_mut(),
      },
    };

    // We don't need to map the stack at this point. We operate under lazy mapping
    // Finally we can return the process
    process
  }

  pub const fn null_proc() -> Self {
    // TODO: Make the number of pages parametrizable
    let process = Process {
      stack: null_mut(),
      start_pc: 0,
      pc: 0,
      frame: ProcessFrame {
        registers: [0;32],
        floating_points_registers: [0;32],
        stack_pointer: null_mut(),
      },
    };

    // We don't need to map the stack at this point. We operate under lazy mapping
    // Finally we can return the process
    process
  }


}

pub fn process1() {
	println!("We are in process 1!");
	loop {}
}

pub fn process2() {
	println!("We are in process 2!");
	loop {}
}