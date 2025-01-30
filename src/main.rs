#![no_std]
#![no_main]
#![feature(
    panic_info_message,
    allocator_api,
    alloc_error_handler,
    raw_ref_op,
    asm_const,
    const_refs_to_static
)]

use core::arch::asm;
use core::arch::global_asm;
use core::fmt::Write;

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
        println!("line {}, file {}: {}", p.line(), p.file(), info.message());
    } else {
        println!("no information available.");
    }
    loop {}
}

// Memory layout, defined in the linker script.
extern "C" {
    static _start: u8;
    static _text_start: u8;
    static _text_end: u8;
    static _rodata_start: u8;
    static _rodata_end: u8;
    static _data_start: u8;
    static _data_end: u8;
    static _bss_start: u8;
    static _bss_end: u8;
    static _stack_start: u8;
    static _stack_end: u8;

    static _heap_start: usize;
}

// This is the entry point of the operating system
global_asm!(
    r#"
.attribute arch, "rv64imac"
.align 4
.text
.global _start
_start:
	# Any hardware threads (hart) that are not bootstrapping
	# need to wait for an IPI
	csrr	t0, mhartid
	bnez	t0, _end

	# Clear BSS secion
	la 		a0, __bss_start
	la		a1, __bss_end
	bgeu	a0, a1, 2f
1:
	sd		zero, (a0)
	addi	a0, a0, 8
	bltu	a0, a1, 1b
2:
	# Setup pmp registers
	# Allow all
	li t0, 0xFFFFFFFF
	csrw pmpaddr0,t0
	li t0, 0xFF
	csrw pmpcfg0,t0

	# Disable paging
	csrw	satp, zero
	# Stack pointer at the very end of the stack space
	# TODO: Find previous stack pointer here
	li		sp, 0x80f00000
	# ld		sp, __stack_end
	# Setting `mstatus` register:
	# 0b11 << 11: Machine's previous protection mode is 11 (MPP=1). --> We enter in supervisor mode
	# 1 << 7    : Machine's previous interrupt-enable bit is 1 (MPIE=1).
	# 1 << 3    : Machine's interrupt-enable bit is 1 (MIE=1).
	li t0, (0b11 << 11) | (1 << 7) | (1 << 3)
	csrw	mstatus, t0

    # Jump to harware initialisation code
    jal init

	# Machine's exception program counter (MEPC) is set to `kinit`.
	la		t1, kmain
	csrw	mepc, t1

	# Setup trigger first timer interrupt
	li      t0, 5000000
	li      t1, 0x2004000
	sd      t0, 0(t1)

	# Setting Machine's interrupt-enable bits (`mie` register):
	# 1 << 3 : Machine's M-mode software interrupt-enable bit is 1 (MSIE=1).
	# 1 << 7 : Machine's timer interrupt-enable bit is 1 (MTIE=1).
	# 1 << 11: Machine's external interrupt-enable bit is 1 (MEIE=1).
	li		t3, (1 << 3) | (1 << 7) | (1 << 11)
	csrw	mie, t3

	# Machine's trap vector base address is set to `asm_trap_vector`.
	la		t2, asm_trap_vector
	csrw	mtvec, t2

	# Jump now to the kernel process
	csrr tp, mscratch

	# Load program counter
	ld a0, 264(tp)
	csrw	mepc, a0

	ld		ra, 0(tp)
	ld		sp, 8(tp)
	ld		gp, 16(tp)
	// ld	    tp, 24(tp)
	ld		t0, 32(tp)
	ld		t1, 40(tp)
	ld		t2, 48(tp)
	ld		s0, 56(tp)
	ld		s1, 64(tp)
	ld		a0, 72(tp)
	ld		a1, 80(tp)
	ld		a2, 88(tp)
	ld		a3, 96(tp)
	ld		a4, 104(tp)
	ld		a5, 112(tp)
	ld		a6, 120(tp)
	ld		a7, 128(tp)
	ld		s3, 144(tp)
	ld		s4, 152(tp)
	ld		s5, 160(tp)
	ld		s6, 168(tp)
	ld		s7, 176(tp)
	ld		s8, 184(tp)
	ld		s9, 192(tp)
	ld		s10, 200(tp)
	ld		s11, 208(tp)
	ld		t3, 216(tp)
	ld		t4, 224(tp)
	ld		t5, 232(tp)
	ld		t6, 240(tp)

	# mret allows us to jump to supervisor mode
	mret
_end:
	wfi
	j		_end
.align 8
__bss_start:
    .dword {_bss_start}
__bss_end:
    .dword {_bss_end}
__stack_end:
    .dword {_stack_end}
"#,
    _bss_start = sym _bss_start,
    _bss_end = sym _bss_end,
    _stack_end = sym _stack_end);

global_asm!(
    r#".global asm_trap_vector
# This must be aligned by 4 since the last two bits
# of the mtvec register do not contribute to the address
# of this vector.
.align 4
asm_trap_vector:

    # Switch t6 with mscratch
    csrrw	t6, mscratch, t6

	# save the registers.
	sd		ra, 0(t6)
	sd		sp, 8(t6)
	sd		gp, 16(t6)
	sd		tp, 24(t6)
	sd		t0, 32(t6)
	sd		t1, 40(t6)
	sd		t2, 48(t6)
	sd		s0, 56(t6)
	sd		s1, 64(t6)
	sd		a0, 72(t6)
	sd		a1, 80(t6)
	sd		a2, 88(t6)
	sd		a3, 96(t6)
	sd		a4, 104(t6)
	sd		a5, 112(t6)
	sd		a6, 120(t6)
	sd		a7, 128(t6)
	sd		s2, 136(t6)
	sd		s3, 144(t6)
	sd		s4, 152(t6)
	sd		s5, 160(t6)
	sd		s6, 168(t6)
	sd		s7, 176(t6)
	sd		s8, 184(t6)
	sd		s9, 192(t6)
	sd		s10, 200(t6)
	sd		s11, 208(t6)
	sd		t3, 216(t6)
	sd		t4, 224(t6)
	sd		t5, 232(t6)
	sd		t6, 240(t6)

    # Store the current pc in the process structure
    csrr tp, mepc
    sd tp, 264(t6)

    # Finally, we can store t6 in the process structure
	mv		t5, t6
	csrr	t6, mscratch
	csrw mscratch, t5

    # Jump now to the trap handler
	call	m_trap

	# Jump now to the kernel process
	csrr tp, mscratch

	# Load program counter
	ld a0, 264(tp)
	csrw	mepc, a0

	ld		ra, 0(tp)
	ld		sp, 8(tp)
	ld		gp, 16(tp)
	// sd		tp, 24(tp)
	ld		t0, 32(tp)
	ld		t1, 40(tp)
	ld		t2, 48(tp)
	ld		s0, 56(tp)
	ld		s1, 64(tp)
	ld		a0, 72(tp)
	ld		a1, 80(tp)
	ld		a2, 88(tp)
	ld		a3, 96(tp)
	ld		a4, 104(tp)
	ld		a5, 112(tp)
	ld		a6, 120(tp)
	ld		a7, 128(tp)
	ld		s3, 144(tp)
	ld		s4, 152(tp)
	ld		s5, 160(tp)
	ld		s6, 168(tp)
	ld		s7, 176(tp)
	ld		s8, 184(tp)
	ld		s9, 192(tp)
	ld		s10, 200(tp)
	ld		s11, 208(tp)
	ld		t3, 216(tp)
	ld		t4, 224(tp)
	ld		t5, 232(tp)
	ld		t6, 240(tp)

	mret"#
);

#[no_mangle]
extern "C" fn init() {
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

    // Init virtio
    virtio::init();
    virtio::init_sanity_check();
    println!("Virtio : \x1b[32m[DONE]\x1b[0m");

    // Init block device
    block::init();
    block::init_sanity_check();
    println!("Block device : \x1b[32m[DONE]\x1b[0m");

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
extern "C" fn kmain() {
    let lock = lock::SpinLock::new();
    lock.lock();
    // Print on screen
    println!("\x1b[1m\x1b[32mWelcome on my rust risc-v operating system !!!\x1b[0m");
    lock.unlock();

    let mut i: usize = 0;
    loop {
        println!("Init process {}", i);
        for _ in 0..500000 {}

        i += 1;
    }
}

mod block;
pub mod kmalloc;
pub mod lock;
pub mod page_allocator;
pub mod paging;
pub mod plic;
pub mod process;
pub mod reg;
pub mod scheduler;
pub mod trap;
pub mod uart;
pub mod virtio;
