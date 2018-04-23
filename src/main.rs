#![feature(asm, lang_items, unwind_attributes)]
#![feature(never_type)]

#![no_std]
#![no_main]

extern crate arduino_attiny;

pub mod process;
pub mod lang;
pub mod util;

#[cfg(feature = "test_context_switch")]
#[no_mangle]
pub extern fn main() {
    unsafe {
        process::test();
    }
}

#[cfg(not(feature = "test_context_switch"))]
#[no_mangle]
pub extern fn main() {
    // ...
    loop {}
}
