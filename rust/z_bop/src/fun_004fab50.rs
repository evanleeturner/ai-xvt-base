// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a5aa: u32;
    static mut DAT_00aa6118: u32;
    static mut DAT_0052c208: u32;
    static mut DAT_00bb2810: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut u32, param_2: u32, param_3: u32, param_4: u32, param_5: u32);
    fn FUN_004ddc10(param_1: u32) -> *mut i8;
    fn FUN_004db7d0(
        param_1: u32,
        param_2: *mut i8,
        param_3: *mut u32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
    ) -> u32;
    fn FUN_004dbbd0(
        param_1: u32,
        param_2: u32,
        param_3: *mut u32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
    ) -> i32;
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
pub unsafe extern "stdcall" fn FUN_004fab50() -> u32 {
    let mut local_10: [u32; 4] = [0; 4];
    let p = local_10.as_mut_ptr();
    // SAFETY: the callees and globals are the game's own; p points at a
    // local four-word rectangle that the callees fill and read.
    unsafe {
        FUN_004d6460(p, 0x58, 0x5a, 0x1ae, 0x6b);
        let u_var3: u32 = if DAT_00b6a5aa == 2 {
            0x1d7
        } else if DAT_00b6a5aa == 4 {
            0x1d8
        } else if DAT_00b6a5aa == 5 {
            0x309
        } else {
            0xc0
        };
        let pc_var1 = FUN_004ddc10(u_var3);
        FUN_004db7d0(0xf, pc_var1, p, 0, 1, 0xffff);
        FUN_004d6460(p, 0x58, 0x6c, 0x1a4, 0xfc);
        let i_var2 = FUN_004dbbd0(0xc, DAT_00aa6118, p, 0xffff, 4, 0x1000);
        let mut u_var6: u32 = 0x1ae;
        if 9 < i_var2.wrapping_add(1) {
            FUN_004d6460(p, 0x1a5, 0x6c, 0x1ae, 0xfc);
            DAT_0052c208 = FUN_004d9d70(
                p,
                DAT_0052c208,
                i_var2.wrapping_add(1) as u32,
                0,
                5,
                DAT_00bb2810,
                9,
            );
            u_var6 = 0x1a4;
        }
        FUN_004d6460(p, 0x58, 0x6c, u_var6, 0xfc);
        FUN_004dbbd0(0xc, DAT_00aa6118, p, 0xffff, 4, DAT_0052c208);
    }
    1
}
