// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb72ca: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: u32, param_3: u32, param_4: u32, param_5: u32);
    fn FUN_004ddc10(param_1: u32) -> *mut i8;
    fn FUN_004db7d0(
        param_1: u32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004b93f0(param_1: *mut i32, param_2: *mut i32, param_3: i32, param_4: i32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b86e0(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    let pi_var2: *mut i32 = local_10.as_mut_ptr();
    // SAFETY: the callees and the global are the game's own; local_10 is a
    // live 4-int buffer passed where the original passes it.
    unsafe {
        FUN_004d6460(pi_var2, 0x58, 0xf3, 0x14c, 0x101);
        let pc_var1 = FUN_004ddc10(0xff);
        FUN_004db7d0(0xc, pc_var1, pi_var2, 0, 1, 0xffff);
        FUN_004d64a0(pi_var2, 0, 0xf);
        FUN_004b93f0(
            (&raw mut DAT_00bb72ca).wrapping_offset(param_1 as isize),
            pi_var2,
            8,
            0x100,
            1,
        );
    }
}
