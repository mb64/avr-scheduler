
use attiny85_defs::*;
use core::ptr;

use layout;
use super::main;

#[no_mangle]
pub extern "C" fn k_main() -> ! {
    unsafe {
        // Set up timers -- interrupt every 1.6 ms = 200 counts * 8 prescaled

        // Enable timer interrupts
        #[allow(non_snake_case)]
        let old_TIMSK = *TIMSK;
        ptr::write_volatile(TIMSK, old_TIMSK | OCIE0A);

        // Interrupt when counter = 200
        ptr::write_volatile(OCR0A, 200);

        // Clear Timer on Compare Match
        #[allow(non_snake_case)]
        let old_TCCR0A = *TCCR0A;
        ptr::write_volatile(TCCR0A, old_TCCR0A | WGM01);

        // Prescaler = 1 counter inc every 8 clks
        #[allow(non_snake_case)]
        let old_TCCR0B = *TCCR0B;
        ptr::write_volatile(TCCR0B, old_TCCR0B | CS01);
    }
    main();
    layout::die()
}

#[cfg(feature = "test_interrupts")]
static mut counter: u8 = 0;

// __vector_10 is the avr-gcc name for interrupt vector entry 10, TIMER0_COMPA
#[cfg(feature = "test_interrupts")]
#[no_mangle]
pub extern "avr-interrupt" fn __vector_10() {
    unsafe {
        counter += 1;
        if counter == 250 {
            let old_portb = *PORTB;
            ptr::write_volatile(PORTB, old_portb ^ PORTB0);
            counter = 0;
        }
    }
    // Schedule things...
}

#[cfg(not(feature = "test_interrupts"))]
#[no_mangle]
pub extern "avr-interrupt" fn __vector_10() {
    unsafe {
        let old_portb = *PORTB;
        ptr::write_volatile(PORTB, old_portb ^ PORTB0);
    }
    // Schedule things...
}
