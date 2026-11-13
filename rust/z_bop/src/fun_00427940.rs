// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07c68: i32;
    static mut DAT_00a07c6c: i32;
    static mut DAT_00a07cc4: i32;
    static mut DAT_00a004d0: i32;
    static mut DAT_00a004ac: i32;
    static mut DAT_00a004b4: i32;
    static mut DAT_00a0049c: i32;
    static mut DAT_00a0048c: i32;
    static mut DAT_00a00494: i32;
    static mut DAT_00a004a8: i32;
    static mut DAT_00a004cc: i32;
    static mut DAT_00a00488: i32;
    static mut DAT_00a004a4: i32;
    static mut DAT_00a004b8: i32;
    static mut DAT_00a00480: i32;
    static mut DAT_00a004b0: i32;
    static mut DAT_009fe7f4: i32;
    static mut DAT_00a00490: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427bd0(param_1: i16, param_2: i16, param_3: i32);
    fn FUN_00427d30(param_1: i16, param_2: i16, param_3: i32);
    fn FUN_004290b0(param_1: i32, param_2: i32, param_3: i32, param_4: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00427940(
    param_1: i16,
    param_2: i16,
    param_3: i16,
    param_4: i16,
    param_5: i16,
    param_6: i16,
    param_7: i32,
) {
    // SAFETY: the callees and the globals are the game's own memory; the
    // reads, writes and calls happen in the original's order.
    unsafe {
        FUN_00427bd0(param_2, param_3, param_7);
        FUN_00427d30(param_1, param_4, param_7);
        DAT_00a07c68 = DAT_00a07c68.wrapping_neg();
        DAT_00a07c6c = DAT_00a07c6c.wrapping_neg();
        DAT_00a07cc4 = DAT_00a07cc4.wrapping_neg();
        let i_var1 = DAT_00a004d0.wrapping_neg();
        let i_var2 = DAT_00a004ac.wrapping_neg();
        let i_var3 = DAT_00a004b4.wrapping_neg();
        DAT_00a004ac = i_var2;
        DAT_00a004b4 = i_var3;
        DAT_00a004d0 = i_var1;
        FUN_004290b0(DAT_00a0049c, DAT_00a0048c, DAT_00a00494, param_5);
        FUN_004290b0(i_var1, i_var2, i_var3, param_6);
        DAT_00a004a8 = DAT_00a0049c;
        DAT_00a004cc = DAT_00a0048c;
        DAT_00a00488 = DAT_00a00494;
        DAT_00a004a4 = DAT_00a004d0;
        DAT_00a004b8 = DAT_00a004ac;
        DAT_00a00480 = DAT_00a004b4;
        DAT_00a004b0 = DAT_00a07c68;
        DAT_009fe7f4 = DAT_00a07c6c;
        DAT_00a00490 = DAT_00a07cc4;
    }
}
