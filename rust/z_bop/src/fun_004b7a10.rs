// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7205: i32;
    static mut DAT_00aa7201: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b7a10(param_1: i32) {
    let mut param_1 = param_1;
    // SAFETY: the globals are the game's own; the table at DAT_00aa7201 holds
    // 0x44-byte entries, and each copy moves one entry down over the previous
    // one, as the original does.
    unsafe {
        if -1 < param_1 && param_1 < DAT_00aa7205 {
            if param_1 < DAT_00aa7205.wrapping_sub(1) {
                let mut i_var2: i32 = param_1.wrapping_mul(0x44);
                loop {
                    let mut p_var4 = DAT_00aa7201.wrapping_add(i_var2) as *mut u32;
                    param_1 = param_1.wrapping_add(1);
                    i_var2 = i_var2.wrapping_add(0x44);
                    let mut p_var3 = p_var4.wrapping_add(0x11) as *const u32;
                    let mut i_var1: i32 = 0x11;
                    while i_var1 != 0 {
                        p_var4.write_unaligned(p_var3.read_unaligned());
                        p_var3 = p_var3.wrapping_add(1);
                        p_var4 = p_var4.wrapping_add(1);
                        i_var1 -= 1;
                    }
                    if !(param_1 < DAT_00aa7205.wrapping_sub(1)) {
                        break;
                    }
                }
            }
            DAT_00aa7205 = DAT_00aa7205.wrapping_sub(1);
        }
    }
}
