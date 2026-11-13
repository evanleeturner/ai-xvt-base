// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa607c: i32;
    static mut DAT_00aa6084: i32;
    static mut DAT_00aa6090: i32;
    static mut DAT_00aa60d0: i32;
    static mut DAT_00aa615e: i32;
    static mut DAT_00aa62ae: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d1400() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004e0490();
    fn FUN_004f02f0();
    fn FUN_004f0510();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f07d0(param_1: i32) -> i32 {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the pointers walk the game's tables exactly as the original does.
    unsafe {
        let mut u_var6: u32 = 0;
        if param_1 == 0 {
            let u_var1 = FUN_004d1400() as u32;
            let mut p_var5: *mut i32 = &raw mut DAT_00aa6090;
            let mut p_var3: *mut i32 = &raw mut DAT_00aa615e;
            let end3: *mut i32 = &raw mut DAT_00aa62ae;
            loop {
                if u_var6 < u_var1 {
                    p_var5.write_unaligned(p_var3.read_unaligned());
                    p_var5.add(1).write_unaligned(0);
                } else {
                    p_var5.write_unaligned(0);
                }
                p_var5 = p_var5.add(2);
                p_var3 = p_var3.byte_add(0x2a);
                u_var6 = u_var6.wrapping_add(1);
                if !(p_var3 < end3) {
                    break;
                }
            }
            DAT_00aa607c = 0;
        }
        FUN_004e0490();
        let mut i_var2: i32 = 0;
        let mut pi_var4: *mut i32 = &raw mut DAT_00aa6090;
        let end4: *mut i32 = &raw mut DAT_00aa60d0;
        loop {
            if pi_var4.read_unaligned() != 0 && pi_var4.add(1).read_unaligned() == 0 {
                i_var2 = i_var2.wrapping_add(1);
            }
            pi_var4 = pi_var4.add(2);
            if !(pi_var4 < end4) {
                break;
            }
        }
        if i_var2 == 0 {
            return 1;
        }
        FUN_004f02f0();
        if DAT_00aa6084 == 1 {
            FUN_004f0510();
        }
        0
    }
}
