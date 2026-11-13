// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b2e: i32;
    static mut DAT_00aa7b3a: *mut i32;
    static mut DAT_00aa7b13: i32;
    static mut DAT_00aa7b17: i32;
    static mut DAT_00aa7b0d: i8;
    static mut _DAT_00aa7aa1: i32;
    static mut DAT_00aa7ac5: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d48e0() -> i32 {
    // SAFETY: the globals and the object they point to belong to the game;
    // the call targets are computed exactly as the original computes them.
    unsafe {
        if DAT_00aa7b2e == 0 {
            return 0;
        }
        if DAT_00aa7b3a.is_null() {
            return 0;
        }
        DAT_00aa7b13 = DAT_00aa7b17;
        if DAT_00aa7b0d != 0 {
            return DAT_00aa7ac5;
        }
        _DAT_00aa7aa1 = 0x6c;
        loop {
            let mut i_var1: i32;
            loop {
                let target = DAT_00aa7b3a.read().wrapping_add(100) as *const core::ffi::c_void;
                let f: unsafe extern "stdcall" fn(
                    *mut core::ffi::c_void,
                    i32,
                    *mut i32,
                    i32,
                    i32,
                ) -> i32 = core::mem::transmute(target);
                i_var1 = f(
                    DAT_00aa7b3a as *mut core::ffi::c_void,
                    0,
                    &raw mut _DAT_00aa7aa1,
                    0,
                    0,
                );
                if i_var1 != -0x7789fe3e {
                    break;
                }
                let target2 = DAT_00aa7b3a.read().wrapping_add(0x6c) as *const core::ffi::c_void;
                let g: unsafe extern "stdcall" fn(*mut core::ffi::c_void) =
                    core::mem::transmute(target2);
                g(DAT_00aa7b3a as *mut core::ffi::c_void);
            }
            if !(i_var1 == -0x7789fde4 || i_var1 == -0x7789fe52 || i_var1 == -0x7789fe48) {
                break;
            }
        }
        DAT_00aa7b0d = 1;
        DAT_00aa7ac5
    }
}
