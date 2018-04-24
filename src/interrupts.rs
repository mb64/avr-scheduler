
use arduino_attiny::*;
use core::ptr;

use layout;

extern "C" {
    fn main();
}

#[no_mangle]
pub fn k_main() -> ! {
    unsafe {
        // Set up timers -- interrupt every 1.6 ms = 200 counts * 8 prescaled
        #[allow(non_snake_case)]
        {
            let old_TCCR0A = *TCCR0A;
            let old_TCCR0B = *TCCR0B;
            // Clear Timer on Compare Match
            ptr::write_volatile(TCCR0A, old_TCCR0A | WGM01);
            // Prescaler = 1 counter inc every 8 clks
            ptr::write_volatile(TCCR0B, old_TCCR0B | CS01);
        }

        // Interrupt when counter = 200
        ptr::write_volatile(OCR0A, 200);

        // Enable timer interrupts
        {
            #[allow(non_snake_case)]
            let old_TIMSK = *TIMSK;
            ptr::write_volatile(TIMSK, old_TIMSK | OCIE0A);
        }
    }
    unsafe { main(); }
    layout::die()
}

#[no_mangle]
pub extern "avr-interrupt" fn _ivr_timer0_compare_a() {
    // Schedule things...
}
