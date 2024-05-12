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


fn init() {
	// Setup driver
	uart::Uart::start_driver(0x1000_0000);
	println!("Uart driver : \x1b[32m[DONE]\x1b[0m");

	// Init page allocator
	page_allocator::init_allocator();
	page_allocator::init_sanity_check();
	println!("Page allocator : \x1b[32m[DONE]\x1b[0m");

	// Init memory allocator
	kmalloc::init();
	kmalloc::init_sanity_check();
	println!("Memory allocator : \x1b[32m[DONE]\x1b[0m");

	// Init plic
	plic::init();
	plic::init_sanity_check();
	println!("Plic : \x1b[32m[DONE]\x1b[0m");
	
	// Init paging
	paging::init();
	paging::init_sanity_check();
	println!("Paging : \x1b[32m[DONE]\x1b[0m");

	// Init scheduler
	scheduler::init();
	scheduler::init_sanity_check();
	println!("Scheduler : \x1b[32m[DONE]\x1b[0m");

	// Install page table
	unsafe {
		let root_address = (paging::ROOT) as usize;
		let satp_val = paging::craft_satp(8, 0, root_address);
		asm!("csrw satp, {}", in(reg)satp_val);
		asm!("sfence.vma");	
	}

	println!("Installing page table : \x1b[32m[DONE]\x1b[0m");
}

#[no_mangle]
extern "C"
fn kmain() {
	// Init os
	init();

	let lock = lock::SpinLock::new();
	lock.lock();
	// Print on screen
	println!("\x1b[1m\x1b[32mWelcome on my rust risc-v operating system !!!\x1b[0m");
	lock.unlock();
}

pub mod page_allocator;
pub mod uart;
pub mod paging;
pub mod kmalloc;
pub mod trap;
pub mod reg; 
pub mod plic;
pub mod lock;
pub mod process;
pub mod scheduler;