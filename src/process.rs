use core::ptr::null_mut;

use crate::{page_allocator, println};
use core::fmt::Write;

#[repr(C)]
pub struct Process {
    frame: ProcessFrame,
    stack: *mut u8,
    pc: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcessFrame {
    pub registers: [usize; 32],
}

impl Process {
    pub fn new_process(start_pc: usize) -> Self {
        // TODO: Make the number of pages parametrizable
        let mut process = Process {
            stack: page_allocator::alloc(10),
            pc: start_pc,
            frame: ProcessFrame { registers: [0; 32] },
        };

        process.frame.registers[1] = process.stack as usize + 10 * 4096;

        // We don't need to map the stack at this point. We operate under lazy mapping
        // Finally we can return the process
        process
    }

    pub const fn null_proc() -> Self {
        // TODO: Make the number of pages parametrizable
        let process = Process {
            stack: null_mut(),
            pc: 0,
            frame: ProcessFrame { registers: [0; 32] },
        };

        // We don't need to map the stack at this point. We operate under lazy mapping
        // Finally we can return the process
        process
    }
}

pub fn process1() {
    let mut i: usize = 0;
    loop {
        println!("PROCESS 1 | Value {}", i);
        for _ in 0..500000 {}

        i += 1;
    }
}

pub fn process2() {
    let mut i: usize = 0;
    loop {
        println!("PROCESS 2 | Value {}", i);
        for _ in 0..500000 {}

        i += 1;
    }
}
