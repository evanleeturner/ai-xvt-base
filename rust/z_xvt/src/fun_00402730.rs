// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052552c: i32;
    static mut DAT_0071e524: i8;
    static mut DAT_006da538: i32;
    static mut DAT_006db594: i32;
    static mut DAT_0070ac34: u8;
    static mut DAT_006d9984: *const core::ffi::c_void;
    static mut DAT_0070ac35: u8;
    static mut DAT_007092d2: u16;
    static mut DAT_0071ecf4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048eda0(param_1: u32);
    fn FUN_0048ed70(param_1: u32);
    fn FUN_0048ee10(param_1: i32);
    fn FUN_0048ee20(param_1: i32);
    fn FUN_0048ed00(param_1: i32, param_2: i16, param_3: i16, param_4: i16);
    fn FUN_0048ece0(param_1: u16, param_2: i16);
    fn FUN_0040ccc0(param_1: u16, param_2: i32, param_3: i32);
    fn FUN_004a6a80();
    fn FUN_004a80d0(param_1: i32);
    fn FUN_004a6f90();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00402730() {
    // SAFETY: the globals and callees are the game's own; the function
    // pointer in DAT_006d9984 is the game's cdecl callback taking one u32.
    unsafe {
        let s_var3: i16;
        let u_var1: u16;
        let u_var2: u16;
        if DAT_0052552c == 0x13 {
            s_var3 = 0xbe;
            u_var1 = 200;
            u_var2 = 0xff;
        } else if DAT_0052552c == 0x101 {
            s_var3 = 0x1c8;
            u_var1 = 0x168;
            u_var2 = 0x1d1;
        } else {
            s_var3 = 0xbe;
            u_var1 = 200;
            u_var2 = 0xff;
        }
        DAT_0071e524 = 1;
        FUN_0048eda0(0x2c);
        FUN_0048ed70(0x43);
        FUN_0048ee10(0);
        FUN_0048ee20(1);
        FUN_0048ed00(0, s_var3, DAT_006da538 as i16, DAT_006db594 as i16);
        FUN_0048ece0(u_var1, s_var3);
        FUN_0040ccc0(DAT_0070ac34 as u16, 2, 2);
        let cb: unsafe extern "C" fn(u32) = core::mem::transmute(DAT_006d9984);
        cb(0x3a);
        FUN_0040ccc0(DAT_0070ac35 as u16, 2, 2);
        FUN_0048ece0(u_var2, s_var3);
        FUN_0040ccc0(DAT_007092d2, 5, 5);
        FUN_004a6a80();
        FUN_004a80d0(DAT_0071ecf4);
        FUN_004a6f90();
    }
}
