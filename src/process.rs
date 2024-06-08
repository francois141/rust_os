use core::{fmt::write, ptr::{null, null_mut}};

use crate::{page_allocator::{self, PAGE_SIZE}};
use core::fmt::Write;


#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcessFrame {
  pub registers:       [usize; 31], // 0 - 247
  pub pc: usize, // 248 - 255
  pub floating_points_registers:      [usize; 32], // Use later
}


pub fn process1() {
  let mut a = 0;
  let mut counter:u64 = 0;
  loop {
    counter += 1;
    if counter % 100000000 == 0 {
      println!("tick {}", a);
      a += 1;
      counter = 0
    }
  }
}