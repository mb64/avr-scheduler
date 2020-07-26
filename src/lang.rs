use attiny85_defs::*;
use util::busy_loop_ms;

#[lang = "eh_personality"]
#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality(
    _state: (),
    _exception_object: *mut (),
    _context: *mut (),
) -> () {
}

#[lang = "panic_fmt"]
#[unwind]
#[no_mangle]
pub extern "C" fn oh_no_bad_stuff(_msg: (), _file: &'static str, _line: u32) -> ! {
    unsafe {
        DDRB::set(0x1F);
        loop {
            PORTB::set(0x1F);
            busy_loop_ms(50);
            PORTB::set(0x00);
            busy_loop_ms(500);
        }
    }
}
