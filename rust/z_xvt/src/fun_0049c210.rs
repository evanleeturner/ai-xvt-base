// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052accc: i32;
    static mut DAT_00645a88: i32;
    static mut DAT_0052acc8: i32;
    static mut DAT_0052acc4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049b8a0(param_1: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0049c210() {
    // SAFETY: the globals and the callee are the game's own; the pointers
    // walk the game's packed record buffer exactly as the original does.
    unsafe {
        let mut i_var4: i32 = 0;
        while DAT_0052accc != 0 {
            DAT_0052accc = DAT_0052accc.wrapping_sub(1);
            let p_var3 = DAT_00645a88.wrapping_add(i_var4) as usize as *mut u32;
            i_var4 = i_var4.wrapping_add(9);
            let mut pb_var6 = (p_var3 as *mut u8).wrapping_add(9);
            let mut u_var2: u32 = u32::from((p_var3 as *const u8).wrapping_add(8).read());
            while u_var2 != 0 {
                i_var4 = i_var4.wrapping_add(1);
                let mut u_var1: u32 = u32::from(pb_var6.read());
                pb_var6 = pb_var6.wrapping_add(1);
                while u_var1 != 0 {
                    let b = pb_var6.read();
                    let mut pb_var7 = pb_var6.wrapping_add(1);
                    let mut i_var5 = i_var4.wrapping_add(1);
                    if (b & 0x7f) == 0x7f {
                        pb_var7 = pb_var6.wrapping_add(5);
                        i_var5 = i_var4.wrapping_add(5);
                    }
                    if (b & 0x80) != 0 {
                        pb_var7 = pb_var7.wrapping_add(1);
                        i_var5 = i_var5.wrapping_add(1);
                    }
                    pb_var6 = pb_var7.wrapping_add(2);
                    i_var4 = i_var5.wrapping_add(2);
                    u_var1 = u_var1.wrapping_sub(1);
                }
                u_var2 = u_var2.wrapping_sub(1);
            }
            let p1 = p_var3.byte_add(4);
            p1.write_unaligned(p1.read_unaligned() & 0x7fff_ffff);
            FUN_0049b8a0(p_var3);
        }
        DAT_0052acc8 = DAT_0052acc4;
    }
}
