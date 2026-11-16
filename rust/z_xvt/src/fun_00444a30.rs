// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: *mut u8;
    static mut DAT_006d9d34: u16;
    static mut DAT_0070a1d2: i16;
    static mut DAT_0071b71c: i32;
    static mut DAT_0071e690: i32;
    static mut DAT_0071e6be: u16;
    static mut DAT_00732a3c: u16;
    static mut DAT_00732e54: *mut u8;
    static mut DAT_0052b095: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(param_1: *mut u8) -> *mut u8;
    fn FUN_00478320(param_1: u32, param_2: u16) -> u32;
    fn FUN_00445d10(param_1: u32);
    fn FUN_00445920(param_1: u32, param_2: u32, param_3: i32, param_4: u16);
}

unsafe fn rs16(p: *const u8) -> i16 {
    // SAFETY: p points into the game's own record memory.
    unsafe { p.cast::<i16>().read_unaligned() }
}

unsafe fn ru16(p: *const u8) -> u16 {
    // SAFETY: p points into the game's own record memory.
    unsafe { p.cast::<u16>().read_unaligned() }
}

unsafe fn ri32(p: *const u8) -> i32 {
    // SAFETY: p points into the game's own record memory.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn rpb(p: *const u8) -> *mut u8 {
    // SAFETY: p points at a pointer field in the game's own record memory.
    unsafe { p.cast::<*mut u8>().read_unaligned() }
}

unsafe fn w16(p: *mut u8, v: u16) {
    // SAFETY: p points into the game's own record memory.
    unsafe { p.cast::<u16>().write_unaligned(v) }
}

unsafe fn drain(uvar9: u32, uvar15: u32, uvar7: u32) -> (u16, u16) {
    // SAFETY: the record table and the pointers stored in it are the game's own.
    unsafe {
        let iv14 = uvar9.wrapping_mul(0x23) as usize;
        let iv19 = rpb(DAT_006d4988.add(0x1f + iv14));
        let old = ru16(iv19.add(0x7b));
        let d = uvar7
            .wrapping_mul(0xFFFFFF14)
            .wrapping_add(uvar15)
            .wrapping_mul(0x10000)
            / 0xec;
        let new = old.wrapping_sub(d as u16);
        w16(iv19.add(0x7b), new);
        let iv19b = rpb(DAT_006d4988.add(0x1f + iv14));
        if old < ru16(iv19b.add(0x7b)) {
            let p = iv19b.add(0x79);
            w16(p, (rs16(p) as u16).wrapping_sub(1));
        }
        let p = rpb(DAT_006d4988.add(0x1f + iv14)).add(0x79);
        w16(p, (rs16(p) as u16).wrapping_sub(uvar7 as u16));
        let p2 = rpb(DAT_006d4988.add(0x1f + iv14)).add(0x79);
        if 0x8000 < ru16(p2) {
            w16(p2, 0);
        }
        (old, new)
    }
}

unsafe fn fill(uvar9: u32, mult: u32) {
    // SAFETY: the record table and the pointers stored in it are the game's own.
    unsafe {
        let dat = DAT_0071e6be as u32;
        let uvar7 = dat.wrapping_mul(mult) / 0xec;
        let iv14 = uvar9.wrapping_mul(0x23) as usize;
        let iv19 = rpb(DAT_006d4988.add(0x1f + iv14));
        let old = ru16(iv19.add(0x7b));
        let d = dat
            .wrapping_mul(mult << 16)
            .wrapping_add(uvar7.wrapping_mul(0xFF140000))
            / 0xec;
        w16(iv19.add(0x7b), (d as u16).wrapping_add(old));
        let iv19b = rpb(DAT_006d4988.add(0x1f + iv14));
        if ru16(iv19b.add(0x7b)) < old {
            let p = iv19b.add(0x79);
            w16(p, (rs16(p) as u16).wrapping_add(1));
        }
        let p = rpb(DAT_006d4988.add(0x1f + iv14)).add(0x79);
        w16(p, (rs16(p) as u16).wrapping_add(uvar7 as u16));
        let p2 = rpb(DAT_006d4988.add(0x1f + iv14)).add(0x79);
        if 0xe10 < ru16(p2) {
            w16(p2, 0xe10);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00444a30() {
    // SAFETY: all globals, callees and record pointers belong to the game;
    // the record table is read and written at the offsets the original uses.
    unsafe {
        macro_rules! rec {
            ($o:expr) => {
                DAT_006d4988.add($o)
            };
        }
        macro_rules! e {
            ($o:expr) => {
                DAT_00732e54.add($o)
            };
        }

        let u12 = DAT_0071e6be;
        let s11 = DAT_0070a1d2;
        let mut local_16 = DAT_00732a3c;
        let mut local_10: u32 = 0;
        let mut b10 = false;

        if (local_16 as i32) < DAT_006d4980 {
            loop {
                if DAT_0071e690 != -1 {
                    if b10 {
                        DAT_0070a1d2 = s11;
                        DAT_0071e6be = u12;
                        return;
                    }
                    b10 = true;
                    local_16 = DAT_0071e690 as u16;
                }
                let u9 = local_16 as u32;
                let iv8 = u9.wrapping_mul(0x23) as usize;
                let mut iv19 = rec!(iv8);
                DAT_0070a1d2 = s11;
                DAT_0071e6be = u12;
                let mut run = false;
                if ri32(iv19.add(0x1f)) == 0 {
                    run = true;
                } else {
                    let iv14 = ri32(rpb(iv19.add(0x1f)).add(2));
                    if iv14 == 0 {
                        run = true;
                    } else {
                        DAT_0071e6be = u12
                            .wrapping_add((DAT_0071b71c as i16).wrapping_sub(iv14 as i16) as u16);
                        if DAT_0071e6be != 0 {
                            DAT_0070a1d2 = (0xecu32 / (DAT_0071e6be as u32)) as i16;
                            if DAT_0070a1d2 == 0 {
                                DAT_0070a1d2 = 1;
                            }
                            run = true;
                        }
                    }
                }
                if run && *iv19.add(3) != 0 && *rpb(iv19.add(0x1f)) == 0 {
                    DAT_00732e54 = rpb(rpb(iv19.add(0x1f)).add(0xa7));
                    let pu14 = FUN_0042b150(DAT_00732e54);
                    let s4 = rs16(rec!(0x12 + iv8));
                    iv19 = rec!(iv8);
                    let s5 = rs16(iv19.add(0x10));
                    let s6 = rs16(iv19.add(0x14));
                    let mut u22: u16 = 0;
                    DAT_006d9d34 = *e!(4) as u16;
                    let c = if ri32(iv19.add(0x1b)) == -1 {
                        rs16(e!(0x1df)) != 0
                    } else {
                        (*e!(0x1df) & 0x40) != 0
                    };
                    if c {
                        u22 = ru16(e!(0xbd));
                    }
                    if ri32(iv19.add(0x1b)) == -1 && rs16(e!(0x1df)) != 0 && ri32(e!(0x19)) == 0 {
                        if *e!(0xa4) != 0 && *e!(0xa4) < 4 {
                            let u13 =
                                rs16(pu14.add(0x30)).wrapping_sub(rs16(iv19.add(0x14))) as u16;
                            let mut u15 = FUN_00478320(
                                ((ru16(e!(0xa0)) as u32).wrapping_mul(DAT_0071e6be as u32) as i32
                                    / 0xec) as u32,
                                ru16(e!(0xa2)),
                            );
                            u15 = FUN_00478320(u15, ru16(e!(0xa5)));
                            let u17 = u15.wrapping_mul(2) as u16;
                            let act: u8;
                            if *e!(0xa4) == 3 {
                                act = if ru16(pu14.add(0x30)) < 0x8000 { 1 } else { 2 };
                            } else if u13 < 0x8000 {
                                act = if u17 < u13 { 1 } else { 3 };
                            } else {
                                act = if u17 < 0u16.wrapping_sub(u13) { 2 } else { 3 };
                            }
                            match act {
                                1 => {
                                    let p = rec!(0x14 + iv8);
                                    w16(p, (rs16(p) as u16).wrapping_add(u17));
                                }
                                2 => {
                                    let p = rec!(0x14 + iv8);
                                    w16(
                                        p,
                                        (rs16(p) as u16).wrapping_sub((u15 as u16).wrapping_mul(2)),
                                    );
                                }
                                _ => {
                                    w16(rec!(0x14 + iv8), rs16(pu14.add(0x30)) as u16);
                                    *e!(0xa4) = 4;
                                }
                            }
                        }
                        if *e!(0x9c) != 0 {
                            let mut ustk = rs16(pu14.add(0x2e)).wrapping_sub(rs16(e!(0xd))) as u16;
                            if 0x7fff < ustk {
                                ustk = 0u16.wrapping_sub(ustk);
                            }
                            let mut u15 = FUN_00478320(
                                ((ru16(e!(0x98)) as u32).wrapping_mul(DAT_0071e6be as u32) as i32
                                    / 0xec) as u32,
                                ru16(e!(0x9a)),
                            );
                            u15 = FUN_00478320(u15, ru16(e!(0x9e)));
                            let u13 = u15 as u16;
                            local_10 = (local_10 & 0xffff0000) | (u13 as u32);
                            FUN_00478320(local_10, 0x8000);
                            if *e!(0x9c) == 1 {
                                if u13 < ustk || *e!(0x9d) != 0 {
                                    w16(e!(0xd), (rs16(e!(0xd)) as u16).wrapping_sub(u13));
                                    if 0xdfff < ru16(e!(0xd)) {
                                        w16(e!(0xd), 0u16.wrapping_sub(ru16(e!(0xd))));
                                        let p = rec!(0x10 + iv8);
                                        w16(p, (rs16(p) as u16).wrapping_add(0x8000));
                                        let p = rec!(0x14 + iv8);
                                        w16(p, (rs16(p) as u16).wrapping_add(0x8000));
                                        *e!(0x9d) = 0;
                                        *e!(0x9c) = 2;
                                    }
                                } else {
                                    w16(e!(0xd), rs16(pu14.add(0x2e)) as u16);
                                    *e!(0x9c) = 3;
                                }
                            } else if *e!(0x9c) == 2 {
                                if u13 < ustk || *e!(0x9d) != 0 {
                                    w16(e!(0xd), (rs16(e!(0xd)) as u16).wrapping_add(u13));
                                    if 0x7fff < ru16(e!(0xd)) {
                                        w16(e!(0xd), 0u16.wrapping_sub(ru16(e!(0xd))));
                                        let p = rec!(0x10 + iv8);
                                        w16(p, (rs16(p) as u16).wrapping_add(0x8000));
                                        let p = rec!(0x14 + iv8);
                                        w16(p, (rs16(p) as u16).wrapping_add(0x8000));
                                        *e!(0x9d) = 0;
                                        *e!(0x9c) = 1;
                                    }
                                } else {
                                    w16(e!(0xd), rs16(pu14.add(0x2e)) as u16);
                                    *e!(0x9c) = 3;
                                }
                            }
                        }
                        if *e!(7) != 2 && *e!(0xab) != 0 {
                            let u13 =
                                rs16(pu14.add(0x32)).wrapping_sub(rs16(rec!(0x10 + iv8))) as u16;
                            local_10 = (local_10 & 0xffff0000) | (u13 as u32);
                            if u13 != 0 {
                                let mut u15 = FUN_00478320(
                                    ((ru16(e!(0xa7)) as u32).wrapping_mul(DAT_0071e6be as u32)
                                        as i32
                                        / 0xec) as u32,
                                    ru16(e!(0xa9)),
                                );
                                u15 = FUN_00478320(u15, ru16(e!(0xac)));
                                let u17 = u15 as u16;
                                let act: u8;
                                if u13 < 0x8000 {
                                    act = if u17 < u13 { 1 } else { 3 };
                                } else {
                                    act = if 0u16.wrapping_sub(u13) <= u17 { 3 } else { 2 };
                                }
                                match act {
                                    1 => {
                                        let p = rec!(0x10 + iv8);
                                        w16(p, (rs16(p) as u16).wrapping_add(u17));
                                    }
                                    2 => {
                                        let p = rec!(0x10 + iv8);
                                        w16(p, (rs16(p) as u16).wrapping_sub(u17));
                                    }
                                    _ => {
                                        w16(rec!(0x10 + iv8), rs16(pu14.add(0x32)) as u16);
                                        u15 &= 0xffff0000;
                                        *e!(0xab) = 3;
                                    }
                                }
                                if *e!(0xa4) == 0 || *e!(0xa4) == 4 {
                                    let tp = (&raw mut DAT_0052b095)
                                        .add((DAT_006d9d34 as usize).wrapping_mul(0xef));
                                    u15 = FUN_00478320(u15, ru16(tp));
                                    let p = rec!(0x14 + iv8);
                                    if u13 < 0x8000 {
                                        w16(p, (rs16(p) as u16).wrapping_sub(u15 as u16));
                                    } else {
                                        w16(p, (rs16(p) as u16).wrapping_add(u15 as u16));
                                    }
                                }
                            }
                        }
                    }
                    if ri32(rec!(iv8 + 0x1b)) == -1
                        && *e!(0x96) == 1
                        && rs16(e!(0x1df)) != 0
                        && ri32(e!(0x19)) == 0
                        && ri32(pu14.add(0x27)) <= ri32(rec!(iv8 + 0xc))
                    {
                        *e!(0x96) = 0;
                        w16(e!(0xd), 0x4000);
                    }
                    if ri32(rec!(0x1b + iv8)) == -1
                        && *e!(0x97) == 1
                        && rs16(e!(0x1df)) != 0
                        && ri32(e!(0x19)) == 0
                    {
                        FUN_00445d10(u9);
                    }
                    let mut fall6 = false;
                    match *e!(7) {
                        0 => {
                            w16(rec!(0x12 + iv8), rs16(e!(0xd)) as u16);
                            let mut u15 = ru16(e!(0xc1)) as u32;
                            if ru16(e!(0xc1)) == 0 || u22 != 0xffff {
                                let s2 = rs16(e!(0x92));
                                let u21 = ((pu14 as usize as u32) >> 16) as u16;
                                let s18 = (6i32
                                    .wrapping_sub(*e!(0x234) as i32)
                                    .wrapping_sub(*e!(0x212) as i32)
                                    .wrapping_sub(*e!(0x20f) as i32))
                                    as i16;
                                let c3 = *rec!(3 + iv8) as i8;
                                let u13: u16 = if c3 == 7 {
                                    0x1000
                                } else if c3 == 5 && 0 < s18 {
                                    0x3000
                                } else {
                                    0x2000
                                };
                                u15 = FUN_00478320(((u21 as u32) << 16) | (s2 as u16 as u32), u13);
                                let t = (s2 as i32)
                                    .wrapping_add((u15 as i16 as i32).wrapping_mul(s18 as i32))
                                    as u16;
                                u15 = FUN_00478320(((u21 as u32) << 16) | (t as u32), u22);
                                if rs16(e!(0xbf)) == 0 {
                                    u15 = (u15 as i16).wrapping_mul(2) as u16 as u32;
                                }
                            }
                            let u13 = ru16(rpb(rec!(0x1f + iv8)).add(0x79));
                            if (u15 as u16) < u13 {
                                let u20 = (u13 as u32).wrapping_sub(u15 & 0xffff);
                                if u20 < 200 {
                                    FUN_00445920(u9, u15 & 0xffff, 1, u22);
                                } else {
                                    let n = (u20 / 3).wrapping_mul(DAT_0071e6be as u32);
                                    let u7 = n / 0xec;
                                    let (o, _) = drain(u9, n, u7);
                                    local_10 = (local_10 & 0xffff0000) | (o as u32);
                                }
                            } else {
                                FUN_00445920(u9, u15 & 0xffff, 1, u22);
                            }
                        }
                        1 | 3 | 4 => {
                            let u22b = ru16(rpb(rec!(0x1f + iv8)).add(0x79));
                            if ru16(e!(0x92)) < u22b {
                                let n = (DAT_0071e6be as u32).wrapping_mul(
                                    ((u22b as i32).wrapping_sub(ru16(e!(0x92)) as i32) / 3) as u32,
                                );
                                let u7 = n / 0xec;
                                let (o, _) = drain(u9, n, u7);
                                local_10 = (local_10 & 0xffff0000) | (o as u32);
                            }
                            fall6 = true;
                        }
                        6 => {
                            fall6 = true;
                        }
                        2 => {
                            if rs16(rpb(rec!(0x1f + iv8)).add(0x79)) != 0 {
                                let n = (DAT_0071e6be as u32).wrapping_mul(0x14);
                                let u7 = n / 0xec;
                                let (_, nw) = drain(u9, n, u7);
                                local_10 = (local_10 & 0xffff0000) | (nw as u32);
                            }
                        }
                        5 => {
                            if rs16(e!(0x1df)) == 0 {
                                if rs16(rpb(rec!(0x1f + iv8)).add(0x79)) != 0 {
                                    let n = (DAT_0071e6be as u32).wrapping_mul(0x14);
                                    let u7 = n / 0xec;
                                    let (_, nw) = drain(u9, n, u7);
                                    local_10 = (local_10 & 0xffff0000) | (nw as u32);
                                }
                                if rs16(rpb(rec!(0x1f + iv8)).add(0x79)) == 0 {
                                    *e!(7) = 0;
                                }
                            } else if rs16(pu14.add(0x3a)) == 0 {
                                if ri32(pu14.add(0x36)) == 0 {
                                    local_10 = rpb(rec!(0x1f + iv8)).add(0x7b) as usize as u32;
                                    fill(u9, 500);
                                } else {
                                    fill(u9, 200);
                                }
                            } else {
                                fill(u9, 0x32);
                            }
                        }
                        _ => {}
                    }
                    if fall6 {
                        *e!(0x96) = 0;
                        *e!(0x97) = 0;
                        *e!(0xa4) = 0;
                        *e!(0x9c) = 0;
                    }
                    w16(e!(0xf), rs16(rec!(0x10 + iv8)) as u16);
                    iv19 = rec!(iv8);
                    if rs16(iv19.add(0x12)) != s4
                        || rs16(iv19.add(0x10)) != s5
                        || rs16(iv19.add(0x14)) != s6
                    {
                        *rpb(iv19.add(0x1f)).add(0x89) = 1;
                        *rpb(rec!(0x1f + iv8)).add(0x90) = 1;
                    }
                    if ru16(e!(0x66)) != 0xffff {
                        let iv14 = ((ru16(e!(0x66)) as u32).wrapping_mul(0x23)) as usize;
                        if ri32(rec!(iv14 + 0x1f)) != 0 {
                            w16(rec!(iv14 + 0x12), rs16(rec!(0x12 + iv8)) as u16);
                            w16(rec!(0x10 + iv14), rs16(rec!(0x10 + iv8)) as u16);
                            w16(rec!(0x14 + iv14), rs16(rec!(0x14 + iv8)) as u16);
                            w16(
                                rpb(rpb(rec!(0x1f + iv14)).add(0xa7)).add(0xd),
                                rs16(e!(0xd)) as u16,
                            );
                            *rpb(rec!(0x1f + iv14)).add(0x89) = 1;
                            *rpb(rec!(0x1f + iv14)).add(0x90) = 1;
                        }
                    }
                }
                local_16 = local_16.wrapping_add(1);
                if !((local_16 as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
        DAT_0070a1d2 = s11;
        DAT_0071e6be = u12;
    }
}
