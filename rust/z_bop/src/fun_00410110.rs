// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08108: i16;
    static mut DAT_009d8c00: i16;
    static mut DAT_0054f9ac: i16;
    static mut DAT_0054f9b4: i16;
    static mut DAT_00a08102: i16;
    static mut DAT_009a20ae: u8;
    static mut DAT_009a6fe0: i16;
    static mut DAT_00a0813c: i16;
    static mut DAT_0054f9b8: i16;
    static mut DAT_0054f9a8: i16;
    static mut DAT_00a07ce4: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040fa30(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00410110() {
    // SAFETY: all globals and the callee belong to the game; accessed
    // by value in the same order as the original.
    unsafe {
        if DAT_00a08108 < DAT_009d8c00 {
            DAT_0054f9ac = DAT_009d8c00;
            DAT_0054f9b4 = DAT_00a08108;
            DAT_0054f9b8 = DAT_00a08102;
            let u_var1: u32 = (DAT_009a20ae as u32).wrapping_add(DAT_00a08102 as i32 as u32);
            if (DAT_00a08108 as i32) < (DAT_009a6fe0 as i32) {
                DAT_0054f9b4 = DAT_009a6fe0;
            }
            if (DAT_00a08102 as i32) < (DAT_00a0813c as i32) {
                DAT_0054f9b8 = DAT_00a0813c;
            }
            DAT_0054f9a8 = u_var1 as u16 as i16;
            if (DAT_00a07ce4 as i32) < ((u_var1 & 0xffff) as i32) {
                DAT_0054f9a8 = DAT_00a07ce4;
            }
            if (DAT_0054f9b8 as u16) < (DAT_0054f9a8 as u16) {
                FUN_0040fa30(0);
            }
        }
    }
}
