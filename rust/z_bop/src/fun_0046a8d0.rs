// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ecc46: u16;
    static mut DAT_009ecc48: u16;
    static mut DAT_009ecc50: u16;
    static mut DAT_009a8e14: u32;
    static mut DAT_009e9648: u32;
    static mut DAT_00a00520: u32;
    static mut DAT_00a080fe: i16;
    static mut DAT_009a8070: i16;
    static mut DAT_009a8c00: u32;
    static mut DAT_00a080f0: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046aa70(param_1: u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046a8d0(param_1: u32, param_2: u32, param_3: u32) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    // SAFETY: the globals and the callee are the game's own memory and code,
    // accessed in the same order as the original.
    unsafe {
        let mut b_var2 = (param_1 as i32) < 0;
        if b_var2 {
            param_1 = param_1.wrapping_neg();
        }
        DAT_009ecc46 = b_var2 as u16;
        b_var2 = (param_2 as i32) < 0;
        if b_var2 {
            param_2 = param_2.wrapping_neg();
        }
        DAT_009ecc48 = b_var2 as u16;
        b_var2 = (param_3 as i32) < 0;
        if b_var2 {
            param_3 = param_3.wrapping_neg();
        }
        DAT_009ecc50 = b_var2 as u16;
        DAT_009a8e14 = param_1;
        DAT_009e9648 = param_2;
        DAT_00a00520 = param_3;
        FUN_0046aa70(param_1, param_2);
        let mut s_var1 = DAT_00a080fe;
        if DAT_009ecc48 != 0 {
            s_var1 = s_var1.wrapping_neg();
        }
        if DAT_009ecc46 != 0 {
            s_var1 = (-0x8000i16).wrapping_sub(s_var1);
        }
        DAT_009a8070 = 0x4000i16.wrapping_sub(s_var1);
        FUN_0046aa70(DAT_009a8c00, DAT_00a00520);
        s_var1 = DAT_00a080fe;
        if DAT_009ecc50 != 0 {
            s_var1 = s_var1.wrapping_neg();
        }
        DAT_00a080f0 = 0x4000i16.wrapping_sub(s_var1);
    }
}
