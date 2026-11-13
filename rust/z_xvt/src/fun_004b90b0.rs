// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e6e4: [u8; 0];
    static mut DAT_0085e6e6: [u8; 0];
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
    fn FUN_004b9b40(param_1: *mut i8, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b90b0(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: callees and globals are the game's own; the arrays are indexed
    // by param_1 exactly as the original does.
    unsafe {
        FUN_004dcfd0(local_10.as_mut_ptr(), 0x58, 0xa0, 0x14c, 0xae);
        let pi_var3 = local_10.as_mut_ptr();
        let pc_var2 = FUN_004dcfa0(0xd3);
        FUN_004daac0(0xc, pc_var2, pi_var3, 0, 1, 0xffff);
        FUN_004dd030(local_10.as_mut_ptr(), 0, 0xf);
        let pb_var1 = (&raw mut DAT_0085e6e6)
            .cast::<u8>()
            .offset(param_1 as isize);
        FUN_004b9b40(pb_var1.cast::<i8>(), local_10.as_mut_ptr(), 0xd4);
        let limit = (&raw mut DAT_0085e6e4)
            .cast::<u8>()
            .offset(param_1 as isize)
            .read();
        if limit < pb_var1.read() {
            pb_var1.write(limit);
        }
    }
}
