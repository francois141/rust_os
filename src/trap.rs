
use core::fmt::Write;
use crate::reg;

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
			11 => {
				// Environment (system) call from Machine mode
				println!("E-call from Machine mode! from core : {} -> 0x{:08x}", hart, return_pc);
				// Go to next instruction
				return_pc += 4
			},
            _ => {
                println!("Unhandled interrupt!");
            }
		}
	};

	return_pc
}
