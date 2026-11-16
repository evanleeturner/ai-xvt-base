// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006fedd0: i16;
    static mut DAT_0071f714: i16;
    static mut DAT_0070a1d0: i16;
    static mut DAT_006d4a46: u16;
    static mut DAT_0071e6ca: u16;
    static mut DAT_006d498c: u16;
    static mut DAT_007091ac: u16;
    static mut DAT_006d4980: u32;
    static mut DAT_006d4988: i32;
    static mut DAT_006d4984: u16;
    static mut DAT_0070a134: i32;
    static mut DAT_0070a00c: i32;
    static mut DAT_0070a130: u32;
    static mut DAT_0070a010: u32;
    static mut DAT_0070a12c: u32;
    static mut DAT_0070a138: u32;
    static mut DAT_006db7ac: u32;
    static mut DAT_006db7b0: u32;
    static mut DAT_006db7b4: u32;
    static mut DAT_00528230: [u16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004490f0(param_1: i32) -> i32;
    fn FUN_00449790(param_1: i32, param_2: u32) -> u32;
    fn FUN_00419100(param_1: u32, param_2: u32) -> u32;
    fn FUN_0040ca90(param_1: u32, param_2: u32) -> u32;
    fn FUN_0040cad0(param_1: u32, param_2: u32) -> u32;
    fn FUN_0047f360(
        param_1: i16,
        param_2: u16,
        param_3: i32,
        param_4: i16,
        param_5: i16,
        param_6: u32,
        param_7: i16,
    );
}

unsafe fn rd32(addr: i32) -> i32 {
    // SAFETY: the address is a game-memory location the original reads.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049c2e0(param_1: u32, param_2: i32) {
    // SAFETY: globals and callees are the game's own; pointers are read
    // at the addresses the original computes.
    unsafe {
        let mut local_4: i16 = 0;
        let s_var2 = DAT_006fedd0;
        DAT_0071f714 = param_1 as i16;
        if (param_1 as i16) == DAT_0070a1d0 {
            DAT_006fedd0 = param_1 as i16;
        }
        DAT_006d4a46 = ((param_1 as i16) == DAT_0070a1d0) as u16;
        let mut u_var9: u16 = 0;
        let i_var3 = FUN_004490f0(param_2);
        DAT_0071e6ca = i_var3 as u16;
        let mut s_var10: i16 = local_4;
        if DAT_0071e6ca != 0 {
            loop {
                DAT_006d498c = u_var9;
                let u_var4 = FUN_00449790(param_2, u_var9 as u32);
                if DAT_006d4a46 == 2 {
                    DAT_006d4a46 = 0;
                }
                if u_var9 == DAT_007091ac && DAT_006d4a46 == 0 {
                    DAT_006d4a46 = 2;
                }
                let mut c_var7: i8 = 0;
                if param_1 < DAT_006d4980 {
                    let mut i3 = rd32(
                        DAT_006d4988
                            .wrapping_add(0x1f)
                            .wrapping_add(param_1.wrapping_mul(0x23) as i32),
                    );
                    if i3 != 0 {
                        i3 = rd32(i3.wrapping_add(0xa7));
                        if i3 != 0 {
                            c_var7 = ((u_var9 as u32).wrapping_add(0x2ca).wrapping_add(i3 as u32)
                                as usize as *const i8)
                                .read();
                        }
                    }
                }
                if c_var7 == 0 && (u_var4 as i16) == 3 && param_1 < DAT_006d4980 {
                    let mut u_var5: u32 = 0;
                    let mut i3 = rd32(
                        DAT_006d4988
                            .wrapping_add(0x1f)
                            .wrapping_add(param_1.wrapping_mul(0x23) as i32),
                    );
                    if i3 != 0 {
                        i3 = rd32(i3.wrapping_add(0xa7));
                        if i3 != 0 {
                            u_var5 = ((DAT_0071e6ca as u32)
                                .wrapping_add(0x2ca)
                                .wrapping_add(i3 as u32)
                                as usize as *const u8)
                                .read() as u32;
                        }
                    }
                    let u_var1 = (&raw mut DAT_00528230)
                        .cast::<u16>()
                        .add(u_var5 as usize)
                        .read();
                    if 0x7fff < u_var1 && u_var1 < 0xff00 {
                        if local_4 == 0 {
                            let mut a = DAT_0070a134;
                            if a < 0 {
                                a = a.wrapping_neg();
                            }
                            let mut b = DAT_0070a00c;
                            if b < 0 {
                                b = b.wrapping_neg();
                            }
                            let mut u5 = DAT_0070a130;
                            let mut u6 = DAT_0070a010;
                            if a < b {
                                u5 = DAT_0070a12c;
                                u6 = DAT_0070a138;
                            }
                            if (u5 as i32) < 0 {
                                let r = FUN_00419100(u6, 0u32.wrapping_sub(u5));
                                s_var10 = r as i16;
                            } else {
                                let r = FUN_00419100(u6, u5);
                                s_var10 = 0u32.wrapping_sub((r as i16 as u16) as u32) as i16;
                            }
                            local_4 = 1;
                        }
                        let stmp = ((DAT_006d4988
                            .wrapping_add(0x14)
                            .wrapping_add(param_1.wrapping_mul(0x23) as i32))
                            as usize as *const i16)
                            .read_unaligned();
                        s_var10 = s_var10.wrapping_add(stmp);
                        let u5 = FUN_0040ca90(DAT_006db7ac, DAT_006db7b4);
                        let m5 = (u5 & 0xffff0000) as i32;
                        if m5 < 1 && -0x10001 < m5 {
                            let u6 = FUN_0040cad0(DAT_006db7b0, DAT_006db7b4);
                            let m6 = (u6 & 0xffff0000) as i32;
                            if m6 < 1 && -0x10001 < m6 {
                                FUN_0047f360(
                                    DAT_0071f714,
                                    u_var1,
                                    0x100,
                                    u5 as i16,
                                    (((DAT_006d4984 & 0xfffe) as i32)
                                        .wrapping_sub(u6 as i16 as i32))
                                        as i16,
                                    DAT_006db7b4,
                                    s_var10,
                                );
                            }
                        }
                    }
                }
                u_var9 = u_var9.wrapping_add(1);
                if !(u_var9 < DAT_0071e6ca) {
                    break;
                }
            }
        }
        DAT_006fedd0 = s_var2;
    }
}
