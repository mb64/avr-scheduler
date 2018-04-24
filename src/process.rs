
use attiny_defs::*;
use core::ptr;
use util::*;

#[derive(Clone,Copy)]
#[repr(C,packed)]
pub struct ProcContext {
    pub sp: u16,
}

#[no_mangle]
pub static mut KERNEL_CONTEXT: ProcContext = ProcContext {
    sp: 0,
};

extern "C" {
    fn _asm_switch_context(from: *mut ProcContext, to: u16);
    fn _asm_start_fn(f: extern "C" fn(*mut ProcContext) -> !, stack_loc: usize);
}

impl ProcContext {
    pub fn new(sp: u16) -> Self {
        ProcContext {
            sp: sp,
        }
    }

    pub unsafe fn start_fn(f: extern "C" fn(*mut ProcContext) -> !, stack_loc: usize) -> *const Self {
        _asm_start_fn(f, stack_loc);
        stack_loc as *const Self
    }
    pub unsafe fn switch_to(&mut self, to: ProcContext) {
        _asm_switch_context(self as *mut Self, to.sp);
    }
}

#[cfg(feature = "test_context_switch")]
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


    let proc_context_ptr = ProcContext::start_fn(proc_fn, new_stack_addr);

    let orange_led = LED::new(Pin::Pin0);
    loop {
        for _ in 0..5 {
            orange_led.on();
            busy_loop_ms(100);
            orange_led.off();
            busy_loop_ms(100);
        }

        KERNEL_CONTEXT.switch_to(*proc_context_ptr);

        orange_led.on();
        busy_loop_ms(500);
        orange_led.off();
        busy_loop_ms(500);

        KERNEL_CONTEXT.switch_to(*proc_context_ptr);
    }
}

#[cfg(feature = "test_context_switch")]
pub extern "C" fn proc_fn(my_context_ptr: *mut ProcContext) -> ! {
    let green_led = LED::new(Pin::Pin2);
    loop {
        for _ in 0..5 {
            green_led.off();
            busy_loop_ms(100);
            green_led.on();
            busy_loop_ms(100);
        }
        unsafe { _asm_switch_context(my_context_ptr, KERNEL_CONTEXT.sp); }
        //unsafe { (&mut *my_context_ptr).switch_to(KERNEL_CONTEXT); }
    }
}

