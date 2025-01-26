rustflags           := "RUSTFLAGS='-C link-arg=-Tconfig/linker-script.x'"
build_std           := "-Zbuild-std=core,alloc"
build_features      := "-Zbuild-std-features=compiler-builtins-mem"
cargo_args          := build_std + " " + build_features
os_target       := "--target ./config/riscv-unknown-os.json"
os_elf          := "target/riscv-unknown-os/debug/os"
os_img          := "target/riscv-unknown-os/debug/os.img"


build:
	{{rustflags}} cargo build {{os_target}} {{cargo_args}}
	rust-objcopy -O binary {{os_elf}} {{os_img}}


fmt:
	cargo fmt

run:
	@just build
	qemu-system-riscv64 -machine virt -bios {{os_img}} -nographic