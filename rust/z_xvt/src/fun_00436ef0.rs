// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0048f800();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00436ef0() {
    // SAFETY: the callee is the game's own function and takes no arguments.
    unsafe {
        FUN_0048f800();
    }
}
