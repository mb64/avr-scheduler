
use core::ptr;
use attiny_defs::*;
use util::busy_loop_ms;

#[lang = "eh_personality"]
#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality(_state: (), _exception_object: *mut (), _context: *mut ()) -> () {
}

#[lang = "panic_fmt"]
#[unwind]
#[no_mangle]
pub extern "C" fn oh_no_bad_stuff(_msg: (), _file: &'static str, _line: u32) -> ! {
    unsafe {
        ptr::write_volatile(DDRB, 0x1F);
        loop {
            ptr::write_volatile(PORTB, 0x1F);
            busy_loop_ms(50);
            ptr::write_volatile(PORTB, 0x1F);
            busy_loop_ms(500);
        }
    }
}
