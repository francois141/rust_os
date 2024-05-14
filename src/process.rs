use core::ptr::{null, null_mut};

use crate::{page_allocator::{self, PAGE_SIZE}, paging};
use core::fmt::Write;

#[repr(C)]
pub struct Process {
    pub frame: *mut ProcessFrame,
    stack: *mut u8,
    pub start_pc: usize,
    pc: usize,
}


#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcessFrame {
  pub registers:       [usize; 32], // 0 - 255
  pub floating_points_registers:      [usize; 32], // 256 - 511
  pub stack_pointer: *mut u8, // 512 - 520
}


impl Process {

  pub fn new_process(start_pc: usize) -> Self {

    let nb_pages_to_allocate = 10;

    let mut processFrame = ProcessFrame {
      registers: [0;32],
      floating_points_registers: [0;32],
      stack_pointer: null_mut(),
    };

    // TODO: Make the number of pages parametrizable
    let process = Process {
      stack: page_allocator::alloc(nb_pages_to_allocate),
      start_pc: start_pc,
      pc: start_pc,
      frame: &mut processFrame,
    };


		unsafe {
			(*process.frame).registers[1] = process.stack as usize + nb_pages_to_allocate*PAGE_SIZE; // TODO : Improve that
		}

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
      frame: null_mut(),
    };

    // We don't need to map the stack at this point. We operate under lazy mapping
    // Finally we can return the process
    process
  }


}

pub fn process1() {
  let mut counter = 0;
  loop {
    println!("We are in process 1! {}", counter);
    for x in 0..3000000 {}
    counter += 1;
  }
}

pub fn process2() {
  let mut counter = 0;
  loop {
    println!("We are in process 2! {}", counter);
    for x in 0..3000000 {}
    counter += 1;
  }
}