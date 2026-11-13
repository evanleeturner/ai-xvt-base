// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00acebc5: i32;
    static mut DAT_00acebfd: i32;
    static mut DAT_00acec45: i32;
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00acec49: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d48e0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d1330(param_1: i32) -> i32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the table pointer is walked and written exactly as the original does.
    unsafe {
        let c_var1 = DAT_00aa7b0d;
        if DAT_00acebc5 == 0 {
            return 0;
        }
        FUN_004d4970();
        let mut i_var2: i32 = 0;
        let mut b_var4: i32 = (DAT_00acebfd == 0) as i32;
        if DAT_00acebfd > 0 {
            let mut pi_var3: *mut i32 = &raw mut DAT_00acec45;
            loop {
                if pi_var3.read_unaligned() == param_1 {
                    break;
                }
                pi_var3 = pi_var3.wrapping_offset(10);
                i_var2 = i_var2.wrapping_add(1);
                if !(i_var2 < DAT_00acebfd) {
                    break;
                }
            }
            b_var4 = (DAT_00acebfd == i_var2) as i32;
        }
        if b_var4 != 0 {
            if c_var1 != 0 {
                DAT_00aa6cfc = FUN_004d48e0();
            }
            return 0;
        }
        (&raw mut DAT_00acec49)
            .wrapping_offset(i_var2.wrapping_mul(10) as isize)
            .write_unaligned(1);
        if c_var1 != 0 {
            DAT_00aa6cfc = FUN_004d48e0();
        }
        1
    }
}
