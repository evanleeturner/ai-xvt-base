// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00531990: [i8; 0];
    static mut DAT_00532abc: [i8; 0];
    static mut DAT_00532ac4: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ddc70(param_1: *mut i32, param_2: *mut i32) -> i32;
    fn FUN_004dcfd0(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> i32;
    fn FUN_004dd7d0(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004db530() -> i32;
    fn FUN_004db550() -> i32;
    fn FUN_004db570() -> i32;
    fn FUN_004db590() -> i32;
    fn FUN_004d9ea0(param_1: *mut i32);
    fn FUN_004dcfa0(param_1: u32) -> *mut i8;
    fn FUN_004d9a80(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: *mut i8,
        param_4: *mut i8,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: *mut i8,
    ) -> i32;
    fn FUN_004d8a00();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d83f0() -> i32 {
    let mut local_34: i32 = 0;
    let mut local_30: [i32; 12] = [0; 12];

    // SAFETY: the callees and the string globals are the game's own; all
    // pointers passed point at live locals or the game's static data.
    unsafe {
        local_30[2] = 0;
        local_30[3] = 0;
        local_30[4] = 0;
        local_30[5] = 0;
        local_30[6] = 0;
        local_30[7] = 0;
        local_30[1] = 1;
        FUN_004ddc70(local_30.as_mut_ptr(), &raw mut local_34);
        FUN_004dcfd0(local_30.as_mut_ptr().add(8), 0x16, 0x72, 0x2a, 0x8a);
        let i_var2 = FUN_004dd7d0(local_30.as_mut_ptr().add(8), local_30[0], local_34);
        if i_var2 != 0 {
            if FUN_004db530() == 0
                && FUN_004db550() == 0
                && FUN_004db570() == 0
                && FUN_004db590() == 0
            {
            } else {
                local_30[1] = 2;
            }
        }
        FUN_004d9ea0(local_30.as_mut_ptr().add(1));
        FUN_004dcfd0(local_30.as_mut_ptr().add(8), 0x16, 0x72, 0x2a, 0x8a);
        let pc_var6 = &raw mut DAT_00531990 as *mut i8;
        let pc_var3 = FUN_004dcfa0(0x2d0);
        let i_var2 = FUN_004d9a80(
            local_30.as_mut_ptr().add(8),
            &raw mut DAT_00532abc as *mut i8,
            &raw mut DAT_00532ac4 as *mut i8,
            pc_var3,
            0xc,
            0,
            0xb,
            pc_var6,
        );
        if i_var2 != 0 {
            FUN_004d8a00();
        }
    }
    1
}
