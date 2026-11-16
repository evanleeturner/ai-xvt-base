// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051bea0: [u8; 1];
    static mut DAT_00a65ce0: [u8; 1];
    static mut DAT_00a60a60: [u8; 1];
    static mut DAT_00a633c0: [u8; 1];
    static mut DAT_00a686e0: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004269e0(param_1: u32) -> u32;
    fn FUN_00415c40(param_1: u32, param_2: u32, param_3: *mut u8) -> u32;
    fn FUN_004a9960(param_1: *mut u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00415a70(
    param_1: u32,
    param_2: u16,
    param_3: u16,
    param_4: i16,
) -> u32 {
    let extraout_var: u16 = 0;
    let extraout_edx: u32 = 0;
    let mut pb_var3: *mut u8 = core::ptr::null_mut();
    let mut u_var2: u32 = 0;
    let mut u_var4: u16 = 0;
    // SAFETY: the globals and callees are the game's own memory and code;
    // indices and offsets are used exactly as the original computes them.
    unsafe {
        let flag = (&raw mut DAT_0051bea0 as *mut u8)
            .add(param_2 as usize)
            .read();
        if flag != 0x01 {
            u_var2 = param_3 as u32;
        }
        let u_var5: u32 = param_2.wrapping_add(param_4 as u16) as u32;
        let off: u32 = u_var2.wrapping_add(u_var5.wrapping_mul(0xe));
        let mut u_var1 = FUN_004269e0(param_1);
        if (u_var1 as i16) as i32 == 0xff {
            let p = param_1 as u16;
            if 0x45 < p && p < 0x55 {
                pb_var3 = (&raw mut DAT_00a65ce0 as *mut u8).byte_add(off as usize);
                u_var2 = FUN_00415c40(0, 0, pb_var3);
                u_var4 = u_var2 as u16;
                u_var1 = FUN_004a9960(pb_var3);
            }
            return ((((u_var1 >> 0x10) as i16) as i32).wrapping_shl(0x10) as u32)
                | (u_var4 as u32);
        }
        let u_var1 = (&raw mut DAT_00a686e0 as *mut u8)
            .add((u_var1 & 0xffff) as usize)
            .read() as u32;
        if u_var1 == 0 {
            pb_var3 = (&raw mut DAT_00a65ce0 as *mut u8).byte_add(off as usize);
        } else if u_var1 == 1 {
            pb_var3 = (&raw mut DAT_00a60a60 as *mut u8).byte_add(off as usize);
        } else if u_var1 == 2 {
            pb_var3 = (&raw mut DAT_00a633c0 as *mut u8).byte_add(off as usize);
        }
        u_var2 = FUN_00415c40(u_var1, extraout_edx, pb_var3);
        FUN_004a9960(pb_var3);
        ((extraout_var as u32) << 0x10) | (((u_var2 as i16) as i32) as u32)
    }
}
