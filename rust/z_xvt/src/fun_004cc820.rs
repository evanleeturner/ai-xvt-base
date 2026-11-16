// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759645: i8;
    static mut DAT_007806fa: i32;
    static mut DAT_008050aa: i32;
    static mut DAT_008050ae: i32;
    static mut DAT_00758830: i32;
    static mut DAT_007810a6: i32;
    static mut DAT_007810b6: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d4d90();
    fn FUN_004ca990();
    fn FUN_004d4d00() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cc820(param_1: i32) -> i32 {
    // SAFETY: the globals and callees are the game's own; the table reads
    // stay at the same computed offsets as the original.
    unsafe {
        let c_var2 = DAT_00759645;
        if DAT_007806fa == 0 {
            return 0;
        }
        FUN_004d4d90();
        FUN_004ca990();
        let mut i_var3 = DAT_008050aa;
        let mut i_var1 = DAT_008050ae;
        if i_var1 < 0x201 {
            loop {
                if i_var1 < 1 {
                    if c_var2 != 0 {
                        DAT_00758830 = FUN_004d4d00();
                    }
                    return 0;
                }
                let idx = i_var3.wrapping_mul(0x84) as isize;
                let a = (&raw mut DAT_007810a6)
                    .wrapping_offset(idx)
                    .read_unaligned();
                if a != 0 {
                    let b = (&raw mut DAT_007810b6)
                        .wrapping_offset(idx)
                        .read_unaligned();
                    if b == param_1 {
                        break;
                    }
                }
                i_var3 = i_var3.wrapping_add(1);
                if 0x3ff < i_var3 {
                    i_var3 = 0;
                }
                i_var1 = i_var1.wrapping_sub(1);
            }
            if c_var2 != 0 {
                DAT_00758830 = FUN_004d4d00();
            }
            return 1;
        }
        if c_var2 != 0 {
            DAT_00758830 = FUN_004d4d00();
        }
        1
    }
}
