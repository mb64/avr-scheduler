
use attiny85_defs::*;
use core::{ptr,mem};

use process;

#[repr(C,packed)]
pub struct ProcInfo {
    pub context: process::ProcContext,
    pub priority: i8, // TODO Assign meaning to the priority
    pub asleep: u8,
}

pub const STACK_SIZE: usize = 0x80;
pub const FIRST_STACK: usize = 0x260;

/********** PLAN ***********
 * - Forking a new process allocates a stack
 * - Even intervals of STACK_SIZE
 * - First 4 bytes go to the ProcInfo struct
 * - Free stack space will have a zero'd out pointer
 * - STACK_SIZE will always be a power of two
 *      (makes it easier for a process to find its ProcInfo)
 */

impl ProcInfo {
    // Returns true on success, false on failure
    pub fn fork(f: extern "C" fn(), priority: i8) -> bool
    {
        let info = ProcInfo {
            context: process::ProcContext::new(0xffff),
            priority: priority,
            asleep: 0,
        };
        unsafe {
            let mut addr: usize = FIRST_STACK - 4; // size of ProcInfo = 4
            while (*(addr as *const ProcInfo)).context.sp != 0 {
                addr -= STACK_SIZE;
                if addr < 0x90 { // Last usable stack is at 0xD0
                    return false;
                }
            }
            ptr::write_volatile(addr as *mut ProcInfo, info);
            // These have the same calling convention
            let f_with_arg: extern "C" fn(x: *mut process::ProcContext) = mem::transmute(f);
            process::ProcContext::start_fn(f_with_arg, addr);
        }
        true
    }
    pub unsafe fn at(stack_addr: usize) -> *mut Self {
        (stack_addr - 4) as *mut Self
    }
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn get_proc_info_addr() -> *mut ProcInfo {
    let sp = SP::get() as usize;
    let mut next_stack_addr: usize = FIRST_STACK - STACK_SIZE;
    while next_stack_addr > sp {
        next_stack_addr -= STACK_SIZE;
    }
    let this_stack_start = next_stack_addr + STACK_SIZE;
    ProcInfo::at(this_stack_start)
}

pub struct StacksIter {
    addr: usize,
}
impl Default for StacksIter {
    fn default() -> Self {
        StacksIter {
            addr: 0x60,
        }
    }
}
impl Iterator for StacksIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.addr += STACK_SIZE;
        if self.addr > FIRST_STACK {
            None
        } else {
            Some(self.addr)
        }
    }
}

pub fn is_occupied(addr: usize) -> bool {
    unsafe {
        let info_addr = ProcInfo::at(addr);
        (*info_addr).context.sp != 0
    }
}

