// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6ae8: i32;
    static mut DAT_00aa615e: i32;
    static mut DAT_00aa62ae: i32;
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_0052a6b0: [i8; 1];
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
    fn FUN_004d1400();
    fn FUN_004d4cf0(param_1: *mut i32);
    fn FUN_004d4d20(param_1: *mut i32);
    #[link_name = "sprintf"]
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d8540() -> i32 {
    let mut local_20: [i32; 4] = [0; 4];
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: all callees and globals are the game's own; the pointers passed
    // reference locals or the game's tables, accessed as the original does.
    unsafe {
        FUN_004d6460(local_20.as_mut_ptr(), 0x58, 0xda, 0x1ae, 0xe9);
        let mut pi_var2: *mut i32 = local_20.as_mut_ptr();
        let u_var6: u32 = 0xffff;
        let mut i_var4: i32 = 1;
        let mut i_var3: i32 = 0;
        let pc_var1 = FUN_004ddc10(0xc9);
        FUN_004db7d0(0xc, pc_var1, pi_var2, i_var3, i_var4, u_var6);
        if DAT_00aa6ae8 != -1 {
            i_var3 = 0;
            FUN_004d1400();
            pi_var2 = &raw mut DAT_00aa615e;
            FUN_004d6460(local_20.as_mut_ptr(), 0x58, 0xee, 0x102, 0xfc);
            let end = (&raw mut DAT_00aa62ae) as usize;
            loop {
                if pi_var2.read_unaligned() != 0 {
                    i_var3 = i_var3.wrapping_add(1);
                    FUN_004d4cf0(local_10.as_mut_ptr());
                    FUN_004d4d20(local_20.as_mut_ptr());
                    i_var4 = (pi_var2 as i32).wrapping_sub(0xe);
                    let u_var5: i32 = 4;
                    let pc = FUN_004ddc10(
                        pi_var2.byte_add(4).read_unaligned().wrapping_add(0x9a) as u32
                    );
                    _sprintf(
                        (&raw mut DAT_00b69d20) as *mut i8,
                        (&raw const DAT_0052a6b0) as *const i8,
                        6i32,
                        pc,
                        u_var5,
                        i_var4,
                    );
                    FUN_004db7d0(
                        0xc,
                        (&raw mut DAT_00b69d20) as *mut i8,
                        local_20.as_mut_ptr(),
                        0,
                        1,
                        0xffff,
                    );
                    FUN_004d4d20(local_10.as_mut_ptr());
                    if i_var3 == 4 {
                        FUN_004d6460(local_20.as_mut_ptr(), 0x103, 0xee, 0x1ae, 0xfc);
                    } else {
                        FUN_004d64a0(local_20.as_mut_ptr(), 0, 0xf);
                    }
                }
                pi_var2 = pi_var2.byte_add(0x2a);
                if (pi_var2 as usize) >= end {
                    break;
                }
            }
        }
    }
    1
}
