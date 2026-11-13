// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb72c0: i32;
    static mut DAT_00b6a2c0: i32;
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
    fn FUN_004b93f0(param_1: *mut i32, param_2: *mut i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b8a60(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: all callees and globals are the game's own; local_10 is a
    // valid 4-int buffer passed where the original passes it.
    unsafe {
        FUN_004d6460(local_10.as_mut_ptr(), 0x164, 0xe4, 600, 0xf2);
        let p_var2 = local_10.as_mut_ptr();
        let pc_var1 = FUN_004ddc10(0xf7);
        FUN_004db7d0(0xc, pc_var1, p_var2, 0, 1, 0xffff);
        FUN_004d64a0(local_10.as_mut_ptr(), 0, 0xf);
        FUN_004b93f0(
            (&raw mut DAT_00bb72c0).wrapping_add(param_1 as usize),
            local_10.as_mut_ptr(),
            0x14,
            0xf8,
            1,
        );
        FUN_004d64a0(local_10.as_mut_ptr(), 0, 0xf);
        let p_var2 = local_10.as_mut_ptr();
        let u_var5 = DAT_00b6a2c0 as u32;
        let pc_var1 = FUN_004ddc10(0xfa);
        FUN_004db640(0xc, pc_var1, p_var2, u_var5);
    }
}
