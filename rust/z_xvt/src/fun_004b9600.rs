// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e6d6: i32;
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
    fn FUN_004b9970(param_1: *mut i32, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b9600(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and the global are the game's own; local_10 is a
    // live local buffer passed as the original does.
    unsafe {
        FUN_004dcfd0(local_10.as_mut_ptr(), 0x164, 0x110, 600, 0x11e);
        let pc_var1 = FUN_004dcfa0(0xe2);
        FUN_004daac0(0xc, pc_var1, local_10.as_mut_ptr(), 0, 1, 0xffff);
        FUN_004dd030(local_10.as_mut_ptr(), 0, 0xf);
        let p = (&raw mut DAT_0085e6d6).wrapping_offset(param_1 as isize);
        FUN_004b9970(p, local_10.as_mut_ptr(), 0xdf);
    }
}
