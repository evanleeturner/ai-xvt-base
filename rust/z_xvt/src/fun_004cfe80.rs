// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759b08: i32;
    static mut DAT_00759b10: i32;
    static mut DAT_00759b34: [u32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cfe80(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; the table is indexed
    // as the original does, with param_1 checked against DAT_00759b10.
    unsafe {
        if DAT_00759b08 == 0 {
            return 0;
        }
        if 0 < param_1 && param_1 <= DAT_00759b10 {
            let u_var1: u32 = (&raw mut DAT_00759b34)
                .cast::<u32>()
                .add(param_1 as usize)
                .read();
            let frames = ((u_var1 >> 0x10) & 0xff).wrapping_mul(1000) / 0x4b;
            let secs = ((u_var1 & 0xff)
                .wrapping_mul(0x3c)
                .wrapping_add((u_var1 >> 8) & 0xff))
            .wrapping_mul(1000);
            return frames.wrapping_add(secs) as i32;
        }
        0
    }
}
