// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7209: i32;
    static mut DAT_00aa720d: i32;
    static mut DAT_00aa7211: i32;
    static mut DAT_00aa7215: i32;
    static mut DAT_00aa7346: i32;
    static mut DAT_00aa734a: i32;
    static mut DAT_00aa734e: i32;
    static mut DAT_00aa7352: i32;
    static mut DAT_00aa7356: u32;
    static mut DAT_00aa735a: i32;
    static mut DAT_00aa735e: i8;
    static mut DAT_00aa7b13: i32;
    static mut DAT_00aa7b1f: i32;
    static mut DAT_00aa6cfc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d48e0() -> i32;
    fn FUN_004d4970();
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004d64e0(param_1: *mut i32);
    fn FUN_004b5570(param_1: *mut i8, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ddc90() {
    // SAFETY: all globals, callees and the addresses derived from them are the
    // game's own memory; reads and writes follow the original's order.
    unsafe {
        let i_var2: i32 = (&raw const DAT_00aa7352).read_unaligned();
        let i_var1: i32 = (&raw const DAT_00aa734e).read_unaligned();
        let x209: i32 = (&raw const DAT_00aa7209).read_unaligned();
        if !(-1 < x209 && x209 < 0x280) {
            return;
        }
        let x20d: i32 = (&raw const DAT_00aa720d).read_unaligned();
        if !(-1 < x20d && x20d < 0x1e0) {
            return;
        }
        let mut local_20: [i32; 4] = [0; 4];
        let mut local_10: [i32; 4] = [0; 4];
        local_20[0] = 0;
        local_20[2] = i_var1.wrapping_sub(1);
        local_20[1] = 0;
        local_20[3] = i_var2.wrapping_sub(1);
        let x209: i32 = (&raw const DAT_00aa7209).read_unaligned();
        let x20d: i32 = (&raw const DAT_00aa720d).read_unaligned();
        FUN_004d64a0(local_20.as_mut_ptr(), x209, x20d);
        FUN_004d6480(local_10.as_mut_ptr(), local_20.as_mut_ptr());
        FUN_004d64e0(local_20.as_mut_ptr());
        let u_var8: u32 =
            (i_var1 as u32).wrapping_add((local_20[2] as u32).wrapping_sub(local_10[2] as u32));
        let i_var2: i32 = (local_20[3] as u32)
            .wrapping_sub(local_10[3] as u32)
            .wrapping_add(i_var2 as u32) as i32;
        let x6cfc: i32 = FUN_004d48e0();
        DAT_00aa6cfc = x6cfc;
        let mut local_28: i32 = i_var2;
        let x735e: i8 = (&raw const DAT_00aa735e).read_unaligned();
        let x7b1f: i32 = (&raw const DAT_00aa7b1f).read_unaligned();
        let x7b13: i32 = (&raw const DAT_00aa7b13).read_unaligned();
        let x209: i32 = (&raw const DAT_00aa7209).read_unaligned();
        let x20d: i32 = (&raw const DAT_00aa720d).read_unaligned();
        let x7346: i32 = (&raw const DAT_00aa7346).read_unaligned();
        let x734a: i32 = (&raw const DAT_00aa734a).read_unaligned();

        if x735e == 0 {
            if x7b1f == 8 {
                let mut p3 = (x6cfc as u32)
                    .wrapping_add((x7b13 as u32).wrapping_mul(x20d as u32))
                    .wrapping_add(x209 as u32) as *mut u8;
                let mut i_var7: i32 = x7346;
                let mut p9 = x734a as u32 as *mut u8;
                let mut local_2c: i32 = i_var2;
                if 0 < i_var2 {
                    loop {
                        let mut p11 = p3;
                        let mut p12 = p9;
                        let mut u4 = u_var8 >> 2;
                        while u4 != 0 {
                            p12.cast::<u32>()
                                .write_unaligned(p11.cast::<u32>().read_unaligned());
                            p11 = p11.wrapping_add(4);
                            p12 = p12.wrapping_add(4);
                            u4 -= 1;
                        }
                        let mut u4 = u_var8 & 3;
                        while u4 != 0 {
                            *p12 = *p11;
                            p11 = p11.wrapping_add(1);
                            p12 = p12.wrapping_add(1);
                            u4 -= 1;
                        }
                        let mut i_var6: i32 = 0;
                        if 0 < u_var8 as i32 {
                            loop {
                                let src = (i_var6 as u32).wrapping_add(i_var7 as u32) as *const u8;
                                if *src != 0 {
                                    *p3.wrapping_offset(i_var6 as isize) = *src;
                                }
                                i_var6 = i_var6.wrapping_add(1);
                                if !(i_var6 < u_var8 as i32) {
                                    break;
                                }
                            }
                        }
                        p3 = p3.wrapping_offset(x7b13 as isize);
                        local_2c = local_2c.wrapping_sub(1);
                        i_var7 = i_var7.wrapping_add(i_var1);
                        p9 = p9.wrapping_offset(i_var1 as isize);
                        if local_2c == 0 {
                            break;
                        }
                    }
                }
            } else if x7b1f == 0x10 {
                let mut local_24 = (x6cfc as u32)
                    .wrapping_add((x7b13 as u32).wrapping_mul(x20d as u32))
                    .wrapping_add((x209 as u32).wrapping_mul(2))
                    as *mut u8;
                if 0 < i_var2 {
                    let mut i_var7: i32 = x7346;
                    let mut p9 = x734a as u32 as *mut u8;
                    loop {
                        let mut i_var6: i32 = 0;
                        let mut p3 = p9;
                        let mut p5 = local_24;
                        if 0 < u_var8 as i32 {
                            loop {
                                p3.cast::<u16>()
                                    .write_unaligned(p5.cast::<u16>().read_unaligned());
                                let c = *((i_var6 as u32).wrapping_add(i_var7 as u32) as *const i8);
                                if c == 1 {
                                    p5.cast::<u16>().write_unaligned(0x1f);
                                } else if c == -1 {
                                    p5.cast::<u16>().write_unaligned(0xffff);
                                }
                                i_var6 = i_var6.wrapping_add(1);
                                p3 = p3.wrapping_add(2);
                                p5 = p5.wrapping_add(2);
                                if !(i_var6 < u_var8 as i32) {
                                    break;
                                }
                            }
                        }
                        i_var7 = i_var7.wrapping_add(i_var1);
                        p9 = p9.wrapping_offset((i_var1 as u32).wrapping_mul(2) as i32 as isize);
                        local_24 =
                            local_24.wrapping_offset((x7b13 as u32 & 0xffff_fffe) as i32 as isize);
                        local_28 = local_28.wrapping_sub(1);
                        if local_28 == 0 {
                            break;
                        }
                    }
                }
            }
        } else {
            if x7b1f == 8 {
                let mut p3 = (x6cfc as u32)
                    .wrapping_add((x7b13 as u32).wrapping_mul(x20d as u32))
                    .wrapping_add(x209 as u32) as *mut u8;
                let mut i_var7: i32 = i_var2;
                let mut p9 = x734a as u32 as *mut u8;
                if 0 < i_var2 {
                    loop {
                        let mut p11 = p3;
                        let mut p12 = p9;
                        let mut u4 = u_var8 >> 2;
                        while u4 != 0 {
                            p12.cast::<u32>()
                                .write_unaligned(p11.cast::<u32>().read_unaligned());
                            p11 = p11.wrapping_add(4);
                            p12 = p12.wrapping_add(4);
                            u4 -= 1;
                        }
                        let mut u4 = u_var8 & 3;
                        while u4 != 0 {
                            *p12 = *p11;
                            p11 = p11.wrapping_add(1);
                            p12 = p12.wrapping_add(1);
                            u4 -= 1;
                        }
                        p3 = p3.wrapping_offset(x7b13 as isize);
                        i_var7 = i_var7.wrapping_sub(1);
                        p9 = p9.wrapping_offset(i_var1 as isize);
                        if i_var7 == 0 {
                            break;
                        }
                    }
                }
            } else if x7b1f == 0x10 {
                let mut p5 = (x7b13 as u32)
                    .wrapping_mul(x20d as u32)
                    .wrapping_add((x209 as u32).wrapping_mul(2))
                    .wrapping_add(x6cfc as u32) as *mut u8;
                if 0 < i_var2 {
                    let mut p9 = x734a as u32 as *mut u8;
                    loop {
                        let mut u4 = u_var8;
                        let mut p3 = p9;
                        let mut p10 = p5;
                        if 0 < u_var8 as i32 {
                            loop {
                                p3.cast::<u16>()
                                    .write_unaligned(p10.cast::<u16>().read_unaligned());
                                u4 = u4.wrapping_sub(1);
                                p3 = p3.wrapping_add(2);
                                p10 = p10.wrapping_add(2);
                                if u4 == 0 {
                                    break;
                                }
                            }
                        }
                        p9 = p9.wrapping_offset((i_var1 as u32).wrapping_mul(2) as i32 as isize);
                        p5 = p5.wrapping_offset((x7b13 as u32 & 0xffff_fffe) as i32 as isize);
                        local_28 = local_28.wrapping_sub(1);
                        if local_28 == 0 {
                            break;
                        }
                    }
                }
            }
            let x209: i32 = (&raw const DAT_00aa7209).read_unaligned();
            let x20d: i32 = (&raw const DAT_00aa720d).read_unaligned();
            FUN_004b5570(&raw mut DAT_00aa735e, x209, x20d);
        }
        FUN_004d4970();
        let tmp: i32 = (&raw const DAT_00aa7209).read_unaligned();
        (&raw mut DAT_00aa7211).write_unaligned(tmp);
        let tmp: i32 = (&raw const DAT_00aa720d).read_unaligned();
        (&raw mut DAT_00aa7215).write_unaligned(tmp);
        (&raw mut DAT_00aa7356).write_unaligned(u_var8);
        (&raw mut DAT_00aa735a).write_unaligned(i_var2);
    }
}
