// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb72cc: u8;
    static mut _DAT_00b69ccc: u32;
    static mut DAT_00bb72b8: u8;
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
    fn FUN_004b8e80(param_1: *mut u8, param_2: *mut i32, param_3: i32);
    fn FUN_004b8ea0(param_1: *mut u8, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b8ba0(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; local_10 is a
    // local buffer passed by pointer, and param_1 indexes the game's tables.
    unsafe {
        FUN_004d6460(local_10.as_mut_ptr(), 0x164, 0x12d, 600, 0x13b);
        if (*(&raw mut DAT_00bb72cc).byte_offset(param_1 as isize)) != 0 {
            let uv = _DAT_00b69ccc;
            let pc = FUN_004ddc10(0xf0);
            FUN_004db7d0(0xc, pc, local_10.as_mut_ptr(), 0, 1, uv);
            FUN_004d64a0(local_10.as_mut_ptr(), 0, 0xf);
            FUN_004b8e80(
                (&raw mut DAT_00bb72b8).byte_offset(param_1 as isize),
                local_10.as_mut_ptr(),
                0xec,
            );
            return;
        }
        let pc = FUN_004ddc10(0xf0);
        FUN_004db7d0(0xc, pc, local_10.as_mut_ptr(), 0, 1, 0xffff);
        FUN_004d64a0(local_10.as_mut_ptr(), 0, 0xf);
        FUN_004b8ea0(
            (&raw mut DAT_00bb72b8).byte_offset(param_1 as isize),
            local_10.as_mut_ptr(),
            0xec,
        );
    }
}
