// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a00520: i32;
    static mut DAT_009a8e14: i32;
    static mut DAT_009d12b0: i16;
    static mut DAT_009d6824: i32;
    static mut DAT_009e9648: i32;
    static mut DAT_00a07c5c: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046a650(param_1: i32, param_2: i16) -> i32;
    fn FUN_0046a760(param_1: i32, param_2: i16) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0046a7c0() {
    // SAFETY: the callees and globals are the game's own memory; reads and
    // writes occur in the same order as the original.
    unsafe {
        DAT_00a00520 = FUN_0046a650(DAT_009d6824, DAT_009d12b0);
        DAT_009a8e14 = FUN_0046a760(DAT_00a00520, DAT_00a07c5c);
        DAT_009e9648 = FUN_0046a650(DAT_00a00520, DAT_00a07c5c);
        DAT_00a00520 = FUN_0046a760(DAT_009d6824, DAT_009d12b0);
    }
}
