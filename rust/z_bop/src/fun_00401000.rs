// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009ed664: i16;
    static mut DAT_00a90a60: i32;
    static mut DAT_00a90a5e: u16;
    static mut DAT_009a1fe6: u16;
    static mut DAT_009a1fe4: i16;
    static mut DAT_00521f26: [u8; 0];
    static mut DAT_0051a060: [u16; 0];
    static mut DAT_009a8e34: u32;
    static mut DAT_009a8e2c: i32;
    static mut DAT_009a8e30: i32;
    static mut DAT_009d77b4: i32;
    static mut DAT_009d768c: i32;
    static mut DAT_009d77b0: u32;
    static mut DAT_009d7690: u32;
    static mut DAT_009d77ac: u32;
    static mut DAT_009d77b8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004728d0(param_1: i32, param_2: u32);
    fn FUN_0046abf0(param_1: u32, param_2: u32) -> u32;
    fn FUN_00427570(param_1: i32, param_2: u32) -> u32;
    fn FUN_00427590(param_1: i32, param_2: u32) -> u32;
    fn FUN_00401250(
        param_1: i16,
        param_2: u16,
        param_3: u16,
        param_4: i16,
        param_5: i16,
        param_6: u32,
        param_7: i16,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00401000(param_1: i32) {
    // SAFETY: all globals and callees are the game's own; the pointers
    // computed from game tables are read where the original reads them.
    unsafe {
        let i_var2 = (param_1 as u32).wrapping_mul(0x23) as i32;
        let mut i_var8 = DAT_009a1fe8.wrapping_add(i_var2);
        DAT_009ed664 = param_1 as i16;
        let mut local_2: u16 = (i_var8.wrapping_add(3) as *const u8).read() as u16;
        let src = (&raw mut DAT_00521f26)
            .cast::<u8>()
            .byte_add((local_2 as usize).wrapping_mul(0x1a));
        DAT_00a90a60 = src.cast::<i32>().read_unaligned();
        let mut u_var10: u16;
        if local_2 == 0x59 {
            u_var10 = ((i_var8.wrapping_add(0x19) as *const u8).read() >> 1) as u16;
        } else {
            if DAT_00a90a60 == 0 {
                return;
            }
            DAT_00a90a5e = (i_var8.wrapping_add(0x19) as *const u8).read() as u16;
            u_var10 = ((DAT_00a90a60.wrapping_add((DAT_00a90a5e as i32).wrapping_mul(2)))
                as *const u16)
                .read_unaligned();
        }
        if u_var10 < 0xff00 {
            if u_var10 < 0x8000 {
                if local_2 == 0x59 {
                    let tmp = (i_var8.wrapping_add(0x1f) as *const i32).read_unaligned();
                    local_2 = (tmp.wrapping_add(0x87) as *const u8).read() as u16;
                }
                DAT_009a1fe6 = u_var10;
                FUN_004728d0(i_var8, u_var10 as u32);
                if local_2 == 0x59 {
                    DAT_00a90a5e = (DAT_009a1fe8.wrapping_add(0x1a).wrapping_add(i_var2)
                        as *const u8)
                        .read() as u16;
                    u_var10 = (&raw mut DAT_0051a060)
                        .cast::<u16>()
                        .add(DAT_00a90a5e as usize)
                        .read_unaligned();
                }
            }
            if u_var10 < 0xff00 && 0x7fff < u_var10 && (DAT_009a8e34 as i32) >= 0 {
                i_var8 = DAT_009d77b4;
                if DAT_009d77b4 < 0 {
                    i_var8 = DAT_009d77b4.wrapping_neg();
                }
                let mut i_var9 = DAT_009d768c;
                if DAT_009d768c < 0 {
                    i_var9 = DAT_009d768c.wrapping_neg();
                }
                let mut u_var6 = DAT_009d77b0;
                let mut u_var7 = DAT_009d7690;
                if i_var8 < i_var9 {
                    u_var6 = DAT_009d77ac;
                    u_var7 = DAT_009d77b8;
                }
                let s_var3: i16;
                if (u_var6 as i32) < 0 {
                    let u_var5 = FUN_0046abf0(u_var7, 0u32.wrapping_sub(u_var6));
                    s_var3 = u_var5 as i16;
                } else {
                    let u_var5 = FUN_0046abf0(u_var7, u_var6);
                    s_var3 = 0u32.wrapping_sub((u_var5 as u16) as u32) as i16;
                }
                u_var6 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
                let m6 = (u_var6 & 0xffff0000) as i32;
                if m6 < 1 && -0x10001 < m6 {
                    u_var7 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
                    let m7 = (u_var7 & 0xffff0000) as i32;
                    if m7 < 1 && -0x10001 < m7 {
                        let tmp = (DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var2)
                            as *const i32)
                            .read_unaligned();
                        let b_var1 = (tmp.wrapping_add(1) as *const u8).read();
                        let mut u_var4: u16;
                        if b_var1 == 0 {
                            u_var4 = 0x100;
                        } else {
                            u_var4 = (b_var1 as u16).wrapping_mul(0x40);
                            if 0xff < u_var4 {
                                u_var4 = u_var4.wrapping_add(0x100);
                            }
                        }
                        FUN_00401250(
                            DAT_009ed664,
                            u_var10,
                            u_var4,
                            u_var6 as i16,
                            DAT_009a1fe4.wrapping_sub(u_var7 as i16),
                            DAT_009a8e34,
                            s_var3,
                        );
                    }
                }
            }
        }
    }
}
