// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052ae28: i32;
    static mut DAT_0052aef8: i32;
    static mut DAT_0052ae30: i32;
    static mut DAT_0071f2eb: i32;
    static mut DAT_00658020: *mut i32;
    static mut DAT_0052ae54: i32;
    static mut DAT_006572c0: *mut i32;
    static mut DAT_0052ae50: i32;
    static mut DAT_0052ae2c: i32;
    static mut DAT_00657c00: *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a6f90() {
    // SAFETY: the globals are the game's own memory; the object pointers
    // are the game's, and the call targets are computed exactly as the
    // original does (vtable pointer plus an int-scaled offset).
    unsafe {
        if DAT_0052ae28 == 1 {
            return;
        }
        if 1 < DAT_0052aef8 {
            DAT_0052aef8 = DAT_0052aef8.wrapping_sub(1);
            return;
        }
        if DAT_0052aef8 < 1 {
            DAT_0052aef8 = 0;
            return;
        }
        DAT_0052aef8 = DAT_0052aef8.wrapping_sub(1);
        let mut p_var1: *mut i32 = DAT_00657c00;
        if DAT_0052ae30 != 0 {
            if DAT_0071f2eb != 0 {
                p_var1 = DAT_00658020;
                if DAT_0052ae54 != 0 {
                    p_var1 = DAT_006572c0;
                }
            }
        }
        let target = (p_var1 as *const *const i32)
            .read_unaligned()
            .wrapping_add(0x80) as *const core::ffi::c_void;
        let f: unsafe extern "stdcall" fn(*mut i32, i32) = core::mem::transmute(target);
        f(p_var1, DAT_0052ae50);
        if DAT_0052ae2c == 0 && !DAT_00657c00.is_null() {
            let target2 = (DAT_00657c00 as *const *const i32)
                .read_unaligned()
                .wrapping_add(8) as *const core::ffi::c_void;
            let g: unsafe extern "stdcall" fn(*mut i32) = core::mem::transmute(target2);
            g(DAT_00657c00);
            DAT_00657c00 = core::ptr::null_mut();
        }
    }
}
