
use arduino_attiny::*;
use core::{mem, ptr};
use util::delay_ms;

#[derive(Clone,Copy)]
#[repr(C,packed)]
pub struct StackPointer {
    spl: u8,
    sph: u8,
}
impl StackPointer {
    pub fn new(x: u16) -> Self {
        unsafe { mem::transmute(x) }
    }
    #[inline]
    pub fn as_u16(self) -> u16 {
        unsafe { mem::transmute(self) }
    }
}

#[derive(Clone,Copy)]
#[repr(C,packed)]
pub struct ProcContext {
    sp: StackPointer,
}

#[no_mangle]
pub static mut KERNEL_CONTEXT: ProcContext = ProcContext {
    sp: StackPointer { spl: 0, sph: 0 },
};

extern "C" {
    fn _asm_switch_context(from: *mut ProcContext, to: ProcContext);
    fn _asm_start_fn(f: extern "C" fn(ProcContext) -> !, stack_loc: usize);
}

impl ProcContext {
    pub fn new(sp: StackPointer) -> Self {
        ProcContext {
            sp: sp,
        }
    }

    pub unsafe fn start_fn(f: extern "C" fn(ProcContext) -> !, stack_loc: usize) -> *const Self {
        _asm_start_fn(f, stack_loc);
        (stack_loc - 2) as *const Self
    }
    pub unsafe fn switch_to(&mut self, to: ProcContext) {
        _asm_switch_context(self as *mut Self, to);
    }
}

pub unsafe fn test() {
    ptr::write_volatile(DDRB, 0x1F);

    let new_stack_addr: usize = 0x160;
    // +------- 0x260 (presumably the start)
    // | |
    // | | main stack
    // | v 
    // |
    // +------- 0x160 proc's stack start
    // | |
    // | | proc's stack
    // | v 
    // |
    // +------- 0x060
    // | IO Registers,
    // | register file
    // +------- 0x000

    ptr::write_volatile(PORTB, 0x01);
    delay_ms(500);
    ptr::write_volatile(PORTB, 0x00);
    delay_ms(500);

    let proc_context_ptr = ProcContext::start_fn(proc_fn, new_stack_addr);

    loop {
        ptr::write_volatile(PORTB, 0x01);
        delay_ms(500);
        ptr::write_volatile(PORTB, 0x00);
        delay_ms(500);
        
        KERNEL_CONTEXT.switch_to(*proc_context_ptr);
    }
}

#[no_mangle]
pub extern "C" fn proc_fn(mut my_context: ProcContext) -> ! {
    unsafe {
        loop {
            for _ in 0..5 {
                ptr::write_volatile(PORTB, 0x04);
                delay_ms(100);
                ptr::write_volatile(PORTB, 0x00);
                delay_ms(100);
            }
            my_context.switch_to(KERNEL_CONTEXT);
        }
    }
}

