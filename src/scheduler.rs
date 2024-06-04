use core::ptr::addr_of;

use crate::process::{self, Process};
use core::fmt::Write;

pub struct Scheduler {
    proc1: process::Process,
}

pub static mut SCHEDULER: Scheduler = Scheduler {
    proc1: Process::null_proc(),
};

pub fn init() {
    unsafe {
        SCHEDULER = Scheduler::new_scheduler()
    }
}

pub fn init_sanity_check() {

}

impl Scheduler {
    pub fn new_scheduler() -> Self {

        let proc1 = process::Process::new_process(process::process1 as usize);

        let current_scheduler = Scheduler{
            proc1: proc1,
        };

        current_scheduler
    }

    pub fn next(&mut self) -> (usize,usize) {
        unsafe {
            println!("{} {}", self.proc1.pc, (*self.proc1.frame).pc);
        }
        

        (self.proc1.pc, addr_of!(self.proc1.frame) as usize)
    }
}

