
use attiny85_defs::*;
use core::ptr;

use layout;
use process;
use super::main;

#[no_mangle]
pub extern "C" fn k_main() -> ! {
    // Don't initialize timers yet
    // If it's only single-threaded, there's no need, so it happens on calls
    // to fork()

    // When it restarts/resets, the memory is in an undefined state.
    // The ProcInfos need to be reset to zero.
    unsafe {
        for stack in layout::StacksIter::default()
            .take_while(|&addr| addr != layout::FIRST_STACK)
            .map(|addr| layout::ProcInfo::at(addr))
        {
            ptr::write_volatile(stack, layout::ProcInfo {
                context: process::ProcContext::new(0),
                priority: 0,
                asleep: 0,
            });
        }
    }

    main();
    die()
}

pub fn init_timers() {
    unsafe {
        // Set up timers -- interrupt every 3.2 ms = 200 counts * 64 prescaled / 4_000_000 Hz

        // Enable timer interrupts
        TIMSK::modify(|old| { old | OCIE0A });
        asm!("sei" :::: "volatile");

        // Interrupt when counter = 200
        OCR0A::set(200);

        // Clear Timer on Compare Match
        TCCR0A::modify(|old| { old | WGM01 });

        // Prescaler = 1 counter inc every 64 clks
        TCCR0B::modify(|old| { old | CS01 | CS00 });
    }
}

pub fn die() -> ! {
    unsafe {
        ptr::write_volatile(layout::get_proc_info_addr() as *mut u32, 0);
        run_scheduler(); // Give the leftover time to other processes
        loop { }
    }
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
            PORTB::modify(|old| { old ^ PORTB0 });
            counter = 0;
        }
    }
}

pub unsafe fn run_scheduler() {
    // Current (naive) algorithm: just pick the next one
    let new_proc: process::ProcContext = {
        let current_stack = SP::get() as usize;
        let mut awake_procs = layout::StacksIter::default()
            .filter(|&addr| layout::is_occupied(addr))
            .filter(|&addr| (*layout::ProcInfo::at(addr)).asleep == 0)
            .peekable();
        let first_stack: Option<usize> = awake_procs.peek().map(|&x| x);
        let next_stack_opt = awake_procs
            .find(|&stack_addr| stack_addr > current_stack+layout::STACK_SIZE)
            .or(first_stack);
        if let Some(next_stack) = next_stack_opt {
            (*layout::ProcInfo::at(next_stack)).context
        } else {
            // Without any processes, just wait...
            loop {}
        }
    };
    let this_proc_addr = layout::get_proc_info_addr();
    let this_proc = &mut (*this_proc_addr).context;
    this_proc.switch_to(new_proc);
}

// Should only be called every 3.2 ms
unsafe fn do_bookkeeping() {
    // Decrement asleep counts -- this is currently the only bookkeeping that happens
    for addr in layout::StacksIter::default()
            .filter(|&addr| layout::is_occupied(addr)) {
        let info = layout::ProcInfo::at(addr);
        if (*info).asleep != 0 {
            (*info).asleep -= 1;
        }
    }
}

#[cfg(not(feature = "test_interrupts"))]
#[no_mangle]
pub extern "avr-interrupt" fn __vector_10() {
    unsafe {
        uninterrupted(|| {
            do_bookkeeping();
            run_scheduler();
        });
    }
}

