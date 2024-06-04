.section .text
.global asm_trap_vector
# This must be aligned by 4 since the last two bits
# of the mtvec register do not contribute to the address
# of this vector.
.align 4
asm_trap_vector:
	
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

	add t5, t6, zero
	csrr t6, mscratch
	sd		t6, 240(t5)

	# csrr	a0, mepc
	# ssd		a0, 248(t5)

	csrw mscratch, t5

	call	m_trap

	# Restore Machine Exception Program Counter with return address
	csrw	mepc, a0

	csrr t6, mscratch
	

	# Load register backs
	ld		ra, 0(t6)
	ld		sp, 8(t6)
	ld		gp, 16(t6)
	ld		tp, 24(t6)
	ld		t0, 32(t6)
	ld		t1, 40(t6)
	ld		t2, 48(t6)
	ld		s0, 56(t6)
	ld		s1, 64(t6)
	ld		a0, 72(t6)
	ld		a1, 80(t6)
	ld		a2, 88(t6)
	ld		a3, 96(t6)
	ld		a4, 104(t6)
	ld		a5, 112(t6)
	ld		a6, 120(t6)
	ld		a7, 128(t6)
	ld		s2, 136(t6)
	ld		s3, 144(t6)
	ld		s4, 152(t6)
	ld		s5, 160(t6)
	ld		s6, 168(t6)
	ld		s7, 176(t6)
	ld		s8, 184(t6)
	ld		s9, 192(t6)
	ld		s10, 200(t6)
	ld		s11, 208(t6)
	ld		t3, 216(t6)
	ld		t4, 224(t6)
	ld		t5, 232(t6)
	ld		t6, 240(t6)


	addi	sp, sp, 256

	mret

.global switch_to_other_process
switch_to_other_process:

	# Step 1 - Write trap frame pointer to mscratch
	csrw mscratch, a0

	# Step 2 - Write program counter 
	csrw mepc, a1
	
	# Step 3 - Setup correct page table (ignored for the moment)

	# Step 4 - Restore general purpose registers
	mv t6, a0
	ld		ra, 0(t6)
	ld		sp, 8(t6)
	ld		gp, 16(t6)
	ld		tp, 24(t6)
	ld		t0, 32(t6)
	ld		t1, 40(t6)
	ld		t2, 48(t6)
	ld		s0, 56(t6)
	ld		s1, 64(t6)
	ld		a0, 72(t6)
	ld		a1, 80(t6)
	ld		a2, 88(t6)
	ld		a3, 96(t6)
	ld		a4, 104(t6)
	ld		a5, 112(t6)
	ld		a6, 120(t6)
	ld		a7, 128(t6)
	ld		s2, 136(t6)
	ld		s3, 144(t6)
	ld		s4, 152(t6)
	ld		s5, 160(t6)
	ld		s6, 168(t6)
	ld		s7, 176(t6)
	ld		s8, 184(t6)
	ld		s9, 192(t6)
	ld		s10, 200(t6)
	ld		s11, 208(t6)
	ld		t3, 216(t6)
	ld		t4, 224(t6)
	ld		t5, 232(t6)
	ld		t6, 240(t6)


	# Step 5 - Restore floating point registers (ignored for the moment)

	# Step 6 - jump to the user function (in the future, we jump to userspace)
	mret


.global make_syscall
make_syscall:
	ecall
	ret
