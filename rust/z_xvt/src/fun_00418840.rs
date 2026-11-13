// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523c60: [u16; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00418840(param_1: u32, param_2: i16) -> i32 {
    let mut param_1 = param_1;
    let mut b_var2: u8 = 0;
    if (param_1 as i32) < 0 {
        param_1 = 0u32.wrapping_sub(param_1);
        b_var2 = 0x80;
    }
    let u_var3: u16 = (param_2 as u16).wrapping_add(0x4000);
    let idx = ((((u_var3 >> 5) as u32) & 0x3fe) >> 1) as usize;
    // SAFETY: the table is the game's own; idx is at most 0x1ff.
    let u_var4: u32 = unsafe {
        u32::from(
            (&raw mut DAT_00523c60)
                .cast::<u16>()
                .add(idx)
                .read_unaligned(),
        )
    };
    let mut i_var1: u32 = ((param_1 & 0xffff).wrapping_mul(u_var4).wrapping_add(0x8000) >> 0x10)
        .wrapping_add((param_1 >> 0x10).wrapping_mul(u_var4));
    if ((b_var2 ^ (u_var3 >> 8) as u8) & 0x80) != 0 {
        i_var1 = 0u32.wrapping_sub(i_var1);
    }
    i_var1 as i32
}
