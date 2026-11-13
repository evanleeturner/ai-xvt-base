// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb72b2: i32;
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
    fn FUN_004b9090(param_1: *mut i32, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b8860(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and the global are the game's own; local_10 is a
    // live local buffer passed by pointer as the original does.
    unsafe {
        FUN_004d6460(local_10.as_mut_ptr(), 0x58, 0x168, 0x14c, 0x176);
        let pi_var2 = local_10.as_mut_ptr();
        let pc_var1 = FUN_004ddc10(0xe4);
        FUN_004db7d0(0xc, pc_var1, pi_var2, 0, 1, 0xffff);
        FUN_004d64a0(local_10.as_mut_ptr(), 0, 0xf);
        let addr = (&raw mut DAT_00bb72b2 as usize as i32).wrapping_add(param_1);
        FUN_004b9090(addr as usize as *mut i32, local_10.as_mut_ptr(), 0xe5);
    }
}
