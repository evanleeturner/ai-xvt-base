// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004c04d0(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004bacf0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004c9680() -> u32 {
    // SAFETY: both callees are the game's own functions, called in the
    // original's order with the original's arguments.
    unsafe {
        FUN_004c04d0(0);
        FUN_004bacf0();
    }
    1
}
