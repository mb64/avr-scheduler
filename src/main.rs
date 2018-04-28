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
pub mod interface;

use interface::*;

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
        util::busy_loop_ms(200);
        green_led.off();
        util::busy_loop_ms(200);
    }
}

#[cfg(all(not(feature = "test_context_switch"),
          not(feature = "test_interrupts")))]
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
