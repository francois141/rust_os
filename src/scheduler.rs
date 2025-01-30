use crate::kmain;
use crate::process::{self, Process};
use core::arch::asm;

pub struct Scheduler {
    pub init: process::Process,
    proc1: process::Process,
    proc2: process::Process,
    counter: u32,
}

pub static mut SCHEDULER: Scheduler = Scheduler {
    init: Process::null_proc(),
    proc1: Process::null_proc(),
    proc2: Process::null_proc(),
    counter: 0,
};

pub fn init() {
    unsafe {
        SCHEDULER = Scheduler::new_scheduler();
        Scheduler::propagate_decision(&raw const SCHEDULER.init as usize);
    }
}

pub fn init_sanity_check() {}

impl Scheduler {
    pub fn new_scheduler() -> Self {
        Scheduler {
            init: Process::new_process(kmain as usize),
            proc1: Process::new_process(process::process1 as usize),
            proc2: Process::new_process(process::process2 as usize),
            counter: 0,
        }
    }

    pub unsafe fn next(&mut self) {
        match self.counter % 3 {
            0 => Self::propagate_decision(&raw const self.proc1 as usize),
            1 => Self::propagate_decision(&raw const self.proc2 as usize),
            2 => Self::propagate_decision(&raw const self.init as usize),
            _ => unreachable!(),
        }

        self.counter += 1;
    }

    pub(crate) fn propagate_decision(value: usize) {
        unsafe {
            asm!(
            "csrw mscratch, {0}",
            in(reg) value
            );
        }
    }
}
