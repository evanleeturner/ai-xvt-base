// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb72b0: *mut u8;
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
pub unsafe extern "C" fn FUN_004b87e0(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and the global are the game's own; local_10 is a
    // live local buffer passed to them as the original does.
    unsafe {
        FUN_004d6460(local_10.as_mut_ptr(), 0x58, 0x146, 0x14c, 0x154);
        let pi_var2 = local_10.as_mut_ptr();
        let u_var5: u32 = 0xffff;
        let i_var4: i32 = 1;
        let i_var3: i32 = 0;
        let pc_var1 = FUN_004ddc10(0xeb);
        FUN_004db7d0(0xc, pc_var1, pi_var2, i_var3, i_var4, u_var5);
        FUN_004d64a0(local_10.as_mut_ptr(), 0, 0xf);
        FUN_004b8ea0(
            DAT_00bb72b0.wrapping_offset(param_1 as isize),
            local_10.as_mut_ptr(),
            0xec,
        );
    }
}
