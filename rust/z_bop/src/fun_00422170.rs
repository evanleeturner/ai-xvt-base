// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052361c: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_009a8c18: i32;
    static mut DAT_009a8d50: i32;
    static mut DAT_009cd266: i16;
    static mut DAT_009d114c: i16;
    static mut DAT_009d12e0: i16;
    static mut DAT_009e95f2: i16;
    static mut DAT_009e9646: i16;
    static mut DAT_009e9650: i16;
    static mut DAT_009e9652: i16;
    static mut DAT_009e9654: i16;
    static mut DAT_009ec458: i16;
    static mut DAT_009ec462: i16;
    static mut DAT_009ec616: u8;
    static mut DAT_009ec617: u8;
    static mut DAT_009ed230: i16;
    static mut DAT_009fe7d4: i32;
    static mut DAT_00a00850: i16;
    static mut DAT_00a00852: i16;
    static mut DAT_00a60a50: i32;
    static mut DAT_009a57e0: [i32; 0];
    static mut DAT_009ec818: [u16; 256];
    static mut DAT_009ec618: [i16; 256];
    static mut DAT_00523610: [u8; 0];
    static mut DAT_00523600: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004226a0() -> i32;
    fn FUN_004214a0(p: *mut i32, a: i32, b: i32);
    fn FUN_004213e0(p: *mut i32, a: i32, b: i32);
    fn FUN_004214f0(p: *mut i32, a: i32, b: i32);
    fn FUN_00421430(p: *mut i32, a: i32, b: i32);
    fn FUN_00422710() -> i32;
    fn FUN_00422560();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00422170(param_1: *mut u8, param_2: i32) {
    let mut param_1 = param_1;
    // SAFETY: all globals and callees are the game's own; param_1 points at
    // the caller's command stream, read where the original reads.
    unsafe {
        DAT_0052361c = 1;
        if FUN_004226a0() == 0 {
            return;
        }
        DAT_00a00852 = DAT_009e9654;
        DAT_00a00850 = DAT_009e9652;
        DAT_009e9646 = DAT_009ec458;
        DAT_009e9650 = DAT_009d114c;
        let base = DAT_009fe7d4;
        if DAT_009e95f2 < 1 {
            let v = ((DAT_009ed230 as i32).wrapping_sub(DAT_009e9652 as i32) as u32)
                .wrapping_mul(DAT_009a8d50 as u32)
                .wrapping_add((DAT_009e9654 as i32 as u32).wrapping_mul(DAT_005233d8 as u32))
                .wrapping_add(DAT_009a8c18 as u32);
            ((base.wrapping_add(0x25b4)) as *mut i32).write_unaligned(v as i32);
            ((base.wrapping_add(0x3eb8)) as *mut i32).write_unaligned(DAT_009a8d50);
        } else {
            let v = (DAT_009e9654 as i32 as u32)
                .wrapping_mul(DAT_005233d8 as u32)
                .wrapping_add((DAT_009e9652 as i32 as u32).wrapping_mul(DAT_009a8d50 as u32))
                .wrapping_add(DAT_009a8c18 as u32);
            ((base.wrapping_add(0x25b4)) as *mut i32).write_unaligned(v as i32);
            ((base.wrapping_add(0x3eb8)) as *mut i32)
                .write_unaligned(0u32.wrapping_sub(DAT_009a8d50 as u32) as i32);
        }
        let mut local_10: u8 = 0;
        let mut local_f: u8 = 0;
        let mut local_11: u8 = 0;
        let mut b_var2: u8 = 0;
        let mut local_12: u8 = 0;
        DAT_009a8c18 = ((DAT_009fe7d4.wrapping_add(0x25b4)) as *const i32).read_unaligned();
        DAT_009cd266 = 1;
        DAT_009d12e0 = 0;
        let mut b_var8: u8 = param_1.read();
        while b_var8 != 0xff {
            let mut u_var9: u16 = local_11 as u16;
            b_var8 = local_f;
            if DAT_009ec616.wrapping_add(local_10) < local_10 {
                b_var8 = local_f.wrapping_add(1);
                if b_var8 == 0 {
                    u_var9 = local_11.wrapping_add(1) as u16;
                }
            }
            if b_var8.wrapping_add(DAT_009ec617) < b_var8 {
                u_var9 = (u_var9 as u8).wrapping_add(1) as u16;
            }
            let mut i_var6: i32 = 0;
            let mut i_var12: i32 = 0;
            let s_var10: i16 = u_var9
                .wrapping_mul(0x100)
                .wrapping_sub(
                    (local_11 as u16)
                        .wrapping_mul(0x100)
                        .wrapping_add(local_f as u16),
                )
                .wrapping_add(b_var8.wrapping_add(DAT_009ec617) as u16)
                as i16;
            let mut local_e: i16 = s_var10;
            if DAT_0052361c == 1 {
                b_var8 = param_1.read();
                let mut pi3: *mut i32 = (&raw mut DAT_009a57e0).cast::<i32>();
                let mut u9: u16 = 0;
                while b_var8 != 0xfe {
                    b_var8 = param_1.read();
                    let mut pi11 = pi3;
                    let i_var7: i32;
                    let u_var4: u16;
                    if b_var8 == 0xfb {
                        b_var2 = param_1.add(1).read();
                        param_1 = param_1.add(3);
                        i_var7 = i_var12;
                        u_var4 = u9;
                    } else if b_var8 == 0xfc {
                        let idx = param_1.add(1).read() as usize;
                        param_1 = param_1.add(2);
                        let mut t = i_var12.wrapping_add(DAT_009ec818[idx] as i32);
                        u_var4 = u9.wrapping_add(DAT_009ec618[idx] as u16);
                        if u_var4 < u9 {
                            t = t.wrapping_add(1);
                        }
                        i_var7 = t;
                    } else {
                        if b_var8 == 0xfd {
                            b_var8 = param_1.add(1).read();
                            local_12 = param_1.add(2).read();
                            param_1 = param_1.add(3);
                        } else {
                            param_1 = param_1.add(1);
                            let sh = (&raw const DAT_00523610)
                                .cast::<u8>()
                                .offset(param_2 as isize)
                                .read()
                                & 0x1f;
                            local_12 = ((b_var8 as u32) >> sh).wrapping_add(b_var2 as u32) as u8;
                            b_var8 &= (&raw const DAT_00523600)
                                .cast::<u8>()
                                .offset(param_2 as isize)
                                .read();
                        }
                        let mut t = i_var12.wrapping_add(DAT_009ec818[b_var8 as usize] as i32);
                        u_var4 = u9.wrapping_add(DAT_009ec618[b_var8 as usize] as u16);
                        if u_var4 < u9 {
                            t = t.wrapping_add(1);
                        }
                        i_var7 = t;
                        pi3.write_unaligned(i_var12);
                        pi11 = pi3.add(3);
                        i_var6 = i_var6.wrapping_add(1);
                        pi3.add(1).write_unaligned(local_12 as i32);
                        pi3.add(2)
                            .write_unaligned(i_var7.wrapping_sub(i_var12).wrapping_add(1));
                    }
                    i_var12 = i_var7;
                    pi3 = pi11;
                    u9 = u_var4;
                    b_var8 = param_1.read();
                }
                param_1 = param_1.add(1);
            }
            loop {
                if i_var6 != 0 && -1 < DAT_009d114c {
                    let s_var5 = DAT_009ec462.wrapping_add(i_var12 as i16);
                    DAT_00a60a50 = i_var6;
                    let arr = (&raw mut DAT_009a57e0).cast::<i32>();
                    let arg = DAT_009fe7d4.wrapping_add(0x25b8);
                    if s_var5 < 0
                        || DAT_009d114c <= s_var5
                        || DAT_009ec462 < 0
                        || DAT_009ec462 < DAT_009ec458
                    {
                        if DAT_005233d8 == 2 {
                            FUN_004214f0(arr, DAT_009a8c18, arg);
                        } else {
                            FUN_00421430(arr, DAT_009a8c18, arg);
                        }
                    } else if DAT_005233d8 == 2 {
                        FUN_004214a0(arr, DAT_009a8c18, arg);
                    } else {
                        FUN_004213e0(arr, DAT_009a8c18, arg);
                    }
                }
                if s_var10 != 0 {
                    if FUN_00422710() == 0 {
                        return;
                    }
                    FUN_00422560();
                }
                if local_e.wrapping_sub(1) != 0 {
                    local_e = local_e.wrapping_sub(1);
                    if s_var10 != 0 {
                        continue;
                    }
                }
                break;
            }
            if local_10.wrapping_add(DAT_009ec616) < local_10 {
                local_f = local_f.wrapping_add(1);
            }
            if local_f.wrapping_add(DAT_009ec617) < local_f {
                local_11 = local_11.wrapping_add(1);
            }
            local_10 = local_10.wrapping_add(DAT_009ec616);
            local_f = local_f.wrapping_add(DAT_009ec617);
            b_var8 = param_1.read();
        }
    }
}
