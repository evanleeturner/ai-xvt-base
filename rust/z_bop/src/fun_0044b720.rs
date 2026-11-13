// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005569d4: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044b790(a: i32, b: *mut i8, c: i32, d: *mut *mut core::ffi::c_void, e: *mut u32)
    -> i32;
    fn FUN_0044bad0(a: *mut i32, b: *mut core::ffi::c_void, c: u32) -> i32;
    fn _free(p: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044b720(param_1: *mut i32, param_2: *mut i8) -> i32 {
    let mut local_8: u32 = 0;
    let mut local_4: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut u_var2: i32 = 0;
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // an object whose first field is a vtable pointer, slot 0x14 being a
    // stdcall function taking the object.
    unsafe {
        DAT_005569d4 = core::ptr::null_mut();
        let mut i_var1 = FUN_0044b790(0, param_2, 0, &raw mut local_4, &raw mut local_8);
        if i_var1 != 0 {
            let vtable = param_1.read() as usize as *const i32;
            let f = vtable.add(0x14).read();
            let f: unsafe extern "stdcall" fn(*mut i32) -> i32 =
                core::mem::transmute(f as usize as *const core::ffi::c_void);
            i_var1 = f(param_1);
            if i_var1 >= 0 {
                i_var1 = FUN_0044bad0(param_1, local_4, local_8);
                if i_var1 != 0 {
                    u_var2 = 1;
                }
            }
        }
        if !DAT_005569d4.is_null() {
            _free(DAT_005569d4);
        }
    }
    u_var2
}
