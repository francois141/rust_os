#![no_std]
#![feature(panic_info_message, allocator_api, alloc_error_handler)]

use core::ptr::addr_of;
use core::fmt::Write;
use core::arch::asm;

use page_allocator::alloc;
use process::process1;


extern "C" {
    fn switch_to_other_process(v1: usize) -> !;
}

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

fn init() {
	// Setup driver
	uart::Uart::start_driver(0x1000_0000);

	// Init page allocator
	page_allocator::init_allocator();

	// Jump to init process
	unsafe {
		switch_to_other_process(process::process1 as usize);
	}
}

#[no_mangle]
extern "C"
fn kmain() {
	// Init os
	init();
}

pub mod page_allocator;
pub mod uart;
pub mod trap;
pub mod reg; 
pub mod plic;
pub mod process;