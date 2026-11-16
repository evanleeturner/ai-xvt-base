// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9680: [u8; 0];
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009e96c5: [u8; 0];
    static mut DAT_009e96c9: [u8; 0];
    static mut DAT_009e96cd: [u8; 0];
    static mut DAT_009e96d1: [u8; 0];
    static mut DAT_009e96d5: [u8; 0];
    static mut DAT_009e96d9: [u8; 0];
    static mut DAT_009ecc34: u32;
    static mut DAT_009ec5d0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009d8a58: u32;
    static mut DAT_009d8a54: u32;
    static mut DAT_009d8b60: u32;
    static mut DAT_009ec5fe: u16;
    static mut DAT_009d8112: i16;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009e9658: i32;
    static mut DAT_009d6940: [u8; 2];
    static mut DAT_009a20b0: u32;
    static mut DAT_009cd26c: u32;
    static mut DAT_009fd438: u32;
    static mut DAT_00a07cdc: u32;
    static mut DAT_00a07cd8: u32;
    static mut DAT_00a07cd4: u32;
    static mut DAT_009a8070: i16;
    static mut DAT_00a080f0: i16;
    static mut DAT_009a8da4: i32;
    static mut DAT_009a8d84: i32;
    static mut DAT_009a8d88: i32;
    static mut DAT_0051c635: [u8; 0];
    static mut DAT_0051c633: [u8; 0];
    static mut DAT_0051c593: [u8; 0];
    static mut DAT_0051c63d: [u8; 0];
    static mut DAT_0051c63f: [u8; 0];
    static mut DAT_0051c639: [u8; 0];
    static mut DAT_0051c63b: [u8; 0];
    static mut DAT_0051c637: [u8; 0];
    static mut DAT_00521f1c: [u8; 0];
    static mut DAT_0051a44f: [u8; 0];
    static mut DAT_00521c20: [i8; 0];
    static mut DAT_00521cc8: [u8; 0];
    static mut DAT_00521d20: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004269e0(a: u32) -> u32;
    fn FUN_004035d0(a: i32, b: i32, c: i16, d: i16);
    fn FUN_0042b290();
    fn FUN_00458fa0(a: u32, b: i32);
    fn FUN_00425cb0() -> u8;
    fn FUN_0041f300(a: u32, b: i8);
    fn FUN_00433d30(a: u16, b: u16, c: i32);
    fn FUN_004af5b0(a: u32) -> i32;
    fn FUN_004a7480(a: u32, b: i32);
    fn FUN_00427bd0(a: i16, b: i16, c: i32);
    fn FUN_004464c0(a: i32);
    fn FUN_00458e20(a: u32, b: i32);
    fn FUN_004ae640(a: u32, b: u32) -> i32;
    fn FUN_004ae6e0(a: u32, b: u32) -> i32;
    fn FUN_004ae5a0(a: u32, b: u32) -> i32;
    fn FUN_004037b0(a: i32, b: i32, c: i32, d: i32);
    fn FUN_0046a8d0(a: u32, b: u32, c: u32);
}

macro_rules! ad {
    ($x:ident) => {
        ((&raw mut $x) as usize as i32)
    };
}

unsafe fn ld<T: Copy>(a: i32, o: i32) -> T {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (a.wrapping_add(o) as usize as *const T).read_unaligned() }
}

unsafe fn st<T>(a: i32, o: i32, v: T) {
    // SAFETY: the address is the game's own memory, written where the original writes.
    unsafe { (a.wrapping_add(o) as usize as *mut T).write_unaligned(v) }
}

unsafe fn xd(r: i32) -> i32 {
    // SAFETY: reads the record's pointer field.
    unsafe { ld::<i32>(r, 0x1f) }
}

fn mul15(sv: i32, u: u32) -> u32 {
    let l = (sv as i64).wrapping_mul(u as i32 as i64);
    ((l as u32) >> 15) | ((((l as u64) >> 32) as u32) << 17)
}

unsafe fn blk(rec: i32, ent: i32, idx: i32) {
    // SAFETY: the game's own globals, functions and records.
    unsafe {
        let u = FUN_004269e0(((idx as u32) & 0xffff0000) | ld::<u8>(ent, 3) as u32);
        let k = (u & 0xffff).wrapping_mul(0x101) as i32;
        FUN_004035d0(
            ld::<i32>(ad!(DAT_009e9670), rec)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_009a1fe8),
            0,
            ld::<i16>(ad!(DAT_0051c635), k),
            ld::<i16>(ad!(DAT_0051c633), k),
        );
        st::<u32>(ad!(DAT_009e96d1), rec, ld::<u32>(ad!(DAT_009e96c5), rec));
        let u22 = DAT_009d8a58;
        st::<u32>(ad!(DAT_009e96d5), rec, ld::<u32>(ad!(DAT_009e96c9), rec));
        let u18 = DAT_009d8a54;
        st::<u32>(ad!(DAT_009e96d9), rec, ld::<u32>(ad!(DAT_009e96cd), rec));
        let u25 = DAT_009d8b60;
        st::<u32>(ad!(DAT_009e96c5), rec, u22);
        st::<u32>(ad!(DAT_009e96c9), rec, u18);
        st::<u32>(ad!(DAT_009e96cd), rec, u25);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00445570() {
    // SAFETY: everything touched is the game's own memory, functions and records.
    unsafe {
        let unaff_bp: u16 = 0;

        let mut k: u16 = 0;
        loop {
            let u18 = k as u32;
            let i6 = u18.wrapping_mul(0x5bd) as i32;
            if ld::<u8>(ad!(DAT_009e9680), i6) != 0 && u18 != DAT_009ecc34 {
                let i5 = ld::<i32>(ad!(DAT_009e9670), i6);
                if i5 != -1 {
                    let i19 = i5.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
                    if ld::<u8>(i19, 2) == 0 && (DAT_009ec5d0 == -1 || DAT_009ec5d0 == i5) {
                        blk(i6, i19, i5);
                    }
                }
            }
            k = k.wrapping_add(1);
            if !(k < 8) {
                break;
            }
        }

        let i5 = (DAT_009ecc34 & 0xffff).wrapping_mul(0x5bd) as i32;
        let i6 = ld::<i32>(ad!(DAT_009e9670), i5);
        if i6 != -1 {
            let i19 = i6.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
            if ld::<u8>(i19, 2) == 0 && (DAT_009ec5d0 == -1 || DAT_009ec5d0 == i6) {
                blk(i5, i19, i6);
            }
        }

        let u_var23: u16 = DAT_009ec5fe;
        let s_var9: i16 = DAT_009d8112;
        let mut local_6: u16 = 0;
        let mut b_var8 = false;
        if 0 < DAT_00a07ce8 {
            loop {
                'next: {
                    if DAT_009ec5d0 != -1 {
                        if b_var8 {
                            DAT_009d8112 = s_var9;
                            DAT_009ec5fe = u_var23;
                            return;
                        }
                        b_var8 = true;
                        local_6 = DAT_009ec5d0 as u16;
                    }
                    let u_var18 = local_6 as u32;
                    let i_var5 = u_var18.wrapping_mul(0x23) as i32;
                    let i_var6 = DAT_009a1fe8.wrapping_add(i_var5);
                    DAT_009d8112 = s_var9;
                    DAT_009ec5fe = u_var23;
                    'a: {
                        if xd(i_var6) != 0 {
                            let i19 = ld::<i32>(xd(i_var6), 2);
                            if i19 != 0 {
                                DAT_009ec5fe = (u_var23 as i32).wrapping_add(
                                    (DAT_009e9658 as i16 as i32).wrapping_sub(i19 as i16 as i32),
                                ) as u16;
                                if DAT_009ec5fe != 0 {
                                    DAT_009d8112 = (0xecu32 / DAT_009ec5fe as u32) as i16;
                                    if DAT_009d8112 == 0 {
                                        DAT_009d8112 = 1;
                                    }
                                    st::<u32>(
                                        xd(i_var6),
                                        2,
                                        ld::<u32>(xd(i_var6), 2).wrapping_add(DAT_009ec5fe as u32),
                                    );
                                    break 'a;
                                }
                                if DAT_009ec5d0 == -1 && ld::<u32>(i_var6, 0x1b) != 0xffffffff {
                                    if ld::<u8>(ad!(DAT_009d6940), 1) != 0
                                        && ld::<u32>(i_var6, 0x1b) == DAT_009ecc34
                                    {
                                        FUN_0042b290();
                                    }
                                    st::<u32>(xd(i_var6), 6, ld::<u32>(i_var6, 4));
                                    st::<u32>(xd(i_var6), 10, ld::<u32>(i_var6, 8));
                                    st::<u32>(xd(i_var6), 0xe, ld::<u32>(i_var6, 0xc));
                                }
                                break 'next;
                            }
                        }
                    }
                    // LAB_004458bd
                    if ld::<u8>(i_var6, 3) == 0 {
                        break 'next;
                    }
                    let u_var2 = ld::<u8>(i_var6, 2);
                    let u_var4 = ld::<u16>(xd(i_var6), 0x81);
                    'l45: {
                        if u_var4 == 0 {
                            break 'l45;
                        }
                        let mut u_var21 = (u_var4 as i32).wrapping_sub(DAT_009ec5fe as i32) as u16;
                        if u_var4 < u_var21 {
                            u_var21 = 0;
                        }
                        st::<u16>(xd(i_var6), 0x81, u_var21);
                        if u_var21 != 0 {
                            break 'l45;
                        }
                        match u_var2 {
                            0 => {
                                FUN_00458fa0(((unaff_bp as u32) << 16) | local_6 as u32, 1);
                                let b = FUN_00425cb0();
                                FUN_0041f300(u_var18, ((b & 1) as i32 + 0x7f) as i8);
                                FUN_00433d30(
                                    local_6,
                                    ld::<u8>(DAT_009a1fe8, 0x16 + i_var5) as u16,
                                    2,
                                );
                            }
                            1..=5 => {
                                let i19 = FUN_004af5b0(ld::<u8>(i_var6, 3) as u32);
                                if i19 == 0 {
                                    FUN_004a7480(((unaff_bp as u32) << 16) | local_6 as u32, 1);
                                    FUN_0041f300(u_var18, 0x82u8 as i8);
                                    let p = ld::<i32>(DAT_009a1fe8, 0x1f + i_var5);
                                    st::<u8>(
                                        p,
                                        1,
                                        (ld::<i32>(
                                            ad!(DAT_00521f1c),
                                            (ld::<u8>(i_var6, 3) as i32) * 0x1a,
                                        ) >> 9) as u8,
                                    );
                                } else {
                                    FUN_0041f300(u_var18, 0x82u8 as i8);
                                }
                                FUN_00433d30(
                                    local_6,
                                    ld::<u8>(DAT_009a1fe8, 0x16 + i_var5) as u16,
                                    2,
                                );
                            }
                            6 | 7 => {
                                if ld::<u8>(ad!(DAT_0051a44f), ld::<u8>(i_var6, 3) as i32) != 0 {
                                    FUN_0041f300(u_var18, -0x7f);
                                } else {
                                    st::<u8>(i_var6, 3, 0);
                                    break 'next;
                                }
                            }
                            0xb => {
                                let b = FUN_00425cb0();
                                FUN_0041f300(u_var18, ((b & 1) as i32 + 0x7f) as i8);
                            }
                            _ => {
                                st::<u8>(i_var6, 3, 0);
                                break 'next;
                            }
                        }
                    }
                    // LAB_00445a45
                    if DAT_009ec5d0 == -1 {
                        if ld::<u8>(ad!(DAT_009d6940), 1) != 0
                            && ld::<u32>(DAT_009a1fe8, 0x1b + i_var5) == DAT_009ecc34
                        {
                            FUN_0042b290();
                        }
                        st::<u32>(xd(i_var6), 6, ld::<u32>(i_var6, 4));
                        st::<u32>(xd(i_var6), 10, ld::<u32>(i_var6, 8));
                        st::<u32>(xd(i_var6), 0xe, ld::<u32>(i_var6, 0xc));
                    }
                    let mut s_var12 = ld::<i16>(xd(i_var6), 0x77);
                    let ps20 = xd(i_var6).wrapping_add(0x77);
                    if s_var12 != 0 {
                        if (u_var18 as i32) < DAT_009a1fe0 {
                            let i19 = ld::<i32>(ld::<i32>(DAT_009a1fe8, 0x1f + i_var5), 0xa9);
                            if ld::<i16>(i19, 0x70) != -1 {
                                let mut z = false;
                                if s_var12 < 0 {
                                    st::<i16>(
                                        ps20,
                                        0,
                                        ((((DAT_009ec5fe as u32) << 0xc) / 0xec)
                                            .wrapping_add(s_var12 as i32 as u32))
                                            as i16,
                                    );
                                    let p = xd(i_var6).wrapping_add(0x77);
                                    if -1 < ld::<i16>(p, 0) {
                                        z = true;
                                    }
                                } else {
                                    let q = (((DAT_009ec5fe as u32) << 0xc) as i32) / -0xec;
                                    st::<i16>(ps20, 0, (q as i16).wrapping_add(s_var12));
                                    let p = xd(i_var6).wrapping_add(0x77);
                                    if ld::<i16>(p, 0) < 1 {
                                        z = true;
                                    }
                                }
                                if z {
                                    let p = xd(i_var6).wrapping_add(0x77);
                                    st::<i16>(p, 0, 0);
                                    s_var12 = ld::<i16>(xd(i_var6), 0x77);
                                    st::<u16>(i19, 0x70, 0xffff);
                                }
                            }
                        }
                        let t = ((s_var12 as i32 as u32).wrapping_mul(DAT_009ec5fe as u32) as i32
                            / 0xec)
                            .wrapping_mul(4);
                        st::<i16>(
                            i_var6,
                            0x14,
                            (ld::<i16>(i_var6, 0x14) as i32 as u32).wrapping_add(t as u32) as i16,
                        );
                        st::<u8>(xd(i_var6), 0x92, 1);
                    }
                    let mut u_var22: u32 = 0;
                    let mut i_var19 = xd(i_var6);
                    if ld::<u16>(i_var19, 0x79) != 0 {
                        let t = (((ld::<u16>(i_var19, 0x79) as u32)
                            .wrapping_mul(0x1234)
                            .wrapping_add(0x80) as i32
                            >> 8) as u32)
                            .wrapping_mul(DAT_009ec5fe as u32)
                            as i32;
                        u_var22 = ((t / 0xec) as u32) & 0xffff;
                    }
                    match u_var2 {
                        0..=4 => {
                            let i14 = ld::<i32>(i_var19, 0xa9);
                            if ld::<u8>(i_var19, 0x8b) != 0 {
                                FUN_00427bd0(
                                    ld::<i16>(i_var6, 0x12),
                                    ld::<i16>(i_var6, 0x10),
                                    i_var6,
                                );
                            }
                            DAT_009a20b0 = mul15(ld::<i16>(xd(i_var6), 0x8c) as i32, u_var22);
                            DAT_009cd26c = mul15(ld::<i16>(xd(i_var6), 0x8e) as i32, u_var22);
                            DAT_009fd438 = mul15(ld::<i16>(xd(i_var6), 0x90) as i32, u_var22);
                            if ld::<i16>(i14, 0x1e5) != 0 {
                                let mut u18: u32;
                                if ld::<u8>(i14, 0x5e) == 0x12 {
                                    u18 = 0xfa;
                                } else if ld::<u8>(i14, 0x5e) == 0x1e {
                                    u18 = 0x2ee;
                                } else {
                                    u18 = ld::<u16>(
                                        ad!(DAT_0051c593),
                                        (ld::<u8>(i14, 4) as i32) * 0x101,
                                    ) as u32;
                                }
                                i_var19 = ld::<i32>(i14, 0xb1);
                                let s12 = u18 as i16;
                                let mut s24: i16;
                                if i_var19 != 0 {
                                    if 0u32.wrapping_sub(i_var19 as u32) == u18
                                        || (0u32.wrapping_sub(u18) as i32) < i_var19
                                    {
                                        s24 = s12;
                                        if i_var19 <= u18 as i32 {
                                            s24 = i_var19 as i16;
                                        }
                                    } else {
                                        s24 = 0u32.wrapping_sub(s12 as i32 as u32) as i16;
                                    }
                                    let mut i15 = ((s24 as i32 as u32)
                                        .wrapping_mul(DAT_009ec5fe as u32)
                                        as i32)
                                        / 0xec;
                                    if i15 == 0 {
                                        i15 = i_var19;
                                    }
                                    st::<u32>(i14, 0xb1, (i_var19 as u32).wrapping_sub(i15 as u32));
                                    DAT_009a20b0 = DAT_009a20b0.wrapping_add(i15 as u32);
                                }
                                i_var19 = ld::<i32>(i14, 0xb5);
                                if i_var19 != 0 {
                                    if 0u32.wrapping_sub(i_var19 as u32) == u18
                                        || (0u32.wrapping_sub(u18) as i32) < i_var19
                                    {
                                        s24 = s12;
                                        if i_var19 <= u18 as i32 {
                                            s24 = i_var19 as i16;
                                        }
                                    } else {
                                        s24 = 0u32.wrapping_sub(s12 as i32 as u32) as i16;
                                    }
                                    let mut i15 = ((s24 as i32 as u32)
                                        .wrapping_mul(DAT_009ec5fe as u32)
                                        as i32)
                                        / 0xec;
                                    if i15 == 0 {
                                        i15 = i_var19;
                                    }
                                    st::<u32>(i14, 0xb5, (i_var19 as u32).wrapping_sub(i15 as u32));
                                    DAT_009cd26c = DAT_009cd26c.wrapping_add(i15 as u32);
                                }
                                let u22b = ld::<u32>(i14, 0xb9);
                                if u22b != 0 {
                                    if 0u32.wrapping_sub(u22b) == u18
                                        || (0u32.wrapping_sub(u18) as i32) < u22b as i32
                                    {
                                        if u22b as i32 <= u18 as i32 {
                                            u18 = u22b & 0xffff;
                                        }
                                    } else {
                                        u18 = 0u32.wrapping_sub(s12 as i32 as u32) as u16 as u32;
                                    }
                                    u18 = (((u18 as i16 as i32 as u32)
                                        .wrapping_mul(DAT_009ec5fe as u32)
                                        as i32)
                                        / 0xec) as u32;
                                    if u18 == 0 {
                                        u18 = u22b;
                                    }
                                    st::<u32>(i14, 0xb9, u22b.wrapping_sub(u18));
                                    DAT_009fd438 = DAT_009fd438.wrapping_add(u18);
                                }
                            }
                            FUN_004464c0(i_var6);
                            if ld::<u16>(i14, 0x66) != 0xffff {
                                let i15 = (ld::<u16>(i14, 0x66) as i32).wrapping_mul(0x23);
                                let i16 = DAT_009a1fe8.wrapping_add(i15);
                                let i19 = ld::<i32>(i16, 0x1f);
                                if i19 != 0 {
                                    let u2 = ld::<u8>(ld::<i32>(i19, 0xa9), 4);
                                    let u4 = (((u2 as u32) << 8) | u2 as u32) as u16 as i32;
                                    let kk = (ld::<u8>(i14, 4) as i32) * 0x101;
                                    let s12: i16;
                                    if ld::<u8>(i16, 2) < 3 {
                                        s12 = (ld::<i16>(ad!(DAT_0051c63d), kk) as i32 as u32)
                                            .wrapping_sub(
                                                ld::<i16>(ad!(DAT_0051c639), u4) as i32 as u32
                                            ) as i16;
                                    } else if ld::<u8>(DAT_009a1fe8, 2 + i_var5) < 3 {
                                        s12 = (ld::<i16>(ad!(DAT_0051c63f), kk) as i32 as u32)
                                            .wrapping_sub(
                                                ld::<i16>(ad!(DAT_0051c639), u4) as i32 as u32
                                            ) as i16;
                                    } else {
                                        s12 = (ld::<i16>(ad!(DAT_0051c63f), kk) as i32 as u32)
                                            .wrapping_sub(
                                                ld::<i16>(ad!(DAT_0051c63b), u4) as i32 as u32
                                            ) as i16;
                                    }
                                    FUN_004035d0(i_var6, 0, s12, ld::<i16>(ad!(DAT_0051c637), u4));
                                    let i15 = DAT_009a1fe8.wrapping_add(i15);
                                    st::<u32>(ld::<i32>(i15, 0x1f), 6, ld::<u32>(i15, 4));
                                    st::<u32>(ld::<i32>(i15, 0x1f), 10, ld::<u32>(i15, 8));
                                    st::<u32>(ld::<i32>(i15, 0x1f), 0xe, ld::<u32>(i15, 0xc));
                                    st::<u32>(
                                        i15,
                                        4,
                                        ld::<u32>(i_var6, 4).wrapping_add(DAT_009d8a58),
                                    );
                                    st::<u32>(
                                        i15,
                                        8,
                                        ld::<u32>(i_var6, 8).wrapping_add(DAT_009d8a54),
                                    );
                                    st::<u32>(
                                        i15,
                                        0xc,
                                        ld::<u32>(i_var6, 0xc).wrapping_add(DAT_009d8b60),
                                    );
                                }
                            }
                        }
                        6 | 7 => {
                            let i5b = ld::<i32>(i_var19, 0xa5);
                            let b10 = ld::<u8>(i5b, 1);
                            if b10 != 0 && ld::<u16>(i5b, 4) != 0xffff {
                                let mut u25 = ld::<u16>(i5b, 4) as u32;
                                if (u25 as i32) < DAT_00a07ce8 {
                                    let p =
                                        (u25 as i32).wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
                                    if ld::<u8>(p, 3) == 0 || ld::<i16>(i5b, 6) != ld::<i16>(p, 0) {
                                        FUN_0041f300(u_var18, 0x81u8 as i8);
                                        break 'next;
                                    }
                                }
                                let mut b17 = false;
                                let u4 = ld::<u16>(i5b, 2);
                                let b3 =
                                    ld::<i8>(ad!(DAT_00521c20), (ld::<u8>(i_var6, 3) as i32) + 7)
                                        as u8;
                                if (u25 as i32) < DAT_009a1fe0 {
                                    let i19 = ld::<i32>(
                                        ld::<i32>(
                                            (u25 as i32).wrapping_mul(0x23),
                                            0x1f + DAT_009a1fe8,
                                        ),
                                        0xa9,
                                    );
                                    if ld::<u8>(i19, 0x23d) == 3 && ld::<u8>(i19, 0x241) != 0 {
                                        b17 = true;
                                    }
                                }
                                if !b17 {
                                    FUN_00458e20(u25, 0);
                                    let mut cond = false;
                                    let mut i19 = 0;
                                    if (u25 as i32) < DAT_009a1fe0 {
                                        i19 = (u25 as i32)
                                            .wrapping_mul(0x23)
                                            .wrapping_add(DAT_009a1fe8);
                                        if ld::<u8>(ld::<i32>(i19, 0x1f), 0) == 0 {
                                            cond = true;
                                        }
                                    }
                                    if cond {
                                        let u13: u32;
                                        let i14: i32;
                                        let i15: i32;
                                        if u4 == 0xffff {
                                            u13 = ld::<u8>(i19, 3) as u32;
                                            let t = FUN_004ae640(u13, 0);
                                            i14 = 0i32.wrapping_sub(t);
                                            i15 = FUN_004ae6e0(u13, 0);
                                            u25 = 0;
                                        } else {
                                            u25 = u4 as u32;
                                            u13 = ld::<u8>(i19, 3) as u32;
                                            let t = FUN_004ae640(u13, u25);
                                            i14 = 0i32.wrapping_sub(t);
                                            i15 = FUN_004ae6e0(u13, u25);
                                        }
                                        let i16 = FUN_004ae5a0(u13, u25);
                                        FUN_004037b0(i19, i16, i15, i14);
                                        DAT_009d8a58 = DAT_009d8a58.wrapping_add(DAT_00a07cdc);
                                        DAT_009d8a54 = DAT_009d8a54.wrapping_add(DAT_00a07cd8);
                                        DAT_009d8b60 = DAT_009d8b60.wrapping_add(DAT_00a07cd4);
                                    } else {
                                        DAT_009d8a58 = DAT_00a07cdc;
                                        DAT_009d8a54 = DAT_00a07cd8;
                                        DAT_009d8b60 = DAT_00a07cd4;
                                    }
                                    FUN_00458e20(u_var18, 0);
                                    DAT_009d8a58 = DAT_009d8a58.wrapping_sub(DAT_00a07cdc);
                                    DAT_009d8a54 = DAT_009d8a54.wrapping_sub(DAT_00a07cd8);
                                    DAT_009d8b60 = DAT_009d8b60.wrapping_sub(DAT_00a07cd4);
                                    FUN_0046a8d0(DAT_009d8a58, DAT_009d8a54, DAT_009d8b60);
                                    let s24 = (DAT_009a8070 as i32 as u32).wrapping_sub(ld::<i16>(
                                        i_var6, 0x10,
                                    )
                                        as i32
                                        as u32)
                                        as i16;
                                    let i19 = ((b10 as u16).wrapping_add(b3 as u16) as i32) * 2;
                                    let u18 = (((ld::<u16>(ad!(DAT_00521cc8), i19) as u32)
                                        .wrapping_mul(DAT_009ec5fe as u32)
                                        as i32)
                                        / 0xec)
                                        as u32;
                                    let mut s12 = u18 as i16;
                                    let mut s_stack8 = s24;
                                    if s24 < 0 {
                                        s_stack8 = 0u32.wrapping_sub(s24 as i32 as u32) as i16;
                                    }
                                    if ((u18 & 0xffff) as i32) < s_stack8 as i32 {
                                        if s24 < 0 {
                                            s12 = 0u32.wrapping_sub(s12 as i32 as u32) as i16;
                                        }
                                        st::<i16>(
                                            i_var6,
                                            0x10,
                                            (ld::<i16>(i_var6, 0x10) as i32 as u32)
                                                .wrapping_add(s12 as i32 as u32)
                                                as i16,
                                        );
                                        let p26 = xd(i_var6).wrapping_add(0x79);
                                        if 200 < ld::<u16>(p26, 0) {
                                            let q = (((ld::<u16>(ad!(DAT_00521d20), i19) as u32)
                                                .wrapping_mul(DAT_009ec5fe as u32)
                                                as i32)
                                                / -0xec)
                                                as i16;
                                            st::<u16>(
                                                p26,
                                                0,
                                                (ld::<u16>(p26, 0) as u32)
                                                    .wrapping_add(q as i32 as u32)
                                                    as u16,
                                            );
                                            let p26 = xd(i_var6).wrapping_add(0x79);
                                            if ld::<u16>(p26, 0) < 200 {
                                                st::<u16>(p26, 0, 200);
                                            }
                                        }
                                    } else {
                                        st::<i16>(i_var6, 0x10, DAT_009a8070);
                                        let p20 = xd(i_var6).wrapping_add(0x79);
                                        if ld::<u16>(xd(i_var6), 0x79) < ld::<u16>(i5b, 8) {
                                            let q = (((ld::<u16>(ad!(DAT_00521d20), i19) as u32)
                                                .wrapping_mul(DAT_009ec5fe as u32)
                                                as i32)
                                                / 0xec)
                                                as i16;
                                            st::<i16>(
                                                p20,
                                                0,
                                                (ld::<i16>(p20, 0) as i32 as u32)
                                                    .wrapping_add(q as i32 as u32)
                                                    as i16,
                                            );
                                        }
                                    }
                                    let s24 = (DAT_00a080f0 as i32 as u32).wrapping_sub(ld::<i16>(
                                        i_var6, 0x12,
                                    )
                                        as i32
                                        as u32)
                                        as i16;
                                    let u18 = (((ld::<u16>(ad!(DAT_00521cc8), i19) as u32)
                                        .wrapping_mul(DAT_009ec5fe as u32)
                                        as i32)
                                        / 0xec)
                                        as u32;
                                    let mut s12 = u18 as i16;
                                    let mut s_stack8 = s24;
                                    if s24 < 0 {
                                        s_stack8 = 0u32.wrapping_sub(s24 as i32 as u32) as i16;
                                    }
                                    if ((u18 & 0xffff) as i32) < s_stack8 as i32 {
                                        if s24 < 0 {
                                            s12 = 0u32.wrapping_sub(s12 as i32 as u32) as i16;
                                        }
                                        st::<i16>(
                                            i_var6,
                                            0x12,
                                            (ld::<i16>(i_var6, 0x12) as i32 as u32)
                                                .wrapping_add(s12 as i32 as u32)
                                                as i16,
                                        );
                                    } else {
                                        st::<i16>(i_var6, 0x12, DAT_00a080f0);
                                    }
                                    st::<u8>(xd(i_var6), 0x92, 1);
                                    let v = ld::<u8>(xd(i_var6), 0x92);
                                    st::<u8>(xd(i_var6), 0x8b, v);
                                    FUN_00427bd0(
                                        ld::<i16>(i_var6, 0x12),
                                        ld::<i16>(i_var6, 0x10),
                                        i_var6,
                                    );
                                    st::<i16>(xd(i_var6), 0x8c, DAT_009a8da4 as i16);
                                    st::<i16>(xd(i_var6), 0x8e, DAT_009a8d84 as i16);
                                    st::<i16>(xd(i_var6), 0x90, DAT_009a8d88 as i16);
                                }
                            }
                            if ld::<u8>(xd(i_var6), 0x8b) != 0 {
                                FUN_00427bd0(
                                    ld::<i16>(i_var6, 0x12),
                                    ld::<i16>(i_var6, 0x10),
                                    i_var6,
                                );
                            }
                            DAT_009a20b0 = mul15(ld::<i16>(xd(i_var6), 0x8c) as i32, u_var22);
                            DAT_009cd26c = mul15(ld::<i16>(xd(i_var6), 0x8e) as i32, u_var22);
                            DAT_009fd438 = mul15(ld::<i16>(xd(i_var6), 0x90) as i32, u_var22);
                            FUN_004464c0(i_var6);
                        }
                        0xb | 0xd => {
                            if ld::<u8>(i_var19, 0x8b) != 0 {
                                FUN_00427bd0(
                                    ld::<i16>(i_var6, 0x12),
                                    ld::<i16>(i_var6, 0x10),
                                    i_var6,
                                );
                            }
                            DAT_009a20b0 = mul15(ld::<i16>(xd(i_var6), 0x8c) as i32, u_var22);
                            DAT_009cd26c = mul15(ld::<i16>(xd(i_var6), 0x8e) as i32, u_var22);
                            DAT_009fd438 = mul15(ld::<i16>(xd(i_var6), 0x90) as i32, u_var22);
                            FUN_004464c0(i_var6);
                        }
                        _ => {}
                    }
                }
                local_6 = local_6.wrapping_add(1);
                if !((local_6 as i32) < DAT_00a07ce8) {
                    break;
                }
            }
        }
        DAT_009d8112 = s_var9;
        DAT_009ec5fe = u_var23;
    }
}
