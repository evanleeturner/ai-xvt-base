// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528c84: i32;
    static mut DAT_00528ca0: i32;
    static mut DAT_00528ca4: i32;
    static mut DAT_00528ca8: i32;
    static mut DAT_00528c98: *mut core::ffi::c_void;
    static mut DAT_00528cc0: i32;
    static mut DAT_00664754: i32;
    static mut DAT_00664758: i32;
    static mut DAT_00664768: i32;
    static mut DAT_00664c80: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b1600(param_1: i32, param_2: *mut i32, param_3: i32) -> *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004079e0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b1c90() -> i32 {
    // SAFETY: the globals and callees are the game's own; DAT_00528c98 points
    // at a COM-style object whose vtable slot 4 is called as the original does.
    unsafe {
        DAT_00528c84 = DAT_00528c84.wrapping_add(1);
        DAT_00528ca0 = 0;
        DAT_00528ca4 = 0;
        DAT_00528ca8 = 1;
        let obj = DAT_00528c98;
        let vtbl = obj.cast::<*const core::ffi::c_void>().read_unaligned();
        let slot = vtbl
            .byte_add(0x10)
            .cast::<*const core::ffi::c_void>()
            .read_unaligned();
        let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> i32 =
            core::mem::transmute(slot);
        let i_var1 = f(obj, (&raw mut DAT_00664758).cast());
        if i_var1 != 0 {
            FUN_004b1600(i_var1, &raw mut DAT_00528cc0, 0x79);
            FUN_004079e0();
            return 0;
        }
        DAT_00664754 = DAT_00664768;
        DAT_00664c80 = DAT_00664768;
        1
    }
}
