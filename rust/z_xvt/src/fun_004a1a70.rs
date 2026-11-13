// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052571c: i32;
    static mut DAT_0052552c: i32;
    static mut DAT_006da840: i32;
    static mut DAT_006d9d30: i32;
    static mut DAT_006db714: i32;
    static mut DAT_00703ad0: i32;
    static mut DAT_006da4e4: i32;
    static mut DAT_00703c78: i32;
    static mut DAT_006db710: i32;
    static mut DAT_0071e530: i32;
    static mut DAT_0071ecf8: i32;
    static mut DAT_00703c74: i32;
    static mut DAT_007092b4: i32;
    static mut DAT_006da9ec: i32;
    static mut DAT_006da538: i32;
    static mut DAT_006db594: i32;
    static mut DAT_00525518: i32;
    static mut DAT_00525508: i32;
    static mut DAT_00703c80: *const core::ffi::c_void;
    static mut DAT_00645b8c: i32;
    static mut DAT_0070a178: i32;
    static mut DAT_0070a014: i32;
    static mut _DAT_0070a006: i32;
    static mut DAT_006d9990: i32;
    static mut DAT_00645ae0: i32;
    static mut DAT_00645b38: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00461f20(a: i32, b: i32, c: i32, d: i32);
    fn FUN_0048eda0(a: u32);
    fn FUN_0048ed00(a: i16, b: i16, c: i16, d: i16);
    fn FUN_004a6a80();
    fn FUN_0048ee00(a: i32);
    fn FUN_0048ee10(a: i32);
    fn FUN_0048ee20(a: i32);
    fn FUN_004a2e70();
    fn FUN_0048ed70(a: u32);
    fn FUN_004a6f90();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a1a70(param_1: i32) {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the function pointer in DAT_00703c80 is the game's callback taking no arguments.
    unsafe {
        if DAT_0052571c == -1 {
            if DAT_0052552c == 0x13 {
                DAT_0052571c = 0x2a;
                DAT_006da840 = 0x115;
                DAT_006d9d30 = 0x2a;
                DAT_006db714 = 0x115;
                DAT_00703ad0 = 3;
                DAT_006da4e4 = 0x1a;
                DAT_00703c78 = 0x20;
                DAT_006db710 = 0x25;
                DAT_0071e530 = 0x4b;
                DAT_0071ecf8 = 0x2c;
                DAT_00703c74 = 0x109;
                DAT_007092b4 = 0x31;
            } else if DAT_0052552c == 0x101 {
                DAT_00703ad0 = 6;
                DAT_006da840 = 0x1f9;
                DAT_006da4e4 = 0x35;
                DAT_00703c78 = 0x40;
                DAT_006db714 = 0x22b;
                DAT_006db710 = 0x4b;
                DAT_0071e530 = 0x96;
                DAT_0071ecf8 = 0x59;
                DAT_00703c74 = 0x212;
                DAT_007092b4 = 100;
                DAT_0052571c = 0x55;
                DAT_006d9d30 = 0x55;
            } else if DAT_0052552c == 0x112 {
                DAT_00703ad0 = 4;
                DAT_006da840 = 0x17a;
                DAT_006da4e4 = 0x27;
                DAT_00703c78 = 0x30;
                DAT_006db714 = 0x19f;
                DAT_006db710 = 0x38;
                DAT_0071e530 = 0x70;
                DAT_0071ecf8 = 0x42;
                DAT_00703c74 = 0x18d;
                DAT_007092b4 = 0x4a;
                DAT_0052571c = 0x3f;
                DAT_006d9d30 = 0x3f;
            }
            FUN_00461f20(
                DAT_006da9ec,
                DAT_006da538,
                DAT_006db594,
                DAT_00525518.wrapping_mul(DAT_006da538),
            );
            FUN_0048eda0(DAT_00525508 as u32);
            FUN_0048ed00(
                DAT_0052571c as i16,
                DAT_00703ad0 as i16,
                DAT_006da840 as i16,
                DAT_006da4e4 as i16,
            );
            core::mem::transmute::<*const core::ffi::c_void, unsafe extern "C" fn()>(DAT_00703c80)(
            );
            FUN_0048ed00(
                DAT_006d9d30 as i16,
                DAT_00703c78 as i16,
                DAT_006db714 as i16,
                DAT_006db710 as i16,
            );
            core::mem::transmute::<*const core::ffi::c_void, unsafe extern "C" fn()>(DAT_00703c80)(
            );
            FUN_0048ed00(
                DAT_0071e530 as i16,
                DAT_0071ecf8 as i16,
                DAT_00703c74 as i16,
                DAT_007092b4 as i16,
            );
            core::mem::transmute::<*const core::ffi::c_void, unsafe extern "C" fn()>(DAT_00703c80)(
            );
        } else if param_1 != 0 {
            DAT_00645b8c = DAT_00645b8c.wrapping_add(1);
        }
        FUN_004a6a80();
        FUN_00461f20(0, 0x140, 200, 0);
        FUN_0048ee00(0);
        FUN_0048ee10(0);
        FUN_0048ee20(1);
        FUN_004a2e70();
        FUN_0048ed70(0x43);
        DAT_0070a178 = 0;
        DAT_0070a014 = 0x141;
        (&raw mut _DAT_0070a006).write_unaligned(DAT_006d9990);
        DAT_006d9990 = 0xffff;
        DAT_00645ae0 = 0xffff;
        DAT_00645b38 = 0xffff;
        FUN_004a6f90();
    }
}
