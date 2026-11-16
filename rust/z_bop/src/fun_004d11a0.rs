// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebfd: i32;
    static mut DAT_00acec45: [u32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d11a0(param_1: u32) -> i32 {
    let mut i_var1: i32 = 0;
    // SAFETY: the globals are the game's own memory; the table is walked
    // in 40-byte strides for DAT_00acebfd entries, read unaligned as the
    // original address is not 4-byte aligned.
    unsafe {
        if DAT_00acebfd > 0 {
            let mut p = (&raw mut DAT_00acec45).cast::<u32>();
            let mut i_var2: i32 = DAT_00acebfd;
            loop {
                if p.read_unaligned() < param_1 {
                    i_var1 = i_var1.wrapping_add(1);
                }
                p = p.byte_add(40);
                i_var2 = i_var2.wrapping_sub(1);
                if i_var2 == 0 {
                    break;
                }
            }
        }
    }
    i_var1
}
