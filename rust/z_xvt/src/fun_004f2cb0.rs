// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0065047a: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004f2c10();
    fn FUN_004f2e40(param_2: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f2cb0(param_1: i16, param_2: i16) -> u32 {
    // SAFETY: the globals and callees are the game's own; they are read,
    // written and called in the same order as the original.
    unsafe {
        if (DAT_0065047a as i32).wrapping_sub(param_1 as i32) == 1 {
            return 0;
        }
        if DAT_0065047a > param_1 {
            FUN_004f2c10();
        }
        while DAT_0065047a <= param_1 {
            DAT_0065047a = DAT_0065047a.wrapping_add(1);
            FUN_004f2e40(param_2);
        }
        ((DAT_0065047a as i32 as u32).wrapping_shl(16)) | 1
    }
}
