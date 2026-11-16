// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_00557360: i32;
    static mut DAT_00557364: i32;
    static mut DAT_009a8c2c: i32;
    static mut DAT_0052340c: i32;
    static mut DAT_00556ed0: i32;
    static mut DAT_00557368: i32;
    static mut _DAT_009ed23c: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0046f700() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046d350(param_1: i32, param_2: *mut u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00463b60() {
    // SAFETY: the globals and callees are the game's own; the pointer passed
    // is the address of the game's global block, as in the original.
    unsafe {
        _DAT_00557360 = 0x1e;
        DAT_00557364 = DAT_009a8c2c.wrapping_add(DAT_0052340c);
        let u_var3: u32 = 0xc;
        let pu_var2 = &raw mut _DAT_00557360 as *mut u32;
        DAT_00556ed0 = DAT_009a8c2c.wrapping_add(DAT_0052340c);
        DAT_00557368 = _DAT_009ed23c;
        let i_var1 = FUN_0046f700();
        FUN_0046d350(i_var1, pu_var2, u_var3);
    }
}
