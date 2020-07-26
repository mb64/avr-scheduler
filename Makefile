
.PHONY: all upload elf debug test-interrupts

all: elf

upload: elf
	avrdude -pattiny85 -cusbtiny -Uflash:w:target/avr-attiny85/release/scheduler.elf

elf:
	cargo build -Z build-std=core --target avr-attiny85.json --release

test-interrupts:
	cargo build -Z build-std=core --target avr-attiny85.json --release --features test_interrupts

debug: elf
	simavr -gdb -m attiny85 -f 1000000 target/avr-attiny85/release/scheduler.elf &
	avr-gdb target/avr-attiny85/release/scheduler.elf \
		-ex 'target remote localhost:1234' \
		-ex 'layout asm' \
		-ex 'layout reg' \
		-ex 'break _asm_start_fn'
