// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_006d49a0: i16;
    static mut _DAT_00732a3c: i32;
    static mut DAT_006d4a6a: u8;
    static mut DAT_006d4a70: u8;
    static mut DAT_006d4a74: u8;
    static mut DAT_006d4a76: u8;
    static mut DAT_006d4a78: u8;
    static mut DAT_006d4a80: u8;
    static mut DAT_006d4a84: u8;
    static mut DAT_006d4a88: u8;
    static mut DAT_006d4a8c: u8;
    static mut DAT_006d4a8e: u8;
    static mut DAT_006d4a90: u8;
    static mut DAT_006d4a92: u8;
    static mut DAT_006d4a94: u8;
    static mut DAT_006d4a96: u8;
    static mut DAT_006d4a9a: u8;
    static mut DAT_006d4aa2: u8;
    static mut DAT_006d4aa7: u8;
    static mut DAT_006d4aa8: u8;
    static mut DAT_006d4aa9: u8;
    static mut DAT_006d4aad: u8;
    static mut DAT_006d4aaf: u8;
    static mut DAT_006d4ab1: u8;
    static mut DAT_006d4ab7: u8;
    static mut DAT_006d4ab8: u8;
    static mut DAT_006d4aba: u8;
    static mut DAT_006d4ad2: u8;
    static mut DAT_006d4adc: u8;
    static mut DAT_006d4afa: u8;
    static mut DAT_006d4b04: u8;
    static mut DAT_0070ad40: u8;
    static mut DAT_0070ad48: u8;
    static mut DAT_0070ad8a: u8;
    static mut DAT_0070ad8b: u8;
    static mut DAT_0070ad8c: u8;
    static mut DAT_0070ad8d: u8;
    static mut DAT_0070ad8e: u8;
    static mut DAT_0070ad8f: u8;
    static mut DAT_0070ad90: u8;
    static mut DAT_0070ad91: u8;
    static mut DAT_0070ad92: u8;
    static mut DAT_00709d0a: u8;
    static mut DAT_00703ca0: u8;
    static mut DAT_006da3e0: u8;
    static mut DAT_00524798: u8;
    static mut s_dropoffldr1pln_00524fdc: i8;
    static mut DAT_0071b740: i8;
    static mut DAT_0071e528: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a00e0(param_1: u32);
}

macro_rules! adr {
    ($x:ident) => {
        ((&raw mut $x) as usize as u32)
    };
}

#[inline(always)]
unsafe fn r8(a: u32, o: u32) -> u8 {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a.wrapping_add(o) as usize as *const u8).read() }
}
#[inline(always)]
unsafe fn r16(a: u32, o: u32) -> u16 {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a.wrapping_add(o) as usize as *const u16).read_unaligned() }
}
#[inline(always)]
unsafe fn r32(a: u32, o: u32) -> u32 {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a.wrapping_add(o) as usize as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn w8(a: u32, o: u32, v: u8) {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a.wrapping_add(o) as usize as *mut u8).write(v) }
}
#[inline(always)]
unsafe fn w16(a: u32, o: u32, v: u16) {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a.wrapping_add(o) as usize as *mut u16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049f9c0(param_1: u16, param_2: u16, param_3: u16) {
    const S: u32 = 0x126;
    // SAFETY: all globals and pointers are the game's own memory, accessed
    // exactly where the original does.
    unsafe {
        let base = DAT_006d4988 as u32;
        let p1 = param_1 as u32;
        let p3 = param_3 as u32;
        let i_tmp = r32(base.wrapping_add(0x1f), p1.wrapping_mul(0x23));
        let i6 = r32(i_tmp, 0xa7);
        if r8(i6, 8) == 1 {
            return;
        }
        let u3 = param_2 as u32;
        let u3s = u3.wrapping_mul(S);
        w8(i6, 8, 1);
        let a = adr!(DAT_006d4a6a);
        let o = p3.wrapping_mul(2).wrapping_add(u3s);
        w16(a, o, r16(a, o).wrapping_add(1));
        let i8_ = u3.wrapping_mul(0x566);
        let d40 = adr!(DAT_0070ad40);
        let c1 = r8(d40, i8_);
        if r8(i6, 0xb0) == c1 {
            w8(adr!(DAT_006d4aa2), u3s.wrapping_add(p3), 1);
        }
        if param_3 == 2 {
            if r8(i6, 0x1e5) != 0 {
                let a = adr!(DAT_006d4a76);
                w16(a, u3s, r16(a, u3s).wrapping_sub(1));
                if r8(i6, 0xb0) == c1 {
                    w8(adr!(DAT_006d4aa8), u3s, 0);
                }
                w8(i6, 0x1e5, 0);
            }
            if r8(i6, 0x74) != 0 {
                let a = adr!(DAT_006d4a96);
                w16(a, u3s, r16(a, u3s).wrapping_sub(1));
                if r8(d40, i8_) == r8(i6, 0xb0) {
                    w8(adr!(DAT_006d4ab8), u3s, 0);
                }
            }
            if r8(i6, 0x73) != 0 {
                let a = adr!(DAT_006d4a94);
                w16(a, u3s, r16(a, u3s).wrapping_sub(1));
                if r8(d40, i8_) == r8(i6, 0xb0) {
                    w8(adr!(DAT_006d4ab7), u3s, 0);
                }
            }
            let a = adr!(DAT_006d4a9a);
            w16(a, u3s, r16(a, u3s).wrapping_add(1));
            if r8(d40, i8_) == r8(i6, 0xb0) {
                w8(adr!(DAT_006d4aba), u3s, 1);
            }
        } else {
            let i4 = (DAT_006d4988 as u32).wrapping_add(p1.wrapping_mul(0x23));
            let t = r32(i4, 0x1b);
            let i5 = r32(i4, 0x1f);
            if t != 0xffff_ffff
                && r8(i6, 0x74) == 0
                && r8(i6, 0x73) == 0
                && r8(adr!(DAT_00709d0a), (r8(i5, 0x87) as u32).wrapping_mul(3)) != 1
            {
                let a = adr!(DAT_006d4a96);
                w16(a, u3s, r16(a, u3s).wrapping_add(1));
                if r8(i6, 0xb0) == c1 {
                    w8(adr!(DAT_006d4ab8), u3s, 1);
                }
            }
        }
        let mut u9: u32 = 0;
        loop {
            if r8(i6, 0x1f0u32.wrapping_add(u9)) == 0 {
                let a = adr!(DAT_006d4ad2);
                let o = u3s.wrapping_add(u9);
                w8(a, o, r8(a, o).wrapping_add(1));
                if r8(d40, i8_) == r8(i6, 0xb0) {
                    w8(adr!(DAT_006d4adc), o, 1);
                }
            }
            u9 = (u9 as u16).wrapping_add(1) as u32;
            if !((u9 as u16) < 10) {
                break;
            }
        }
        if r8(i6, 0x1e4) == 0 {
            let a = adr!(DAT_006d4a88);
            w16(a, u3s, r16(a, u3s).wrapping_add(1));
            if r8(d40, i8_) == r8(i6, 0xb0) {
                w8(adr!(DAT_006d4ab1), u3s, 1);
            }
        }
        if r8(i6, 0x1e5) == 0 {
            let c = r8(d40, i8_);
            let a = adr!(DAT_006d4a78);
            w16(a, u3s, r16(a, u3s).wrapping_add(1));
            let i4 = u3s;
            if c == r8(i6, 0xb0) {
                w8(adr!(DAT_006d4aa9), i4, 1);
            }
            let mut u9: u32 = 0;
            let b2 = r8(adr!(DAT_0070ad48), i8_) as u32;
            loop {
                if b2 != u9 {
                    let a = adr!(DAT_006d4afa);
                    let o = i4.wrapping_add(u9);
                    w8(a, o, r8(a, o).wrapping_add(1));
                    w8(adr!(DAT_006d4b04), o, 1);
                }
                u9 = (u9 as u16).wrapping_add(1) as u32;
                if !((u9 as u16) < 10) {
                    break;
                }
            }
        }
        let mut b13 = false;
        let mut u11: u16 = 0;
        loop {
            if r8(i6, (u11 as u32).wrapping_add(0x1e6)) == 1 {
                b13 = true;
            }
            u11 = u11.wrapping_add(1);
            if !(u11 < 10) {
                break;
            }
        }
        if !b13 {
            let a = adr!(DAT_006d4a74);
            w16(a, u3s, r16(a, u3s).wrapping_add(1));
            if r8(d40, i8_) == r8(i6, 0xb0) {
                w8(adr!(DAT_006d4aa7), u3s, 1);
            }
        }
        if r8(i6, 0x7b) == 0 {
            let a = adr!(DAT_006d4a80);
            w16(a, u3s, r16(a, u3s).wrapping_add(1));
            if r8(d40, i8_) == r8(i6, 0xb0) {
                w8(adr!(DAT_006d4aad), u3s, 1);
            }
        }
        if r8(i6, 0x7d) == 0 {
            let c = r8(d40, i8_);
            let a = adr!(DAT_006d4a84);
            w16(a, u3s, r16(a, u3s).wrapping_add(1));
            if c == r8(i6, 0xb0) {
                w8(adr!(DAT_006d4aaf), u3s, 1);
            }
        }
        if param_3 == 2 {
            let mut u11: u16 = 0;
            if 0 < DAT_006d49a0 {
                loop {
                    if u11 != param_2 {
                        let u9 = u11 as u32;
                        let i6b = u9.wrapping_mul(0x566);
                        let us = u9.wrapping_mul(S);
                        if r8(adr!(DAT_0070ad8b), i6b) != 0
                            && (r8(adr!(DAT_0070ad8a), i6b) as u32) == u3
                        {
                            FUN_004a00e0(u9);
                        }
                        if r8(adr!(DAT_0070ad8d), i6b) != 0
                            && (r8(adr!(DAT_0070ad8c), i6b) as u32) == u3
                        {
                            let a = adr!(DAT_006d4a70);
                            let b = adr!(DAT_006d4a8e);
                            w16(a, us, r16(a, us).wrapping_add(r16(b, us)));
                            w16(b, us, 0);
                        }
                        if r8(adr!(DAT_0070ad8f), i6b) != 0
                            && (r8(adr!(DAT_0070ad8e), i6b) as u32) == u3
                        {
                            let a = adr!(DAT_006d4a70);
                            let b = adr!(DAT_006d4a90);
                            w16(a, us, r16(a, us).wrapping_add(r16(b, us)));
                            w16(b, us, 0);
                        }
                        if r8(adr!(DAT_0070ad91), i6b) != 0
                            && (r8(adr!(DAT_0070ad90), i6b) as u32) == u3
                        {
                            let a = adr!(DAT_006d4a70);
                            let b = adr!(DAT_006d4a92);
                            w16(a, us, r16(a, us).wrapping_add(r16(b, us)));
                            w16(b, us, 0);
                        }
                    }
                    u11 = u11.wrapping_add(1);
                    let n = DAT_006d49a0 as i32;
                    let eq = (n as u32) == (u11 as u32);
                    if !(!eq && (u11 as i32) <= n) {
                        break;
                    }
                }
            }
            let mut local_4: i32 = 4;
            let mut pb = adr!(DAT_0070ad92).wrapping_add(i8_);
            loop {
                let mut c: i32 = 0xf;
                let idx1 = r8(adr!(DAT_00524798), r8(pb, 0) as u32) as u32;
                let idx2 = r8(adr!(DAT_006da3e0), idx1) as u32;
                let mut p7 = adr!(DAT_00703ca0).wrapping_add(idx2.wrapping_mul(0x55));
                let mut p12 = adr!(s_dropoffldr1pln_00524fdc);
                let mut eq;
                loop {
                    if c == 0 {
                        eq = true;
                        break;
                    }
                    c -= 1;
                    eq = r8(p7, 0) == r8(p12, 0);
                    p7 = p7.wrapping_add(1);
                    p12 = p12.wrapping_add(1);
                    if !eq {
                        break;
                    }
                }
                if eq {
                    let v = (r8(pb, 3) as i32).wrapping_sub(1) as u16 as u32;
                    FUN_004a00e0(v);
                }
                pb = pb.wrapping_add(0x52);
                local_4 -= 1;
                if local_4 == 0 {
                    break;
                }
            }
        }
        if param_3 == 0x11 {
            let mut u11: u16 = 0;
            if 0 < DAT_006d49a0 {
                let n = DAT_006d49a0 as i32;
                loop {
                    if u11 != param_2 {
                        let u9 = u11 as u32;
                        let i8b = u9.wrapping_mul(0x566);
                        let us = u9.wrapping_mul(S);
                        if r8(adr!(DAT_0070ad8d), i8b) != 0
                            && (r8(adr!(DAT_0070ad8c), i8b) as u32) == u3
                        {
                            let a = adr!(DAT_006d4a8c);
                            let b = adr!(DAT_006d4a8e);
                            w16(a, us, r16(a, us).wrapping_add(r16(b, us)));
                            w16(b, us, 0);
                        }
                        if r8(adr!(DAT_0070ad8f), i8b) != 0
                            && (r8(adr!(DAT_0070ad8e), i8b) as u32) == u3
                        {
                            let a = adr!(DAT_006d4a8c);
                            let b = adr!(DAT_006d4a90);
                            w16(a, us, r16(a, us).wrapping_add(r16(b, us)));
                            w16(b, us, 0);
                        }
                        if r8(adr!(DAT_0070ad91), i8b) != 0
                            && (r8(adr!(DAT_0070ad90), i8b) as u32) == u3
                        {
                            let a = adr!(DAT_006d4a8c);
                            let b = adr!(DAT_006d4a92);
                            w16(a, us, r16(a, us).wrapping_add(r16(b, us)));
                            w16(b, us, 0);
                        }
                    }
                    u11 = u11.wrapping_add(1);
                    if !((u11 as i32) < n) {
                        break;
                    }
                }
            }
        }
        if _DAT_00732a3c < DAT_006d4980 {
            let mut i6c = (_DAT_00732a3c as u32).wrapping_mul(0x23);
            let mut i8c = _DAT_00732a3c;
            loop {
                let b = DAT_006d4988 as u32;
                if r8(b.wrapping_add(i6c), 3) != 0 {
                    let t = r32(b.wrapping_add(i6c), 0x1f);
                    let i4 = r32(t, 0xa7);
                    if r16(i4, 0x6a) == param_1 {
                        w16(i4, 0x6a, 0xffff);
                    }
                }
                i6c = i6c.wrapping_add(0x23);
                i8c = i8c.wrapping_add(1);
                if !(i8c < DAT_006d4980) {
                    break;
                }
            }
        }
        let mut p7 = adr!(DAT_0071b740);
        loop {
            if r8(p7, 0) != 0 {
                let mut pp = p7.wrapping_add(0xe);
                let mut c = 4;
                loop {
                    if r16(pp, 0) == param_1 {
                        w16(pp, 0, 0xffff);
                    }
                    pp = pp.wrapping_add(2);
                    c -= 1;
                    if c == 0 {
                        break;
                    }
                }
            }
            p7 = p7.wrapping_add(0x5bd);
            if !(p7 < adr!(DAT_0071e528)) {
                break;
            }
        }
    }
}
