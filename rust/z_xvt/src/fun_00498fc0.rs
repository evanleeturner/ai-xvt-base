// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00645ad4: i16;
    static mut DAT_00645a80: u32;
    static mut DAT_00645a8c: i16;
    static mut DAT_00645778: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00499c60() -> u32;
    fn FUN_0048f3c0(param_1: u32, param_2: u32) -> u32;
    fn FUN_0047f010(param_1: i32);
    fn FUN_0048f4f0(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00498fc0() {
    // SAFETY: the callees and globals are the game's own memory and code,
    // called and accessed in the original's order.
    unsafe {
        let mut u_var1 = FUN_00499c60();
        let u_var2 = FUN_0048f3c0(u_var1, 0);
        DAT_00645ad4 = u_var2 as i16;
        if DAT_00645ad4 == 0 {
            FUN_0047f010(0);
        }
        DAT_00645a80 = FUN_0048f4f0(u_var2);
        u_var1 = FUN_0048f3c0(u_var1, 0);
        DAT_00645a8c = u_var1 as i16;
        if DAT_00645a8c == 0 {
            FUN_0047f010(0);
        }
        DAT_00645778 = FUN_0048f4f0(u_var1);
    }
}
