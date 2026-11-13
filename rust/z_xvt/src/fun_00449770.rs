// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0048f510();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00449770() {
    // SAFETY: the callee is the game's own function; it takes no arguments.
    unsafe {
        FUN_0048f510();
    }
}
