#![feature(asm, lang_items, unwind_attributes)]
#![feature(never_type)]

#![no_std]
#![no_main]

extern crate arduino_attiny;

pub mod process;
pub mod lang;
pub mod util;

#[no_mangle]
pub extern fn main() {
    unsafe {
        process::test();
    }
}
