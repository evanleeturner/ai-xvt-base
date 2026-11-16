// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f1f4: *mut u32;
    static mut DAT_0064ee44: *mut u32;
    static mut DAT_0064ee48: *mut u32;
    static mut DAT_00742818: *mut u32;
    static mut _DAT_0064ee50: u32;
    static mut _DAT_0064ee54: u32;
    static mut _DAT_0064ee58: u32;
    static mut _DAT_0064ee5c: u32;
    static mut _DAT_00520260: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b51f0(param_1: i32, param_2: u32, param_3: u32);
    fn FUN_0045bbe0();
    fn _memset(dst: *mut core::ffi::c_void, val: i32, size: u32) -> *mut core::ffi::c_void;
}

/// Reads the method pointer at byte offset `off` of the object's vtable.
unsafe fn vslot(obj: *mut u32, off: usize) -> *const core::ffi::c_void {
    // SAFETY: obj is a live COM-style object whose first word is its vtable.
    unsafe {
        let vt = obj.read_unaligned() as usize;
        ((vt + off) as *const u32).read_unaligned() as usize as *const core::ffi::c_void
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b7343(param_1: u32, param_2: u32) -> u32 {
    let mut local_84: [u32; 20] = [0; 20];
    let mut local_34: u32 = 0;
    let mut vp: [u32; 11] = [0; 11];

    // SAFETY: the globals and the COM-style objects they point at are the
    // game's own; the out-pointers handed to the callees reference locals
    // that live for the whole function.
    unsafe {
        let mut iv: i32;

        let f1: unsafe extern "stdcall" fn(*mut u32, *mut *mut u32, u32) -> i32 =
            core::mem::transmute(vslot(DAT_0052f1f4, 0x1c));
        iv = f1(DAT_0052f1f4, &raw mut DAT_0064ee44, 0);
        'fail: {
            if iv != 0 {
                break 'fail;
            }
            let f2: unsafe extern "stdcall" fn(*mut u32, *mut u32) -> i32 =
                core::mem::transmute(vslot(DAT_00742818, 0x24));
            iv = f2(DAT_00742818, DAT_0064ee44);
            if iv != 0 {
                break 'fail;
            }
            _memset(vp.as_mut_ptr().cast(), 0, 0x2c);
            vp[0] = 0x2c;
            vp[2] = 0;
            vp[1] = 0;
            vp[3] = param_1;
            vp[4] = param_2;
            let local_1c: f32 = (param_1 as f32) / _DAT_00520260;
            let local_18: f32 = (param_2 as f32) / _DAT_00520260;
            let local_14: f32 = (param_1 as f32) / (local_1c * _DAT_00520260);
            let local_10: f32 = (param_2 as f32) / (local_18 * _DAT_00520260);
            vp[5] = local_1c.to_bits();
            vp[6] = local_18.to_bits();
            vp[7] = local_14.to_bits();
            vp[8] = local_10.to_bits();
            let f3: unsafe extern "stdcall" fn(*mut u32, *mut core::ffi::c_void) -> i32 =
                core::mem::transmute(vslot(DAT_0064ee44, 0x14));
            iv = f3(DAT_0064ee44, vp.as_mut_ptr().cast());
            if iv != 0 {
                break 'fail;
            }
            _DAT_0064ee50 = 0;
            _DAT_0064ee54 = 0;
            _DAT_0064ee58 = param_1;
            _DAT_0064ee5c = param_2;
            let f4: unsafe extern "stdcall" fn(*mut u32, *mut *mut u32, u32) -> i32 =
                core::mem::transmute(vslot(DAT_0052f1f4, 0x18));
            iv = f4(DAT_0052f1f4, &raw mut DAT_0064ee48, 0);
            if iv != 0 {
                break 'fail;
            }
            _memset(local_84.as_mut_ptr().cast(), 0, 0x50);
            local_84[0] = 0x50;
            local_84[1] = 0;
            local_84[2] = 0;
            local_84[3] = 0x3f000000;
            local_84[19] = 1;
            let f5: unsafe extern "stdcall" fn(*mut u32, *mut core::ffi::c_void) -> i32 =
                core::mem::transmute(vslot(DAT_0064ee48, 0x10));
            f5(DAT_0064ee48, local_84.as_mut_ptr().cast());
            let f6: unsafe extern "stdcall" fn(*mut u32, *mut u32, *mut u32) -> i32 =
                core::mem::transmute(vslot(DAT_0064ee48, 0x18));
            f6(DAT_0064ee48, DAT_00742818, &raw mut local_34);
            let f7: unsafe extern "stdcall" fn(*mut u32, u32) -> i32 =
                core::mem::transmute(vslot(DAT_0064ee44, 0x20));
            iv = f7(DAT_0064ee44, local_34);
            if iv != 0 {
                break 'fail;
            }
            FUN_0045bbe0();
            return 1;
        }
        FUN_004b51f0(iv, 0x52f210, 0x79);
        FUN_0045bbe0();
        0
    }
}
