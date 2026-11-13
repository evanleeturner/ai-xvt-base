// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f864: i32;
    static mut DAT_00a68750: [u32; 0x400];
    static mut DAT_00a697c8: [u32; 0];
    static mut DAT_00a69750: [u8; 0];
    static mut DAT_00a8f7c8: [u8; 0];
    static mut DAT_0051a5d7: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004acc30(param_1: *const i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004079f0(param_1: u32) -> *mut core::ffi::c_void {
    // SAFETY: all globals and the callee are the game's own memory; the
    // pointers stay within the tables the original walks.
    unsafe {
        if DAT_0052f864 == -1 {
            let mut p_var2 = (&raw mut DAT_00a697c8).cast::<u32>();
            let mut p_var4 = (&raw mut DAT_00a68750).cast::<u32>();
            let mut i_var1: i32 = 0x400;
            while i_var1 != 0 {
                p_var4.write_unaligned(0);
                p_var4 = p_var4.add(1);
                i_var1 = i_var1.wrapping_sub(1);
            }
            let end = (&raw mut DAT_00a8f7c8).cast::<u32>();
            loop {
                p_var2.write_unaligned(0);
                p_var2 = p_var2.wrapping_add(0x26);
                if !(p_var2 < end) {
                    break;
                }
            }
        }
        let mut i_var1: i32 = 0;
        DAT_0052f864 = (param_1 & 0x3ff) as i32;
        let mut u_var3: u32 = DAT_0052f864 as u32;
        let tab = (&raw mut DAT_00a68750).cast::<u32>();
        loop {
            if tab.add(u_var3 as usize).read_unaligned() == param_1 {
                break;
            }
            u_var3 = u_var3.wrapping_add(1);
            if u_var3 == 0x400 {
                u_var3 = 0;
            }
            i_var1 = i_var1.wrapping_add(1);
            if !(i_var1 < 0x400) {
                break;
            }
        }
        let base = (&raw mut DAT_00a69750).cast::<u8>();
        if i_var1 == 0x400 {
            i_var1 = 0;
            let flags = (&raw mut DAT_00a697c8).cast::<u32>();
            loop {
                let idx = DAT_0052f864.wrapping_mul(0x26) as isize;
                if flags.offset(idx).read_unaligned() == 0 {
                    break;
                }
                DAT_0052f864 = DAT_0052f864.wrapping_add(1);
                if DAT_0052f864 == 0x400 {
                    DAT_0052f864 = 0;
                }
                i_var1 = i_var1.wrapping_add(1);
                if !(i_var1 < 0x400) {
                    break;
                }
            }
            u_var3 = DAT_0052f864 as u32;
            if i_var1 == 0x400 {
                FUN_004acc30((&raw mut DAT_0051a5d7).cast::<i8>().add(1));
                let off = (DAT_0052f864.wrapping_mul(0x98)) as isize;
                return base.offset(off).cast::<core::ffi::c_void>();
            }
        }
        DAT_0052f864 = u_var3 as i32;
        tab.add(u_var3 as usize).write_unaligned(param_1);
        base.wrapping_add(u_var3.wrapping_mul(0x98) as usize)
            .cast::<core::ffi::c_void>()
    }
}
