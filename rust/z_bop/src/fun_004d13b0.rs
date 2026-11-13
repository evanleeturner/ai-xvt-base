// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00acebc5: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d1280(param_1: i32);
    fn FUN_004d48e0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d13b0(param_1: i32) {
    // SAFETY: the globals and callees are the game's own; the order of reads
    // and calls follows the original.
    unsafe {
        let c_var1 = DAT_00aa7b0d;
        if DAT_00acebc5 != 0 {
            FUN_004d4970();
            FUN_004d1280(param_1);
            if c_var1 != 0 {
                DAT_00aa6cfc = FUN_004d48e0();
            }
        }
    }
}
