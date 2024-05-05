#####
## BUILD
#####
CC=/opt/riscv64_1/bin/riscv64-unknown-linux-gnu-g++
CFLAGS=-Wall -Wextra -pedantic -Wextra -O0 -g -std=c++17
CFLAGS+=-static -ffreestanding -nostdlib -fno-rtti -fno-exceptions
CFLAGS+=-march=rv64gc -mabi=lp64d
INCLUDES=
LINKER_SCRIPT=-Tsrc/lds/virt.lds
TYPE=debug
RUST_TARGET=./target/riscv64gc-unknown-none-elf/$(TYPE)
LIBS=-L$(RUST_TARGET)
SOURCES_ASM=$(wildcard src/asm/*.S)
LIB=-lsos -lgcc
OUT=os.elf

#####
## QEMU
#####
QEMU=qemu-system-riscv64
MACH=virt
CPU=rv64
CPUS=4
MEM=128M
DRIVE=hdd.dsk

all:
	cargo build
	$(CC) $(CFLAGS) $(LINKER_SCRIPT) $(INCLUDES) -o $(OUT) $(SOURCES_ASM) $(LIBS) $(LIB)
	
run: all
	$(QEMU) -s -S -machine $(MACH) -cpu $(CPU) -smp $(CPUS) -m $(MEM)  -nographic -serial mon:stdio -bios none -kernel $(OUT) -drive if=none,format=raw,file=$(DRIVE),id=foo -device virtio-blk-device,scsi=off,drive=foo


.PHONY: clean
clean:
	cargo clean
	rm -f $(OUT)

analyse:
	riscv64-unknown-linux-gnu-objdump --disassemble os.elf > output.S

gdb: 
	riscv64-unknown-linux-gnu-gdb os.elf 
