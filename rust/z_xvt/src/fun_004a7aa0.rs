// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052687b: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a7aa0(param_1: u32) -> u32 {
    let u_var1: u32 = param_1 & 0xffff;
    let u_var2: u32 = (u_var1.wrapping_mul(0xb) >> 8) & 0x00ff_ffff;
    // SAFETY: DAT_0052687b is the game's own table; the index is read where the original reads it.
    let b = unsafe {
        (&raw mut DAT_0052687b)
            .cast::<u8>()
            .add(u_var1.wrapping_mul(0x16) as usize)
            .read()
    };
    ((u_var2 << 8) | u32::from(b)) & 0xffff_00ff
}
