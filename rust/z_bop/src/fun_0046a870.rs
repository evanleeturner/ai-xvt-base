// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07c5c: i16;
    static mut DAT_009d6824: i32;
    static mut DAT_009d12b0: i16;
    static mut DAT_009a20b0: i32;
    static mut DAT_009a8e14: i32;
    static mut DAT_009cd26c: i32;
    static mut DAT_009e9648: i32;
    static mut DAT_009fd438: i32;
    static mut DAT_00a00520: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046a7c0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046a870(param_1: u32, param_2: i16, param_3: u16) {
    // SAFETY: the globals and the callee are the game's own memory and code.
    unsafe {
        DAT_00a07c5c = 0x4000i32.wrapping_sub(param_2 as i32) as i16;
        DAT_009d6824 = (param_1 & 0xffff) as i32;
        DAT_009d12b0 = param_3 as i16;
        FUN_0046a7c0();
        DAT_009a20b0 = DAT_009a8e14;
        DAT_009cd26c = DAT_009e9648;
        DAT_009fd438 = DAT_00a00520;
    }
}
