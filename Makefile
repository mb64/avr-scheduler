
.PHONY: all upload elf-new elf-old upload-new upload-old debug test-context-switch upload-this

upload-this:
	avrdude -pattiny85 -cusbtiny -Uflash:w:target/avr-attiny85/release/scheduler.elf

elf-new:
	rm Xargo.toml
	XARGO_RUST_SRC=$(shell pwd)/../avr-rust/new-avr/rust/src \
		RUST_TARGET_PATH=$(shell pwd) \
		rustup run avr-new xargo build --target avr-attiny85 --release

upload-new: elf-new upload-this

Xargo.toml: Xargo.toml.old
	cp Xargo.toml{.old,}

elf-old: Xargo.toml
	XARGO_RUST_SRC=$(shell pwd)/../avr-rust/rust/src \
		RUST_TARGET_PATH=$(shell pwd) \
		rustup run avr-old xargo build --target avr-attiny85 --release --verbose

test-context-switch: Xargo.toml
	XARGO_RUST_SRC=$(shell pwd)/../avr-rust/rust/src \
		RUST_TARGET_PATH=$(shell pwd) \
		rustup run avr-old xargo build --target avr-attiny85 --release --verbose --features test_context_switch

upload-old: elf-old upload-this

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
