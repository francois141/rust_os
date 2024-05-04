
use core::fmt::Write;
use crate::paging;
use crate::reg;
use crate::plic;

use crate::uart;

#[no_mangle]
extern "C" fn m_trap() -> usize
{
	let mut return_pc = reg::mepc_read();
	let tval = reg::mtval_read();
	let cause = reg::mcause_read();
	let hart = reg::mhartid_read();
	let status = reg::mstatus_read();

	let is_async: bool = cause >> 63 & 1 == 1;

	let cause_num = cause & 0xfff;

	if !is_async {
		match cause_num {
			9 => {
				// Environment (system) call from Supervisor mode
				println!("E-call from Supervisor mode from core : {} -> 0x{:08x}", hart, return_pc);
				// Go to next instruction
				return_pc += 4
			},
			11 => {
				// Environment (system) call from Machine mode
				println!("E-call from Machine mode from core : {} -> 0x{:08x}", hart, return_pc);
				// Go to next instruction
				return_pc += 4
			},
			// Page faults
			12 => {
				// Instruction page fault
				println!("Instruction page fault from core : {} -> 0x{:08x}", hart, tval);
				return_pc += 4;
			},
			13 => {
				// Load page fault
				println!("Load page fault from core : {} -> 0x{:08x}", hart, tval);
				return_pc += 4;
			},
			15 => {
				// Store page fault
				println!("Store page fault from core : {} -> 0x{:08x}", hart, tval);
				return_pc += 4;
			},
            _ => {
                println!("Unhandled interrupt! {}", cause_num);
            }
		}
	}

	if is_async {
		match cause_num {
			11 => {
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
				println!("Unhandled async interrupt");
			}
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
			},
			_ => {
				print!("{}", c as char);
			}
		}
	}
}