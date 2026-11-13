// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523410: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009d80c9: i8;
    static mut DAT_00bb7372: i8;
    static mut DAT_00bb737b: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9300(param_1: i32, param_2: i32) -> u32;
    fn FUN_004a9380(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042e460(param_1: i32) {
    // SAFETY: the globals and callees are the game's own memory and functions,
    // accessed in the same order as the original.
    unsafe {
        if DAT_00523410 == 0
            && param_1 == DAT_009ecc34
            && DAT_009d80c9 != 0
            && DAT_00bb7372 != 0
            && DAT_00bb737b != 0
        {
            let u_var1 = FUN_004a9300(0x51, 0x100);
            if u_var1 != 0 {
                FUN_004a9380(0x51);
            }
            let u_var1 = FUN_004a9300(0x54, 0x100);
            if u_var1 != 0 {
                FUN_004a9380(0x54);
            }
        }
    }
}
