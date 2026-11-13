// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052ad74: *mut i8;
    static mut DAT_0052bfe4: *mut i8;
    static mut DAT_0052bfec: *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004de090(param_1: *mut i32, param_2: *mut i32) -> i32;
    fn FUN_004d6460(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> i32;
    fn FUN_004d6b90(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004daba0(param_1: *mut i32) -> i32;
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004da780(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: *mut i8,
        param_4: *mut i8,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: *mut i8,
    ) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004dc1e0() -> i32;
    fn FUN_004dc200() -> i32;
    fn FUN_004dc220() -> i32;
    fn FUN_004dc240() -> i32;
    fn FUN_004d9780() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d9170() -> i32 {
    let mut local_34: [i32; 13] = [0; 13];
    let mut local_38: i32 = 0;
    // SAFETY: the callees and globals are the game's own; the pointers
    // passed refer to locals that live for the whole function.
    unsafe {
        let p = local_34.as_mut_ptr();
        local_34[2] = 0;
        local_34[3] = 0;
        local_34[4] = 0;
        local_34[5] = 0;
        local_34[6] = 0;
        local_34[7] = 0;
        local_34[1] = 1;
        local_34[8] = 0;
        FUN_004de090(p, &raw mut local_38);
        FUN_004d6460(p.add(9), 0x16, 0x72, 0x2a, 0x8a);
        let i_var2 = FUN_004d6b90(p.add(9), local_34[0], local_38);
        if i_var2 != 0 {
            let mut set = true;
            if FUN_004dc1e0() == 0
                && FUN_004dc200() == 0
                && FUN_004dc220() == 0
                && FUN_004dc240() == 0
            {
                set = false;
            }
            if set {
                local_34[1] = 2;
            }
        }
        FUN_004daba0(p.add(1));
        FUN_004d6460(p.add(9), 0x16, 0x72, 0x2a, 0x8a);
        let pc_var6 = DAT_0052ad74;
        let pc_var3 = FUN_004ddc10(0x2de);
        let i_var2 = FUN_004da780(
            p.add(9),
            DAT_0052bfe4,
            DAT_0052bfec,
            pc_var3,
            0xc,
            0,
            0xb,
            pc_var6,
        );
        if i_var2 != 0 {
            FUN_004d9780();
        }
    }
    1
}
