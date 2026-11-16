// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08108: i16;
    static mut DAT_009d8c00: i16;
    static mut DAT_0055698c: i16;
    static mut DAT_00556990: i16;
    static mut DAT_009a20ae: u8;
    static mut DAT_00a08102: u16;
    static mut DAT_009a6fe0: i16;
    static mut DAT_00556994: i16;
    static mut DAT_00a0813c: i16;
    static mut DAT_00556988: u16;
    static mut DAT_00a07ce4: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044a5b0(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0044aa50() {
    // SAFETY: the globals and the callee are the game's own memory and code,
    // read, written and called in the original's order.
    unsafe {
        if DAT_00a08108 < DAT_009d8c00 {
            DAT_0055698c = DAT_009d8c00;
            DAT_00556990 = DAT_00a08108;
            let u_var1: u32 = (DAT_009a20ae as u32).wrapping_add(DAT_00a08102 as u32);
            if (DAT_00a08108 as i32) < (DAT_009a6fe0 as i32) {
                DAT_00556990 = DAT_009a6fe0;
            }
            DAT_00556994 = DAT_00a08102 as i16;
            if (DAT_00a08102 as i32) < (DAT_00a0813c as i32) {
                DAT_00556994 = DAT_00a0813c;
            }
            DAT_00556988 = u_var1 as u16;
            if (DAT_00a07ce4 as i32) < ((u_var1 & 0xffff) as i32) {
                DAT_00556988 = DAT_00a07ce4 as u16;
            }
            if (DAT_00556994 as i32) < (DAT_00556988 as i32) {
                FUN_0044a5b0(0);
            }
        }
    }
}
