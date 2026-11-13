// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005373d0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00432ca0() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00431110(param_1: i32, param_2: *mut u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042c5a0() {
    // SAFETY: the global and callees are the game's own; the global's address
    // is passed on as the original does.
    unsafe {
        _DAT_005373d0 = 0x1d;
        let u_var3: u32 = 4;
        let pu_var2: *mut u32 = &raw mut _DAT_005373d0;
        let i_var1 = FUN_00432ca0();
        FUN_00431110(i_var1, pu_var2, u_var3);
    }
}
