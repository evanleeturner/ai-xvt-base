// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005f0fa4: i32;
    static mut DAT_006a8a60: [u32; 0];
    static mut DAT_006a9a88: [u32; 0];
    static mut DAT_006cca88: u32;
    static mut DAT_006a9a80: [i8; 0];
    static mut s_0_RAN_OUT_OF_MYCACHETEXTURES____00527a7f: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048fb30(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045bcb0(param_1: u32) -> *mut core::ffi::c_void {
    // SAFETY: all globals and the callee belong to the game; the tables are
    // indexed within their original bounds exactly as the C does.
    unsafe {
        let tab_a = (&raw mut DAT_006a8a60).cast::<u32>();
        let tab_b = (&raw mut DAT_006a9a88).cast::<u32>();
        let base = (&raw mut DAT_006a9a80).cast::<u8>();

        if DAT_005f0fa4 == -1 {
            let mut p2 = tab_b;
            let mut p4 = tab_a;
            let mut n: i32 = 0x400;
            while n != 0 {
                p4.write(0);
                p4 = p4.add(1);
                n -= 1;
            }
            loop {
                p2.write(0);
                p2 = p2.add(0x23);
                if (p2 as usize) >= (&raw mut DAT_006cca88 as usize) {
                    break;
                }
            }
        }
        let mut i_var1: i32 = 0;
        DAT_005f0fa4 = (param_1 & 0x3ff) as i32;
        let mut u_var3: u32 = DAT_005f0fa4 as u32;
        loop {
            if tab_a.add(u_var3 as usize).read() == param_1 {
                break;
            }
            u_var3 = u_var3.wrapping_add(1);
            if u_var3 == 0x400 {
                u_var3 = 0;
            }
            i_var1 += 1;
            if !(i_var1 < 0x400) {
                break;
            }
        }
        if i_var1 == 0x400 {
            i_var1 = 0;
            loop {
                let idx = DAT_005f0fa4.wrapping_mul(0x23);
                if tab_b.offset(idx as isize).read() == 0 {
                    break;
                }
                DAT_005f0fa4 = DAT_005f0fa4.wrapping_add(1);
                if DAT_005f0fa4 == 0x400 {
                    DAT_005f0fa4 = 0;
                }
                i_var1 += 1;
                if !(i_var1 < 0x400) {
                    break;
                }
            }
            u_var3 = DAT_005f0fa4 as u32;
            if i_var1 == 0x400 {
                FUN_0048fb30(
                    (&raw mut s_0_RAN_OUT_OF_MYCACHETEXTURES____00527a7f)
                        .cast::<i8>()
                        .add(1),
                );
                return base
                    .offset(DAT_005f0fa4.wrapping_mul(0x8c) as isize)
                    .cast::<core::ffi::c_void>();
            }
        }
        DAT_005f0fa4 = u_var3 as i32;
        tab_a.add(u_var3 as usize).write(param_1);
        base.offset(u_var3.wrapping_mul(0x8c) as i32 as isize)
            .cast::<core::ffi::c_void>()
    }
}
