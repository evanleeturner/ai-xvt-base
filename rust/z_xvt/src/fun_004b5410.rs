// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f1f4: *mut core::ffi::c_void;
    static mut DAT_0052f1cc: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0045bbe0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b5410() {
    // SAFETY: the globals and the callee are the game's own; the object in
    // DAT_0052f1f4 has a vftable whose slot 2 is a stdcall method taking the
    // object pointer, called exactly as the original does.
    unsafe {
        if !DAT_0052f1f4.is_null() {
            let vftable = (DAT_0052f1f4 as *const *const *const core::ffi::c_void).read_unaligned();
            let f = vftable.add(2).read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void) = core::mem::transmute(f);
            f(DAT_0052f1f4);
        }
        DAT_0052f1f4 = core::ptr::null_mut();
        FUN_0045bbe0();
        DAT_0052f1cc = core::ptr::null_mut();
    }
}
