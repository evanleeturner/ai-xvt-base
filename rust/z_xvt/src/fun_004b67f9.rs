// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f1e8: *mut u32;
    static mut DAT_0052f1ec: u32;
    static mut _DAT_0052f1e4: u32;
    static mut DAT_0052f1e0: u32;
    static mut DAT_00742820: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0045bbf0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b67f9() {
    // SAFETY: the globals, the list nodes and the callees are the game's own
    // memory; nodes and their objects are read and written where the original does.
    unsafe {
        let mut local_8: *mut u32 = DAT_0052f1e8;
        while !local_8.is_null() {
            let o6 = local_8.add(6).read_unaligned();
            if o6 != 0 {
                let vt = (o6 as *const u32).read_unaligned();
                let f: unsafe extern "C" fn(*mut core::ffi::c_void) =
                    core::mem::transmute(vt.wrapping_add(8) as *const core::ffi::c_void);
                f(o6 as *mut core::ffi::c_void);
                local_8.add(6).write_unaligned(0);
            }
            let o5 = local_8.add(5).read_unaligned();
            if o5 != 0 {
                let vt = (o5 as *const u32).read_unaligned();
                let f: unsafe extern "C" fn(*mut core::ffi::c_void) =
                    core::mem::transmute(vt.wrapping_add(8) as *const core::ffi::c_void);
                f(o5 as *mut core::ffi::c_void);
                local_8.add(5).write_unaligned(0);
            }
            local_8.add(2).write_unaligned(0);
            DAT_00742820 = DAT_00742820.wrapping_add(local_8.add(4).read_unaligned());
            FUN_0045bbf0();
            let next = local_8.add(1).read_unaligned() as *mut u32;
            local_8.write_unaligned(0);
            local_8.add(1).write_unaligned(0);
            local_8 = next;
        }
        DAT_0052f1e8 = core::ptr::null_mut();
        DAT_0052f1ec = 0;
        _DAT_0052f1e4 = 0;
        DAT_0052f1e0 = 1;
    }
}
