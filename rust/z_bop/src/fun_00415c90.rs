// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ec5e0: [i16; 0];
    static mut DAT_009ec5f6: u8;
    static mut DAT_009ec5fe: u16;
    static mut DAT_00523410: i32;
    static mut DAT_009d8b80: [i16; 0];
    static mut DAT_009d8b90: u8;
    static mut DAT_009e9680: i8;
    static mut DAT_009ec468: u8;
    static mut DAT_009e9c1d: u8;
    static mut DAT_009eca05: u8;
    static mut _DAT_00a07cec: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08104: i32;
    static mut DAT_009d1144: i32;
    static mut DAT_009fe7dc: i32;
    static mut _DAT_009a8064: i16;
    static mut _DAT_009d77bc: u16;
    static mut DAT_009ecc34: i32;
    static mut DAT_009e968a: u8;
    static mut DAT_009e9670: u8;
    static mut DAT_009e9699: u8;
    static mut DAT_009a8c00: i32;
    static mut DAT_00521f1c: u8;
    static mut DAT_009d682e: u16;
    static mut DAT_009cc450: u16;
    static mut _DAT_00a0810a: u16;
    static mut _DAT_009ed22e: i16;
    static mut DAT_009ed22d: u8;
    static mut DAT_009ed22c: u8;
    static mut DAT_009ed22b: u8;
    static mut DAT_009d8b74: u8;
    static mut DAT_009d8b75: u8;
    static mut DAT_009d695c: u8;
    static mut DAT_009d6940: u8;
    static mut DAT_009d6964: i32;
    static mut DAT_009d695d: u8;
    static mut DAT_009d695e: u8;
    static mut DAT_009a2062: u8;
    static mut DAT_009d8c94: [u8; 0];
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009d738a: [u8; 0];
    static mut DAT_009d738b: [u8; 0];
    static mut DAT_009d738d: u8;
    static mut DAT_0051bf60: [u16; 0];
    static mut DAT_0051bf74: u8;
    static mut DAT_0051bf78: [u8; 0];
    static mut DAT_009ec458: u8;
    static mut _DAT_00a07be0: u16;
    static mut _DAT_00a07be2: u16;
    static mut DAT_00a07ce8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004034e0(param_1: u32, param_2: u32);
    fn FUN_0042e4d0(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_00450650(param_1: i32, param_2: i32);
    fn FUN_00451590();
}

#[inline(always)]
unsafe fn rd<T>(a: u32) -> T {
    // SAFETY: the address is one the original reads at this point.
    unsafe { (a as usize as *const T).read_unaligned() }
}

#[inline(always)]
unsafe fn wr<T>(a: u32, v: T) {
    // SAFETY: the address is one the original writes at this point.
    unsafe { (a as usize as *mut T).write_unaligned(v) }
}

#[inline(always)]
fn ad<T>(p: *mut T) -> u32 {
    p as usize as u32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00415c90() {
    // SAFETY: all globals, pointers and callees are the game's own memory,
    // accessed at the addresses and in the order the original does.
    unsafe {
        let mut local_c: [i32; 3] = [0; 3];
        let lcp = local_c.as_mut_ptr();
        let lcb = lcp.cast::<u8>();

        let mut p = ad(&raw mut DAT_009ec5e0);
        loop {
            let v: i16 = rd(p);
            if v != 0 {
                let s = (v as i32).wrapping_sub(DAT_009ec5fe as i32) as i16;
                wr(p, s);
                if s < 0 {
                    wr(p, 0i16);
                }
            }
            p = p.wrapping_add(2);
            if !(p < ad(&raw mut DAT_009ec5f6)) {
                break;
            }
        }

        if DAT_00523410 == 0 {
            let mut p10 = ad(&raw mut DAT_009d8b80);
            loop {
                let mut p5 = ad(&raw mut DAT_009e9680);
                let mut p11 = p10;
                loop {
                    if rd::<i8>(p5) != 0 && rd::<i16>(p11) != 0 {
                        let s = (rd::<i16>(p11) as i32).wrapping_sub(DAT_009ec5fe as i32) as i16;
                        wr(p11, s);
                        if s < 0 {
                            wr(p11, 0i16);
                        }
                    }
                    p11 = p11.wrapping_add(16);
                    p5 = p5.wrapping_add(0x5bd);
                    if !(p5 < ad(&raw mut DAT_009ec468)) {
                        break;
                    }
                }
                p10 = p10.wrapping_add(2);
                if !(p10 < ad(&raw mut DAT_009d8b90)) {
                    break;
                }
            }
        }

        let mut p13 = ad(&raw mut DAT_009e9c1d);
        loop {
            let ti: i32 = rd(p13);
            if ti != 0 {
                let v = (ti as u32).wrapping_sub(DAT_009ec5fe as u32) as i32;
                wr(p13, v);
                if v < 0 {
                    wr(p13, 0i32);
                }
            }
            let ti: i32 = rd(p13.wrapping_add(4));
            if ti != 0 {
                let v = (ti as u32).wrapping_sub(DAT_009ec5fe as u32) as i32;
                wr(p13.wrapping_add(4), v);
                if v < 0 {
                    wr(p13.wrapping_add(4), 0i32);
                }
            }
            p13 = p13.wrapping_add(0x5bd);
            if !(p13 < ad(&raw mut DAT_009eca05)) {
                break;
            }
        }

        if _DAT_00a07cec < DAT_009a1fe0 {
            let mut iv6 = (_DAT_00a07cec as u32).wrapping_mul(0x23);
            let mut iv14 = _DAT_00a07cec;
            loop {
                let g = DAT_009a1fe8 as u32;
                if rd::<i8>(g.wrapping_add(iv6).wrapping_add(3)) != 0 {
                    let ti: i32 = rd(g.wrapping_add(iv6).wrapping_add(0x1f));
                    let iv16: i32 = rd((ti as u32).wrapping_add(0xa9));
                    DAT_00a08104 = iv16;
                    let rp = iv16 as u32;
                    if rd::<i32>(rp.wrapping_add(0x3c)) != 0 {
                        wr(
                            rp.wrapping_add(0x3c),
                            rd::<u32>(rp.wrapping_add(0x3c)).wrapping_sub(DAT_009ec5fe as u32),
                        );
                    }
                    if rd::<i32>(rp.wrapping_add(0x60)) != 0 {
                        let iv7 = rd::<u32>(rp.wrapping_add(0x60)).wrapping_sub(DAT_009ec5fe as u32)
                            as i32;
                        wr(rp.wrapping_add(0x60), iv7);
                        if iv7 < 0 {
                            wr(rp.wrapping_add(0x60), 0i32);
                        }
                    }
                    if rd::<i16>(rp.wrapping_add(100)) != 0 {
                        let s = (rd::<i16>(rp.wrapping_add(100)) as i32)
                            .wrapping_sub(DAT_009ec5fe as i32)
                            as i16;
                        wr(rp.wrapping_add(100), s);
                        if s < 0 {
                            wr(rp.wrapping_add(100), 0i16);
                        }
                    }
                    let u12: u16 = rd((DAT_00a08104 as u32).wrapping_add(0x1e7));
                    if u12 != 0 {
                        let th = (u12 as i32).wrapping_sub(DAT_009ec5fe as i32) as u16;
                        wr((DAT_00a08104 as u32).wrapping_add(0x1e7), th);
                        if u12 < th {
                            wr((DAT_00a08104 as u32).wrapping_add(0x1e7), 0u16);
                        }
                    }
                    let a24a = (DAT_00a08104 as u32).wrapping_add(0x24a);
                    if rd::<i16>(a24a) != 0 {
                        wr(
                            a24a,
                            (rd::<i16>(a24a) as i32).wrapping_sub(DAT_009ec5fe as i32) as i16,
                        );
                        if rd::<i16>(a24a) < 0 {
                            wr(a24a, 0i16);
                        }
                    }
                    let mut iv16: i32 = 0;
                    if rd::<u8>((DAT_00a08104 as u32).wrapping_add(0x21d)) != 0 {
                        let mut iv7: u32 = 0;
                        loop {
                            let iv1 = (DAT_00a08104 as u32).wrapping_add(0x3b2).wrapping_add(iv7);
                            let s4: i16 = rd(iv1.wrapping_add(2));
                            if 0 < s4 {
                                wr(
                                    iv1.wrapping_add(2),
                                    (s4 as i32).wrapping_sub(DAT_009ec5fe as i32) as u16,
                                );
                            }
                            iv7 = iv7.wrapping_add(4);
                            iv16 = iv16.wrapping_add(1);
                            if !(iv16 < rd::<u8>((DAT_00a08104 as u32).wrapping_add(0x21d)) as i32)
                            {
                                break;
                            }
                        }
                    }
                }
                iv6 = iv6.wrapping_add(0x23);
                iv14 = iv14.wrapping_add(1);
                if !(iv14 < DAT_009a1fe0) {
                    break;
                }
            }
        }

        if DAT_009d1144 < DAT_009fe7dc {
            let mut iv6 = (DAT_009d1144 as u32).wrapping_mul(0x23);
            let mut iv14 = DAT_009d1144;
            loop {
                let g = DAT_009a1fe8 as u32;
                if rd::<i8>(g.wrapping_add(iv6).wrapping_add(3)) != 0 {
                    let ti: i32 = rd(g.wrapping_add(iv6).wrapping_add(0x1f));
                    let iv16: i32 = rd((ti as u32).wrapping_add(0xad));
                    let rp = iv16 as u32;
                    let ti2: i32 = rd(rp.wrapping_add(0x16));
                    if ti2 != 0 {
                        let tu = (ti2 as u32).wrapping_sub(DAT_009ec5fe as u32);
                        wr(rp.wrapping_add(0x16), tu);
                    }
                    let ti2: i32 = rd(rp.wrapping_add(0x3a));
                    if ti2 != 0 {
                        let iv7 = (ti2 as u32).wrapping_sub(DAT_009ec5fe as u32) as i32;
                        wr(rp.wrapping_add(0x3a), iv7);
                        if iv7 < 0 {
                            wr(rp.wrapping_add(0x3a), 0i32);
                        }
                    }
                    if rd::<i16>(rp.wrapping_add(0x3e)) != 0 {
                        let s = (rd::<i16>(rp.wrapping_add(0x3e)) as i32)
                            .wrapping_sub(DAT_009ec5fe as i32)
                            as i16;
                        wr(rp.wrapping_add(0x3e), s);
                        if s < 0 {
                            wr(rp.wrapping_add(0x3e), 0i16);
                        }
                    }
                }
                iv6 = iv6.wrapping_add(0x23);
                iv14 = iv14.wrapping_add(1);
                if !(iv14 < DAT_009fe7dc) {
                    break;
                }
            }
        }

        _DAT_009a8064 = (_DAT_009a8064 as i32).wrapping_sub(DAT_009ec5fe as i32) as i16;
        if _DAT_009a8064 < 0 {
            _DAT_009d77bc ^= 0x400;
            let u12: u16 =
                rd(ad(&raw mut DAT_009e968a)
                    .wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd)));
            if u12 != 0xffff {
                let u18: u32 = rd(ad(&raw mut DAT_009e9670)
                    .wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd)));
                if u18 != 0xffff_ffff {
                    FUN_004034e0(u12 as u32, u18);
                    DAT_009a8c00 >>= 5;
                    let th: u16 = rd(ad(&raw mut DAT_009e968a)
                        .wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd)));
                    let b: u8 = rd((th as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(3)
                        .wrapping_add(DAT_009a1fe8 as u32));
                    lcp.write(rd::<i32>(
                        ad(&raw mut DAT_00521f1c).wrapping_add((b as u32).wrapping_mul(0x1a)),
                    ));
                }
            }
            if (_DAT_009d77bc & 0x400) == 0 {
                _DAT_009a8064 = 0xe;
                if lcp.read() <= DAT_009a8c00 {
                    _DAT_009a8064 = 0x76;
                }
            } else if DAT_009a8c00 < lcp.read() {
                _DAT_009a8064 = 0x76;
            } else {
                _DAT_009a8064 = 0xe;
            }
        }

        DAT_009d682e =
            rd(ad(&raw mut DAT_009e9699).wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd)));
        let th: u16 =
            rd(ad(&raw mut DAT_009e968a).wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd)));
        DAT_009cc450 = th | _DAT_00a0810a | _DAT_009d77bc;

        _DAT_009ed22e = (_DAT_009ed22e as i32).wrapping_sub(DAT_009ec5fe as i32) as i16;
        if 0 < _DAT_009ed22e {
            return;
        }
        _DAT_009ed22e = (_DAT_009ed22e as i32).wrapping_add(0xec) as i16;
        DAT_009ed22d = DAT_009ed22d.wrapping_add(1);
        if 0x3b < DAT_009ed22d {
            DAT_009ed22d = 0;
            DAT_009ed22c = DAT_009ed22c.wrapping_add(1);
            if 0x3b < DAT_009ed22c {
                DAT_009ed22c = 0;
                DAT_009ed22b = DAT_009ed22b.wrapping_add(1);
                if 0x17 < DAT_009ed22b {
                    DAT_009ed22b = 0;
                }
            }
        }

        if !(DAT_009d8b74 == 0 && DAT_009d8b75 == 0) {
            DAT_009d8b75 = DAT_009d8b75.wrapping_sub(1);
            if DAT_009d8b75 == 0xff {
                DAT_009d8b75 = 0x3b;
                DAT_009d8b74 = DAT_009d8b74.wrapping_sub(1);
                if DAT_009d8b74 == 0xff {
                    DAT_009d8b75 = 0;
                    DAT_009d8b74 = 0;
                }
            }
            if DAT_009d695c != 0 {
                if DAT_009d8b74 == 0 && DAT_009d8b75 == 0 && DAT_00523410 == 0 {
                    let mut p5 = ad(&raw mut DAT_009e9680);
                    loop {
                        wr(p5, 2i8);
                        p5 = p5.wrapping_add(0x5bd);
                        if !(p5 < ad(&raw mut DAT_009ec468)) {
                            break;
                        }
                    }
                    DAT_009d6940 = 1;
                }
            }
        }

        if DAT_009d695c != 0 {
            if DAT_009d8b74 == 2 && DAT_009d8b75 == 0 {
                FUN_0042e4d0(0x23, -1, DAT_009ecc34);
                FUN_00450650(0xc9, DAT_009ecc34);
            }
            if DAT_009d8b74 == 1 && DAT_009d8b75 == 0 {
                FUN_0042e4d0(0x5c, -1, DAT_009ecc34);
                FUN_00450650(0xca, DAT_009ecc34);
            }
            if DAT_009d8b74 == 0 {
                if DAT_009d8b75 == 0x0f {
                    let iv6: i32 = rd(ad(&raw mut DAT_009e9670)
                        .wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd)));
                    let b3: bool = if iv6 == -1 {
                        false
                    } else {
                        let c2: i8 = rd((iv6 as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(3)
                            .wrapping_add(DAT_009a1fe8 as u32));
                        c2 == 1 || c2 == 2 || c2 == 3 || c2 == 0xe || c2 == 4
                    };
                    let u18: u32;
                    if b3 {
                        u18 = 0x57;
                    } else {
                        FUN_0042e4d0(0x5c, -1, DAT_009ecc34);
                        u18 = 0x5c;
                    }
                    FUN_0042e4d0(u18 as i32, -1, DAT_009ecc34);
                }
                if DAT_009d8b74 == 0 && DAT_009d8b75 == 2 {
                    FUN_00450650(0xcb, DAT_009ecc34);
                }
            }
        }

        'a: {
            if DAT_009d6964 < 2 || DAT_009d695d == 0 || DAT_009d695e != 0 {
                break 'a;
            }
            if (DAT_009a2062 == 2 || DAT_009a2062 == 4)
                && (DAT_009d695d < DAT_009d8b74 || (DAT_009d8b74 == 0 && DAT_009d8b75 == 0))
            {
                lcp.write(0);
                lcp.add(1).write(0);
                lcb.add(8).cast::<i16>().write_unaligned(0);
                let mut p5 = ad(&raw mut DAT_009e9680);
                loop {
                    if rd::<i8>(p5) == 1 {
                        let th: u16 = rd(p5.wrapping_sub(4));
                        lcb.add(th as usize).write(1);
                    }
                    p5 = p5.wrapping_add(0x5bd);
                    if !(p5 < ad(&raw mut DAT_009ec468)) {
                        break;
                    }
                }
                if DAT_009a2062 == 2 && _DAT_00a07cec < DAT_009a1fe0 {
                    let mut iv6 = (DAT_009a1fe0 as u32).wrapping_sub(_DAT_00a07cec as u32);
                    let mut pb17 = (_DAT_00a07cec as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(0x16)
                        .wrapping_add(DAT_009a1fe8 as u32);
                    loop {
                        let k = rd::<u8>(pb17) as u32;
                        if rd::<u8>(pb17.wrapping_sub(0x13)) != 0
                            && rd::<u8>(
                                ad(&raw mut DAT_009d8c94).wrapping_add(k.wrapping_mul(0x566)),
                            ) != 0
                        {
                            let idx = rd::<u8>(
                                ad(&raw mut DAT_009d8c88).wrapping_add(k.wrapping_mul(0x566)),
                            );
                            lcb.add(idx as usize).write(1);
                        }
                        pb17 = pb17.wrapping_add(0x23);
                        iv6 = iv6.wrapping_sub(1);
                        if iv6 == 0 {
                            break;
                        }
                    }
                }
                let mut iv14: i32 = 0;
                let mut iv6: i32 = 0;
                loop {
                    if lcb.add(iv6 as usize).read() != 0 {
                        iv14 = iv14.wrapping_add(1);
                        lcp.write(iv6);
                    }
                    iv6 = iv6.wrapping_add(1);
                    if !(iv6 < 10) {
                        break;
                    }
                }
                if iv14 == 1 {
                    let mut go = true;
                    if DAT_009a2062 != 2 {
                        let o = (lcp.read() as u32).wrapping_mul(3);
                        if rd::<u8>(ad(&raw mut DAT_009d738a).wrapping_add(o)) != 1
                            && rd::<u8>(ad(&raw mut DAT_009d738a).wrapping_add(o)) != 2
                            && rd::<u8>(ad(&raw mut DAT_009d738b).wrapping_add(o)) != 1
                        {
                            go = false;
                        }
                    }
                    if go {
                        DAT_009d8b75 = 0;
                        DAT_009d695e = 1;
                        DAT_009d8b74 = DAT_009d695d;
                        DAT_009d695c = DAT_009d695d;
                    }
                }
            }
            if DAT_009d695e == 0
                && DAT_009a2062 == 4
                && (rd::<u8>(ad(&raw mut DAT_009d738a)) == 1 || DAT_009d738d == 1)
            {
                DAT_009d8b75 = 0;
                DAT_009d695e = 1;
                DAT_009d8b74 = DAT_009d695d;
                DAT_009d695c = DAT_009d695d;
            }
        }

        lcp.write(0);
        let mut p13 = ad(&raw mut DAT_009e9670);
        loop {
            let ti: i32 = rd(p13);
            if rd::<u8>(p13.wrapping_add(0x10)) != 0 && ti != -1 {
                let mut u12: u16 = 0xffff;
                let mut local_12: u16 = 0xffff;
                let ti2: i32 = rd((ti as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_009a1fe8 as u32));
                let iv6: i32 = rd((ti2 as u32).wrapping_add(0xa9));
                let rp = iv6 as u32;
                let th: u16 = rd(rp.wrapping_add(0x1e5));
                if th != 0 {
                    let mut iv14: i32 = 0;
                    loop {
                        let o = iv14 as u32;
                        if rd::<i16>(rp.wrapping_add(0x2ac).wrapping_add(o.wrapping_mul(2))) == 0 {
                            let u9 = rd::<u8>(rp.wrapping_add(0x2a1).wrapping_add(o)) as u16;
                            if u9 < u12 {
                                local_12 = iv14 as u16;
                                u12 = u9;
                            }
                        }
                        iv14 = iv14.wrapping_add(1);
                        if !(iv14 < 10) {
                            break;
                        }
                    }
                    let mut u18: u32 = 0;
                    let mut pu15 = ad(&raw mut DAT_0051bf60);
                    let mut ps10 = rp.wrapping_add(0x2c2);
                    loop {
                        let pm = ps10.wrapping_sub(0x16);
                        if rd::<i16>(pm) == 0 && local_12 as u32 == u18 {
                            if rd::<i16>(ps10) == 0 {
                                wr(pm, 100i16);
                                let mut th: u16 = rd(rp.wrapping_add(0x1e5));
                                th |= rd::<u16>(pu15);
                                wr(rp.wrapping_add(0x1e5), th);
                                _DAT_00a07be2 = 0x58;
                                _DAT_00a07be0 =
                                    rd::<u8>(ad(&raw mut DAT_0051bf78).wrapping_add(u18)) as u16;
                                FUN_00450650(0x56, lcp.read());
                            } else {
                                wr(ps10, (rd::<i16>(ps10) as i32).wrapping_add(-1) as i16);
                            }
                        }
                        pu15 = pu15.wrapping_add(2);
                        ps10 = ps10.wrapping_add(2);
                        u18 = u18.wrapping_add(1);
                        if !(pu15 < ad(&raw mut DAT_0051bf74)) {
                            break;
                        }
                    }
                }
            }
            p13 = p13.wrapping_add(0x5bd);
            lcp.write(lcp.read().wrapping_add(1));
            if !(p13 < ad(&raw mut DAT_009ec458)) {
                break;
            }
        }

        let mut iv6: i32 = 0;
        if 0 < DAT_00a07ce8 {
            let mut iv14: u32 = 0;
            loop {
                let g = DAT_009a1fe8 as u32;
                if rd::<i8>(g.wrapping_add(iv14).wrapping_add(3)) != 0 {
                    let ti: i32 = rd(g.wrapping_add(iv14).wrapping_add(0x1f));
                    let ts: i16 = rd((ti as u32).wrapping_add(0x83));
                    wr(
                        (ti as u32).wrapping_add(0x83),
                        (ts as i32).wrapping_add(1) as i16,
                    );
                }
                iv14 = iv14.wrapping_add(0x23);
                iv6 = iv6.wrapping_add(1);
                if !(iv6 < DAT_00a07ce8) {
                    break;
                }
            }
        }
        FUN_00451590();
    }
}
