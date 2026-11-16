// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009cc450: i16;
    static mut DAT_009ed664: i16;
    static mut DAT_009d8110: i16;
    static mut DAT_009a20a6: u16;
    static mut DAT_00a00870: [u32; 1];
    static mut DAT_00a00874: [u32; 1];
    static mut DAT_009ec60a: u16;
    static mut DAT_009a1fe6: u16;
    static mut DAT_009d682e: u16;
    static mut DAT_009a1fe0: u32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a1fe4: u16;
    static mut DAT_0051a0b0: [u16; 1];
    static mut DAT_009d77b4: i32;
    static mut DAT_009d768c: i32;
    static mut DAT_009d77b0: u32;
    static mut DAT_009d7690: u32;
    static mut DAT_009d77ac: u32;
    static mut DAT_009d77b8: u32;
    static mut DAT_009a8e2c: i32;
    static mut DAT_009a8e30: i32;
    static mut DAT_009a8e34: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004adc40(param_1: i32) -> i32;
    fn FUN_004ae2a0(param_1: i32, param_2: u32) -> u32;
    fn FUN_0046abf0(param_1: u32, param_2: u32) -> u32;
    fn FUN_00427570(param_1: i32, param_2: i32) -> u32;
    fn FUN_00427590(param_1: i32, param_2: i32) -> u32;
    fn FUN_00401250(
        param_1: i16,
        param_2: u16,
        param_3: i32,
        param_4: i16,
        param_5: i16,
        param_6: i32,
        param_7: i16,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041fc50(param_1: u32, param_2: i32) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers computed from game tables are read where the original reads.
    unsafe {
        let s_var2 = DAT_009cc450;
        DAT_009ed664 = param_1 as i16;
        if (param_1 as i16) == DAT_009d8110 {
            DAT_009cc450 = param_1 as i16;
        }
        DAT_009a20a6 = ((param_1 as i16) == DAT_009d8110) as u16;
        let idx = (param_2 as u32).wrapping_mul(0x65) as usize;
        let tab0 = (&raw mut DAT_00a00870).cast::<u32>();
        let tab4 = (&raw mut DAT_00a00874).cast::<u32>();
        if param_2 < 0x49 {
            DAT_009ec60a = tab0.wrapping_add(idx).read() as u16;
        } else {
            DAT_009ec60a = FUN_004adc40(param_2) as u16;
        }
        let mut u_stack_2: u16 = 0;
        let mut local_6: i16 = 0;
        let mut s_var9: i16 = local_6;
        if DAT_009ec60a != 0 {
            loop {
                DAT_009a1fe6 = u_stack_2;
                let mut u_var10 = u_stack_2 as u32;
                let s_var3: i16;
                if param_2 < 0x49 {
                    let mut u_var6 = u_var10;
                    let cnt = tab0.wrapping_add(idx).read();
                    if (cnt as i32) <= (u_var10 as i32) {
                        u_var6 = cnt.wrapping_sub(1);
                    }
                    s_var3 = tab4.wrapping_add(idx.wrapping_add(u_var6 as usize)).read() as i16;
                } else {
                    s_var3 = FUN_004ae2a0(param_2, u_var10) as i16;
                }
                if DAT_009a20a6 == 2 {
                    DAT_009a20a6 = 0;
                }
                if u_stack_2 == DAT_009d682e && DAT_009a20a6 == 0 {
                    DAT_009a20a6 = 2;
                }
                let mut skip = false;
                if param_1 < DAT_009a1fe0 {
                    let mut c_var7: u8 = 0;
                    let mut i_var4 = ((DAT_009a1fe8
                        .wrapping_add(0x1f)
                        .wrapping_add(param_1.wrapping_mul(0x23)))
                        as *const i32)
                        .read_unaligned();
                    if i_var4 != 0 {
                        i_var4 =
                            ((i_var4 as u32).wrapping_add(0xa9) as *const i32).read_unaligned();
                        if i_var4 != 0 {
                            c_var7 = (u_var10.wrapping_add(0x2d8).wrapping_add(i_var4 as u32)
                                as *const u8)
                                .read();
                        }
                    }
                    if c_var7 != 0 {
                        skip = true;
                    }
                }
                if !skip && s_var3 == 3 && param_1 < DAT_009a1fe0 {
                    u_var10 = 0;
                    let mut i_var4 = ((DAT_009a1fe8
                        .wrapping_add(0x1f)
                        .wrapping_add(param_1.wrapping_mul(0x23)))
                        as *const i32)
                        .read_unaligned();
                    if i_var4 != 0 {
                        i_var4 =
                            ((i_var4 as u32).wrapping_add(0xa9) as *const i32).read_unaligned();
                        if i_var4 != 0 {
                            u_var10 = ((DAT_009ec60a as u32)
                                .wrapping_add(0x2d8)
                                .wrapping_add(i_var4 as u32)
                                as *const u8)
                                .read() as u32;
                        }
                    }
                    let u_var1 = (&raw mut DAT_0051a0b0)
                        .cast::<u16>()
                        .wrapping_add(u_var10 as usize)
                        .read();
                    if 0x7fff < u_var1 && u_var1 < 0xff00 {
                        if local_6 == 0 {
                            let mut i4 = DAT_009d77b4;
                            if i4 < 0 {
                                i4 = i4.wrapping_neg();
                            }
                            let mut i8 = DAT_009d768c;
                            if i8 < 0 {
                                i8 = i8.wrapping_neg();
                            }
                            let mut v10 = DAT_009d77b0;
                            let mut v6 = DAT_009d7690;
                            if i4 < i8 {
                                v10 = DAT_009d77ac;
                                v6 = DAT_009d77b8;
                            }
                            if (v10 as i32) < 0 {
                                let r = FUN_0046abf0(v6, 0u32.wrapping_sub(v10));
                                s_var9 = r as i16;
                            } else {
                                let r = FUN_0046abf0(v6, v10);
                                s_var9 = 0u32.wrapping_sub((r as u16) as u32) as i16;
                            }
                            local_6 = 1;
                        }
                        let stmp = ((DAT_009a1fe8
                            .wrapping_add(0x14)
                            .wrapping_add(param_1.wrapping_mul(0x23)))
                            as *const i16)
                            .read_unaligned();
                        s_var9 = (s_var9 as i32).wrapping_add(stmp as i32) as i16;
                        let v10 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
                        if ((v10 & 0xffff0000) as i32) < 1 && -0x10001 < ((v10 & 0xffff0000) as i32)
                        {
                            let v6 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
                            if ((v6 & 0xffff0000) as i32) < 1
                                && -0x10001 < ((v6 & 0xffff0000) as i32)
                            {
                                FUN_00401250(
                                    DAT_009ed664,
                                    u_var1,
                                    0x100,
                                    v10 as i16,
                                    ((DAT_009a1fe4 & 0xfffe) as i32)
                                        .wrapping_sub((v6 as i16) as i32)
                                        as i16,
                                    DAT_009a8e34,
                                    s_var9,
                                );
                            }
                        }
                    }
                }
                u_stack_2 = u_stack_2.wrapping_add(1);
                if !(u_stack_2 < DAT_009ec60a) {
                    break;
                }
            }
        }
        DAT_009cc450 = s_var2;
    }
}
