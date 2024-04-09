#![no_std]
#![feature(panic_info_message,asm)]

use core::fmt::Write;

pub mod uart;


#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({

		let _ = write!(crate::uart::Uart::create(0x1000_0000), $($args)+);
	});
}

#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
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
	loop {
		println!("endless loop");
	}
}


#[no_mangle]
extern "C"
fn kmain() {
	// Main should initialize all sub-systems and get
	// ready to start scheduling. The last thing this
	// should do is start the timer.

	// Setup driver
	uart::Uart::start_driver(0x1000_0000);

	// Print on screen
	println!("This is my operating system!");

	// End of the kernel
	loop {
		
	}
}


