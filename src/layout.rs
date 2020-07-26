use crate::attiny85_defs::*;
use core::ptr;

#[repr(C, packed)]
pub struct ProcInfo {
    pub sp: u16,
    pub alive: bool,
    pub asleep: u8,
}

pub const STACK_SIZE: usize = 0x80;
pub const FIRST_STACK: usize = 0x260;

/********** PLAN ***********
 * - Forking a new process allocates a stack
 * - Even intervals of STACK_SIZE
 * - First 4 bytes go to the ProcInfo struct
 * - Free stack space will have a zero'd out pointer
 *   (so will currently running processes)
 * - STACK_SIZE will always be a power of two
 *      (makes it easier for a process to find its ProcInfo)
 */

extern "C" {
    fn _asm_switch_context(from: *mut u16, to: u16);
    fn _asm_start_fn(f: extern "C" fn(), stack_loc: usize);
}

impl ProcInfo {
    pub fn dead() -> Self {
        ProcInfo {
            sp: 0x0000,
            alive: false,
            asleep: 0,
        }
    }
    pub unsafe fn switch_to(&mut self, to: &mut Self) {
        let dest_sp = to.sp;
        to.sp = 0;
        _asm_switch_context(&mut self.sp as *mut u16, dest_sp);
    }

    // Returns true on success, false on failure
    pub fn fork(f: extern "C" fn()) -> bool {
        let info = ProcInfo {
            sp: 0x0000,
            alive: true,
            asleep: 0,
        };
        unsafe {
            let mut addr: usize = FIRST_STACK;
            while is_occupied(addr) {
                addr -= STACK_SIZE;
                if addr < 0x90 {
                    // TODO
                    return false;
                }
            }
            let proc_info = ProcInfo::at(addr);
            ptr::write_volatile(proc_info, info);
            _asm_start_fn(f, proc_info as usize);
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
        StacksIter { addr: 0x60 }
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

pub fn is_suspended(addr: usize) -> bool {
    unsafe {
        let info_addr = ProcInfo::at(addr);
        (*info_addr).sp != 0
    }
}

pub fn is_occupied(addr: usize) -> bool {
    unsafe {
        let info_addr = ProcInfo::at(addr);
        (*info_addr).alive
    }
}
