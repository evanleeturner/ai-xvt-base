// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e6ec: [i8; 1];
    static mut DAT_0085e6d8: [i8; 1];
    static mut _DAT_0081b6fc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dcfa0(param_1: u32) -> *mut i8;
    fn FUN_004daac0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004b9950(param_1: *mut core::ffi::c_void, param_2: *mut i32, param_3: i32);
    fn FUN_004b9970(param_1: *mut core::ffi::c_void, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b9680(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; local_10 is a
    // valid 4-int buffer, and the arrays are indexed by param_1 as the
    // original does.
    unsafe {
        FUN_004dcfd0(local_10.as_mut_ptr(), 0x164, 0x12d, 600, 0x13b);
        let flag = (&raw mut DAT_0085e6ec)
            .cast::<i8>()
            .offset(param_1 as isize)
            .read();
        if flag != 0 {
            let u_var5 = _DAT_0081b6fc;
            let pc = FUN_004dcfa0(0xe3);
            FUN_004daac0(0xc, pc, local_10.as_mut_ptr(), 0, 1, u_var5);
            FUN_004dd030(local_10.as_mut_ptr(), 0, 0xf);
            FUN_004b9950(
                (&raw mut DAT_0085e6d8)
                    .cast::<i8>()
                    .offset(param_1 as isize)
                    .cast::<core::ffi::c_void>(),
                local_10.as_mut_ptr(),
                0xdf,
            );
            return;
        }
        let pc = FUN_004dcfa0(0xe3);
        FUN_004daac0(0xc, pc, local_10.as_mut_ptr(), 0, 1, 0xffff);
        FUN_004dd030(local_10.as_mut_ptr(), 0, 0xf);
        FUN_004b9970(
            (&raw mut DAT_0085e6d8)
                .cast::<i8>()
                .offset(param_1 as isize)
                .cast::<core::ffi::c_void>(),
            local_10.as_mut_ptr(),
            0xdf,
        );
    }
}
