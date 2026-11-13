// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6118: *mut i8;
    static mut DAT_00b6a5aa: i32;
    static mut DAT_00b6a606: i32;
    static mut DAT_0052c208: i32;
    static mut DAT_00bb2810: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(
        param_1: *mut u32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
    ) -> u32;
    fn FUN_004ddc10(param_1: u32) -> *mut i8;
    fn FUN_004db640(param_1: u32, param_2: *mut i8, param_3: *mut u32, param_4: u32) -> u32;
    fn FUN_004dbbd0(
        param_1: u32,
        param_2: *mut i8,
        param_3: *mut u32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
    ) -> u32;
    fn FUN_004d9d70(
        param_1: *mut u32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
        param_7: u32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00504f50() -> u32 {
    let mut local_10: [u32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; local_10 is a
    // valid 16-byte rect buffer passed where the original passes it.
    unsafe {
        if DAT_00aa6118.is_null() {
            return 0;
        }
        let p = local_10.as_mut_ptr();
        FUN_004d6460(p, 0x58, 0x5a, 0x1ae, 0x6a);
        let u_var3: u32 = if DAT_00b6a5aa == 1 && DAT_00b6a606 == 1 {
            0x337
        } else if DAT_00b6a5aa == 3 && DAT_00b6a606 == 1 {
            0x338
        } else if DAT_00b6a5aa == 0 && DAT_00b6a606 == 1 {
            0x339
        } else {
            0x336
        };
        let pc = FUN_004ddc10(u_var3);
        FUN_004db640(0xf, pc, p, 0xffff);
        FUN_004d6460(p, 0x58, 0x6f, 0x1a4, 0x1af);
        let i_var2 = FUN_004dbbd0(0xc, DAT_00aa6118, p, 0xffff, 4, 0x1000) as i32;
        let mut u_var5: u32 = 0x1ae;
        if 0x14 < i_var2.wrapping_add(1) {
            FUN_004d6460(p, 0x1a5, 0x6f, 0x1ae, 0x1af);
            DAT_0052c208 = FUN_004d9d70(
                p,
                DAT_0052c208 as u32,
                i_var2.wrapping_add(1) as u32,
                0,
                5,
                DAT_00bb2810 as u32,
                9,
            ) as i32;
            u_var5 = 0x1a4;
        }
        FUN_004d6460(p, 0x58, 0x6f, u_var5, 0x1af);
        FUN_004dbbd0(0xc, DAT_00aa6118, p, 0xffff, 4, DAT_0052c208 as u32);
        1
    }
}
