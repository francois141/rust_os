# Disable generation of compressed instructions.
.option norvc


# Rodata section
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

# Text section
.section .text.init


.global _start
_start:
	# Any hardware threads (hart) that are not bootstrapping
	# need to wait for an IPI
	csrr	t0, mhartid
	bnez	t0, _end

	# Clear BSS secion
	la 		a0, _bss_start
	la		a1, _bss_end
	bgeu	a0, a1, 2f
1:
	sd		zero, (a0)
	addi	a0, a0, 8
	bltu	a0, a1, 1b
2:	
	# Setup pmp registers
	li t0, 0xFFFFFFFF
	csrw pmpaddr0,t0
	li t0, 0xFF
	csrw pmpcfg0,t0

	# Disable paging
	csrw	satp, zero
	# Stack pointer at the very end of the stack space
	la		sp, _stack_end

	# Setting `mstatus` register:
	# 0b01 << 11: Machine's previous protection mode is 1 (MPP=1). --> We enter in supervisor mode
	# 1 << 7    : Machine's previous interrupt-enable bit is 1 (MPIE=1).
	# 1 << 3    : Machine's interrupt-enable bit is 1 (MIE=1).
	li t0, (0b01 << 11) | (1 << 7) | (1 << 3)
	csrw	mstatus, t0

	# Machine's exception program counter (MEPC) is set to `kinit`.
	la		t1, kmain
	csrw	mepc, t1

	# Setting Machine's interrupt-enable bits (`mie` register):
	# 1 << 3 : Machine's M-mode software interrupt-enable bit is 1 (MSIE=1).
	# 1 << 7 : Machine's timer interrupt-enable bit is 1 (MTIE=1).
	# 1 << 11: Machine's external interrupt-enable bit is 1 (MEIE=1).
	li		t3, (1 << 3) | (0 << 7) | (1 << 11)
	csrw	mie, t3

	# Machine's trap vector base address is set to `asm_trap_vector`.
	la		t2, asm_trap_vector
	csrw	mtvec, t2

	# mret allows us to jump to supervisor mode
	mret
_end:
	wfi
	j		_end

