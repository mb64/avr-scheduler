use crate::attiny85_defs::*;
use crate::util::busy_loop_ms;
use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}

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
