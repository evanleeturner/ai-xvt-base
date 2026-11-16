// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e54: *mut u8;
    static mut DAT_006d4988: *mut u8;
    static mut DAT_006db798: *mut u8;
    static mut DAT_0071e6be: u16;
    static mut DAT_0071f2de: u16;
    static mut DAT_0070a1d2: u16;
    static mut DAT_00732e40: i16;
    static mut DAT_006da9f0: i16;
    static mut DAT_0071b704: i16;
    static mut DAT_0070ab18: i32;
    static mut DAT_0070ab14: i32;
    static mut DAT_0070ac20: i32;
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_006d4974: i32;
    static mut DAT_006d4978: i32;
    static mut DAT_006d4970: i32;
    static mut DAT_0071e6c4: i32;
    static mut DAT_00732f38: i32;
    static mut DAT_00732fe8: i32;
    static mut DAT_006db580: i32;
    static mut DAT_006db720: i32;
    static mut DAT_006db704: i32;
    static mut DAT_006db708: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0052b0a8: [u8; 0];
    static mut DAT_0052b0aa: [u8; 0];
    static mut DAT_0052b0ac: [u8; 0];
    static mut DAT_0052b0bc: [u8; 0];
    static mut DAT_0052b0be: [u8; 0];
    static mut DAT_0052b0c0: [u8; 0];
    static mut DAT_0052b0c2: [u8; 0];
    static mut DAT_0052b0c3: [u8; 0];
    static mut DAT_00525aae: [u8; 0];
    static mut DAT_00525a7e: [u8; 0];
    static mut s_starshipintohyperpln_00525b38: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b250(param_1: *mut u8) -> u32;
    fn FUN_0044aa70(param_1: u32, param_2: u32, param_3: u32) -> i32;
    fn FUN_0044ab00(param_1: u32, param_2: u32, param_3: u32) -> i32;
    fn FUN_0044ab90(param_1: u32, param_2: u32, param_3: u32) -> i32;
    fn FUN_00449790(param_1: u32, param_2: u32) -> i32;
    fn FUN_00483950(
        param_1: u16,
        param_2: u32,
        param_3: u32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
    fn FUN_0042a230(param_3: *mut u8, param_4: i16, param_5: i16, param_6: i16);
    fn FUN_00421c70(param_1: u32, param_2: i32);
    fn FUN_00412810(param_1: u32, param_2: u32, param_3: u32) -> u32;
    fn FUN_00419190(param_1: u16, param_2: u16) -> i32;
    fn FUN_00418cd0(param_1: u32, param_2: u32, param_3: u32);
    fn FUN_00478470() -> u16;
    fn FUN_00478320(param_1: u32, param_2: u16) -> u32;
    fn FUN_00422710(param_1: i32) -> u16;
    fn FUN_00446fe0(param_1: u32) -> i16;
    fn FUN_00483410(param_1: i16, param_2: i16, param_3: *mut u8);
    fn FUN_00413ca0(param_3: i32, param_4: i32) -> i64;
    fn FUN_0040a3c0(param_1: u32, param_2: i32);
}

#[inline(always)]
unsafe fn rd<T: Copy>(p: *const u8, off: u32) -> T {
    // SAFETY: caller guarantees the game's memory is valid at this offset.
    unsafe { p.byte_add(off as usize).cast::<T>().read_unaligned() }
}

#[inline(always)]
unsafe fn wr<T: Copy>(p: *mut u8, off: u32, v: T) {
    // SAFETY: caller guarantees the game's memory is valid at this offset.
    unsafe { p.byte_add(off as usize).cast::<T>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041aba0(param_1: u16, param_2: u32, param_3: u16) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are read and written at the offsets the original uses.
    unsafe {
        let s_1df: i16 = rd(DAT_00732e54, 0x1df);
        if s_1df == 0 {
            return;
        }
        let u_var12: u32 = param_2 & 0xffff;
        let i_var5: u32 = (*DAT_00732e54.add(4) as u32).wrapping_mul(0xef);
        let tidx: usize = u_var12.wrapping_mul(8).wrapping_add(i_var5) as usize;
        let mut b_var24: u8 = *(&raw mut DAT_0052b0c3).cast::<u8>().add(tidx);
        let b_var2: u8 = *(&raw mut DAT_0052b0c2).cast::<u8>().add(tidx);
        if *DAT_00732e54.add(0x31a + b_var24 as usize) == 0 {
            return;
        }
        let i_var13: *mut u8 = DAT_006d4988.byte_add((param_1 as u32).wrapping_mul(0x23) as usize);
        let u_var14 = FUN_0042b250(DAT_00732e54);
        let mut u_var10: u16 = u_var14 as u16;
        let off345: usize = 0x345usize.wrapping_add(u_var12.wrapping_mul(4) as usize);
        let b_var3: u8 = *DAT_00732e54.add(off345);
        let pbvar1: *mut u8 = DAT_00732e54.add(off345);
        let b_var23: u8 = b_var3 & 0x7f;
        if b_var23 != 0 {
            let u14: u32 = rd(DAT_00732e54, 0x1d);
            if u14 == 0 {
                if u_var10 < 0xaaaa {
                    if u_var10 < 0x5555 {
                        if u_var10 < 0x4000 {
                            b_var24 = (DAT_0071e6be >> 3) as u8;
                        } else {
                            b_var24 = (DAT_0071e6be / 6) as u8;
                        }
                    } else {
                        b_var24 = (DAT_0071e6be >> 2) as u8;
                    }
                } else {
                    b_var24 = (DAT_0071e6be >> 1) as u8;
                }
            } else {
                if 0x27fff < u14 {
                    return;
                }
                if u14 < 0x18000 {
                    if 0x7fff < u14 {
                        if DAT_0071f2de < 0x76 {
                            return;
                        }
                        b_var24 = 2;
                    }
                } else {
                    if DAT_0071f2de < 0x76 {
                        return;
                    }
                    b_var24 = 1;
                }
            }
            if b_var24 == 0 {
                b_var24 = 1;
            }
            b_var24 = b_var23.wrapping_sub(b_var24);
            if b_var23 < b_var24 {
                b_var24 = 0;
            }
            *pbvar1 = b_var3 & 0x80;
            *DAT_00732e54.add(off345) = *DAT_00732e54.add(off345) | b_var24;
            return;
        }
        *pbvar1 = b_var3 & 0x80;
        *DAT_00732e54.add(off345) = *DAT_00732e54.add(off345) | 0x3b;
        let mut i_var18: i32 = rd(i_var13, 4);
        let mut i_var26: i32 = rd(i_var13, 8);
        let mut i_var28: i32 = rd(i_var13, 0xc);
        let mut s_var27: i16;
        let mut s_var8: i16;
        let mut s_var11: i16;
        if b_var2 == 0xff || 0x75 < DAT_0071f2de {
            s_var27 = (&raw mut DAT_0052b0bc)
                .cast::<u8>()
                .add(tidx)
                .cast::<i16>()
                .read_unaligned();
            s_var8 = (&raw mut DAT_0052b0be)
                .cast::<u8>()
                .add(tidx)
                .cast::<i16>()
                .read_unaligned();
            s_var11 = (&raw mut DAT_0052b0c0)
                .cast::<u8>()
                .add(tidx)
                .cast::<i16>()
                .read_unaligned();
        } else if *i_var13.add(3) == 0x35 {
            let u14 = b_var2 as u32;
            let i15 = FUN_0044aa70(0x35, b_var24 as u32, u14);
            s_var27 = (i15 >> 1) as i16;
            let i15 = FUN_0044ab00(*i_var13.add(3) as u32, b_var24 as u32, u14);
            s_var11 = (i15 >> 1) as i16;
            let i15 = FUN_0044ab90(*i_var13.add(3) as u32, b_var24 as u32, u14);
            s_var8 = (i15 >> 1) as i16;
        } else {
            let u14 = b_var2 as u32;
            let i15 = FUN_0044aa70(*i_var13.add(3) as u32, b_var24 as u32, u14);
            s_var27 = i15 as i16;
            let i15 = FUN_0044ab00(*i_var13.add(3) as u32, b_var24 as u32, u14);
            s_var11 = i15 as i16;
            let i15 = FUN_0044ab90(*i_var13.add(3) as u32, b_var24 as u32, u14);
            s_var8 = i15 as i16;
        }
        let i_var17 = FUN_00449790(*i_var13.add(3) as u32, b_var24 as u32);
        if i_var17 == 0x15 {
            DAT_0070ab18 = s_var27 as i32;
            DAT_0070ab14 = s_var11 as i32;
            DAT_0070ac20 = s_var8 as i32;
            if *i_var13.add(3) == 0x35 {
                DAT_0070ab14 = (DAT_0070ab14 as u32).wrapping_mul(2) as i32;
                DAT_0070ab18 = (DAT_0070ab18 as u32).wrapping_mul(2) as i32;
                DAT_0070ac20 = (DAT_0070ac20 as u32).wrapping_mul(2) as i32;
            }
            FUN_00483950(
                ((*DAT_00732e54.add(0x2f2 + b_var24 as usize) as u16) << 8),
                *i_var13.add(3) as u32,
                b_var24 as u32,
                DAT_0070ab18,
                DAT_0070ab14,
                DAT_0070ac20,
            );
            if *i_var13.add(3) == 0x35 {
                DAT_0070ab18 >>= 1;
                DAT_0070ab14 >>= 1;
                DAT_0070ac20 >>= 1;
            }
            s_var27 = DAT_0070ab18 as i16;
            s_var11 = DAT_0070ab14 as i16;
            s_var8 = DAT_0070ac20 as i16;
        }
        FUN_0042a230(i_var13, s_var27, s_var8, s_var11);
        if *i_var13.add(3) == 0x35 {
            DAT_0070ab18 = (DAT_0070ab18 as u32).wrapping_mul(2) as i32;
            DAT_0070ab14 = (DAT_0070ab14 as u32).wrapping_mul(2) as i32;
            DAT_0070ac20 = (DAT_0070ac20 as u32).wrapping_mul(2) as i32;
        }
        i_var18 = i_var18.wrapping_add(DAT_0070ab18);
        i_var26 = i_var26.wrapping_add(DAT_0070ab14);
        i_var28 = i_var28.wrapping_add(DAT_0070ac20);
        FUN_00421c70(param_3 as u32, 0);
        let mut i_var17 = DAT_00732a2c;
        let mut i_var25 = DAT_00732a28;
        let mut i_var15 = DAT_00732a24;
        let u_var14: u32 = (DAT_00732a2c as u32).wrapping_sub(i_var18 as u32);
        let u_var19: u32 = (DAT_00732a28 as u32).wrapping_sub(i_var26 as u32);
        let u_var20: u32 = (DAT_00732a24 as u32).wrapping_sub(i_var28 as u32);
        let u_var21 = FUN_00412810(u_var14, u_var19, u_var20);
        if u_var21 < 0x14001 {
            DAT_006d4974 = i_var17;
            DAT_006d4978 = i_var25;
            DAT_006d4970 = i_var15;
            DAT_0071e6c4 = i_var18;
            DAT_00732f38 = i_var26;
            DAT_00732fe8 = i_var28;
            if FUN_00419190(param_1, param_1) == 0 {
                let mut c_var4: i8 = *DAT_00732e54
                    .add(0x346usize.wrapping_add(u_var12.wrapping_mul(4) as usize))
                    as i8;
                let i_var6: u32 = (param_3 as u32).wrapping_mul(0x23);
                let i_var22: u32 = rd(DAT_006d4988, 0x1fu32.wrapping_add(i_var6));
                if i_var22 != 0 {
                    FUN_00418cd0(u_var14, u_var19, u_var20);
                    if c_var4 == 0 {
                        DAT_006db580 =
                            ((DAT_0070a1d2 as u32).wrapping_mul(DAT_006db580 as u32) as i32) >> 0xe;
                    } else {
                        DAT_006db580 =
                            ((DAT_0070a1d2 as u32).wrapping_mul(DAT_006db580 as u32) as i32) >> 0xf;
                    }
                    let mut s27 = DAT_006db580 as i16;
                    let u_var9 = FUN_00478470();
                    s27 = s27.wrapping_add((u_var9 & 3) as i16).wrapping_sub(1);
                    let p_sub: *mut u8 = rd(DAT_006d4988, 0x1fu32.wrapping_add(i_var6));
                    let s_tmp: i16 = rd(p_sub, 0x79);
                    if s_tmp == 0 {
                        s27 = 0;
                    }
                    let mut u14 = FUN_00478320(
                        (((i_var22 >> 0x10) as u16 as u32) << 16) | (s27 as u16 as u32),
                        u_var10,
                    );
                    u14 &= 0xffff;
                    let p_sub: *mut u8 = rd(DAT_006d4988, 0x1fu32.wrapping_add(i_var6));
                    let i_tmp: i32 = rd(p_sub, 6);
                    i_var17 = (i_var17 as u32).wrapping_add(
                        (i_var17 as u32)
                            .wrapping_sub(i_tmp as u32)
                            .wrapping_mul(u14),
                    ) as i32;
                    let i_tmp: i32 = rd(p_sub, 10);
                    i_var25 = (i_var25 as u32).wrapping_add(
                        (i_var25 as u32)
                            .wrapping_sub(i_tmp as u32)
                            .wrapping_mul(u14),
                    ) as i32;
                    let i_tmp: i32 = rd(p_sub, 0xe);
                    i_var15 = (i_var15 as u32).wrapping_add(
                        (i_var15 as u32)
                            .wrapping_sub(i_tmp as u32)
                            .wrapping_mul(u14),
                    ) as i32;
                }
                FUN_00418cd0(
                    (i_var17 as u32).wrapping_sub(i_var18 as u32),
                    (i_var25 as u32).wrapping_sub(i_var26 as u32),
                    (i_var15 as u32).wrapping_sub(i_var28 as u32),
                );
                let mut s_var8 = DAT_00732e40;
                let s_var27 = DAT_006da9f0;
                FUN_00478470();
                u_var10 = FUN_00422710(7);
                if u_var10 != 0xffff {
                    let i15: u32 = ((u_var10 as u32)
                        .wrapping_mul(8)
                        .wrapping_sub(u_var10 as u32))
                    .wrapping_mul(5);
                    let p_new: *mut u8 = DAT_006d4988.byte_add(i15 as usize);
                    let p_sub: *mut u8 = rd(p_new, 0x1f);
                    *p_sub = 1;
                    *p_new.add(2) = 7;
                    let p_src: *mut u8 = rd(i_var13, 0x1f);
                    *p_sub.add(0x86) = *p_src.add(0x86);
                    let mut u_var9: u16 = 0;
                    let mut b_var7: i32 = 0;
                    loop {
                        let i25: usize = (u_var9 as u32).wrapping_add(i_var5) as usize;
                        let a = *(&raw mut DAT_0052b0aa).cast::<u8>().add(i25) as u32;
                        let b = *(&raw mut DAT_0052b0ac).cast::<u8>().add(i25) as u32;
                        let c = *(&raw mut DAT_0052b0a8).cast::<u8>().add(i25) as i8;
                        if a <= u_var12 && u_var12 <= b && (c == -0x6f || c == -0x6e) {
                            b_var7 = 1;
                        }
                        u_var9 = u_var9.wrapping_add(1);
                        if u_var9 >= 2 {
                            break;
                        }
                    }
                    let not7: i32 = (b_var7 == 0) as i32;
                    if c_var4 == 0 {
                        let p_src: *mut u8 = rd(i_var13, 0x1f);
                        c_var4 = *p_src.add(0x86) as i8;
                        if c_var4 == 0 || c_var4 == 2 {
                            u_var9 = ((not7 as u16).wrapping_neg() & 0xfff9).wrapping_add(0x91);
                        } else {
                            u_var9 = ((not7 as u16).wrapping_neg() & 0xfffa).wrapping_add(0x92);
                        }
                    } else {
                        u_var9 = 0x8ei32.wrapping_sub(not7) as u16;
                    }
                    *p_new.add(3) = u_var9 as u8;
                    let p_sub: *mut u8 = rd(p_new, 0x1f);
                    wr::<i16>(p_sub, 0x81, 1);
                    wr::<u16>(p_sub, 0x83, param_1);
                    *p_sub.add(0x85) = *i_var13.add(3);
                    wr::<i16>(p_new, 0x12, s_var8);
                    wr::<i16>(p_new, 0x14, 0);
                    wr::<i16>(p_new, 0x10, s_var27);
                    let u_var12 = u_var9 as u32;
                    *p_sub.add(0x90) = 1;
                    let p_sub: *mut u8 = rd(p_new, 0x1f);
                    *p_sub.add(0x89) = *p_sub.add(0x90);
                    let p_sub: *mut u8 = rd(p_new, 0x1f);
                    let v: u16 = (&raw mut DAT_00525aae)
                        .cast::<u8>()
                        .add(u_var12.wrapping_mul(2) as usize)
                        .cast::<u16>()
                        .read_unaligned();
                    wr::<u16>(p_sub, 0x79, v);
                    let v: u16 = (&raw mut DAT_00525a7e)
                        .cast::<u8>()
                        .add(u_var12.wrapping_mul(2) as usize)
                        .cast::<u16>()
                        .read_unaligned();
                    wr::<u16>(p_sub, 0x7d, v);
                    let s_var11 = FUN_00446fe0(u_var12);
                    let s_tmp: i16 = s_var11.wrapping_mul(3);
                    let p_sub: *mut u8 = rd(p_new, 0x1f);
                    wr::<i16>(p_sub, 0x7f, s_tmp);
                    FUN_00483410(s_var8, s_var27, p_new);
                    let p_sub: *mut u8 = rd(p_new, 0x1f);
                    wr::<i32>(p_sub, 6, i_var18);
                    let p_sub: *mut u8 = rd(p_new, 0x1f);
                    wr::<i32>(p_sub, 10, i_var26);
                    let p_sub: *mut u8 = rd(p_new, 0x1f);
                    wr::<i32>(p_sub, 0xe, i_var28);
                    let hp = (&raw mut s_starshipintohyperpln_00525b38)
                        .cast::<u8>()
                        .add((u_var12.wrapping_mul(2) as usize).wrapping_add(6));
                    let s_tmp: i16 = hp.cast::<i16>().read_unaligned();
                    let u29 = FUN_00413ca0(DAT_006db720, s_tmp as i32);
                    i_var18 = i_var18.wrapping_add(u29 as i32);
                    let s_tmp: i16 = hp.cast::<i16>().read_unaligned();
                    let u29 = FUN_00413ca0(DAT_006db704, s_tmp as i32);
                    i_var26 = i_var26.wrapping_add(u29 as i32);
                    let s_tmp: i16 = hp.cast::<i16>().read_unaligned();
                    let u29 = FUN_00413ca0(DAT_006db708, s_tmp as i32);
                    wr::<i32>(p_new, 4, i_var18);
                    wr::<i32>(p_new, 8, i_var26);
                    let i_tmp: i32 = i_var28.wrapping_add(u29 as i32);
                    wr::<i32>(p_new, 0xc, i_tmp);
                    let i5: u32 =
                        ((u_var10.wrapping_sub(DAT_0071b704 as u16)) as u32).wrapping_mul(8);
                    let p_slot: *mut u8 = DAT_006db798.byte_add(i5 as usize);
                    *p_slot.add(1) = 0;
                    wr::<u16>(p_slot, 4, param_3);
                    let p_sub: *mut u8 = rd(p_new, 0x1f);
                    let v: u16 = rd(p_sub, 0x79);
                    wr::<u16>(p_slot, 6, v);
                    *p_slot = 0xff;
                    let p_sub: *mut u8 = rd(p_new, 0x1f);
                    wr::<*mut u8>(p_sub, 0xa3, p_slot);
                    FUN_0040a3c0(u_var10 as u32, DAT_0071ecf4);
                    s_var8 = 0;
                    let _ = s_var8;
                }
            }
        }
    }
}
