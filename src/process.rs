use core::ptr::{null, null_mut};

use crate::{page_allocator::{self, PAGE_SIZE}, paging};
use core::fmt::Write;

#[repr(C)]
pub struct Process {
    pub frame: *mut ProcessFrame,
    stack: *mut u8,
    pub pc: usize,
}


#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcessFrame {
  pub registers:       [usize; 31], // 0 - 247
  pub pc: usize, // 248 - 255
  pub floating_points_registers:      [usize; 32], // Use later

}


impl Process {

  pub fn new_process(start_pc: usize) -> Self {

    let nb_pages_to_allocate = 10;

    println!("{}", start_pc);

    let mut processFrame = ProcessFrame {
      registers: [0;31],
      pc: start_pc,
      floating_points_registers: [0;32],
    };

    // TODO: Make the number of pages parametrizable
    let process = Process {
      stack: page_allocator::alloc(nb_pages_to_allocate),
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
    for x in 0..1000000 {}
    counter = (counter + 1) % 100000;
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