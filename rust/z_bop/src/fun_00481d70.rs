// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ec5fe: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425ae0(param_1: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00481d70(param_1: i16) {
    // SAFETY: the callee and the global are the game's own; the call is made
    // with the same arguments, in the same order, as the original.
    unsafe {
        FUN_00425ae0(param_1 as i32, u32::from(DAT_009ec5fe) as i32, 0xec);
    }
}
