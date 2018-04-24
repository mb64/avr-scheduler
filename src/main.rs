#![feature(asm, lang_items, unwind_attributes)]
#![feature(abi_avr_interrupt)]
#![feature(never_type)]

#![no_std]
#![no_main]

pub mod attiny85_defs;
pub mod process;
pub mod lang;
pub mod util;
pub mod layout;
pub mod interrupts;

use util::{LED,Pin,busy_loop_ms};

#[cfg(feature = "test_context_switch")]
pub fn main() {
    unsafe {
        process::test();
    }
}

#[cfg(all(not(feature = "test_context_switch"),
          feature = "test_interrupts"))]
pub fn main() {
    let green_led = LED::new(Pin::Pin2);
    loop {
        green_led.on();
        busy_loop_ms(200);
        green_led.off();
        busy_loop_ms(200);
    }
}

#[cfg(all(not(feature = "test_context_switch"),
          not(feature = "test_interrupts")))]
pub fn main() {
}
