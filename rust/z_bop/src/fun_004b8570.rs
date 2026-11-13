// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb72c6: [u8; 0];
    static mut DAT_00bb72c4: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc10(param_1: u32) -> *mut i8;
    fn FUN_004db7d0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004b9090(param_1: *mut i8, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b8570(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; local_10 is a
    // valid local buffer and the global arrays are indexed as the original does.
    unsafe {
        FUN_004d6460(local_10.as_mut_ptr(), 0x58, 0xa0, 0x14c, 0xae);
        let pi_var3 = local_10.as_mut_ptr();
        let pc_var2 = FUN_004ddc10(0xe0);
        FUN_004db7d0(0xc, pc_var2, pi_var3, 0, 1, 0xffff);
        FUN_004d64a0(local_10.as_mut_ptr(), 0, 0xf);
        let pb_var1 = (&raw mut DAT_00bb72c6)
            .cast::<u8>()
            .offset(param_1 as isize);
        FUN_004b9090(pb_var1.cast::<i8>(), local_10.as_mut_ptr(), 0xe1);
        let limit = (&raw mut DAT_00bb72c4)
            .cast::<u8>()
            .offset(param_1 as isize)
            .read();
        if limit < pb_var1.read() {
            pb_var1.write(limit);
        }
    }
}
