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
  let mut counter = 0;
  loop {
    counter += 1;
    if counter % 1000000 == 0 {
      println!("tick");
    }
  }
}