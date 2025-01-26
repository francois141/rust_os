use crate::process::{self, Process};

pub struct Scheduler {
    proc1: process::Process,
    proc2: process::Process,
    counter: u32,
}

pub static mut SCHEDULER: Scheduler = Scheduler {
    proc1: Process::null_proc(),
    proc2: Process::null_proc(),
    counter: 0,
};

pub fn init() {
    unsafe { SCHEDULER = Scheduler::new_scheduler() }
}

pub fn init_sanity_check() {}

impl Scheduler {
    pub fn new_scheduler() -> Self {
        let proc1 = process::Process::new_process(process::process1 as usize);
        let proc2 = process::Process::new_process(process::process2 as usize);

        let current_scheduler = Scheduler {
            proc1: proc1,
            proc2: proc2,
            counter: 0,
        };

        current_scheduler
    }

    pub fn next(&mut self) -> usize {
        self.counter += 1;
        if self.counter % 2 == 0 {
            self.proc1.start_pc
        } else {
            self.proc2.start_pc
        }
    }
}
