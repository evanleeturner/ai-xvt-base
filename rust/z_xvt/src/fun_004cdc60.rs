// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0078077a: i8;
    static mut DAT_00780c7a: i8;
    static mut DAT_0078077e: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cdc60(param_1: i32) {
    let mut i_var1: i32 = 0;
    // SAFETY: the globals are the game's own memory; only their addresses
    // are taken here.
    let mut p_var2: *mut i8 = unsafe { &raw mut DAT_0078077a };
    let end: *mut i8 = unsafe { &raw mut DAT_00780c7a };
    loop {
        // SAFETY: p_var2 points into the game's table of 0x28-byte entries.
        let local_val: i32 = unsafe { p_var2.cast::<i32>().read_unaligned() };
        if local_val == param_1 {
            break;
        }
        p_var2 = p_var2.wrapping_byte_add(0x28);
        i_var1 = i_var1.wrapping_add(1);
        if !((p_var2 as usize) < (end as usize)) {
            break;
        }
    }

    if i_var1 != 0x20 {
        let local_zero: i32 = 0;
        // SAFETY: the entry at index i_var1 lies within the game's table.
        unsafe {
            (&raw mut DAT_0078077e)
                .wrapping_byte_add(i_var1.wrapping_mul(0x28) as usize)
                .cast::<i32>()
                .write_unaligned(local_zero);
        }
    }
}
