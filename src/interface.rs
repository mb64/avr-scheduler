// Ideally, _all_ user I/O would use this module.
// However, it's currently fantastically incomplete, so for anything
// more complex than blinking lights, that's not yet possible.

use crate::interrupts;
use crate::layout;

pub use crate::attiny85_defs::uninterrupted;
pub use crate::interrupts::die;
pub use crate::util::{Pin, LED};

pub fn delay_ms(ms: u16) {
    // 4 ms counts
    let mut counts = ms >> 2;
    unsafe {
        let proc_info = layout::get_proc_info_addr();
        while counts > 255 {
            counts -= 255;
            (*proc_info).asleep = 255;
            interrupts::run_scheduler();
        }
        (*proc_info).asleep = counts as u8;
        interrupts::run_scheduler();
    }
}

pub fn fork(new_proc: extern "C" fn()) -> bool {
    interrupts::init_timers();
    layout::ProcInfo::fork(new_proc)
}
