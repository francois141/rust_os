use crate::plic;
use crate::reg;
use crate::scheduler::SCHEDULER;
use crate::uart;
use crate::{paging, print, println};
use core::fmt::Write;

const MASK_INTERRUPT_BIT: usize = 1 << (usize::BITS as usize - 1);

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(usize)]
pub enum MCause {
    // Exceptions
    InstrAddrMisaligned = 0,
    InstrAccessFault = 1,
    IllegalInstr = 2,
    Breakpoint = 3,
    LoadAddrMisaligned = 4,
    LoadAccessFault = 5,
    StoreAddrMisaligned = 6,
    StoreAccessFault = 7,
    EcallFromUMode = 8,
    EcallFromSMode = 9,
    EcallFromMMode = 11,
    InstrPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
    UnknownException = 16,

    // Interrupts
    UserSoftInt = MASK_INTERRUPT_BIT,
    SupervisorSoftInt = MASK_INTERRUPT_BIT + 1,
    MachineSoftInt = MASK_INTERRUPT_BIT + 3,
    UserTimerInt = MASK_INTERRUPT_BIT + 4,
    SupervisorTimerInt = MASK_INTERRUPT_BIT + 5,
    MachineTimerInt = MASK_INTERRUPT_BIT + 7,
    UserExternalInt = MASK_INTERRUPT_BIT + 8,
    SupervisorExternalInt = MASK_INTERRUPT_BIT + 9,
    MachineExternalInt = MASK_INTERRUPT_BIT + 11,
    UnknownInt,
}

impl MCause {
    pub fn new(cause: usize) -> Self {
        if (cause as isize) < 0 {
            // Interrupt
            // set last bit to 0
            match cause ^ MASK_INTERRUPT_BIT {
                0 => MCause::UserSoftInt,
                1 => MCause::SupervisorSoftInt,
                3 => MCause::MachineSoftInt,
                4 => MCause::UserTimerInt,
                5 => MCause::SupervisorTimerInt,
                7 => MCause::MachineTimerInt,
                8 => MCause::UserExternalInt,
                9 => MCause::SupervisorExternalInt,
                11 => MCause::MachineExternalInt,
                _ => MCause::UnknownInt,
            }
        } else {
            // Trap
            match cause {
                0 => MCause::InstrAddrMisaligned,
                1 => MCause::InstrAccessFault,
                2 => MCause::IllegalInstr,
                3 => MCause::Breakpoint,
                4 => MCause::LoadAddrMisaligned,
                5 => MCause::LoadAccessFault,
                6 => MCause::StoreAddrMisaligned,
                7 => MCause::StoreAccessFault,
                8 => MCause::EcallFromUMode,
                9 => MCause::EcallFromSMode,
                11 => MCause::EcallFromMMode,
                12 => MCause::InstrPageFault,
                13 => MCause::LoadPageFault,
                15 => MCause::StorePageFault,
                _ => MCause::UnknownException,
            }
        }
    }

    pub fn is_interrupt(self) -> bool {
        self as usize & MASK_INTERRUPT_BIT != 0
    }

    pub fn cause_number(cause: usize) -> usize {
        if (cause as isize) < 0 {
            cause ^ MASK_INTERRUPT_BIT
        } else {
            cause
        }
    }
}

#[no_mangle]
extern "C" fn m_trap() -> usize {
    let mut return_pc = reg::mepc_read();
    let tval = reg::mtval_read();
    let cause = reg::mcause_read();
    let hart = reg::mhartid_read();

    match MCause::new(cause) {
        MCause::EcallFromUMode => {
            println!(
                "E-call from Supervisor mode from core : {} -> 0x{:08x}",
                hart, return_pc
            );
            // Go to next instruction
            return_pc += 4
        }
        MCause::EcallFromSMode => {
            println!(
                "E-call from Supervisor mode from core : {} -> 0x{:08x}",
                hart, return_pc
            );
            // Go to next instruction
            return_pc += 4
        }
        MCause::EcallFromMMode => {
            println!(
                "E-call from Machine mode from core : {} -> 0x{:08x}",
                hart, return_pc
            );
            // Go to next instruction
            return_pc += 4
        }
        MCause::InstrPageFault => {
            // Instruction page fault
            println!(
                "Instruction page fault from core : {} -> 0x{:08x}",
                hart, tval
            );
            paging::map(tval, tval, paging::EntryBits::ReadWriteExecute.val());
        }
        MCause::LoadPageFault => {
            // Load page fault
            println!("Load page fault from core : {} -> 0x{:08x}", hart, tval);
            paging::map(tval, tval, paging::EntryBits::ReadWriteExecute.val());
        }
        MCause::StorePageFault => {
            // Store page fault
            println!("Store page fault from core : {} -> 0x{:08x}", hart, tval);
            paging::map(tval, tval, paging::EntryBits::ReadWriteExecute.val());
        }
        MCause::MachineTimerInt => {
            unsafe {
                // TODO: Make sure it is optimal : https://five-embeddev.com/riscv-priv-isa-manual/Priv-v1.12/machine.html#machine-timer-registers-mtime-and-mtimecmp
                let mtimecmp = 0x0200_4000 as *mut u64;
                let time_second = 10_000_000;
                mtimecmp.write_volatile(mtimecmp.read_volatile() + 1 * time_second);
            }

            println!("\x1b[0;33mReceived a timer interrupt\x1b[0m");

            unsafe {
                // Get the next pc from scheduler
                return_pc = SCHEDULER.next();
            }
        }
        MCause::MachineExternalInt => {
            if let Some(interrupt_code) = plic::next_interrupt() {
                match interrupt_code {
                    10 => {
                        print!("\x1b[1m\x1b[3m\x1b[36m");
                        print_uart_value();
                        print!("\x1b[0m");
                    }
                    _ => {
                        println!("Ignored plic interrupt");
                    }
                }
                // Clear interrupt
                plic::clear_interrupt(interrupt_code);
            }
        }
        _ => {
            panic!("Unhandled trap / interrupt with code : {}", cause)
        }
    }

    return_pc
}

fn print_uart_value() {
    let mut uart_module = uart::Uart::get();
    if let Some(c) = uart_module.read() {
        match c {
            10 | 13 => {
                print!("\n");
            }
            _ => {
                print!("{}", c as char);
            }
        }
    }
}
