// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e6da: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dcfa0(param_1: i32) -> *mut core::ffi::c_void;
    fn FUN_004daac0(
        param_1: i32,
        param_2: *mut core::ffi::c_void,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004b9970(param_1: *mut core::ffi::c_void, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b9760(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and the global are the game's own; local_10 is a
    // live local buffer passed where the original passes it.
    unsafe {
        FUN_004dcfd0(local_10.as_mut_ptr(), 0x164, 0x14a, 600, 0x158);
        let p_var1 = FUN_004dcfa0(0xe4);
        FUN_004daac0(0xc, p_var1, local_10.as_mut_ptr(), 0, 1, 0xffff);
        FUN_004dd030(local_10.as_mut_ptr(), 0, 0xf);
        FUN_004b9970(
            ((&raw mut DAT_0085e6da) as i32).wrapping_add(param_1) as *mut core::ffi::c_void,
            local_10.as_mut_ptr(),
            0xdf,
        );
    }
}
