// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a7230();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0048f580() -> u32 {
    // SAFETY: the callee is the game's own function and takes no arguments.
    unsafe {
        FUN_004a7230();
    }
    1
}
