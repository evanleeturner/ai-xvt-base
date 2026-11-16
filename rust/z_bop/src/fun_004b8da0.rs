// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb72cc: [i8; 1];
    static mut DAT_00bb72ce: [i8; 1];
    static mut _DAT_00b69ccc: i32;
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
    fn FUN_004b8ea0(param_1: *mut i8, param_2: *mut i32, param_3: i32);
    fn FUN_004b8e80(param_1: *mut i8, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b8da0(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; local_10 is a
    // live local buffer passed where the original passes it.
    unsafe {
        FUN_004d6460(local_10.as_mut_ptr(), 0x164, 0x184, 600, 0x192);
        let flag = (&raw mut DAT_00bb72cc as *mut i8)
            .offset(param_1 as isize)
            .read();
        if flag != 0 {
            let pc_var1 = FUN_004ddc10(0x323);
            FUN_004db7d0(0xc, pc_var1, local_10.as_mut_ptr(), 0, 1, 0xffff);
            FUN_004d64a0(local_10.as_mut_ptr(), 0, 0xf);
            FUN_004b8ea0(
                (&raw mut DAT_00bb72ce as *mut i8).offset(param_1 as isize),
                local_10.as_mut_ptr(),
                0xec,
            );
            return;
        }
        let u_var5 = _DAT_00b69ccc as u32;
        let pc_var1 = FUN_004ddc10(0x323);
        FUN_004db7d0(0xc, pc_var1, local_10.as_mut_ptr(), 0, 1, u_var5);
        FUN_004d64a0(local_10.as_mut_ptr(), 0, 0xf);
        FUN_004b8e80(
            (&raw mut DAT_00bb72ce as *mut i8).offset(param_1 as isize),
            local_10.as_mut_ptr(),
            0xec,
        );
    }
}
