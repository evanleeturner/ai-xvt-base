// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f204: i32;
    static mut _DAT_0052f208: i32;
    static mut DAT_0052f20c: i32;
    static mut DAT_0052f1f8: *mut core::ffi::c_void;
    static mut DAT_0064ee2c: i32;
    static mut DAT_0064ee30: i32;
    static mut DAT_0064ee40: i32;
    static mut DAT_0064ee60: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b51f0(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0045bbe0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b5bc2() -> u8 {
    // SAFETY: the globals and callees are the game's own; DAT_0052f1f8 points
    // at an object whose first field is a vtable, slot 4 being called as in
    // the original.
    unsafe {
        DAT_0052f204 = 0;
        _DAT_0052f208 = 0;
        DAT_0052f20c = 1;
        let obj = DAT_0052f1f8;
        let vtable = obj
            .cast::<*const *const core::ffi::c_void>()
            .read_unaligned();
        let f_ptr = vtable.add(4).read_unaligned();
        let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void, *mut i32) -> i32 =
            core::mem::transmute(f_ptr);
        let i_var1 = f(obj, &raw mut DAT_0064ee30);
        if i_var1 == 0 {
            DAT_0064ee2c = DAT_0064ee40;
            DAT_0064ee60 = DAT_0064ee40;
        } else {
            FUN_004b51f0(i_var1, 0x52f210, 0x79);
            FUN_0045bbe0();
        }
        (i_var1 == 0) as u8
    }
}
