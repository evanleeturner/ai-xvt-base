// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004f2410() -> u32;
    fn FUN_004f25e0() -> u32;
    fn FUN_004f2c10() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f22c0() -> u32 {
    // SAFETY: the callees are the game's own functions, called in the original order.
    unsafe {
        FUN_004f2410();
        FUN_004f25e0();
        FUN_004f2c10();
    }
    1
}
