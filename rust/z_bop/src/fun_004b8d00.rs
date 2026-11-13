// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb72cc: u8;
    static mut DAT_00bb72c8: u8;
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
    fn FUN_004b8ea0(param_1: *mut u8, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b8d00(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; local_10 is a
    // live local array, and the indexed globals are read and written
    // where the original does.
    unsafe {
        FUN_004d6460(local_10.as_mut_ptr(), 0x164, 0x167, 600, 0x175);
        let pi_var4 = local_10.as_mut_ptr();
        let pc_var3 = FUN_004ddc10(0x322);
        FUN_004db7d0(0xc, pc_var3, pi_var4, 0, 1, 0xffff);
        FUN_004d64a0(local_10.as_mut_ptr(), 0, 0xf);
        let pb_var1 = (&raw mut DAT_00bb72cc).byte_offset(param_1 as isize);
        let b_var2 = pb_var1.read();
        FUN_004b8ea0(pb_var1, local_10.as_mut_ptr(), 0xec);
        if pb_var1.read() != b_var2 && pb_var1.read() != 0 {
            (&raw mut DAT_00bb72c8)
                .byte_offset(param_1 as isize)
                .write(1);
        }
    }
}
