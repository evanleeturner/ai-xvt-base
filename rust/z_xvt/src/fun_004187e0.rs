// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523c60: [u16; 512];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004187e0(param_1: u16, param_2: i16) -> u32 {
    let mut param_1 = param_1;
    let u_var1: u16 = param_1 & 0x8000;
    if u_var1 != 0 {
        param_1 = 0u32.wrapping_sub(param_1 as u32) as u16;
    }
    let t: u32 = (param_2 as i32 as u32).wrapping_add(0x4000);
    let idx = ((t & 0x7fc0) >> 6) as usize;
    // SAFETY: the table is the game's own memory; idx is at most 0x1ff.
    let entry = unsafe {
        (&raw const DAT_00523c60)
            .cast::<u16>()
            .add(idx)
            .read_unaligned()
    };
    let mut u_var2: u32 = (entry as u32)
        .wrapping_mul(param_1 as u32)
        .wrapping_add(0x8000);
    if (t & 0x8000) != u_var1 as u32 {
        u_var2 = 0u32.wrapping_sub(u_var2);
    }
    u_var2 >> 0x10
}
