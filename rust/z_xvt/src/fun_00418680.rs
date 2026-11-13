// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523c60: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00418680(param_1: u16, param_2: u16) -> u32 {
    let mut param_1 = param_1;
    let u_var1: u16 = param_1 & 0x8000;
    if u_var1 != 0 {
        param_1 = param_1.wrapping_neg();
    }
    let idx = ((param_2 & 0x7fc0) >> 6) as usize;
    // SAFETY: the table is the game's own memory; the index is at most 0x1ff
    // entries of two bytes, read where the original reads.
    let entry = unsafe {
        (&raw mut DAT_00523c60)
            .cast::<u8>()
            .cast_const()
            .byte_add(idx * 2)
            .cast::<u16>()
            .read_unaligned()
    };
    let mut u_var2: u32 = u32::from(entry)
        .wrapping_mul(u32::from(param_1))
        .wrapping_add(0x8000);
    if (param_2 & 0x8000) != u_var1 {
        u_var2 = u_var2.wrapping_neg();
    }
    u_var2 >> 0x10
}
