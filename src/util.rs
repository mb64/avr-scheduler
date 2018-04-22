
pub const CPU_SPEED_HZ: u32 = 2_000_000;

pub fn delay_ms(ms: u16) {
    for _ in 0..ms {
        for _ in (0 as u8)..(250 as u8) {
            unsafe { asm!(""::::"volatile"); }
        }
    }
}



