// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08374: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00440840(param_1: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0043c890(param_1: i32) {
    let mut u_var1: u16 = ((param_1.wrapping_mul(0xa1) as u32) >> 16) as u16;
    if 9999 < u_var1 {
        u_var1 = 9999;
    }
    // SAFETY: the global and the callee are the game's own; arguments match
    // the C prototype.
    unsafe {
        FUN_00440840(i32::from(DAT_00a08374) + 0x53, i32::from(u_var1 / 100), 1);
        FUN_00440840(i32::from(DAT_00a08374) + 0x54, i32::from(u_var1 % 100), 2);
    }
}
