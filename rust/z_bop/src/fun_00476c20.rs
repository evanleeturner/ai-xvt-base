// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00999488: i32;
    static mut DAT_0099947e: [u8; 4];
    static mut DAT_0060f204: i32;
    static mut DAT_0060f1c0: i32;
    static mut DAT_0060f1e8: i32;
    static mut _DAT_0060f1f0: i32;
    static mut DAT_0060f1d8: *mut i32;
    static mut _DAT_0060f208: *mut i32;
    static mut DAT_009e8f54: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_005233c4: i32;
    static mut DAT_005233ac: i32;
    static mut DAT_009994bc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004452d0(a: *mut u8, b: i32, c: i32, d: i32) -> i32;
    fn FUN_00476b00(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> u32;
    fn FUN_004079e0();
    fn FUN_00476b90();
}

unsafe fn ld(p: *const u8) -> u32 {
    // SAFETY: caller supplies a readable address.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn st(p: *mut u8, v: u32) {
    // SAFETY: caller supplies a writable address.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

unsafe fn fld(p: *const u8, i: usize) -> i32 {
    // SAFETY: caller supplies a readable address.
    unsafe { ld(p.wrapping_add(i * 4)) as i32 }
}

unsafe fn cpy(d: *mut u8, s: *const u8, dw: u32, by: u32) {
    let mut d = d;
    let mut s = s;
    let mut n = dw;
    // SAFETY: caller supplies readable source and writable destination.
    unsafe {
        while n != 0 {
            st(d, ld(s));
            s = s.wrapping_add(4);
            d = d.wrapping_add(4);
            n = n.wrapping_sub(1);
        }
        let mut m = by;
        while m != 0 {
            *d = *s;
            s = s.wrapping_add(1);
            d = d.wrapping_add(1);
            m = m.wrapping_sub(1);
        }
    }
}

unsafe fn slen(p: *const u8) -> u32 {
    let mut n: u32 = 0;
    // SAFETY: caller supplies a NUL-terminated string.
    unsafe {
        loop {
            if n == 0xffff_ffff {
                break;
            }
            n = n.wrapping_add(1);
            if *p.wrapping_add((n - 1) as usize) == 0 {
                break;
            }
        }
    }
    n
}

unsafe fn pp(p: *const u8, i: usize) -> *mut u8 {
    // SAFETY: caller supplies a readable address.
    unsafe { ld(p.wrapping_add(i * 4)) as usize as *mut u8 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00476c20(
    param_1: *mut i32,
    param_2: *mut u32,
    param_3: *mut u8,
) -> i32 {
    if param_1.is_null() {
        return 0;
    }
    // SAFETY: all pointers are the game's own structures and buffers, accessed
    // exactly as the original does.
    unsafe {
        let p1 = param_1 as *mut u8;
        let p2 = param_2 as *mut u8;
        let mut param_3 = param_3;
        let mut hdr0: *mut u8 = core::ptr::null_mut();
        let mut i108: i32;
        let mut u100: u32 = 0;
        let mut auStack_d0: [u32; 52] = [0; 52];

        if !param_3.is_null() {
            cpy(param_3, p1, 6, 0);
            hdr0 = param_3;
            param_3 = param_3.wrapping_add(0x18);
        }
        i108 = 0x18;
        if fld(p1, 0) != 0 {
            let s = pp(p1, 0);
            if !param_3.is_null() {
                st(hdr0, param_3 as usize as u32);
                let n = slen(s);
                cpy(param_3, s, n >> 2, n & 3);
                param_3 = param_3.wrapping_add(n as usize);
            }
            let n = slen(s);
            i108 = n.wrapping_add(0x18) as i32;
        }
        if fld(p1, 2) != 0 {
            let n = (fld(p1, 2) as u32).wrapping_mul(4);
            if !param_3.is_null() {
                st(hdr0.add(0xc), param_3 as usize as u32);
                param_3 = param_3.wrapping_add(n as usize);
            }
            i108 = (i108 as u32).wrapping_add(n) as i32;
        }
        let p12 = pp(p1, 5);
        match fld(p1, 1) {
            1 | 0xf | 0x10 | 0x11 => {
                if DAT_00999488 < fld(p12, 0) {
                    DAT_00999488 = fld(p12, 0);
                }
                let mut u9 = (fld(p1, 4) as u32).wrapping_mul(100).wrapping_add(4);
                if ld(p2.add(0x21 * 4)) == 0 {
                    u9 = u9.wrapping_add((DAT_0060f204 as u32).wrapping_mul(0xc));
                }
                if !param_3.is_null() {
                    st(hdr0.add(0x14), param_3 as usize as u32);
                    cpy(param_3, p12, u9 >> 2, 0);
                    param_3 = param_3.wrapping_add(u9 as usize);
                }
                i108 = (i108 as u32).wrapping_add(u9) as i32;
            }
            2 => {
                if !param_3.is_null() {
                    st(hdr0.add(0x14), param_3 as usize as u32);
                    cpy(param_3, p12, 0xc, 0);
                    param_3 = param_3.wrapping_add(0x30);
                }
                i108 = i108.wrapping_add(0x30);
            }
            3 => {
                let u9 = (fld(p1, 4) as u32).wrapping_mul(0xc);
                if !param_3.is_null() {
                    st(hdr0.add(0x14), param_3 as usize as u32);
                    cpy(param_3, p12, u9 >> 2, 0);
                    param_3 = param_3.wrapping_add(u9 as usize);
                }
                i108 = (i108 as u32).wrapping_add(u9) as i32;
                DAT_0060f204 = fld(p1, 4);
                let t = (&raw const DAT_0099947e).cast::<u32>().read_unaligned();
                if (t as i32) < fld(p1, 4) {
                    (&raw mut DAT_0099947e)
                        .cast::<u32>()
                        .write_unaligned(fld(p1, 4) as u32);
                }
            }
            4 | 6 | 0x13 => {
                if !param_3.is_null() {
                    st(hdr0.add(0x14), param_3 as usize as u32);
                    st(param_3, ld(p12));
                    st(param_3.add(4), ld(p12.add(4)));
                    st(param_3.add(8), ld(p12.add(8)));
                    param_3 = param_3.wrapping_add(0xc);
                }
                i108 = i108.wrapping_add(0xc);
            }
            5 => {
                if !param_3.is_null() {
                    st(hdr0.add(0x14), param_3 as usize as u32);
                    cpy(param_3, p12, 9, 0);
                    param_3 = param_3.wrapping_add(0x24);
                }
                i108 = i108.wrapping_add(0x24);
            }
            7 => {
                let u9 = slen(p12);
                if !param_3.is_null() {
                    st(hdr0.add(0x14), param_3 as usize as u32);
                    cpy(param_3, p12, u9 >> 2, u9 & 3);
                    param_3 = param_3.wrapping_add(u9 as usize);
                }
                i108 = (i108 as u32).wrapping_add(u9) as i32;
            }
            9 => {
                let u9 = (fld(p1, 4) as u32).wrapping_mul(0x38);
                if !param_3.is_null() {
                    st(hdr0.add(0x14), param_3 as usize as u32);
                    cpy(param_3, p12, u9 >> 2, 0);
                    param_3 = param_3.wrapping_add(u9 as usize);
                }
                i108 = (i108 as u32).wrapping_add(u9) as i32;
                DAT_0060f1d8 = p12 as *mut i32;
            }
            0xb => {
                let u9 = (fld(p1, 4) as u32).wrapping_mul(0xc);
                if !param_3.is_null() {
                    st(hdr0.add(0x14), param_3 as usize as u32);
                    cpy(param_3, p12, u9 >> 2, 0);
                    param_3 = param_3.wrapping_add(u9 as usize);
                }
                i108 = (i108 as u32).wrapping_add(u9) as i32;
                _DAT_0060f208 = p12 as *mut i32;
                st(p2.add(0x21 * 4), p12 as usize as u32);
            }
            0xd => {
                let u9 = fld(p1, 4) as u32;
                let v8 = u9.wrapping_mul(8);
                if !param_3.is_null() {
                    st(hdr0.add(0x14), param_3 as usize as u32);
                    cpy(param_3, p12, (u9 & 0x1fff_ffff) << 1, 0);
                    param_3 = param_3.wrapping_add(v8 as usize);
                }
                i108 = (i108 as u32).wrapping_add(v8) as i32;
            }
            0x14 => {
                if !param_3.is_null() && DAT_009e8f54 != 0 && DAT_005233d8 == 1 {
                    let mut v8 = fld(p12, 5).wrapping_mul(fld(p12, 4));
                    if fld(p12, 2) == v8 {
                        v8 = fld(p12, 3);
                    }
                    let mut v11 = 0x10;
                    let mut b14 = p12
                        .wrapping_add(24)
                        .wrapping_add(v8 as usize)
                        .wrapping_add(0x1000);
                    loop {
                        FUN_004452d0(
                            p12.wrapping_add(24),
                            b14 as usize as i32,
                            fld(p12, 4),
                            fld(p12, 5),
                        );
                        v11 -= 1;
                        b14 = b14.wrapping_add(0x200);
                        if v11 == 0 {
                            break;
                        }
                    }
                }
                let mut v8 = fld(p12, 5);
                let mut v11 = fld(p12, 4);
                let v7 = (v8 as u32).wrapping_mul(v11 as u32) as i32;
                if fld(p12, 2) == v7 {
                    if param_3.is_null() {
                        u100 = fld(p12, 3).wrapping_add(0x18) as u32;
                        if DAT_005233c4 == 0 && 8 < v11 && 8 < v8 {
                            u100 = u100.wrapping_sub(v7 as u32);
                        }
                    } else {
                        st(hdr0.add(0x14), param_3 as usize as u32);
                        cpy(param_3, p12, 6, 0);
                        let mut b14 = p12.wrapping_add(24);
                        let hdr = pp(hdr0, 5);
                        if DAT_005233c4 == 0 && 8 < fld(hdr, 4) && 8 < fld(hdr, 5) {
                            let v11 = (fld(hdr, 5) as u32).wrapping_mul(fld(hdr, 4) as u32) as i32;
                            b14 = b14.wrapping_add(v11 as usize);
                            st(hdr.add(0xc), ld(hdr.add(0xc)).wrapping_sub(v11 as u32));
                            st(hdr.add(0x10), (fld(hdr, 4) >> 1) as u32);
                            let v11b = fld(hdr, 5) >> 1;
                            st(hdr.add(0x14), v11b as u32);
                            st(hdr.add(8), (fld(hdr, 4) as u32).wrapping_mul(v11b as u32));
                        }
                        let u9 = ld(hdr.add(0xc));
                        cpy(param_3.wrapping_add(0x18), b14, u9 >> 2, u9 & 3);
                        param_3 = param_3
                            .wrapping_add(0x18)
                            .wrapping_add(fld(hdr, 3) as usize);
                        u100 = fld(hdr, 3).wrapping_add(0x18) as u32;
                    }
                } else {
                    u100 = (v7 as u32).wrapping_add(0x18);
                    if param_3.is_null() {
                        while 1 < v11 && 1 < v8 {
                            v11 >>= 1;
                            v8 >>= 1;
                            u100 = u100.wrapping_add((v11 as u32).wrapping_mul(v8 as u32));
                        }
                    } else {
                        st(hdr0.add(0x14), param_3 as usize as u32);
                        cpy(param_3, pp(p1, 5), u100 >> 2, u100 & 3);
                        let mut e0 = p12.wrapping_add(24);
                        let mut b14: *mut u8;
                        if fld(p12, 1) == 0 {
                            b14 = pp(p12, 0);
                        } else {
                            b14 = e0.wrapping_add(
                                (fld(p12, 5) as u32).wrapping_mul(fld(p12, 4) as u32) as usize,
                            );
                        }
                        b14 = b14.wrapping_add(0x2000);
                        let mut v8 = fld(p12, 4);
                        let mut v11 = fld(p12, 5);
                        let hd = pp(hdr0, 5);
                        let prod = (v8 as u32).wrapping_mul(v11 as u32);
                        st(hd.add(8), prod);
                        st(hd.add(0xc), prod);
                        let mut b15 = param_3.wrapping_add(u100 as usize);
                        loop {
                            param_3 = b15;
                            if !(1 < v8 && 1 < v11) {
                                break;
                            }
                            v8 >>= 1;
                            v11 >>= 1;
                            let v6 = (v8 as u32).wrapping_mul(v11 as u32) as i32;
                            st(hd.add(0xc), ld(hd.add(0xc)).wrapping_add(v6 as u32));
                            if DAT_005233ac != 0 && 0 < v11 {
                                let mut dc = e0
                                    .wrapping_add((v8 as u32).wrapping_mul(2) as usize)
                                    .wrapping_add(1);
                                let mut e4 = param_3;
                                let mut d4 = v11;
                                loop {
                                    let mut ec: i32 = 0;
                                    if 0 < v8 {
                                        let mut f0 = dc;
                                        let mut p15 = e0;
                                        loop {
                                            let u2 = b14
                                                .wrapping_add((*p15 as usize) * 2)
                                                .cast::<u16>()
                                                .read_unaligned()
                                                as i32;
                                            let u3 = b14
                                                .wrapping_add((*p15.wrapping_add(1) as usize) * 2)
                                                .cast::<u16>()
                                                .read_unaligned()
                                                as i32;
                                            let u4 = b14
                                                .wrapping_add((*f0 as usize) * 2)
                                                .cast::<u16>()
                                                .read_unaligned()
                                                as i32;
                                            let u5 = b14
                                                .wrapping_add((*f0.wrapping_sub(1) as usize) * 2)
                                                .cast::<u16>()
                                                .read_unaligned()
                                                as i32;
                                            let r = FUN_00476b00(
                                                b14 as usize as i32,
                                                (u2 >> 11)
                                                    .wrapping_add(u3 >> 11)
                                                    .wrapping_add(u4 >> 11)
                                                    .wrapping_add(u5 >> 11)
                                                    >> 2,
                                                ((u3 >> 5 & 0x3f)
                                                    + (u2 >> 5 & 0x3f)
                                                    + (u4 >> 5 & 0x3f)
                                                    + (u5 >> 5 & 0x3f))
                                                    >> 2,
                                                ((u3 & 0x1f)
                                                    + (u2 & 0x1f)
                                                    + (u4 & 0x1f)
                                                    + (u5 & 0x1f))
                                                    >> 2,
                                                0,
                                                0x100,
                                            );
                                            p15 = p15.wrapping_add(2);
                                            *e4.wrapping_offset(ec as isize) = r as u8;
                                            ec += 1;
                                            f0 = f0.wrapping_add(2);
                                            if !(ec < v8) {
                                                break;
                                            }
                                        }
                                    }
                                    e4 = e4.wrapping_offset(v8 as isize);
                                    e0 = e0.wrapping_add((v8 as u32).wrapping_mul(4) as usize);
                                    dc = dc.wrapping_add((v8 as u32).wrapping_mul(4) as usize);
                                    d4 -= 1;
                                    if d4 == 0 {
                                        break;
                                    }
                                }
                            }
                            u100 = u100.wrapping_add(v6 as u32);
                            b15 = param_3.wrapping_offset(v6 as isize);
                            e0 = param_3;
                        }
                    }
                }
                let pu = pp(p1, 5);
                let mut v8 = fld(pu, 1);
                let mut adv: i32 = 0;
                let mut do_adv = false;
                if v8 == 0 {
                    v8 = (ld(pu.add(20))).wrapping_mul(ld(pu.add(16))) as i32;
                    if fld(pu, 2) == v8 {
                        v8 = fld(pu, 3);
                    }
                    let mut b14 = pu.wrapping_add(v8 as usize).wrapping_add(0x18);
                    if pp(pu, 0) == b14 && {
                        u100 = u100.wrapping_add((DAT_005233d8 as u32).wrapping_mul(0x1000));
                        !param_3.is_null()
                    } {
                        if DAT_005233d8 == 2 {
                            b14 = b14.wrapping_add(0x1000);
                        }
                        if DAT_005233d8 == 1 {
                            let mut i: i32 = 0;
                            loop {
                                let idx = b14
                                    .wrapping_offset((i as isize) * 2 + 0x1000)
                                    .cast::<u16>()
                                    .read_unaligned()
                                    as u32;
                                *param_3.wrapping_offset(i as isize) =
                                    *(idx.wrapping_add(DAT_009994bc) as usize as *const u8);
                                i += 1;
                                if !(i < 0x1000) {
                                    break;
                                }
                            }
                        } else {
                            cpy(param_3, b14, ((DAT_005233d8 as u32) & 0xfffff) << 10, 0);
                        }
                        if DAT_005233d8 == 2 {
                            if fld(p1, 0) != 0 {
                                FUN_004079e0();
                            }
                            FUN_00476b90();
                        }
                        adv = ((DAT_005233d8 as u32) << 0xc) as i32;
                        do_adv = true;
                    }
                } else {
                    u100 = u100.wrapping_add(
                        (v8 as u32)
                            .wrapping_mul(DAT_005233d8 as u32)
                            .wrapping_mul(0x100),
                    );
                    if !param_3.is_null() {
                        let mut v11 = (ld(pu.add(20))).wrapping_mul(ld(pu.add(16))) as i32;
                        if fld(pu, 2) == v11 {
                            v11 = fld(pu, 3);
                        }
                        let mut s16 = pu.wrapping_add(v11 as usize).wrapping_add(0x18);
                        if DAT_005233d8 == 2 {
                            s16 = s16.wrapping_add((v8 as u32).wrapping_mul(0x100) as usize);
                        }
                        if DAT_005233d8 == 1 {
                            let mut i: i32 = 0;
                            loop {
                                let idx = s16
                                    .wrapping_offset((i as isize) * 2 + 0x1000)
                                    .cast::<u16>()
                                    .read_unaligned()
                                    as u32;
                                *param_3.wrapping_offset(i as isize) =
                                    *(idx.wrapping_add(DAT_009994bc) as usize as *const u8);
                                i += 1;
                                if !(i < 0x1000) {
                                    break;
                                }
                            }
                        } else {
                            cpy(param_3, s16, ((DAT_005233d8 as u32) & 0xfffff) << 10, 0);
                        }
                        if DAT_005233d8 == 2 {
                            if fld(p1, 0) != 0 {
                                FUN_004079e0();
                            }
                            FUN_00476b90();
                        }
                        adv = ld(pu.add(4))
                            .wrapping_mul(DAT_005233d8 as u32)
                            .wrapping_mul(0x100) as i32;
                        do_adv = true;
                    }
                }
                if do_adv {
                    param_3 = param_3.wrapping_offset(adv as isize);
                }
                i108 = (i108 as u32).wrapping_add(u100) as i32;
            }
            0x15 => {
                let u9 = fld(p1, 4) as u32;
                let v8 = u9.wrapping_mul(4);
                if !param_3.is_null() {
                    st(hdr0.add(0x14), param_3 as usize as u32);
                    cpy(param_3, p12, u9 & 0x3fff_ffff, 0);
                    param_3 = param_3.wrapping_add(v8 as usize);
                }
                i108 = (i108 as u32).wrapping_add(v8) as i32;
            }
            0x16 => {
                if !param_3.is_null() {
                    st(hdr0.add(0x14), param_3 as usize as u32);
                    st(param_3, ld(p12));
                    st(param_3.add(4), ld(p12.add(4)));
                    st(param_3.add(8), ld(p12.add(8)));
                    st(param_3.add(0xc), ld(p12.add(0xc)));
                    param_3 = param_3.wrapping_add(0x10);
                }
                i108 = i108.wrapping_add(0x10);
            }
            0x17 => {
                if !param_3.is_null() {
                    st(hdr0.add(0x14), param_3 as usize as u32);
                    cpy(param_3, p12, 0xc, 0);
                    param_3 = param_3.wrapping_add(0x30);
                }
                i108 = i108.wrapping_add(0x30);
            }
            0x19 => {
                if !param_3.is_null() {
                    st(hdr0.add(0x14), param_3 as usize as u32);
                    cpy(param_3, p12, 0x12, 0);
                    param_3 = param_3.wrapping_add(0x48);
                }
                i108 = i108.wrapping_add(0x48);
            }
            _ => {}
        }
        let mut v8: i32 = 0;
        if fld(p1, 2) != 0 {
            cpy(auStack_d0.as_mut_ptr() as *mut u8, p2, 0x34, 0);
            DAT_0060f1c0 = 0;
            DAT_0060f1e8 = 0;
            let mut v11: i32 = 0;
            _DAT_0060f208 = core::ptr::null_mut();
            _DAT_0060f1f0 = 0;
            DAT_0060f1d8 = core::ptr::null_mut();
            if 0 < fld(p1, 2) {
                loop {
                    if !param_3.is_null() {
                        st(
                            pp(hdr0, 3).wrapping_offset(v8 as isize),
                            param_3 as usize as u32,
                        );
                    }
                    let arr = pp(p1, 3).wrapping_offset(v8 as isize);
                    let v7 = FUN_00476c20(
                        ld(arr) as usize as *mut i32,
                        auStack_d0.as_mut_ptr(),
                        param_3,
                    );
                    if !param_3.is_null() {
                        if v7 == 0 {
                            st(pp(hdr0, 3).wrapping_offset(v8 as isize), 0);
                        }
                        param_3 = param_3.wrapping_offset(v7 as isize);
                    }
                    v8 = v8.wrapping_add(4);
                    i108 = i108.wrapping_add(v7);
                    v11 += 1;
                    if !(v11 < fld(p1, 2)) {
                        break;
                    }
                }
            }
        }
        i108
    }
}
