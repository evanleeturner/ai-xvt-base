// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233ec: i32;
    static mut DAT_009ec464: i32;
    static mut DAT_009a7bac: i32;
    static mut DAT_009a8c14: i32;
    static mut DAT_009d8b74: u16;
    static mut DAT_009a6fe4: *const core::ffi::c_void;
    static mut DAT_009d6952: u16;
    static mut DAT_009ecc34: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a95c0(param_1: u32);
    fn FUN_004a9590(param_1: u32);
    fn FUN_004a9630(param_1: u16);
    fn FUN_004a9640(param_1: u8);
    fn FUN_004a9520(param_1: i32, param_2: i16, param_3: i16, param_4: i16);
    fn FUN_004a9500(param_1: u16, param_2: i16);
    fn FUN_004277f0(param_1: u16, param_2: u32, param_3: u32);
    fn FUN_00438df0(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004aba50();
    fn FUN_004abe50();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042cc40() {
    // SAFETY: the globals and callees are the game's own; the function
    // pointer in DAT_009a6fe4 is the game's callback taking one int.
    unsafe {
        let s_var3: i16;
        let u_var1: u16;
        let u_var2: u16;
        if DAT_005233ec == 0x13 {
            s_var3 = 0xbe;
            u_var1 = 200;
            u_var2 = 0xff;
        } else if DAT_005233ec == 0x101 {
            s_var3 = 0x1c8;
            u_var1 = 0x168;
            u_var2 = 0x1d1;
        } else {
            s_var3 = 0xbe;
            u_var1 = 200;
            u_var2 = 0xff;
        }
        DAT_009ec464 = 1;
        FUN_004a95c0(0x2c);
        FUN_004a9590(0x43);
        FUN_004a9630(0);
        FUN_004a9640(1);
        FUN_004a9520(0, s_var3, DAT_009a7bac as i16, DAT_009a8c14 as i16);
        FUN_004a9500(u_var1, s_var3);
        FUN_004277f0((DAT_009d8b74 as u8) as u16, 2, 2);
        let f: unsafe extern "C" fn(i32) = core::mem::transmute(DAT_009a6fe4);
        f(0x3a);
        FUN_004277f0(DAT_009d8b74 >> 8, 2, 2);
        FUN_004a9500(u_var2, s_var3);
        FUN_004277f0(DAT_009d6952, 5, 5);
        FUN_004aba50();
        FUN_00438df0(DAT_009ecc34);
        FUN_004abe50();
    }
}
