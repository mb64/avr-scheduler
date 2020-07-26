use crate::attiny85_defs::*;
use core::panic::PanicInfo;
use crate::util::busy_loop_ms;

#[lang = "eh_personality"]
#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality(
    _state: (),
    _exception_object: *mut (),
    _context: *mut (),
) -> () {
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
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
