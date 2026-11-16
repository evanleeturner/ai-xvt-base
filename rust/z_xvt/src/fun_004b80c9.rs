// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f1e8: i32;
    static mut DAT_0052f1e0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045bbf0(param_1: i32);
    fn FUN_004b87e1(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b80c9(param_1: u32) -> u8 {
    // SAFETY: the globals, the node list and the callees are the game's own
    // memory; nodes are read and written at the same offsets and in the same
    // order as the original, including reading the next link after the node
    // has been passed to FUN_004b87e1.
    unsafe {
        let mut local_c: u32 = 0;
        let mut local_8: *mut i32 = DAT_0052f1e8 as usize as *mut i32;
        while !local_8.is_null() && local_8.add(3).read_unaligned() != DAT_0052f1e0 {
            if (local_8.add(4).read_unaligned() as u32) == param_1 {
                let o6 = local_8.add(6).read_unaligned();
                let f6 = (o6 as usize as *const i32).read_unaligned().wrapping_add(8);
                let f6: unsafe extern "C" fn(*mut core::ffi::c_void) =
                    core::mem::transmute(f6 as usize as *const core::ffi::c_void);
                f6(o6 as usize as *mut core::ffi::c_void);
                let o5 = local_8.add(5).read_unaligned();
                let f5 = (o5 as usize as *const i32).read_unaligned().wrapping_add(8);
                let f5: unsafe extern "C" fn(*mut core::ffi::c_void) =
                    core::mem::transmute(f5 as usize as *const core::ffi::c_void);
                f5(o5 as usize as *mut core::ffi::c_void);
                local_8.add(2).write_unaligned(0);
                FUN_0045bbf0(0);
                FUN_004b87e1(local_8);
                return 1;
            }
            local_8 = local_8.add(1).read_unaligned() as usize as *mut i32;
        }
        local_8 = DAT_0052f1e8 as usize as *mut i32;
        while !local_8.is_null() && local_c < param_1 {
            if local_8.add(3).read_unaligned() != DAT_0052f1e0 {
                let o6 = local_8.add(6).read_unaligned();
                let f6 = (o6 as usize as *const i32).read_unaligned().wrapping_add(8);
                let f6: unsafe extern "C" fn(*mut core::ffi::c_void) =
                    core::mem::transmute(f6 as usize as *const core::ffi::c_void);
                f6(o6 as usize as *mut core::ffi::c_void);
                let o5 = local_8.add(5).read_unaligned();
                let f5 = (o5 as usize as *const i32).read_unaligned().wrapping_add(8);
                let f5: unsafe extern "C" fn(*mut core::ffi::c_void) =
                    core::mem::transmute(f5 as usize as *const core::ffi::c_void);
                f5(o5 as usize as *mut core::ffi::c_void);
                local_8.add(2).write_unaligned(0);
                FUN_0045bbf0(0);
                local_c = local_c.wrapping_add(local_8.add(4).read_unaligned() as u32);
                FUN_004b87e1(local_8);
            }
            local_8 = local_8.add(1).read_unaligned() as usize as *mut i32;
        }
        (local_c != 0) as u8
    }
}
