#![rustfmt::skip]

use core::marker::PhantomData;
use core::ops::{BitAnd, BitOr, BitXor, Not};
use core::ptr;

pub fn uninterrupted<F: FnOnce()>(f: F) {
    let saved_sreg_i: u8 = unsafe { ptr::read_volatile(SREG::ADDRESS) | I.value };
    unsafe {
        llvm_asm!("cli" :::: "volatile");
    }
    f();
    unsafe {
        let old_sreg = ptr::read_volatile(SREG::ADDRESS);
        ptr::write_volatile(SREG::ADDRESS, old_sreg | saved_sreg_i);
    }
}

pub struct Mask<T> {
    pub value: u8,
    pub phantom: PhantomData<T>,
}

macro_rules! mask {
    ($value:expr) => {
        Mask {
            value: $value,
            phantom: PhantomData,
        }
    };
}

impl<T> Clone for Mask<T> {
    fn clone(&self) -> Self {
        mask!(self.value)
    }
}
impl<T> Copy for Mask<T> {}

impl<T> BitAnd for Mask<T> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        mask!(self.value & rhs.value)
    }
}
impl<T> BitOr for Mask<T> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        mask!(self.value | rhs.value)
    }
}
impl<T> BitXor for Mask<T> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        mask!(self.value ^ rhs.value)
    }
}
impl<T> Not for Mask<T> {
    type Output = Self;
    fn not(self) -> Self {
        mask!(!self.value)
    }
}

pub trait Reg8: Sized {
    const ADDRESS: *mut u8;
    unsafe fn get() -> u8 {
        ptr::read_volatile(Self::ADDRESS)
    }
    unsafe fn set(new_val: u8) {
        ptr::write_volatile(Self::ADDRESS, new_val);
    }
    unsafe fn modify<F>(f: F)
    where
        F: FnOnce(Mask<Self>) -> Mask<Self>,
    {
        uninterrupted(|| {
            let old_val: Mask<Self> = mask!(Self::get());
            Self::set(f(old_val).value);
        });
    }
}

pub trait Reg16: Sized {
    const ADDRESS: *mut u16;
    unsafe fn get() -> u16 {
        ptr::read_volatile(Self::ADDRESS)
    }
    unsafe fn set(new_val: u16) {
        ptr::write_volatile(Self::ADDRESS, new_val);
    }
}

macro_rules! def_bit {
    ($reg:ident, -, $index:expr) => {};
    ($reg:ident, $name:ident, $mask:expr) => {
        pub const $name: Mask<$reg> = mask!($mask);
    };
}

macro_rules! def_reg {
    ($addr:tt is reserved) => {};
    ($addr:expr, $name:ident, [$b7:tt, $b6:tt, $b5:tt, $b4:tt, $b3:tt, $b2:tt, $b1:tt, $b0:tt]) => {
        pub struct $name;
        impl Reg8 for $name {
            const ADDRESS: *mut u8 = $addr as *mut u8;
        }
        def_bit!($name, $b7, 0b1000_0000);
        def_bit!($name, $b6, 0b0100_0000);
        def_bit!($name, $b5, 0b0010_0000);
        def_bit!($name, $b4, 0b0001_0000);
        def_bit!($name, $b3, 0b0000_1000);
        def_bit!($name, $b2, 0b0000_0100);
        def_bit!($name, $b1, 0b0000_0010);
        def_bit!($name, $b0, 0b0000_0001);
    };
    ($addr:expr, $name:ident) => {
        pub struct $name;
        impl Reg8 for $name {
            const ADDRESS: *mut u8 = $addr as *mut u8;
        }
    };
}

macro_rules! def_reg16 {
    ($addr:expr, $name:ident) => {
        pub struct $name;
        impl Reg16 for $name {
            const ADDRESS: *mut u16 = $addr as *mut u16;
        }
    };
}

def_reg!(0x5F, SREG,   [I,       T,       H,       S,       V,       N,       Z,       C      ]);
def_reg!(0x5E, SPH,    [-,       -,       -,       -,       -,       SP10,    SP9,     SP8    ]);
def_reg!(0x5D, SPL,    [SP7,     SP6,     SP5,     SP4,     SP3,     SP2,     SP1,     SP0    ]);
def_reg!(0x5C is reserved                                                                      );
def_reg!(0x5B, GIMSK,  [-,       INT0,    PCIE,    -,       -,       -,       -,       -      ]);
def_reg!(0x5A, GIFR,   [-,       INTF0,   PCIF,    -,       -,       -,       -,       -      ]);
def_reg!(0x59, TIMSK,  [-,       OCIE1A,  OCIE1B,  OCIE0A,  OCIE0B,  TOIE1,   TOIE0,   -      ]);
def_reg!(0x58, TIFR,   [-,       OCF1A,   OCF1B,   OCF0A,   OCF0B,   TOV1,    TOV0,    -      ]);
def_reg!(0x57, SPMCSR, [-,       -,       RSIG,    CTPB,    RFLB,    PGWRT,   PGERS,   SPMEN  ]);
def_reg!(0x56 is reserved                                                                      );
def_reg!(0x55, MCUCR,  [BODS,    PUD,     SE,      SM1,     SM0,     BODSE,   ISC01,   ISC00  ]);
def_reg!(0x54, MCUSR,  [-,       -,       -,       -,       WDRF,    BORF,    EXTRF,   PORF   ]);
def_reg!(0x53, TCCR0B, [FOC0A,   FOC0B,   -,       -,       WGM02,   CS02,    CS01,    CS00   ]);
def_reg!(0x52, TCNT0                                                                           );
def_reg!(0x51, OSCCAL                                                                          );
def_reg!(0x50, TCCR1,  [CTC1,    PWM1A,   COM1A1,  COM1A0,  CS13,    CS12,    CS11,    CS10   ]);
def_reg!(0x4F, TCNT1                                                                           );
def_reg!(0x4E, OCR1A                                                                           );
def_reg!(0x4D, OCR1C                                                                           );
def_reg!(0x4C, GTCCR,  [TSM,     PWM1B,   COM1B1,  COM1B0,  FOC1B,   FOC1A,   PSR1,    PSR0   ]);
def_reg!(0x4B, OCR1B                                                                           );
def_reg!(0x4A, TCCR0A, [COM0A1,  COM0A0,  COM0B1,  COM0B0,  -,       -,       WGM01,   WGM00  ]);
def_reg!(0x49, OCR0A                                                                           );
def_reg!(0x48, OCR0B                                                                           );
def_reg!(0x47, PLLCSR, [LSM,     -,       -,       -,       -,       PCKE,    PLLE,    PLOCK  ]);
def_reg!(0x46, CLKPR,  [CLKPCE,  -,       -,       -,       CLKPS3,  CLKPS2,  CLKPS1,  CLKPS0 ]);
def_reg!(0x45, DT1A,   [DT1AH3,  DT1AH2,  DT1AH1,  DT1AH0,  DT1AL3,  DT1AL2,  DT1AL1,  DT1AL0 ]);
def_reg!(0x44, DT1B,   [DT1BH3,  DT1BH2,  DT1BH1,  DT1BH0,  DT1BL3,  DT1BL2,  DT1BL1,  DT1BL0 ]);
def_reg!(0x43, DTPS1,  [-,       -,       -,       -,       -,       -,       DTPS11,  DTPS10 ]);
def_reg!(0x42, DWDR                                                                            );
def_reg!(0x41, WDTCR,  [WDIF,    WDIE,    WDP3,    WDCE,    WDE,     WDP2,    WDP1,    WDP0   ]);
def_reg!(0x40, PRR,    [-,       -,       -,       -,       PRTIM1,  PRTIM0,  PRUS1,   PRADC  ]);
def_reg!(0x3F, EEARH                                                                           );
def_reg!(0x3E, EEARL                                                                           );
def_reg!(0x3D, EEDR                                                                            );
def_reg!(0x3C, EECR,   [-,       -,       EEPM1,   EEPM0,   EERIE,   EEMPE,   EEPE,    EERE   ]);
def_reg!(0x3B is reserved                                                                      );
def_reg!(0x3A is reserved                                                                      );
def_reg!(0x39 is reserved                                                                      );
def_reg!(0x38, PORTB,  [-,       -,       PORTB5,  PORTB4,  PORTB3,  PORTB2,  PORTB1,  PORTB0 ]);
def_reg!(0x37, DDRB,   [-,       -,       DDB5,    DDB4,    DDB3,    DDB2,    DDB1,    DDB0   ]);
def_reg!(0x36, PINB,   [-,       -,       PINB5,   PINB4,   PINB3,   PINB2,   PINB1,   PINB0  ]);
def_reg!(0x35, PCMSK,  [-,       -,       PCINT5,  PCINT4,  PCINT3,  PCINT2,  PCINT1,  PCINT0 ]);
def_reg!(0x34, DIDR0,  [-,       -,       ADC0D,   ADC2D,   ADC3D,   ADC1D,   AIN1D,   AIN0D  ]);
def_reg!(0x33, GPIOR2                                                                          );
def_reg!(0x32, GPIOR1                                                                          );
def_reg!(0x31, GPIOR0                                                                          );
def_reg!(0x30, USIBR                                                                           );
def_reg!(0x2F, USIDR                                                                           );
def_reg!(0x2E, USISR,  [USISIF,  USIOIF,  USIPF,   USIDC,   USICNT3, USICNT2, USICNT1, USICNT0]);
def_reg!(0x2D, USICR,  [USISIE,  USIOIE,  USIWM1,  USIWM0,  USICS1,  USICS0,  USICLK,  USITC  ]);
def_reg!(0x2C is reserved                                                                      );
def_reg!(0x2B is reserved                                                                      );
def_reg!(0x2A is reserved                                                                      );
def_reg!(0x29 is reserved                                                                      );
def_reg!(0x28, ACSR,   [ACD,     ACBG,    ACO,     ACI,     ACIE,    -,       ACIS1,   ACIS0  ]);
def_reg!(0x27, ADMUX,  [REFS1,   REFS0,   ADLAR,   REFS2,   MUX3,    MUX2,    MUX1,    MUX0   ]);
def_reg!(0x26, ADCSRA, [ADEN,    ADSC,    ADATE,   ADIF,    ADIE,    ADPS2,   ADPS1,   ADPS0  ]);
def_reg!(0x25, ADCH                                                                            );
def_reg!(0x24, ADCL                                                                            );
def_reg!(0x23, ADCSRB, [BIN,     ACME,    IPR,     -,       -,       ADTS2,   ADTS1,   ADTS0  ]);
def_reg!(0x22 is reserved                                                                      );
def_reg!(0x21 is reserved                                                                      );
def_reg!(0x20 is reserved                                                                      );

// 16-bit register pairs
def_reg16!( ADCL::ADDRESS,  ADC);
def_reg16!(EEARL::ADDRESS, EEAR);
def_reg16!(  SPL::ADDRESS,   SP);
