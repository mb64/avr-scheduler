
.PHONY: all upload elf-new elf-old upload-new upload-old debug

elf-new:
	rm Xargo.toml
	XARGO_RUST_SRC=$(shell pwd)/../avr-rust/new-avr/rust/src \
		RUST_TARGET_PATH=$(shell pwd) \
		rustup run avr-new xargo build --target avr-attiny85 --release

upload-new: elf-new
	avrdude -pattiny85 -cusbtiny -Uflash:w:target/avr-attiny85/release/scheduler.elf

Xargo.toml: Xargo.toml.old
	cp Xargo.toml{.old,}

elf-old: Xargo.toml
	XARGO_RUST_SRC=$(shell pwd)/../avr-rust/rust/src \
		RUST_TARGET_PATH=$(shell pwd) \
		rustup run avr-old xargo build --target avr-attiny85 --release --verbose

upload-old: elf-old
	avrdude -pattiny85 -cusbtiny -Uflash:w:target/avr-attiny85/release/scheduler.elf


all: elf-old
upload: upload-old

debug: elf-old
	simavr -gdb -m attiny85 target/avr-attiny85/release/scheduler.elf &
	avr-gdb target/avr-attiny85/release/scheduler.elf \
		-ex 'target remote localhost:1234' \
		-ex 'layout asm' \
		-ex 'layout reg' \
		-ex 'break _asm_start_fn' \
		-ex 'continue'
