// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051a520: u32;
    static mut DAT_009a8062: u16;
    static mut DAT_0054f994: u32;
    static mut DAT_0054f9a0: u32;
    static mut DAT_0053f970: u32;
    static mut DAT_0054f988: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004012c0(param_1: u32);
    fn FUN_004b1d10(param_1: u32, param_2: u32);
    fn FUN_004b1de0(param_1: u32, param_2: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00482be0();
    fn FUN_00408140();
    fn FUN_004b1c10();
    fn FUN_004b1c90();
    fn FUN_004b1d70();
    fn FUN_004b2020();
    fn FUN_004b1c50();
    fn FUN_00408110();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0040b2c0() {
    // SAFETY: the callees and globals are the game's own memory; calls and
    // accesses happen in the original's order.
    unsafe {
        if DAT_0051a520 == 0 {
            FUN_004012c0(0);
        } else {
            FUN_004012c0(1);
            FUN_00482be0();
        }
        DAT_009a8062 = 0;
        if DAT_0054f994 != 0 && DAT_0054f9a0 != 0 {
            FUN_00408140();
            FUN_004b1c10();
            FUN_004b1c90();
            FUN_004b1d10(DAT_0053f970, DAT_0054f994);
            FUN_004b1d70();
            FUN_004b1de0(DAT_0054f988, DAT_0054f9a0);
            FUN_004b2020();
            FUN_004b1c50();
            FUN_00408110();
        }
    }
}
