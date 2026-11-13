// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005251b0: u32;
    static mut DAT_005251b4: i32;
    static mut DAT_005251ac: u32;
    static mut DAT_007330a8: i32;
    static mut DAT_00739290: u32;
    static mut DAT_00733798: [i32; 0];
    static mut DAT_00539a08: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FID_conflict__memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: u32);
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004336c0() -> u32 {
    // SAFETY: all globals and callees are the game's own memory and
    // functions; the arrays are indexed exactly as the original does.
    unsafe {
        let base = (&raw mut DAT_00539a08) as *mut i32;
        let mut i_var3: i32 = 0;
        let mut u_var1: u32 = DAT_005251b0;
        let mut u_var5: u32 = DAT_005251b0;
        while 0 < DAT_005251b4 {
            let src = base.wrapping_add(u_var5.wrapping_mul(0x84) as usize);
            let mut u_var4: u32 = u_var1;
            let mut hit = src.read_unaligned() == 0;
            if !hit {
                u_var4 = u_var1;
                hit = DAT_007330a8 == src.read_unaligned();
            }
            if hit {
                u_var4 = u_var1.wrapping_add(1);
                let dst = base.wrapping_add(u_var1.wrapping_mul(0x84) as usize);
                FID_conflict__memcpy(
                    dst as *mut core::ffi::c_void,
                    src as *const core::ffi::c_void,
                    0x210,
                );
                if 0x3ff < u_var4 {
                    u_var4 = 0;
                }
                i_var3 = i_var3.wrapping_add(1);
            }
            u_var5 = u_var5.wrapping_add(1);
            if 0x3ff < u_var5 {
                u_var5 = 0;
            }
            u_var1 = u_var4;
            DAT_005251b4 = DAT_005251b4.wrapping_sub(1);
        }
        u_var5 = 0;
        DAT_005251ac = u_var1;
        DAT_005251b4 = i_var3;
        if DAT_00739290 != 0 {
            let mut pi_var6 = (&raw mut DAT_00733798) as *mut i32;
            loop {
                if pi_var6.read_unaligned() != DAT_007330a8 {
                    pi_var6
                        .wrapping_add(1)
                        .write_unaligned(pi_var6.wrapping_add(2).read_unaligned());
                    pi_var6
                        .wrapping_add(3)
                        .write_unaligned(pi_var6.wrapping_add(4).read_unaligned());
                    pi_var6
                        .wrapping_add(5)
                        .write_unaligned(pi_var6.wrapping_add(6).read_unaligned());
                    let d_var2 = timeGetTime();
                    pi_var6.wrapping_add(0x89).write_unaligned(d_var2 as i32);
                }
                pi_var6 = pi_var6.wrapping_add(0x8e);
                u_var5 = u_var5.wrapping_add(1);
                if !(u_var5 < DAT_00739290) {
                    break;
                }
            }
        }
        1
    }
}
