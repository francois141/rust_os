#![no_std]
#![feature(panic_info_message)]

use core::fmt::Write;

#[macro_export]
macro_rules! print {
	($($args:tt)+) => ({
		let _ = write!(crate::uart::Uart::create(0x1000_0000), $($args)+);
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
fn kmain() {
	// Main should initialize all sub-systems and get
	// ready to start scheduling. The last thing this
	// should do is start the timer.

	// Setup driver
	uart::Uart::start_driver(0x1000_0000);

	// Init alloc
	page_allocator::init_allocator();

	// Quick test 
	page_allocator::init_sanity_check();

	// A few security assertions
	unsafe {
		// Safety assertion
		assert!(HEAP_START + HEAP_SIZE == HEAP_END);

		println!("BSS Section : 0x{:x} -> 0x{:x}", BSS_START, BSS_END);
		println!("HEAP Section : 0x{:x} -> 0x{:x}", HEAP_START, HEAP_END);
	}

	// Print on screen
	println!("Welcome on my rust risc-v operating system !!!");

	println!("Result first  allocation : {:p}", page_allocator::alloc(1));
	println!("Result second allocation : {:p}", page_allocator::alloc(4));
	println!("Result third allocation : {:p}", page_allocator::alloc(4));
	
	// End of the kernel
	loop {
		
	}
}

pub mod page_allocator;
pub mod uart;


