// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8c2c: i32;
    static mut DAT_00556efc: i32;
    static mut DAT_009e9680: [i8; 0];
    static mut DAT_009ec468: [i8; 0];
    static mut DAT_009ecc34: i32;
    static mut DAT_00a080f8: i32;
    static mut DAT_00557358: i32;
    static mut DAT_0055734c: i32;
    static mut DAT_00a60a08: *mut i8;
    static mut DAT_00a60a10: *mut i8;
    static mut DAT_00a60a20: *mut i8;
    static mut DAT_00a07cc8: i32;
    static mut DAT_009e9658: i32;
    static mut DAT_009d6940: u8;
    static mut DAT_00556ed8: [i32; 0];
    static mut DAT_00557560: [u32; 0];
    static mut DAT_00557360: u32;
    static mut DAT_00557364: i32;
    static mut DAT_00a07bb0: [i32; 0];
    static mut DAT_009d8a30: [i32; 0];
    static mut DAT_005242cc: i32;
    static mut DAT_009a8c24: i32;
    static mut DAT_0052342c: i32;
    static mut DAT_00556f00: i32;
    static mut DAT_00556f08: [i32; 0];
    static mut DAT_005242dc: i32;
    static mut DAT_005242e0: i32;
    static mut DAT_005242e4: i32;
    static mut DAT_009ed23c: i32;
    static mut DAT_00523428: i32;
    static mut DAT_009fd394: i32;
    static mut DAT_005242c4: i32;
    static mut DAT_00556ed0: i32;
    static mut DAT_0052340c: i32;
    static mut DAT_00523418: i32;
    static mut DAT_00523420: i32;
    static mut DAT_0055978c: i32;
    static mut DAT_00557588: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac760() -> i32;
    fn FUN_004aa7f0() -> i32;
    fn FUN_004aa870() -> u32;
    fn FUN_00448f90(a: i32, b: *mut u8, c: i32);
    fn FUN_00448da0();
    fn FUN_00462a10() -> *mut i8;
    fn FUN_0046e040() -> i32;
    fn FUN_00470060();
    fn FUN_00463c10(a: i32);
    fn FUN_0046e050(a: *mut i32, b: *mut i32) -> *mut i32;
    fn FUN_00464c80(a: i32) -> i32;
    fn FUN_00464d70(a: i32);
    fn FUN_0046f660(a: i32) -> i32;
    fn FUN_00418650(a: i32);
    fn FUN_00418760(a: i32, b: u32, c: *mut i32) -> *mut i32;
    fn FUN_0046d350(a: i32, b: *mut u32, c: i32);
    fn FUN_00418f80(a: *mut i32);
    fn FUN_00463c50(a: i32);
    fn FUN_004193c0(a: i32, b: i32);
    fn FUN_00463cc0(a: i32);
    fn FUN_00448ed0();
    fn FUN_00419510(a: i32);
    fn FUN_00463b60();
    fn FUN_0046f700() -> i32;
    fn FUN_00465a20(a: *mut i32);
    fn FUN_00463bd0();
    #[link_name = "sprintf"]
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
}

#[repr(C)]
struct Stk {
    a: u32,
    b: u16,
    c: u16,
}

#[inline(always)]
unsafe fn slot(i: i32) -> *mut i8 {
    // SAFETY: address arithmetic inside the game's own table.
    unsafe {
        (&raw mut DAT_009e9680)
            .cast::<i8>()
            .offset(i.wrapping_mul(0x5bd) as isize)
    }
}

#[inline(always)]
unsafe fn a_i32(base: *mut [i32; 0], i: i32) -> *mut i32 {
    // SAFETY: address arithmetic inside the game's own array.
    unsafe { base.cast::<i32>().offset(i as isize) }
}

#[inline(always)]
unsafe fn fin() {
    // SAFETY: the game's own globals and function.
    unsafe {
        if DAT_00557358 == 0 {
            return;
        }
        DAT_00557358 = 0;
        FUN_00448ed0();
        DAT_009a8c2c = DAT_0055734c;
    }
}

#[inline(always)]
unsafe fn adjust(v: i32) {
    // SAFETY: the game's own global.
    unsafe {
        if DAT_009ed23c < v {
            let mut d = v.wrapping_sub(DAT_009ed23c) >> 1;
            if d == 0 {
                d = 1;
            }
            DAT_009ed23c = DAT_009ed23c.wrapping_add(d);
        } else if v < DAT_009ed23c {
            let mut d = DAT_009ed23c.wrapping_sub(v) >> 1;
            if d == 0 {
                d = 1;
            }
            DAT_009ed23c = DAT_009ed23c.wrapping_sub(d);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00463d00() {
    // SAFETY: the globals and callees are the game's own; every pointer read
    // or written is the game's message or table memory, as in the original.
    unsafe {
        let mut i_var3: i32 = DAT_009a8c2c;
        let i_var4_init: i32 = DAT_00556efc;
        if *slot(DAT_009ecc34) == 0 {
            return;
        }
        let mut local_58: i32 = 0;
        let mut local_70: i32 = 0;
        let mut local_68: i32 = 0;
        let mut local_64: i32 = 0;
        let mut local_6c: i32 = 0;
        let mut local_78: i32 = 0;
        let mut u_stack_74: u32;
        let mut local_60: u32 = 0;
        let mut local_54: i32 = 0;
        let mut ab: [u8; 80] = [0; 80];
        let mut stk = Stk { a: 0, b: 0, c: 0 };
        let local_5c: i32;

        if DAT_00a080f8 == 1 {
            while !FUN_0046e050(&raw mut local_78, &raw mut local_54).is_null() {}
            return;
        }
        if DAT_00557358 == 0 {
            let r = FUN_004ac760();
            i_var3 = i_var3.wrapping_add(r);
            local_5c = i_var3;
        } else {
            local_5c = DAT_0055734c.wrapping_sub(0x33a);
            let r = FUN_004ac760();
            i_var3 = i_var4_init.wrapping_add(1).wrapping_add(r);
        }
        'main: loop {
            if 0x33a < i_var3.wrapping_sub(local_5c) {
                if DAT_00557358 == 0 {
                    local_60 = 1;
                    FUN_00448da0();
                    let src = DAT_00a60a08;
                    let mut n: usize = 0;
                    while *src.add(n) != 0 {
                        n += 1;
                    }
                    n += 1;
                    let mut k = 0;
                    while k < n {
                        ab[k] = *src.add(k) as u8;
                        k += 1;
                    }
                    let p6 = FUN_00462a10();
                    if !p6.is_null() {
                        let mut m: usize = 0;
                        while *p6.add(m) != 0 {
                            m += 1;
                        }
                        m += 1;
                        let mut e: usize = 0;
                        while ab[e] != 0 {
                            e += 1;
                        }
                        let mut k = 0;
                        while k < m {
                            *ab.as_mut_ptr().add(e + k) = *p6.add(k) as u8;
                            k += 1;
                        }
                    }
                    FUN_00448f90(1, ab.as_mut_ptr(), 0x34);
                    DAT_00557358 = 1;
                    DAT_00556efc = i_var3;
                    DAT_0055734c = i_var3;
                    let r = FUN_0046e040();
                    if r == 0 {
                        FUN_00470060();
                        FUN_00463c10(DAT_009ecc34);
                    }
                } else {
                    let r = FUN_004aa7f0();
                    if r != 0 && FUN_004aa870() == 0x1b {
                        DAT_00557358 = 0;
                        FUN_00448ed0();
                        DAT_009a8c2c = DAT_0055734c;
                        DAT_00a07cc8 = DAT_009e9658;
                        DAT_009d6940 = 1;
                        *slot(DAT_009ecc34) = 0;
                        FUN_00463c50(DAT_009ecc34);
                        let r = FUN_0046e040();
                        if r != 0 {
                            FUN_00463bd0();
                            return;
                        }
                        FUN_00463cc0(DAT_009ecc34);
                        return;
                    }
                    if 0x76 < i_var3.wrapping_sub(DAT_00556efc) {
                        local_60 = (local_60 == 0) as u32;
                        if local_60 == 0 {
                            DAT_00556efc = i_var3;
                            FUN_00448f90(3, DAT_00a60a10 as *mut u8, 0x34);
                        } else {
                            DAT_00556efc = i_var3;
                            FUN_00448f90(3, DAT_00a60a20 as *mut u8, 0x34);
                        }
                    }
                }
            }
            let i_var4 = FUN_004ac760();
            let mut pi7 = FUN_0046e050(&raw mut local_78, &raw mut local_54);
            let i_var8 = FUN_004ac760();
            i_var3 = i_var3.wrapping_add(i_var4).wrapping_add(i_var8);
            local_64 = local_64.wrapping_add(i_var8);
            if !pi7.is_null() {
                match *pi7 {
                    1 => {
                        let i_var4 = FUN_004ac760();
                        let i_var8 = FUN_0046f660(local_78);
                        if *slot(i_var8) == 0 {
                            DAT_00557360 = 0x15;
                            DAT_00557364 = i_var8;
                            FUN_0046d350(local_78, &raw mut DAT_00557360, 8);
                        } else {
                            let q = a_i32(&raw mut DAT_00556ed8, i_var8);
                            if 0 < *q {
                                *q = 0;
                            }
                            FUN_00418650(i_var8);
                            stk.a &= 0xff000000;
                            let b = pi7.cast::<u8>();
                            let b2 = *b.add(4);
                            u_stack_74 = b2 as u32;
                            let _ = u_stack_74;
                            let b10 = b2 & 0x7f;
                            let u5: u32;
                            let p11: *mut u8;
                            if b10 == 0x7f {
                                u5 = b.add(5).cast::<u32>().read_unaligned();
                                if b2 & 0x80 == 0 {
                                    p11 = b.add(9);
                                } else {
                                    p11 = b.add(10);
                                    stk.a = (stk.a & 0xff000000) | ((*b.add(9) as u32) << 0x10);
                                }
                            } else {
                                let mut u = (&raw mut DAT_00557560)
                                    .cast::<u32>()
                                    .offset(i_var8 as isize)
                                    .read();
                                if (b10 as u32) < (u & 0x7f) {
                                    u = u.wrapping_add(0x80);
                                }
                                u = (u & 0xffffff80) | b10 as u32;
                                u5 = u;
                                if b2 & 0x80 == 0 {
                                    p11 = b.add(5);
                                } else {
                                    p11 = b.add(6);
                                    stk.a = (stk.a & 0xff000000) | ((*b.add(5) as u32) << 0x10);
                                }
                            }
                            (&raw mut DAT_00557560)
                                .cast::<u32>()
                                .offset(i_var8 as isize)
                                .write(u5);
                            stk.a = (((*p11 as u32) << 24) | (stk.a & 0xffffff)) & 0xfeffffff;
                            stk.b = (((((((*p11.add(1) & 1) as u32) * 2) | ((*p11 & 1) as u32))
                                << 8)
                                | (*p11.add(1) as u32))
                                & 0xfffe) as u16;
                            pi7 = FUN_00418760(i_var8, u5, (&raw mut stk).cast::<i32>());
                            if !pi7.is_null() {
                                let r = FUN_0046e040();
                                *pi7 = 1;
                                if r == 0 {
                                    *pi7 = 0;
                                }
                                *pi7.add(1) = 1;
                            }
                        }
                        let i_var8 = FUN_004ac760();
                        i_var3 = i_var3.wrapping_add(i_var4).wrapping_add(i_var8);
                        local_68 = local_68.wrapping_add(i_var8);
                    }
                    2 => {
                        let mut i_var14: i32 = 0;
                        let i_var4 = FUN_004ac760();
                        DAT_009a8c24 = 0;
                        DAT_005242cc = DAT_005242cc.wrapping_add(1);
                        FUN_00418f80(pi7);
                        let r = FUN_0046e040();
                        if r != 0 {
                            let mut q = (&raw mut DAT_00556ed8).cast::<i32>();
                            let mut pc = (&raw mut DAT_009e9680).cast::<i8>();
                            let end = (&raw mut DAT_009ec468).cast::<i8>();
                            loop {
                                if i_var14 != DAT_009ecc34 && *pc != 0 && *q != -1 {
                                    let v = (*q).wrapping_add(DAT_00523418);
                                    *q = v;
                                    if 0x1ba8 < v {
                                        FUN_00463c50(i_var14);
                                        *q = 0;
                                    }
                                }
                                q = q.add(1);
                                pc = pc.offset(0x5bd);
                                i_var14 = i_var14.wrapping_add(1);
                                if !(pc < end) {
                                    break;
                                }
                            }
                        }
                        if DAT_009d6940 == 1 {
                            fin();
                            return;
                        }
                        let i_var8 = FUN_004ac760();
                        i_var3 = i_var3.wrapping_add(i_var4).wrapping_add(i_var8);
                        local_6c = local_6c.wrapping_add(i_var8);
                        local_58 = local_58.wrapping_add(1);
                    }
                    3 => {
                        let i_var4 = *pi7.add(1);
                        if -1 < i_var4 && i_var4 < 8 {
                            *a_i32(&raw mut DAT_00a07bb0, i_var4) = 0;
                        }
                    }
                    4 => {
                        let i_var4 = FUN_0046f660(local_78);
                        if *slot(i_var4) != 0 {
                            FUN_004193c0(local_78, pi7 as i32);
                        }
                    }
                    8 => {
                        DAT_0052342c = 1;
                        DAT_009d6940 = 1;
                        *slot(DAT_009ecc34) = 0;
                        fin();
                        return;
                    }
                    0x10 => {
                        DAT_00556f00 = 1;
                        let v = *pi7.add(1);
                        if (v as u32) < 0x10 {
                            *a_i32(&raw mut DAT_00556f08, v) = 1;
                        }
                    }
                    0x15 => {
                        let i_var4 = *pi7.add(1);
                        if -1 < i_var4 && i_var4 < 8 {
                            *a_i32(&raw mut DAT_009d8a30, i_var4) = 1;
                        }
                        if i_var4 == DAT_009ecc34 {
                            DAT_009d6940 = 1;
                            *slot(DAT_009ecc34) = 0;
                            i_var3 = DAT_009ecc34;
                            *a_i32(&raw mut DAT_009d8a30, DAT_009ecc34) = 1;
                            FUN_00463cc0(i_var3);
                            fin();
                            return;
                        }
                    }
                    0x16 => {
                        let i_var4 = FUN_0046f660(local_78);
                        if *slot(i_var4) == 0 {
                            DAT_00557360 = 0x15;
                            DAT_00557364 = i_var4;
                            FUN_0046d350(local_78, &raw mut DAT_00557360, 8);
                        } else {
                            let q = a_i32(&raw mut DAT_00556ed8, i_var4);
                            if 0 < *q {
                                *q = 0;
                            }
                            let b = pi7.cast::<u8>();
                            let mut p11 = b.add(5);
                            FUN_00418650(i_var4);
                            let mut u5: u32 = *b.add(4) as u32;
                            if *b.add(4) != 0 {
                                loop {
                                    u_stack_74 = u5;
                                    stk.a &= 0xff000000;
                                    let b2 = *p11;
                                    let b10 = b2 & 0x7f;
                                    let p12: *mut u8;
                                    if b10 == 0x7f {
                                        u5 = p11.add(1).cast::<u32>().read_unaligned();
                                        if b2 & 0x80 == 0 {
                                            p12 = p11.add(5);
                                        } else {
                                            p12 = p11.add(6);
                                            stk.a = (stk.a & 0xff000000)
                                                | ((*p11.add(5) as u32) << 0x10);
                                        }
                                    } else {
                                        let mut u = (&raw mut DAT_00557560)
                                            .cast::<u32>()
                                            .offset(i_var4 as isize)
                                            .read();
                                        if (b10 as u32) < (u & 0x7f) {
                                            u = u.wrapping_add(0x80);
                                        }
                                        u = (u & 0xffffff80) | b10 as u32;
                                        u5 = u;
                                        if b2 & 0x80 == 0 {
                                            p12 = p11.add(1);
                                        } else {
                                            p12 = p11.add(2);
                                            stk.a = (stk.a & 0xff000000)
                                                | ((*p11.add(1) as u32) << 0x10);
                                        }
                                    }
                                    p11 = p12.add(2);
                                    (&raw mut DAT_00557560)
                                        .cast::<u32>()
                                        .offset(i_var4 as isize)
                                        .write(u5);
                                    stk.a =
                                        (((*p12 as u32) << 24) | (stk.a & 0xffffff)) & 0xfeffffff;
                                    stk.b = (((((((*p12.add(1) & 1) as u32) * 2)
                                        | ((*p12 & 1) as u32))
                                        << 8)
                                        | (*p12.add(1) as u32))
                                        & 0xfffe)
                                        as u16;
                                    pi7 = FUN_00418760(i_var4, u5, (&raw mut stk).cast::<i32>());
                                    if !pi7.is_null() {
                                        let r = FUN_0046e040();
                                        *pi7 = 1;
                                        if r == 0 {
                                            *pi7 = 0;
                                        }
                                        *pi7.add(1) = 1;
                                    }
                                    u5 = u_stack_74.wrapping_sub(1);
                                    if u_stack_74.wrapping_sub(1) == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    0x17 => {
                        DAT_005242dc = *pi7.add(1);
                    }
                    0x18 => {
                        FUN_00419510(pi7 as i32);
                        FUN_00463b60();
                    }
                    0x1b => {
                        if DAT_005242e0 != 0 {
                            DAT_005242e0 = DAT_005242e0.wrapping_sub(1);
                            if DAT_005242e0 == 0 {
                                DAT_005242e4 = 0;
                                if DAT_00557358 == 0 {
                                    DAT_005242e4 = 0;
                                    return;
                                }
                                fin();
                                return;
                            }
                        }
                    }
                    0x1c => {
                        DAT_009ed23c = *pi7.add(1);
                    }
                    0x1d => {
                        let r = FUN_0046f700();
                        if r == local_78 {
                            DAT_009a8c24 = 0;
                        } else {
                            let i_var4 = FUN_0046f660(local_78);
                            if *slot(i_var4) != 0 {
                                let q = a_i32(&raw mut DAT_00556ed8, i_var4);
                                if 0 < *q {
                                    *q = 0;
                                }
                            }
                        }
                    }
                    0x1e => {
                        DAT_00557360 = 0x1f;
                        DAT_00557364 = *pi7.add(1);
                        FUN_0046d350(local_78, &raw mut DAT_00557360, 8);
                        let mut i_var4 = *pi7.add(2);
                        if DAT_00523428 == 0 || DAT_009fd394 < DAT_005242c4 {
                            i_var4 >>= 1;
                        }
                        adjust(i_var4);
                    }
                    0x1f => {
                        let r = FUN_0046e040();
                        if r == 0 && *pi7.add(1) == DAT_00556ed0 {
                            let i_var4 = DAT_0052340c
                                .wrapping_add(DAT_009a8c2c)
                                .wrapping_sub(*pi7.add(1))
                                .wrapping_add(0x14);
                            if i_var4 < 0x1d8 {
                                adjust(i_var4);
                            }
                        }
                    }
                    0x3c => {
                        DAT_0055978c = 1;
                        let dst = (&raw mut DAT_00557588).cast::<i32>();
                        let mut k = 0usize;
                        while k < 0x7e {
                            *dst.add(k) = *pi7.add(k + 1);
                            k += 1;
                        }
                    }
                    0x3d | 0x3e | 0x3f => {
                        if *pi7.add(1) == DAT_00523420 {
                            FUN_00465a20(pi7);
                            if 0x1ba8 < DAT_009a8c24 {
                                fin();
                                return;
                            }
                            if *slot(DAT_009ecc34) == 0 {
                                fin();
                                return;
                            }
                        }
                    }
                    _ => {}
                }
                continue 'main;
            }
            let r = FUN_0046e040();
            if r != 0 {
                let i_var4 = FUN_004ac760();
                let i_var8 = FUN_00464c80(DAT_009a8c2c);
                if i_var8 != 0 {
                    FUN_00464d70(DAT_009a8c2c);
                    let i_var8 = FUN_004ac760();
                    i_var3 = i_var3.wrapping_add(i_var4).wrapping_add(i_var8);
                    local_70 = local_70.wrapping_add(i_var8);
                    continue 'main;
                }
                let i_var8 = FUN_004ac760();
                i_var3 = i_var3.wrapping_add(i_var4).wrapping_add(i_var8);
                local_70 = local_70.wrapping_add(i_var8);
            }
            if DAT_00557358 != 0 {
                DAT_00557358 = 0;
                FUN_00448ed0();
                DAT_009a8c2c = DAT_0055734c;
                i_var3 = DAT_0055734c;
            }
            let i_var4 = FUN_004ac760();
            DAT_009a8c2c = i_var3.wrapping_add(i_var4);
            let all = DAT_009a8c2c.wrapping_sub(local_5c);
            let misc = all
                .wrapping_sub(local_70)
                .wrapping_sub(local_68)
                .wrapping_sub(local_64)
                .wrapping_sub(local_6c);
            _sprintf(
                ab.as_mut_ptr().cast::<i8>(),
                b"RcvMsg:%-2d In2Svr:%-2d SvrSnd:%-2d SvrFrm:%-2d NumFrm:%-2d AllPIN:%-2d Misc:%-2d\n\0"
                    .as_ptr()
                    .cast::<i8>(),
                local_64,
                local_68,
                local_70,
                local_6c,
                local_58,
                all,
                misc,
            );
            return;
        }
    }
}
