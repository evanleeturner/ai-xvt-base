// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b0: u16;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a73b6: u32;
    static mut DAT_009a73c0: u16;
    static mut DAT_009a73d0: u16;
    static mut DAT_009a73d2: u16;
    static mut DAT_009a8070: u16;
    static mut DAT_009a8c00: u32;
    static mut DAT_009e9640: u16;
    static mut DAT_00a00854: i32;
    static mut DAT_00a07cec: u16;
    static mut DAT_00a080f0: u16;
    static mut DAT_00a08104: u32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_005243c8: [i32; 0];
    static mut DAT_005243b8: [i32; 0];
    static mut DAT_005243d8: [i8; 0];
    static mut DAT_005243e0: [u8; 0];
    static mut DAT_005243e8: [u8; 0];
    static mut DAT_009d8c84: [i8; 0];
    static mut DAT_009d9155: [i8; 0];
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009fd458: [i8; 0];
    static mut DAT_0051c583: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00403360(a: u32, b: i32) -> u8;
    fn FUN_004034e0(a: u32, b: u32);
    fn FUN_0049f360() -> u32;
    fn FUN_00407090(a: u32, b: u32) -> u32;
    fn FUN_00425cb0() -> u32;
}

unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u8).read() }
}
unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u16).read_unaligned() }
}
unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u32).read_unaligned() }
}
unsafe fn wr8(a: u32, v: u8) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *mut u8).write(v) }
}
unsafe fn wr16(a: u32, v: u16) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *mut u16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00466220() -> u32 {
    // SAFETY: all globals, callees and computed addresses are the game's own
    // memory, accessed exactly as the original does.
    unsafe {
        let mut u_var6: u32 = DAT_009a73b0 as u32;
        let c0 = rd8(DAT_009a1fe8
            .wrapping_add(2)
            .wrapping_add(u_var6.wrapping_mul(0x23))) as i8;
        if c0 != 0 && c0 != 1 {
            return DAT_009a1fe8 & 0xffff0000;
        }
        let mut u_var12: u32 = DAT_009a73d2 as u32;
        let t = DAT_009a73b6;
        if rd8(t.wrapping_add(0x34)) as u32 == u_var12 {
            if rd16(DAT_00a08104.wrapping_add(0x6a)) as i16 == -1 {
                let i_var2: i32 = (&raw mut DAT_005243c8)
                    .cast::<i32>()
                    .add(DAT_009a73d0 as usize)
                    .read();
                let mut u_var11: u16 = DAT_009e9640;
                if (u_var11 as i32) < DAT_00a00854 {
                    loop {
                        let i_var5 = (u_var11 as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(DAT_009a1fe8);
                        let c_var1 = rd8(i_var5.wrapping_add(3)) as i8;
                        if c_var1 != 0 {
                            let t = rd32(i_var5.wrapping_add(0x1f));
                            let iv = rd32(t.wrapping_add(0xa5));
                            if rd8(iv.wrapping_add(1)) != 0
                                && rd16(iv.wrapping_add(4)) == DAT_009a73b0
                            {
                                let mut i5: i32 = i_var2.wrapping_mul(3);
                                if c_var1 != -0x70 {
                                    i5 = i_var2;
                                }
                                let b = FUN_00403360(u_var11 as u32, i5);
                                if b as u32 == 1 {
                                    wr16(DAT_00a08104.wrapping_add(0x6a), u_var11);
                                    FUN_004034e0(DAT_009a73b0 as u32, u_var11 as u32);
                                    let s = rd16(
                                        (DAT_009a73b0 as u32)
                                            .wrapping_mul(0x23)
                                            .wrapping_add(0x10)
                                            .wrapping_add(DAT_009a1fe8),
                                    );
                                    let idx = (DAT_009a8070.wrapping_sub(s) >> 13) as usize;
                                    let tt = DAT_009a73b6;
                                    if (&raw mut DAT_005243d8).cast::<i8>().add(idx).read() == 0 {
                                        wr8(rd32(tt).wrapping_add(0), 0);
                                        wr8(tt.wrapping_add(0x34), 0x18);
                                    } else {
                                        wr8(tt.wrapping_add(0x34), 1);
                                    }
                                    FUN_0049f360();
                                    let d = DAT_00a08104;
                                    u_var6 = d;
                                    if rd8(d.wrapping_add(0x246)) != 0
                                        && rd8(d.wrapping_add(0x247)) != 0
                                    {
                                        if rd8(d.wrapping_add(0x246)) == 1
                                            && (rd8(d.wrapping_add(0x1e5)) & 2) != 0
                                        {
                                            wr16(
                                                d.wrapping_add(0x248),
                                                rd16(d.wrapping_add(0x248)).wrapping_add(10),
                                            );
                                            u_var12 = DAT_00a08104;
                                            u_var6 = (DAT_009a73c0 as u32).wrapping_mul(0x566);
                                            if (&raw mut DAT_009d8c84)
                                                .cast::<i8>()
                                                .add(u_var6 as usize)
                                                .read()
                                                != 0x15
                                                && (&raw mut DAT_009d9155)
                                                    .cast::<i8>()
                                                    .add(u_var6 as usize)
                                                    .read()
                                                    != 0x15
                                            {
                                                let a = DAT_00a08104.wrapping_add(0x247);
                                                wr8(a, rd8(a).wrapping_sub(1));
                                                return u_var12 & 0xffff0000;
                                            }
                                        } else if rd16(d.wrapping_add(0x24a)) == 0 {
                                            u_var6 = FUN_00407090(DAT_009a73b0 as u32, 0x9b);
                                        }
                                    }
                                    return u_var6 & 0xffff0000;
                                }
                            }
                        }
                        u_var11 = u_var11.wrapping_add(1);
                        if !((u_var11 as i32) < DAT_00a00854) {
                            break;
                        }
                    }
                }
                let i_var2: i32 = (&raw mut DAT_005243b8)
                    .cast::<i32>()
                    .add(DAT_009a73d0 as usize)
                    .read();
                if rd16(DAT_00a08104.wrapping_add(0x6a)) as i16 == -1 {
                    u_var11 = DAT_00a07cec;
                    if (u_var11 as i32) < DAT_009a1fe0 {
                        loop {
                            u_var12 = u_var11 as u32;
                            let i_var5 = DAT_009a1fe8.wrapping_add(u_var12.wrapping_mul(0x23));
                            let iv = rd32(i_var5.wrapping_add(0x1b)) as i32;
                            if iv == -1 && rd8(i_var5.wrapping_add(3)) != 0 {
                                let t = rd32(
                                    (DAT_009a73b0 as u32)
                                        .wrapping_mul(0x23)
                                        .wrapping_add(0x1f)
                                        .wrapping_add(DAT_009a1fe8),
                                );
                                let u_var9 = rd8(t.wrapping_add(0x89)) as u32;
                                let k = (rd8(i_var5.wrapping_add(0x16)) as u32).wrapping_mul(0x566);
                                let m = (&raw mut DAT_009d8c88).cast::<u8>().add(k as usize).read()
                                    as u32;
                                let b4: u8 = if m == u_var9 {
                                    0
                                } else {
                                    ((&raw mut DAT_009fd458)
                                        .cast::<i8>()
                                        .add(m.wrapping_add(u_var9.wrapping_mul(0x1e5)) as usize)
                                        .read()
                                        == 0) as u8
                                };
                                if b4 != 0
                                    && rd8(DAT_00a08104.wrapping_add(7)) == 0
                                    && rd8(i_var5.wrapping_add(2)) == 0
                                    && FUN_00403360(u_var12, i_var2) as u32 == 1
                                {
                                    FUN_004034e0(u_var12, u_var6);
                                    let i5 = DAT_009a1fe8.wrapping_add(u_var12.wrapping_mul(0x23));
                                    let s = rd16(i5.wrapping_add(0x10));
                                    let mut u8_: u16 = DAT_009a8070.wrapping_sub(s);
                                    if 0x7fff < u8_ {
                                        u8_ = (u8_ as i32).wrapping_neg() as u16;
                                    }
                                    let s2 = rd16(i5.wrapping_add(0x12));
                                    let mut u10: u16 = DAT_00a080f0.wrapping_sub(s2);
                                    if 0x7fff < u10 {
                                        u10 = (u10 as i32).wrapping_neg() as u16;
                                    }
                                    if u8_ < 0x2000 && u10 < 0x2000 {
                                        wr16(DAT_00a08104.wrapping_add(0x6a), u_var11);
                                        break;
                                    }
                                }
                            }
                            u_var11 = u_var11.wrapping_add(1);
                            if !((u_var11 as i32) < DAT_009a1fe0) {
                                break;
                            }
                        }
                    }
                }
            }
            let mut u_var11 = rd16(DAT_00a08104.wrapping_add(0x6a));
            u_var12 = (DAT_00a08104 & 0xffff0000) | (u_var11 as u32);
            if u_var11 != 0xffff {
                FUN_004034e0(u_var6, u_var11 as u32);
                let s = rd16(
                    DAT_009a1fe8
                        .wrapping_add(0x10)
                        .wrapping_add(u_var6.wrapping_mul(0x23)),
                );
                let idx = (DAT_009a8070.wrapping_sub(s) >> 13) as usize;
                let c_var1 = (&raw mut DAT_005243d8).cast::<i8>().add(idx).read();
                let p = (&raw mut DAT_0051c583).cast::<u8>();
                u_var11 = (p
                    .add((rd8(DAT_00a08104.wrapping_add(4)) as usize).wrapping_mul(0x101))
                    .cast::<u16>())
                .read_unaligned();
                let t = rd32(
                    (rd16(DAT_00a08104.wrapping_add(0x6a)) as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(0x1f)
                        .wrapping_add(DAT_009a1fe8),
                );
                let u_var8: u16;
                if rd8(t) == 0 {
                    let t2 = rd32(t.wrapping_add(0xa9));
                    u_var8 = (p
                        .add((rd8(t2.wrapping_add(4)) as usize).wrapping_mul(0x101))
                        .cast::<u16>())
                    .read_unaligned();
                } else {
                    u_var8 = 900;
                }
                u_var6 = FUN_00425cb0();
                let e0 = (&raw mut DAT_005243e0).cast::<u8>();
                let e8 = (&raw mut DAT_005243e8).cast::<u8>();
                let u_var7: u8;
                if c_var1 == 1 {
                    if DAT_009a8c00 < 0x2000 && (u_var6 as u16) < 0x4000 {
                        u_var7 = 1;
                    } else {
                        u_var7 = e0.add((u_var6 & 3) as usize).read();
                    }
                } else if c_var1 == 0 {
                    if (u_var6 as u16) < 0x8001 {
                        u_var7 = e0.add((u_var6 & 3) as usize).read();
                    } else {
                        u_var7 = 9;
                    }
                } else if u_var11 < u_var8 {
                    u_var7 = e8.add((u_var6 & 7) as usize).read();
                } else {
                    let mut v = 0x10u8;
                    if DAT_009a8c00 < 0x8001 {
                        v = e8.add((u_var6 & 7) as usize).read();
                    }
                    u_var7 = v;
                }
                let t = DAT_009a73b6;
                wr8(t.wrapping_add(0x34), u_var7);
                u_var12 = FUN_0049f360();
            }
        }
        u_var12 & 0xffff0000
    }
}
