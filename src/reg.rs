use core::arch::asm;


pub fn mepc_read() -> usize {
	unsafe {
		let rval;
		asm!("csrr {}, mepc", out(reg) rval);
		rval
	}
}

pub fn mtval_read() -> usize {
	unsafe {
		let rval;
		asm!("csrr {}, mtval", out(reg) rval);
		rval
	}
}


pub fn mcause_read() -> usize {
	unsafe {
		let rval;
		asm!("csrr {}, mcause", out(reg) rval);
		rval
	}
}

pub fn mhartid_read() -> usize {
	unsafe {
		let rval;
		asm!("csrr {}, mhartid", out(reg) rval);
		rval
	}
}


pub fn mstatus_read() -> usize {
	unsafe {
		let rval;
		asm!("csrr {}, mstatus", out(reg) rval);
		rval
	}
}

