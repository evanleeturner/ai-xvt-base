// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb72bc: [u8; 0];
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
pub unsafe extern "C" fn FUN_004b89e0(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and the global buffer are the game's own; local_10
    // is a live local array passed by pointer as the original does.
    unsafe {
        FUN_004d6460(local_10.as_mut_ptr(), 0x164, 199, 600, 0xd5);
        let p_var2 = local_10.as_mut_ptr();
        let u_var5: u32 = 0xffff;
        let i_var4: i32 = 1;
        let i_var3: i32 = 0;
        let p_var1 = FUN_004ddc10(0xf2);
        FUN_004db7d0(0xc, p_var1, p_var2, i_var3, i_var4, u_var5);
        FUN_004d64a0(local_10.as_mut_ptr(), 0, 0xf);
        FUN_004b8ea0(
            (&raw mut DAT_00bb72bc)
                .cast::<u8>()
                .offset(param_1 as isize),
            local_10.as_mut_ptr(),
            0xec,
        );
    }
}
