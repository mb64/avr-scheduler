
use arduino_attiny::*;
use core::{ptr,mem};

use process;

#[repr(C,packed)]
pub struct ProcInfo {
    context: process::ProcContext,
    priority: i8, // TODO these things
    asleep: u8,
}

pub const STACK_SIZE: usize = 0x80;
pub const FIRST_STACK: usize = 0x260;

/********** PLAN ***********
 * - Forking a new process allocates a stack
 * - Even intervals of STACK_SIZE
 * - First 4 bytes go to the ProcInfo struct
 * - Free stack space will have a zero'd out pointer
 * - STACK_SIZE will ALWAYS be a multiple of two
 *      (makes it easier for a process to find its ProcInfo)
 */

impl ProcInfo {
    // Returns true on success, false on failure
    pub fn fork<F>(f: F, priority: i8) -> bool
        where F: FnOnce()
    {
        let info = ProcInfo {
            context: process::ProcContext::new(0),
            priority: priority,
            asleep: 0,
        }
        unsafe {
            extern "C" fn wrapper(ctx: *mut process::ProcContext) -> ! {
                f();
                ptr::write(ctx as *mut u32, 0); // Mark the stack as free
                loop {} // It'll be interrupted, the execution will move to 
                        // another process, and this'll be forgotten.
            }
            let mut addr: usize = FIRST_STACK - 4; // size of ProcInfo = 4
            while (*(addr as *const ProcInfo)).context.sp != 0 {
                addr -= STACK_SIZE;
                if addr < 0x90 { // Last usable stack is at 0xD0
                    return false;
                }
            }
            ptr::write(addr as *mut ProcInfo, info);
            process::ProcContext::start_fn(wrapper, addr);
        }
        true
    }

    pub fn this_proc() -> *const Self {
        let sp: usize;
        unsafe {
            asm!("
                 in r30, 61
                 in r31, 62
                 "
                :"={Z}"(sp)
                : // No inputs
                : // No clobbers
                :"volatile");
        }
        let mut next_stack_addr: usize = FIRST_STACK - STACK_SIZE;
        while next_stack_addr > usize {
            next_stack_addr -= STACK_SIZE;
        }
        let this_stack_start = next_stack_addr + STACK_SIZE;
        (this_stack_start - 4) as *const Self
    }
}

