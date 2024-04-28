# boot.S
# bootloader for SoS
# Stephen Marz
# 8 February 2019

# Disable generation of compressed instructions.
.option norvc

# Define a .data section.
.section .data


# Read only data section, write all variables we need to initialize memory mapping
.section .rodata

.global TEXT_START
TEXT_START: .dword _text_start

.global TEXT_END
TEXT_END: .dword _text_end

.global KERNEL_STACK_START
KERNEL_STACK_START: .dword _stack_start

.global KERNEL_STACK_END
KERNEL_STACK_END: .dword _stack_end

.global DATA_START
DATA_START: .dword _data_start

.global DATA_END
DATA_END: .dword _data_end

.global RODATA_START
RODATA_START: .dword _rodata_start

.global RODATA_END
RODATA_END: .dword _rodata_end


.global BSS_START
BSS_START: .dword _bss_start

.global BSS_END
BSS_END: .dword _bss_end

.global HEAP_START
HEAP_START: .dword _heap_start

.global HEAP_SIZE
HEAP_SIZE: .dword _heap_size

.global HEAP_END
HEAP_END: .dword _heap_end

# Define a .text.init section.
.section .text.init


.global _start
_start:
	# Any hardware threads (hart) that are not bootstrapping
	# need to wait for an IPI
	csrr	t0, mhartid
	bnez	t0, 3f
	# SATP should be zero, but let's make sure
	csrw	satp, zero
	
	# Disable linker instruction relaxation for the `la` instruction below.
	# This disallows the assembler from assuming that `gp` is already initialized.
	# This causes the value stored in `gp` to be calculated from `pc`.
.option push
.option norelax
	la		gp, _global_pointer
.option pop
	# Set all bytes in the BSS section to zero.
	la 		a0, _bss_start
	la		a1, _bss_end
	bgeu	a0, a1, 2f
1:
	sd		zero, (a0)
	addi	a0, a0, 8
	bltu	a0, a1, 1b
2:	
# Control registers, set the stack, mstatus, mepc,
	# and mtvec to return to the main function.
	# li		t5, 0xffff;
	# csrw	medeleg, t5
	# csrw	mideleg, t5
	# The stack grows from bottom to top, so we put the stack pointer
	# to the very end of the stack range.
	la		sp, _stack_end
	# Setting `mstatus` register:
	# 0b11 << 11: Machine's previous protection mode is 3 (MPP=3).




	# Set mepc
	la t0, init
	csrw mepc, t0

	la t1, asm_trap_vector
	csrw mtvec, t1

 	li      t2, 0x1800
	csrw	mstatus, t2

	li      t3, 0x800
    csrs    mstatus, t3

    li      t4, 0x100
    csrs    medeleg, t4

	mret
2:
3:
4:
	wfi
	j		4b

