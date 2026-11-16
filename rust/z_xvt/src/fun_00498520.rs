// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071e6a0: i16;
    static mut DAT_0071e6b6: i16;
    static mut DAT_0071e6be: u16;
    static mut DAT_00525550: i32;
    static mut DAT_0070ac40: i16;
    static mut DAT_0070ac50: i16;
    static mut DAT_0071b740: i8;
    static mut DAT_0071e528: i8;
    static mut DAT_0071bcdd: u8;
    static mut DAT_0071eac5: u8;
    static mut _DAT_00732a3c: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00732e54: i32;
    static mut DAT_00703acc: i32;
    static mut DAT_0073088c: i32;
    static mut _DAT_006da9e4: i16;
    static mut _DAT_0070a13c: u16;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b74a: u8;
    static mut DAT_0071b730: u8;
    static mut DAT_0071b759: u8;
    static mut DAT_006db580: i32;
    static mut DAT_0052686c: u8;
    static mut DAT_007091ac: u16;
    static mut DAT_006fedd0: u16;
    static mut _DAT_00732e5a: u16;
    static mut _DAT_0071f2de: i16;
    static mut DAT_0071f2dd: u8;
    static mut DAT_0071f2dc: u8;
    static mut DAT_0071f2db: u8;
    static mut DAT_0070ac34: u8;
    static mut DAT_0070ac35: i8;
    static mut DAT_007092dc: u8;
    static mut DAT_007092dd: u8;
    static mut DAT_007092de: i8;
    static mut DAT_007092c0: u8;
    static mut DAT_007092e4: i32;
    static mut DAT_006d4a02: i8;
    static mut DAT_0070ad54: i8;
    static mut DAT_0070ad48: u8;
    static mut DAT_00709d0a: i8;
    static mut DAT_00709d0b: i8;
    static mut DAT_00709d0d: i8;
    static mut DAT_0071e518: u8;
    static mut DAT_00523b70: u16;
    static mut DAT_00523b84: u16;
    static mut DAT_00523b88: u8;
    static mut _DAT_00732952: u16;
    static mut _DAT_00732950: u16;
    static mut DAT_00732a38: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042a140(param_1: u32, param_2: u32);
    fn FUN_0040a1a0(param_1: u32, param_2: u32, param_3: i32);
    fn FUN_004a1d70(param_1: i32, param_2: i32);
    fn FUN_004a2e30();
    fn FUN_0042b1e0(param_1: i32, param_2: i32) -> i32;
}

unsafe fn rd<T>(a: u32) -> T {
    // SAFETY: the address is one the original reads in the game's own memory.
    unsafe { (a as usize as *const T).read_unaligned() }
}

unsafe fn wr<T>(a: u32, v: T) {
    // SAFETY: the address is one the original writes in the game's own memory.
    unsafe { (a as usize as *mut T).write_unaligned(v) }
}

fn ad<T>(p: *const T) -> u32 {
    p as usize as u32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00498520() {
    // SAFETY: all globals, callees and computed addresses are the game's own
    // memory, accessed in the same order and width as the original.
    unsafe {
        let mut local_c: [i32; 3] = [0; 3];

        let mut p9: *mut i16 = &raw mut DAT_0071e6a0;
        loop {
            if *p9 != 0 {
                let s = (*p9).wrapping_sub(DAT_0071e6be as i16);
                *p9 = s;
                if s < 0 {
                    *p9 = 0;
                }
            }
            p9 = p9.add(1);
            if !((p9 as usize) < (&raw const DAT_0071e6b6 as usize)) {
                break;
            }
        }
        if DAT_00525550 == 0 {
            let mut p9: *mut i16 = &raw mut DAT_0070ac40;
            loop {
                let mut pc: *mut i8 = &raw mut DAT_0071b740;
                let mut p4: *mut i16 = p9;
                loop {
                    if *pc != 0 && *p4 != 0 {
                        let s = (*p4).wrapping_sub(DAT_0071e6be as i16);
                        *p4 = s;
                        if s < 0 {
                            *p4 = 0;
                        }
                    }
                    p4 = p4.add(8);
                    pc = pc.byte_add(0x5bd);
                    if !((pc as usize) < (&raw const DAT_0071e528 as usize)) {
                        break;
                    }
                }
                p9 = p9.add(1);
                if !((p9 as usize) < (&raw const DAT_0070ac50 as usize)) {
                    break;
                }
            }
        }
        let mut pb: *mut u8 = &raw mut DAT_0071bcdd;
        loop {
            let a = ad(pb as *const u8);
            let t: i32 = rd(a);
            if t != 0 {
                let v = t.wrapping_sub(DAT_0071e6be as i32);
                wr::<i32>(a, v);
                if v < 0 {
                    wr::<i32>(a, 0);
                }
            }
            let t: i32 = rd(a.wrapping_add(4));
            if t != 0 {
                let v = t.wrapping_sub(DAT_0071e6be as i32);
                wr::<i32>(a.wrapping_add(4), v);
                if v < 0 {
                    wr::<i32>(a.wrapping_add(4), 0);
                }
            }
            pb = pb.byte_add(0x5bd);
            if !((pb as usize) < (&raw const DAT_0071eac5 as usize)) {
                break;
            }
        }

        if _DAT_00732a3c < DAT_006d4980 {
            let mut i5: u32 = (_DAT_00732a3c as u32).wrapping_mul(0x23);
            let mut i15: i32 = _DAT_00732a3c;
            loop {
                let base = DAT_006d4988 as u32;
                if rd::<u8>(base.wrapping_add(i5).wrapping_add(3)) != 0 {
                    let mut i12: i32 = rd(base.wrapping_add(i5).wrapping_add(0x1f));
                    i12 = rd((i12 as u32).wrapping_add(0xa7));
                    DAT_00732e54 = i12;
                    let o = i12 as u32;
                    if rd::<i32>(o.wrapping_add(0x3c)) != 0 {
                        let v = rd::<i32>(o.wrapping_add(0x3c)).wrapping_sub(DAT_0071e6be as i32);
                        wr::<i32>(o.wrapping_add(0x3c), v);
                    }
                    if rd::<i32>(o.wrapping_add(0x60)) != 0 {
                        let v6 = rd::<i32>(o.wrapping_add(0x60)).wrapping_sub(DAT_0071e6be as i32);
                        wr::<i32>(o.wrapping_add(0x60), v6);
                        if v6 < 0 {
                            wr::<i32>(o.wrapping_add(0x60), 0);
                        }
                    }
                    if rd::<i16>(o.wrapping_add(100)) != 0 {
                        let s = rd::<i16>(o.wrapping_add(100)).wrapping_sub(DAT_0071e6be as i16);
                        wr::<i16>(o.wrapping_add(100), s);
                        if s < 0 {
                            wr::<i16>(o.wrapping_add(100), 0);
                        }
                    }
                    let u10: u16 = rd((DAT_00732e54 as u32).wrapping_add(0x1e1));
                    if u10 != 0 {
                        let t_us: u16 = u10.wrapping_sub(DAT_0071e6be);
                        wr::<u16>((DAT_00732e54 as u32).wrapping_add(0x1e1), t_us);
                        let t_us2: u16 = rd((DAT_00732e54 as u32).wrapping_add(0x1e1));
                        if u10 < t_us2 {
                            wr::<u16>((DAT_00732e54 as u32).wrapping_add(0x1e1), 0);
                        }
                    }
                    let a240 = (DAT_00732e54 as u32).wrapping_add(0x240);
                    if rd::<i16>(a240) != 0 {
                        let v = rd::<i16>(a240).wrapping_sub(DAT_0071e6be as i16);
                        wr::<i16>((DAT_00732e54 as u32).wrapping_add(0x240), v);
                        if rd::<i16>((DAT_00732e54 as u32).wrapping_add(0x240)) < 0 {
                            wr::<i16>((DAT_00732e54 as u32).wrapping_add(0x240), 0);
                        }
                    }
                    let mut i12: i32 = 0;
                    if rd::<u8>((DAT_00732e54 as u32).wrapping_add(0x213)) != 0 {
                        loop {
                            let i6 = FUN_0042b1e0(DAT_00732e54, i12);
                            let a2 = (i6 as u32).wrapping_add(2);
                            if 0 < rd::<i16>(a2) {
                                let v = rd::<i16>(a2).wrapping_sub(DAT_0071e6be as i16);
                                wr::<i16>(a2, v);
                            }
                            i12 = i12.wrapping_add(1);
                            if !(i12 < rd::<u8>((DAT_00732e54 as u32).wrapping_add(0x213)) as i32) {
                                break;
                            }
                        }
                    }
                }
                i5 = i5.wrapping_add(0x23);
                i15 = i15.wrapping_add(1);
                if !(i15 < DAT_006d4980) {
                    break;
                }
            }
        }

        if DAT_00703acc < DAT_0073088c {
            let mut i5: u32 = (DAT_00703acc as u32).wrapping_mul(0x23);
            let mut i15: i32 = DAT_00703acc;
            loop {
                let base = DAT_006d4988 as u32;
                if rd::<u8>(base.wrapping_add(i5).wrapping_add(3)) != 0 {
                    let mut i12: i32 = rd(base.wrapping_add(i5).wrapping_add(0x1f));
                    i12 = rd((i12 as u32).wrapping_add(0xab));
                    let o = i12 as u32;
                    let t: i32 = rd(o.wrapping_add(0x16));
                    if t != 0 {
                        let v = t.wrapping_sub(DAT_0071e6be as i32);
                        wr::<i32>(o.wrapping_add(0x16), v);
                    }
                    let t: i32 = rd(o.wrapping_add(0x3a));
                    if t != 0 {
                        let v6 = t.wrapping_sub(DAT_0071e6be as i32);
                        wr::<i32>(o.wrapping_add(0x3a), v6);
                        if v6 < 0 {
                            wr::<i32>(o.wrapping_add(0x3a), 0);
                        }
                    }
                    if rd::<i16>(o.wrapping_add(0x3e)) != 0 {
                        let s = rd::<i16>(o.wrapping_add(0x3e)).wrapping_sub(DAT_0071e6be as i16);
                        wr::<i16>(o.wrapping_add(0x3e), s);
                        if s < 0 {
                            wr::<i16>(o.wrapping_add(0x3e), 0);
                        }
                    }
                }
                i5 = i5.wrapping_add(0x23);
                i15 = i15.wrapping_add(1);
                if !(i15 < DAT_0073088c) {
                    break;
                }
            }
        }

        _DAT_006da9e4 = _DAT_006da9e4.wrapping_sub(DAT_0071e6be as i16);
        if _DAT_006da9e4 < 0 {
            _DAT_0070a13c ^= 0x400;
            let u10: u16 =
                rd(ad(&raw const DAT_0071b74a)
                    .wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd)));
            if u10 != 0xffff {
                let u17: u32 = rd(ad(&raw const DAT_0071b730)
                    .wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd)));
                if u17 != 0xffff_ffff {
                    FUN_0042a140(u10 as u32, u17);
                    DAT_006db580 >>= 5;
                    let t_us: u16 = rd(ad(&raw const DAT_0071b74a)
                        .wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd)));
                    let idx: u8 = rd((t_us as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(3)
                        .wrapping_add(DAT_006d4988 as u32));
                    let v: u16 =
                        rd(ad(&raw const DAT_0052686c)
                            .wrapping_add((idx as u32).wrapping_mul(0x16)));
                    local_c[0] = (local_c[0] & !0xffff) | (v as i32);
                }
            }
            let c = local_c[0] as u16 as i32;
            if (_DAT_0070a13c & 0x400) == 0 {
                _DAT_006da9e4 = 0xe;
                if c <= DAT_006db580 {
                    _DAT_006da9e4 = 0x76;
                }
            } else if DAT_006db580 < c {
                _DAT_006da9e4 = 0x76;
            } else {
                _DAT_006da9e4 = 0xe;
            }
        }
        DAT_007091ac =
            rd(ad(&raw const DAT_0071b759).wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd)));
        let t_us: u16 =
            rd(ad(&raw const DAT_0071b74a).wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd)));
        DAT_006fedd0 = t_us | _DAT_00732e5a | _DAT_0070a13c;
        _DAT_0071f2de = _DAT_0071f2de.wrapping_sub(DAT_0071e6be as i16);
        if 0 < _DAT_0071f2de {
            return;
        }
        _DAT_0071f2de = _DAT_0071f2de.wrapping_add(0xec);
        DAT_0071f2dd = DAT_0071f2dd.wrapping_add(1);
        if 0x3b < DAT_0071f2dd {
            DAT_0071f2dd = 0;
            DAT_0071f2dc = DAT_0071f2dc.wrapping_add(1);
            if 0x3b < DAT_0071f2dc {
                DAT_0071f2dc = 0;
                DAT_0071f2db = DAT_0071f2db.wrapping_add(1);
                if 0x17 < DAT_0071f2db {
                    DAT_0071f2db = 0;
                }
            }
        }

        'a: {
            if DAT_0070ac34 == 0 && DAT_0070ac35 == 0 {
            } else {
                DAT_0070ac35 = DAT_0070ac35.wrapping_add(-1);
                if DAT_0070ac35 == -1 {
                    DAT_0070ac35 = 0x3b;
                    DAT_0070ac34 = DAT_0070ac34.wrapping_sub(1);
                    if DAT_0070ac34 == 0xff {
                        DAT_0070ac35 = 0;
                        DAT_0070ac34 = 0;
                    }
                }
                if DAT_007092dc != 0 {
                    if DAT_0070ac34 == 0 && DAT_0070ac35 == 0 && DAT_00525550 == 0 {
                        let mut pc: *mut i8 = &raw mut DAT_0071b740;
                        loop {
                            *pc = 2;
                            pc = pc.byte_add(0x5bd);
                            if !((pc as usize) < (&raw const DAT_0071e528 as usize)) {
                                break;
                            }
                        }
                        DAT_007092c0 = 1;
                    }
                } else {
                    break 'a;
                }
            }
            if DAT_007092dc != 0 {
                if DAT_0070ac34 == 2 && DAT_0070ac35 == 0 {
                    FUN_0040a1a0(0x23, 0xffff_ffff, DAT_0071ecf4);
                    FUN_004a1d70(200, DAT_0071ecf4);
                }
                if DAT_0070ac34 == 1 && DAT_0070ac35 == 0 {
                    FUN_0040a1a0(0x59, 0xffff_ffff, DAT_0071ecf4);
                    FUN_004a1d70(0xc9, DAT_0071ecf4);
                }
                if DAT_0070ac34 == 0 {
                    if DAT_0070ac35 == 0x0f {
                        let i5: i32 = rd(ad(&raw const DAT_0071b730)
                            .wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd)));
                        let b2: i32;
                        if i5 == -1 {
                            b2 = 0;
                        } else {
                            let c: i8 = rd((i5 as u32)
                                .wrapping_mul(0x23)
                                .wrapping_add(3)
                                .wrapping_add(DAT_006d4988 as u32));
                            if c != 1 && c != 2 && c != 3 && c != 0xe && c != 4 {
                                b2 = 0;
                            } else {
                                b2 = 1;
                            }
                        }
                        let u17: u32;
                        if b2 != 0 {
                            u17 = 0x54;
                        } else {
                            FUN_0040a1a0(0x59, 0xffff_ffff, DAT_0071ecf4);
                            u17 = 0x59;
                        }
                        FUN_0040a1a0(u17, 0xffff_ffff, DAT_0071ecf4);
                    }
                    if DAT_0070ac34 == 0 && DAT_0070ac35 == 2 {
                        FUN_004a1d70(0xca, DAT_0071ecf4);
                    }
                }
            }
        }

        'b: {
            if DAT_007092e4 < 2 || DAT_007092dd == 0 || DAT_007092de != 0 {
                break 'b;
            }
            'c: {
                if (DAT_006d4a02 == 2 || DAT_006d4a02 == 4)
                    && (DAT_007092dd < DAT_0070ac34 || (DAT_0070ac34 == 0 && DAT_0070ac35 == 0))
                {
                    local_c = [0; 3];
                    let mut pc: *mut i8 = &raw mut DAT_0071b740;
                    loop {
                        if *pc == 1 {
                            let t_us: u16 = rd(ad(pc as *const i8).wrapping_sub(4));
                            (&raw mut local_c).cast::<u8>().add(t_us as usize).write(1);
                        }
                        pc = pc.byte_add(0x5bd);
                        if !((pc as usize) < (&raw const DAT_0071e528 as usize)) {
                            break;
                        }
                    }
                    if DAT_006d4a02 == 2 && _DAT_00732a3c < DAT_006d4980 {
                        let mut i5: i32 =
                            (DAT_006d4980 as u32).wrapping_sub(_DAT_00732a3c as u32) as i32;
                        let mut p16: u32 = (_DAT_00732a3c as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(0x16)
                            .wrapping_add(DAT_006d4988 as u32);
                        loop {
                            let k: u8 = rd(p16);
                            if rd::<u8>(p16.wrapping_sub(0x13)) != 0
                                && rd::<i8>(
                                    ad(&raw const DAT_0070ad54)
                                        .wrapping_add((k as u32).wrapping_mul(0x566)),
                                ) != 0
                            {
                                let v: u8 = rd(ad(&raw const DAT_0070ad48)
                                    .wrapping_add((k as u32).wrapping_mul(0x566)));
                                (&raw mut local_c).cast::<u8>().add(v as usize).write(1);
                            }
                            p16 = p16.wrapping_add(0x23);
                            i5 = i5.wrapping_sub(1);
                            if i5 == 0 {
                                break;
                            }
                        }
                    }
                    let mut i5: i32 = 0;
                    let mut i15: i32 = 0;
                    loop {
                        if (&raw const local_c).cast::<u8>().add(i15 as usize).read() != 0 {
                            i5 = i5.wrapping_add(1);
                            local_c[0] = i15;
                        }
                        i15 = i15.wrapping_add(1);
                        if !(i15 < 10) {
                            break;
                        }
                    }
                    if i5 == 1 {
                        if DAT_006d4a02 != 2 {
                            let off = local_c[0].wrapping_mul(3) as isize;
                            if (*(&raw const DAT_00709d0a).offset(off) != 1
                                && *(&raw const DAT_00709d0a).offset(off) != 2)
                                && *(&raw const DAT_00709d0b).offset(off) != 1
                            {
                                break 'c;
                            }
                        }
                        DAT_0070ac35 = 0;
                        DAT_007092de = 1;
                        DAT_0070ac34 = DAT_007092dd;
                        DAT_007092dc = DAT_007092dd;
                    }
                }
            }
            if DAT_007092de == 0 && DAT_006d4a02 == 4 && (DAT_00709d0a == 1 || DAT_00709d0d == 1) {
                DAT_0070ac35 = 0;
                DAT_007092de = 1;
                DAT_0070ac34 = DAT_007092dd;
                DAT_007092dc = DAT_007092dd;
            }
        }

        local_c[0] = 0;
        let mut pb: *mut u8 = &raw mut DAT_0071b730;
        loop {
            let a = ad(pb as *const u8);
            let mut i5: i32 = rd(a);
            if rd::<i8>(a.wrapping_add(16)) != 0 && i5 != -1 {
                let mut u10: u16 = 0xffff;
                let mut local_12: u16 = 0xffff;
                let i12: i32 = rd(a);
                i5 = rd((i12 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_006d4988 as u32));
                i5 = rd((i5 as u32).wrapping_add(0xa7));
                let o = i5 as u32;
                let t_s: i16 = rd(o.wrapping_add(0x1df));
                if t_s != 0 {
                    let mut i15: i32 = 0;
                    loop {
                        if rd::<i16>(
                            o.wrapping_add(0x29e)
                                .wrapping_add((i15 as u32).wrapping_mul(2)),
                        ) == 0
                        {
                            let u8v =
                                rd::<u8>(o.wrapping_add(0x293).wrapping_add(i15 as u32)) as u16;
                            if u8v < u10 {
                                local_12 = i15 as u16;
                                u10 = u8v;
                            }
                        }
                        i15 = i15.wrapping_add(1);
                        if !(i15 < 10) {
                            break;
                        }
                    }
                    let mut u17: u32 = 0;
                    let mut p13: *mut u16 = &raw mut DAT_00523b70;
                    let mut p9: *mut i16 = o.wrapping_add(0x2b4) as usize as *mut i16;
                    loop {
                        if *p9.offset(-0xb) == 0 && (local_12 as u32) == u17 {
                            if *p9 == 0 {
                                *p9.offset(-0xb) = 100;
                                let mut t_us: u16 = rd(o.wrapping_add(0x1df));
                                t_us |= *p13;
                                wr::<u16>(o.wrapping_add(0x1df), t_us);
                                _DAT_00732952 = 0x58;
                                _DAT_00732950 = *(&raw const DAT_00523b88).add(u17 as usize) as u16;
                                FUN_004a1d70(0x56, local_c[0]);
                            } else {
                                *p9 = (*p9).wrapping_add(-1);
                            }
                        }
                        p13 = p13.add(1);
                        p9 = p9.add(1);
                        u17 = u17.wrapping_add(1);
                        if !((p13 as usize) < (&raw const DAT_00523b84 as usize)) {
                            break;
                        }
                    }
                }
            }
            pb = pb.byte_add(0x5bd);
            local_c[0] = local_c[0].wrapping_add(1);
            if !((pb as usize) < (&raw const DAT_0071e518 as usize)) {
                break;
            }
        }

        let mut i5: i32 = 0;
        if 0 < DAT_00732a38 {
            let mut i15: u32 = 0;
            loop {
                let base = DAT_006d4988 as u32;
                if rd::<u8>(base.wrapping_add(i15).wrapping_add(3)) != 0 {
                    let i12: i32 = rd(base.wrapping_add(i15).wrapping_add(0x1f));
                    let mut t_s: i16 = rd((i12 as u32).wrapping_add(0x81));
                    t_s = t_s.wrapping_add(1);
                    wr::<i16>((i12 as u32).wrapping_add(0x81), t_s);
                }
                i15 = i15.wrapping_add(0x23);
                i5 = i5.wrapping_add(1);
                if !(i5 < DAT_00732a38) {
                    break;
                }
            }
        }
        FUN_004a2e30();
    }
}
