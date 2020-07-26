#![feature(llvm_asm, lang_items)]
#![feature(abi_avr_interrupt)]
#![no_std]
#![no_main]

pub mod attiny85_defs;
pub mod interface;
pub mod interrupts;
pub mod lang;
pub mod layout;
pub mod util;

use crate::interface::*;

#[cfg(feature = "test_interrupts")]
pub fn main() {
    let green_led = LED::new(Pin::Pin2);
    loop {
        green_led.on();
        util::busy_loop_ms(200);
        green_led.off();
        util::busy_loop_ms(200);
    }
}

#[cfg(not(feature = "test_interrupts"))]
pub fn main() {
    fork(blink_green);
    let orange_led = LED::new(Pin::Pin0);
    loop {
        orange_led.on();
        util::busy_loop_ms(210);
        orange_led.off();
        util::busy_loop_ms(210);
    }
}

extern "C" fn blink_green() {
    let green_led = LED::new(Pin::Pin2);
    loop {
        green_led.off();
        util::busy_loop_ms(250);
        green_led.on();
        util::busy_loop_ms(250);
    }
}
