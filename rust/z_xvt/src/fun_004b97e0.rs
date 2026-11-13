// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e6ec: i8;
    static mut DAT_0085e6e8: i8;
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
    fn FUN_004b9970(param_1: *mut core::ffi::c_void, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b97e0(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; the addresses
    // computed from param_1 are the game's global table entries, accessed
    // as the original does.
    unsafe {
        FUN_004dcfd0(local_10.as_mut_ptr(), 0x164, 0x167, 600, 0x175);
        let p_var1 = FUN_004dcfa0(0x2f2);
        FUN_004daac0(0xc, p_var1, local_10.as_mut_ptr(), 0, 1, 0xffff);
        FUN_004dd030(local_10.as_mut_ptr(), 0, 0xf);
        let a = (&raw mut DAT_0085e6ec as usize).wrapping_add(param_1 as usize);
        FUN_004b9970(a as *mut core::ffi::c_void, local_10.as_mut_ptr(), 0xdf);
        if *(a as *const i8) != 0 {
            let b = (&raw mut DAT_0085e6e8 as usize).wrapping_add(param_1 as usize);
            *(b as *mut i8) = 1;
        }
    }
}
