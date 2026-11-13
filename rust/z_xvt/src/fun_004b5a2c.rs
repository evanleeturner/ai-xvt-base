// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f1d0: i32;
    static mut DAT_0052f1f8: *mut core::ffi::c_void;
    static mut DAT_00646e18: *mut core::ffi::c_void;
    static mut DAT_0064ee44: *mut core::ffi::c_void;
    static mut DAT_0064ee48: *mut core::ffi::c_void;
    static mut DAT_0064eecc: *mut core::ffi::c_void;
    static mut DAT_006572e0: *mut core::ffi::c_void;
    static mut DAT_00742818: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0045bbe0();
    fn FUN_004b67f9();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b5a2c() {
    // SAFETY: the globals and callees are the game's own; each non-null
    // global is a live object whose first field is a vtable pointer, and the
    // vtable slots called are read and invoked exactly where the original does.
    unsafe {
        if DAT_0052f1d0 == 0 {
            FUN_0045bbe0();
        } else {
            if !DAT_0052f1f8.is_null() {
                let p = DAT_0052f1f8;
                let vt = p.cast::<*const u8>().read_unaligned();
                let f: unsafe extern "C" fn(*mut core::ffi::c_void) = core::mem::transmute(
                    vt.byte_add(8)
                        .cast::<*const core::ffi::c_void>()
                        .read_unaligned(),
                );
                f(p);
                DAT_0052f1f8 = core::ptr::null_mut();
            }
            FUN_004b67f9();
            if !DAT_00646e18.is_null() {
                let p = DAT_00646e18;
                let vt = p.cast::<*const u8>().read_unaligned();
                let f: unsafe extern "C" fn(*mut core::ffi::c_void) = core::mem::transmute(
                    vt.byte_add(8)
                        .cast::<*const core::ffi::c_void>()
                        .read_unaligned(),
                );
                f(p);
                DAT_00646e18 = core::ptr::null_mut();
            }
            if !DAT_0064ee44.is_null() {
                let p = DAT_0064ee44;
                let vt = p.cast::<*const u8>().read_unaligned();
                let f: unsafe extern "C" fn(*mut core::ffi::c_void) = core::mem::transmute(
                    vt.byte_add(8)
                        .cast::<*const core::ffi::c_void>()
                        .read_unaligned(),
                );
                f(p);
                DAT_0064ee44 = core::ptr::null_mut();
            }
            if !DAT_0064ee48.is_null() {
                let p = DAT_0064ee48;
                let vt = p.cast::<*const u8>().read_unaligned();
                let f: unsafe extern "C" fn(*mut core::ffi::c_void) = core::mem::transmute(
                    vt.byte_add(8)
                        .cast::<*const core::ffi::c_void>()
                        .read_unaligned(),
                );
                f(p);
                DAT_0064ee48 = core::ptr::null_mut();
            }
            if !DAT_0064eecc.is_null() {
                let q = DAT_006572e0;
                let vt = q.cast::<*const u8>().read_unaligned();
                let f: unsafe extern "C" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) =
                    core::mem::transmute(
                        vt.byte_add(0x20)
                            .cast::<*const core::ffi::c_void>()
                            .read_unaligned(),
                    );
                f(q, 0, DAT_0064eecc);
                let p = DAT_0064eecc;
                let vt2 = p.cast::<*const u8>().read_unaligned();
                let g: unsafe extern "C" fn(*mut core::ffi::c_void) = core::mem::transmute(
                    vt2.byte_add(8)
                        .cast::<*const core::ffi::c_void>()
                        .read_unaligned(),
                );
                g(p);
                DAT_0064eecc = core::ptr::null_mut();
            }
            if !DAT_00742818.is_null() {
                let p = DAT_00742818;
                let vt = p.cast::<*const u8>().read_unaligned();
                let f: unsafe extern "C" fn(*mut core::ffi::c_void) = core::mem::transmute(
                    vt.byte_add(8)
                        .cast::<*const core::ffi::c_void>()
                        .read_unaligned(),
                );
                f(p);
                DAT_00742818 = core::ptr::null_mut();
            }
            FUN_0045bbe0();
            DAT_0052f1d0 = 0;
        }
    }
}
