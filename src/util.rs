
use attiny85_defs::*;
use core::ptr;

pub const CPU_SPEED_HZ: u32 = 2_000_000;

pub fn busy_loop_ms(ms: u16) {
    for _ in 0..ms {
        for _ in (0 as u8)..(250 as u8) {
            unsafe { asm!(""::::"volatile"); }
        }
    }
}

#[derive(Clone,Copy)]
pub enum Pin {
    Pin0 = 0,
    Pin1 = 1,
    Pin2 = 2,
    Pin3 = 3,
    Pin4 = 4,
    Pin5 = 5,
}
impl Pin {
    fn to_mask(self) -> u8 {
        1 << (self as u8)
    }
}

#[derive(Clone,Copy)]
pub struct LED {
    portb_mask: u8,
}

impl LED {
    pub fn new(pin: Pin) -> Self {
        let mask = pin.to_mask();
        unsafe {
            let orig_ddrb = *DDRB;
            ptr::write_volatile(DDRB, orig_ddrb | mask);
        }
        LED {
            portb_mask: mask,
        }
    }
    pub fn on(self) {
        unsafe {
            let orig_portb = *PORTB;
            ptr::write_volatile(PORTB, orig_portb | self.portb_mask);
        }
    }
    pub fn off(self) {
        unsafe {
            let orig_portb = *PORTB;
            ptr::write_volatile(PORTB, orig_portb & (!self.portb_mask));
        }
    }
}



