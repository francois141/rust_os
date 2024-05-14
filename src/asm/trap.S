.section .text
.global asm_trap_vector
# This must be aligned by 4 since the last two bits
# of the mtvec register do not contribute to the address
# of this vector.
.align 4
asm_trap_vector:
	addi	sp, sp, -256

	# save the registers.
	sd		ra, 0(sp)
	sd		sp, 8(sp)
	sd		gp, 16(sp)
	sd		tp, 24(sp)
	sd		t0, 32(sp)
	sd		t1, 40(sp)
	sd		t2, 48(sp)
	sd		s0, 56(sp)
	sd		s1, 64(sp)
	sd		a0, 72(sp)
	sd		a1, 80(sp)
	sd		a2, 88(sp)
	sd		a3, 96(sp)
	sd		a4, 104(sp)
	sd		a5, 112(sp)
	sd		a6, 120(sp)
	sd		a7, 128(sp)
	sd		s2, 136(sp)
	sd		s3, 144(sp)
	sd		s4, 152(sp)
	sd		s5, 160(sp)
	sd		s6, 168(sp)
	sd		s7, 176(sp)
	sd		s8, 184(sp)
	sd		s9, 192(sp)
	sd		s10, 200(sp)
	sd		s11, 208(sp)
	sd		t3, 216(sp)
	sd		t4, 224(sp)
	sd		t5, 232(sp)
	sd		t6, 240(sp)

	call	m_trap

	# Restore Machine Exception Program Counter with return address
	csrw	mepc, a0

	# Load register backs
	ld		ra, 0(sp)
	ld		sp, 8(sp)
	ld		gp, 16(sp)
	ld		tp, 24(sp)
	ld		t0, 32(sp)
	ld		t1, 40(sp)
	ld		t2, 48(sp)
	ld		s0, 56(sp)
	ld		s1, 64(sp)
	ld		a0, 72(sp)
	ld		a1, 80(sp)
	ld		a2, 88(sp)
	ld		a3, 96(sp)
	ld		a4, 104(sp)
	ld		a5, 112(sp)
	ld		a6, 120(sp)
	ld		a7, 128(sp)
	ld		s2, 136(sp)
	ld		s3, 144(sp)
	ld		s4, 152(sp)
	ld		s5, 160(sp)
	ld		s6, 168(sp)
	ld		s7, 176(sp)
	ld		s8, 184(sp)
	ld		s9, 192(sp)
	ld		s10, 200(sp)
	ld		s11, 208(sp)
	ld		t3, 216(sp)
	ld		t4, 224(sp)
	ld		t5, 232(sp)
	ld		t6, 240(sp)

	addi	sp, sp, 256

	mret

.global switch_to_other_process
switch_to_other_process:

	# Step 1 - Write trap frame pointer to mscratch
	csrw mscratch, a0

	# Step 2 - Write program counter 
	#ld a1, 512(a1)
	csrw mepc, a1
	
	# Step 3 - Setup correct page table (ignored for the moment)

	# Step 4 - Restore general purpose registers
	ld		ra, 0(a0)
	ld		sp, 8(a0)
	ld		gp, 16(a0)
	ld		tp, 24(a0)
	ld		t0, 32(a0)
	ld		t1, 40(a0)
	ld		t2, 48(a0)
	ld		s0, 56(a0)
	ld		s1, 64(a0)
	ld		a1, 80(a0)
	ld		a2, 88(a0)
	ld		a3, 96(a0)
	ld		a4, 104(a0)
	ld		a5, 112(a0)
	ld		a6, 120(a0)
	ld		a7, 128(a0)
	ld		s2, 136(a0)
	ld		s3, 144(a0)
	ld		s4, 152(a0)
	ld		s5, 160(a0)
	ld		s6, 168(a0)
	ld		s7, 176(a0)
	ld		s8, 184(a0)
	ld		s9, 192(a0)
	ld		s10, 200(a0)
	ld		s11, 208(a0)
	ld		t3, 216(a0)
	ld		t4, 224(a0)
	ld		t5, 232(a0)
	ld		t6, 240(a0)
	ld		a0, 72(a0)

	# Step 5 - Restore floating point registers (ignored for the moment)

	# Step 6 - jump to the user function (in the future, we jump to userspace)
	mret


.global make_syscall
make_syscall:
	ecall
	ret
