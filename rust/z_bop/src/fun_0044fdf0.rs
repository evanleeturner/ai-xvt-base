// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005235f0: u32;
    static mut DAT_005233b8: u32;
    static mut DAT_009d1308: u32;
    static mut DAT_00a07ce8: u32;
    static mut DAT_009a1fe8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044f880(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044fdf0(param_1: u32) {
    // SAFETY: the globals and the callee are the game's own memory and code.
    unsafe {
        DAT_005235f0 = 0;
        if DAT_005233b8 != 0 && param_1 < DAT_009d1308.wrapping_add(DAT_00a07ce8) {
            FUN_0044f880(param_1.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8));
        }
    }
}
