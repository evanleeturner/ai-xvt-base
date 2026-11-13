// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb72cc: [i8; 0];
    static mut DAT_00bb72c8: i8;
    static mut DAT_00b69ccc: u32;
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
    fn FUN_004b8ec0(param_1: *mut i8, param_2: *mut i32, param_3: i32);
    fn FUN_004b8ea0(param_1: *mut i8, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b8600(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; local_10 is a
    // valid local buffer passed by pointer, and param_1 indexes the game's arrays.
    unsafe {
        let p = local_10.as_mut_ptr();
        FUN_004d6460(p, 0x58, 0xd1, 0x14c, 0xdf);
        let flag = (&raw mut DAT_00bb72cc)
            .cast::<i8>()
            .offset(param_1 as isize)
            .read();
        if flag != 0 {
            let uvar5 = DAT_00b69ccc;
            let pcvar1 = FUN_004ddc10(0xe8);
            FUN_004db7d0(0xc, pcvar1, p, 0, 1, uvar5);
            FUN_004d64a0(p, 0, 0xf);
            FUN_004b8ec0((&raw mut DAT_00bb72c8).offset(param_1 as isize), p, 0xe9);
            return;
        }
        let pcvar1 = FUN_004ddc10(0xe8);
        FUN_004db7d0(0xc, pcvar1, p, 0, 1, 0xffff);
        FUN_004d64a0(p, 0, 0xf);
        FUN_004b8ea0((&raw mut DAT_00bb72c8).offset(param_1 as isize), p, 0xe9);
    }
}
