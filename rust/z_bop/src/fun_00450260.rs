// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005235dc: i32;
    static mut DAT_005233ec: i32;
    static mut DAT_009a7ec0: i32;
    static mut DAT_009a7390: i32;
    static mut DAT_009a8d98: i32;
    static mut DAT_009d1148: i32;
    static mut DAT_009a7b44: i32;
    static mut DAT_009d12f8: i32;
    static mut DAT_009a8d94: i32;
    static mut DAT_009ec470: i32;
    static mut DAT_009ecc38: i32;
    static mut DAT_009d12f4: i32;
    static mut DAT_009d6934: i32;
    static mut DAT_009a806c: i32;
    static mut DAT_009a7bac: i32;
    static mut DAT_009a8c14: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_005233c8: i32;
    static mut DAT_009d1300: *const core::ffi::c_void;
    static mut DAT_00556aa4: i32;
    static mut DAT_009ecc3c: i32;
    static mut DAT_009d77f8: i32;
    static mut DAT_009d7694: i32;
    static mut _DAT_009d7686: u16;
    static mut DAT_009a6ff0: i32;
    static mut DAT_005569f8: i32;
    static mut DAT_00556a50: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e0c0(param_1: i32, param_2: i32, param_3: u32, param_4: i32);
    fn FUN_004a95c0(param_1: u32);
    fn FUN_004a9520(param_1: i16, param_2: i16, param_3: i16, param_4: i16);
    fn FUN_004aba50();
    fn FUN_004a9620(param_1: *mut core::ffi::c_void);
    fn FUN_004a9630(param_1: u16);
    fn FUN_004a9640(param_1: u8);
    fn FUN_004515d0();
    fn FUN_004a9590(param_1: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004abe50();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00450260(param_1: i32) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the 16-bit writes target the low halves of the game's int globals,
    // as the original memcpy does.
    unsafe {
        if DAT_005235dc == -1 {
            if DAT_005233ec == 0x13 {
                DAT_005235dc = 0x2a;
                DAT_009a7ec0 = 0x115;
                DAT_009a7390 = 0x2a;
                DAT_009a8d98 = 0x115;
                DAT_009d1148 = 3;
                DAT_009a7b44 = 0x1a;
                DAT_009d12f8 = 0x20;
                DAT_009a8d94 = 0x25;
                DAT_009ec470 = 0x4b;
                DAT_009ecc38 = 0x2c;
                DAT_009d12f4 = 0x109;
                DAT_009d6934 = 0x31;
            } else if DAT_005233ec == 0x101 {
                DAT_009d1148 = 6;
                DAT_009a7ec0 = 0x1f9;
                DAT_009a7b44 = 0x35;
                DAT_009d12f8 = 0x40;
                DAT_009a8d98 = 0x22b;
                DAT_009a8d94 = 0x4b;
                DAT_009ec470 = 0x96;
                DAT_009ecc38 = 0x59;
                DAT_009d12f4 = 0x212;
                DAT_009d6934 = 100;
                DAT_005235dc = 0x55;
                DAT_009a7390 = 0x55;
            } else if DAT_005233ec == 0x112 {
                DAT_009d1148 = 4;
                DAT_009a7ec0 = 0x17a;
                DAT_009a7b44 = 0x27;
                DAT_009d12f8 = 0x30;
                DAT_009a8d98 = 0x19f;
                DAT_009a8d94 = 0x38;
                DAT_009ec470 = 0x70;
                DAT_009ecc38 = 0x42;
                DAT_009d12f4 = 0x18d;
                DAT_009d6934 = 0x4a;
                DAT_005235dc = 0x3f;
                DAT_009a7390 = 0x3f;
            }
            FUN_0040e0c0(
                DAT_009a806c,
                DAT_009a7bac,
                DAT_009a8c14 as u32,
                DAT_005233d8.wrapping_mul(DAT_009a7bac),
            );
            FUN_004a95c0(DAT_005233c8 as u32);
            FUN_004a9520(
                DAT_005235dc as i16,
                DAT_009d1148 as i16,
                DAT_009a7ec0 as i16,
                DAT_009a7b44 as i16,
            );
            let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
            f();
            FUN_004a9520(
                DAT_009a7390 as i16,
                DAT_009d12f8 as i16,
                DAT_009a8d98 as i16,
                DAT_009a8d94 as i16,
            );
            let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
            f();
            FUN_004a9520(
                DAT_009ec470 as i16,
                DAT_009ecc38 as i16,
                DAT_009d12f4 as i16,
                DAT_009d6934 as i16,
            );
            let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
            f();
        } else if param_1 != 0 {
            DAT_00556aa4 = DAT_00556aa4.wrapping_add(1);
        }
        FUN_004aba50();
        FUN_0040e0c0(0, 0x140, 200, 0);
        FUN_004a9620(core::ptr::null_mut());
        FUN_004a9630(0);
        FUN_004a9640(1);
        FUN_004515d0();
        FUN_004a9590(0x43);
        DAT_009ecc3c = 0;
        DAT_009d77f8 = 0;
        DAT_009d7694 = 0x14b;
        let tmp = DAT_009a6ff0 as u16;
        _DAT_009d7686 = tmp;
        (&raw mut DAT_009a6ff0)
            .cast::<u16>()
            .write_unaligned(0xffff);
        (&raw mut DAT_005569f8)
            .cast::<u16>()
            .write_unaligned(0xffff);
        (&raw mut DAT_00556a50)
            .cast::<u16>()
            .write_unaligned(0xffff);
        FUN_004abe50();
    }
}
