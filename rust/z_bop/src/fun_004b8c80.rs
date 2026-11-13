// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb72ba: u8;
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
pub unsafe extern "C" fn FUN_004b8c80(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and the global are the game's own; local_10 is a
    // live local buffer passed where the original passes it.
    unsafe {
        let p = local_10.as_mut_ptr();
        FUN_004d6460(p, 0x164, 0x14a, 600, 0x158);
        let pc = FUN_004ddc10(0xf1);
        FUN_004db7d0(0xc, pc, p, 0, 1, 0xffff);
        FUN_004d64a0(p, 0, 0xf);
        FUN_004b8ea0((&raw mut DAT_00bb72ba).byte_add(param_1 as usize), p, 0xec);
    }
}
