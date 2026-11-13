// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_008050aa: u32;
    static mut DAT_008050ae: u32;
    static mut DAT_008050a6: u32;
    static mut DAT_007810a6: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cec30(param_1: u32) -> u32 {
    let mut param_1 = param_1;
    // SAFETY: the globals are the game's own memory; the record array is
    // indexed and copied word by word exactly as the original does.
    unsafe {
        if DAT_008050aa == param_1 {
            DAT_008050aa = DAT_008050aa.wrapping_add(1);
            DAT_008050ae = DAT_008050ae.wrapping_sub(1);
            if 0x3ff < DAT_008050aa {
                DAT_008050aa = 0;
            }
            return 1;
        }
        let mut u_var3 = param_1.wrapping_add(1);
        if 0x3ff < u_var3 {
            u_var3 = 0;
        }
        let mut u_var4 = DAT_008050ae.wrapping_add(DAT_008050aa);
        if 0x3ff < u_var4 {
            u_var4 = u_var4.wrapping_sub(0x400);
        }
        let base = (&raw mut DAT_007810a6) as *mut u32;
        while u_var4 != u_var3 {
            let mut u_var2 = param_1.wrapping_add(1);
            let mut src = base.wrapping_add(u_var3.wrapping_mul(0x84) as usize);
            let mut dst = base.wrapping_add(param_1.wrapping_mul(0x84) as usize);
            let mut i_var1: i32 = 0x84;
            while i_var1 != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.wrapping_add(1);
                dst = dst.wrapping_add(1);
                i_var1 -= 1;
            }
            if 0x3ff < u_var2 {
                u_var2 = 0;
            }
            u_var3 = u_var3.wrapping_add(1);
            param_1 = u_var2;
            if 0x3ff < u_var3 {
                u_var3 = 0;
            }
        }
        DAT_008050ae = DAT_008050ae.wrapping_sub(1);
        if DAT_008050a6 == 0 {
            DAT_008050a6 = 0x3ff;
            return 0;
        }
        DAT_008050a6 = DAT_008050a6.wrapping_sub(1);
        0
    }
}
