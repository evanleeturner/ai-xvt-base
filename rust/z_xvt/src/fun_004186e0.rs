// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523c60: [u16; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004186e0(param_1: u32, param_2: u16) -> i32 {
    let mut param_1 = param_1;
    let mut b_var2: u8 = 0;
    if (param_1 as i32) < 0 {
        param_1 = 0u32.wrapping_sub(param_1);
        b_var2 = 0x80;
    }
    let idx = (((param_2 >> 5) & 0x3fe) >> 1) as usize;
    // SAFETY: the table is the game's own; the index is at most 0x1ff.
    let u_var3: u32 = unsafe {
        u32::from(
            (&raw mut DAT_00523c60)
                .cast::<u16>()
                .add(idx)
                .read_unaligned(),
        )
    };
    let mut i_var1: u32 = (param_1 & 0xffff)
        .wrapping_mul(u_var3)
        .wrapping_add(0x8000)
        .wrapping_shr(0x10)
        .wrapping_add((param_1 >> 0x10).wrapping_mul(u_var3));
    if ((b_var2 ^ ((param_2 >> 8) as u8)) & 0x80) != 0 {
        i_var1 = 0u32.wrapping_sub(i_var1);
    }
    i_var1 as i32
}
