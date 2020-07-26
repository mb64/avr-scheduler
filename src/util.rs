use attiny85_defs::*;

pub const CPU_SPEED_HZ: u32 = 4_000_000;

pub fn busy_loop_ms(ms: u16) {
    for _ in 0..2 * ms {
        for _ in (0 as u8)..(250 as u8) {
            unsafe {
                asm!(""::::"volatile");
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Pin {
    Pin0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
}
impl Pin {
    fn to_portb_mask(self) -> Mask<PORTB> {
        match self {
            Pin::Pin0 => PORTB0,
            Pin::Pin1 => PORTB1,
            Pin::Pin2 => PORTB2,
            Pin::Pin3 => PORTB3,
            Pin::Pin4 => PORTB4,
            Pin::Pin5 => PORTB5,
        }
    }
    fn to_ddrb_mask(self) -> Mask<DDRB> {
        match self {
            Pin::Pin0 => DDB0,
            Pin::Pin1 => DDB1,
            Pin::Pin2 => DDB2,
            Pin::Pin3 => DDB3,
            Pin::Pin4 => DDB4,
            Pin::Pin5 => DDB5,
        }
    }
}

#[derive(Clone, Copy)]
pub struct LED {
    portb_mask: Mask<PORTB>,
}

impl LED {
    pub fn new(pin: Pin) -> Self {
        unsafe {
            DDRB::modify(|old| old | pin.to_ddrb_mask());
        }
        LED {
            portb_mask: pin.to_portb_mask(),
        }
    }
    pub fn on(self) {
        unsafe {
            PORTB::modify(|old| old | self.portb_mask);
        }
    }
    pub fn off(self) {
        unsafe {
            PORTB::modify(|old| old & (!self.portb_mask));
        }
    }
}
