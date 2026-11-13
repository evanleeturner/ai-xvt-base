// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00650406: i16;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004f3500();
    fn FUN_004f3680();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f23b0() {
    // SAFETY: the global and the callees are the game's own; they are read
    // and called in the original's order.
    unsafe {
        if DAT_00650406 != 0 {
            FUN_004f3500();
            FUN_004f3680();
        }
    }
}
