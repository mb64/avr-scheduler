#![feature(asm, lang_items, unwind_attributes)]
#![feature(abi_avr_interrupt)]
#![feature(never_type)]

#![no_std]
#![no_main]

extern crate arduino_attiny;

pub mod process;
pub mod lang;
pub mod util;
pub mod layout;
pub mod interrupts;

use util::{LED,Pin,busy_loop_ms};

#[cfg(feature = "test_context_switch")]
#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        process::test();
    }
}

#[cfg(not(feature = "test_context_switch"))]
#[no_mangle]
pub extern "C" fn main() {
    let green_led = LED::new(Pin::Pin2);
    loop {
        green_led.on();
        busy_loop_ms(200);
        green_led.off();
        busy_loop_ms(200);
    }
}
