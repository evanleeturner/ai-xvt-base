// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005f0f00: i8;
    static mut DAT_005f0e24: i32;
    static mut DAT_005f09c4: i32;
    static mut DAT_005f0e28: i32;
    static mut DAT_005f09bc: i32;
    static mut DAT_005f0e2c: i32;
    static mut DAT_005f09c0: i32;
    static mut DAT_005f0e32: i16;
    static mut DAT_005f0920: i16;
    static mut DAT_005f0e30: i16;
    static mut DAT_005f0924: i16;
    static mut _DAT_005254e0: i32;
    static mut DAT_005f0e10: i32;
    static mut DAT_00703c6c: i32;
    static mut DAT_005f09d4: i16;
    static mut DAT_005f0e34: i16;
    static mut DAT_005f0e48: i16;
    static mut DAT_00703c70: i32;
    static mut DAT_005f09d0: i16;
    static mut DAT_00525e08: i16;
    static mut DAT_005f0e44: i16;
    static mut DAT_00703c84: i32;
    static mut DAT_005f09b8: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004526c0(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00453b70() {
    // SAFETY: the callee and the globals are the game's own memory; each is
    // read and written by value in the original's order.
    unsafe {
        FUN_004526c0(&raw mut DAT_005f0f00);
        DAT_005f0e24 = DAT_005f09c4;
        DAT_005f0e28 = DAT_005f09bc;
        DAT_005f0e2c = DAT_005f09c0;
        DAT_005f0e32 = DAT_005f0920;
        DAT_005f0e30 = DAT_005f0924;
        _DAT_005254e0 = DAT_005f0e10;
        DAT_00703c6c = DAT_005f09d4 as i32;
        DAT_005f0e34 = DAT_005f0e48;
        DAT_00703c70 = DAT_005f09d0 as i32;
        DAT_00525e08 = DAT_005f0e44;
        DAT_00703c84 = DAT_005f09b8 as i32;
    }
}
