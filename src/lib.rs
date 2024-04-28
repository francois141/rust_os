#![no_std]
#![feature(panic_info_message, allocator_api, alloc_error_handler)]

use core::ptr::addr_of;
use core::fmt::Write;
use core::arch::asm;

#[macro_export]
macro_rules! print {
	($($args:tt)+) => ({
		let _ = write!(crate::uart::Uart::get(), $($args)+);
	});
}

#[macro_export]
macro_rules! println {
	// Empty token
	() => ({
		print!("\r\n")
	});

	// Expression without arguments
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});

	// Expression with arguments
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	print!("Aborting: ");
	if let Some(p) = info.location() {
		println!(
		         "line {}, file {}: {}",
		         p.line(),
		         p.file(),
		         info.message().unwrap()
		);
	}
	else {
		println!("no information available.");
	}
	loop {}
}

extern "C" {
	static BSS_START: usize;
	static BSS_END: usize;
	static HEAP_START: usize;
	static HEAP_SIZE: usize;
	static HEAP_END: usize;
}

#[no_mangle]
extern "C"
fn init() {

	println!("Done with init");
	kmain()
}

#[no_mangle]
extern "C"
fn kmain() {
	// A few security assertions
	unsafe {
		// Safety assertion
		assert!(HEAP_START + HEAP_SIZE == HEAP_END);

		println!("BSS Section : 0x{:x} -> 0x{:x}", BSS_START, BSS_END);
		println!("HEAP Section : 0x{:x} -> 0x{:x}", HEAP_START, HEAP_END);
	}

	// Print on screen
	println!("Welcome on my rust risc-v operating system !!!");

	page_allocator::alloc(1);

	unsafe {
		//asm!("ecall");
	}

	println!("Second time to test trap");

	unsafe {
		//asm!("ecall");
	}

	println!("Interrupt works!");

	loop {
		
	}
}

pub mod page_allocator;
pub mod uart;
pub mod paging;
pub mod kmalloc;
pub mod trap;
pub mod reg; 
pub mod plic;