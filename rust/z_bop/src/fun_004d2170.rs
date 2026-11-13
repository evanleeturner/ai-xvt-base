// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b53575: u32;
    static mut DAT_00b53579: u32;
    static mut DAT_00b53571: u32;
    static mut DAT_00acf571: [u32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d2170(param_1: u32) -> u32 {
    let mut param_1 = param_1;
    // SAFETY: the globals are the game's own memory; the array is read and
    // written at the same dword offsets the original uses.
    unsafe {
        if DAT_00b53575 == param_1 {
            DAT_00b53575 = DAT_00b53575.wrapping_add(1);
            DAT_00b53579 = DAT_00b53579.wrapping_sub(1);
            if 0x3ff < (DAT_00b53575 as i32) {
                DAT_00b53575 = 0;
            }
            return 1;
        }
        let mut u_var3 = param_1.wrapping_add(1);
        if 0x3ff < u_var3 {
            u_var3 = 0;
        }
        let mut u_var4 = DAT_00b53575.wrapping_add(DAT_00b53579);
        if 0x3ff < u_var4 {
            u_var4 = u_var4.wrapping_sub(0x400);
        }
        let base = (&raw mut DAT_00acf571) as usize;
        while u_var4 != u_var3 {
            let mut u_var2 = param_1.wrapping_add(1);
            let mut src =
                base.wrapping_add(u_var3.wrapping_mul(0x84).wrapping_mul(4) as usize) as *const u32;
            let mut dst =
                base.wrapping_add(param_1.wrapping_mul(0x84).wrapping_mul(4) as usize) as *mut u32;
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
        DAT_00b53579 = DAT_00b53579.wrapping_sub(1);
        if DAT_00b53571 == 0 {
            DAT_00b53571 = 0x3ff;
            return 0;
        }
        DAT_00b53571 = DAT_00b53571.wrapping_sub(1);
        0
    }
}
