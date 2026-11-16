// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051bd08: [i32; 0];
    static mut DAT_0051bd70: [i8; 0];
    static mut DAT_0051bd80: [i8; 0];
    static mut DAT_005506f8: [i8; 0];
    static mut DAT_005505f8: [i8; 0];
    static mut DAT_0051be28: [i8; 0];
    static mut DAT_0051be10: [i8; 0];
    static mut DAT_0051bdf0: [i8; 0];
    static mut DAT_0051bdec: [i8; 0];
    static mut DAT_0051be08: [i8; 0];
    static mut DAT_0051be04: [i8; 0];
    static mut DAT_0051bdfc: [i8; 0];
    static mut DAT_0051bdf8: [i8; 0];
    static mut DAT_0051be14: [i8; 0];
    static mut DAT_0051bdd8: [i8; 0];
    static mut DAT_0051bda0: [i8; 0];
    static mut DAT_0051bdcc: [i8; 0];
    static mut DAT_0051bdc4: [i8; 0];
    static mut DAT_0051add0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn fscanf(f: *mut c_void, fmt: *const i8, ...) -> i32;
    fn printf(fmt: *const i8, ...) -> i32;
    fn _strnicmp(a: *const i8, b: *const i8, n: u32) -> i32;
    fn _strcmpi(a: *const i8, b: *const i8) -> i32;
    fn ftell(f: *mut c_void) -> i32;
    fn fseek(f: *mut c_void, off: i32, whence: i32) -> i32;
    fn fgetc(f: *mut c_void) -> i32;
    fn atoi(s: *const i8) -> i32;

    fn FUN_00413c60(f: *mut c_void);
    fn FUN_004138a0(f: *mut c_void);
    fn FUN_00413aa0(f: *mut c_void) -> i32;
    fn FUN_00413a30(f: *mut c_void) -> i32;
    fn FUN_00413970(f: *mut c_void);
    fn FUN_00413bf0(f: *mut c_void) -> i32;
    fn FUN_004138e0(f: *mut c_void);
    fn FUN_00413b80(f: *mut c_void) -> i32;
    fn FUN_00413920(f: *mut c_void);
    fn FUN_004139f0(f: *mut c_void);
    fn FUN_00413b10(f: *mut c_void) -> i32;
    fn FUN_004139b0(f: *mut c_void);
}

unsafe fn rd(a: i32) -> i32 {
    // SAFETY: the address is one the original dereferences.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

unsafe fn wr(a: i32, v: i32) {
    // SAFETY: the address is one the original writes.
    unsafe { (a as u32 as usize as *mut i32).write_unaligned(v) }
}

unsafe fn rb(a: i32) -> u8 {
    // SAFETY: the address is one the original reads.
    unsafe { (a as u32 as usize as *const u8).read() }
}

unsafe fn wb(a: i32, v: u8) {
    // SAFETY: the address is one the original writes.
    unsafe { (a as u32 as usize as *mut u8).write(v) }
}

fn ad(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

fn pt(a: i32) -> *mut i8 {
    a as u32 as usize as *mut i8
}

unsafe fn sl(a: i32) -> i32 {
    let mut n: i32 = 0;
    // SAFETY: a points at a NUL-terminated string.
    while unsafe { rb(ad(a, n)) } != 0 {
        n = n.wrapping_add(1);
    }
    n
}

unsafe fn cpy(dst: i32, src: i32, n: i32) {
    let mut d = dst;
    let mut s = src;
    let mut c = (n as u32) >> 2;
    // SAFETY: the original copies these same bytes.
    unsafe {
        while c != 0 {
            wr(d, rd(s));
            s = ad(s, 4);
            d = ad(d, 4);
            c -= 1;
        }
        let mut c = (n as u32) & 3;
        while c != 0 {
            wb(d, rb(s));
            s = ad(s, 1);
            d = ad(d, 1);
            c -= 1;
        }
    }
}

unsafe fn rec(f: *mut c_void, a: i32, b: i32) -> i32 {
    // SAFETY: recursive call with the original's arguments.
    unsafe {
        FUN_00412120(
            f,
            a as u32 as usize as *mut i32,
            b as u32 as usize as *mut u32,
        ) as i32
    }
}

unsafe fn find_type(buf: i32, table: i32, end: i32) -> i32 {
    let mut e = table;
    let mut idx: i32 = 0;
    // SAFETY: table is the game's type table; buf its token buffer.
    unsafe {
        loop {
            let len = sl(buf);
            let r = _strnicmp(pt(buf), pt(rd(rd(e))), len as u32);
            if r == 0 {
                break;
            }
            idx += 1;
            e = ad(e, 4);
            if !((e as u32) < (end as u32)) {
                break;
            }
        }
    }
    idx
}

unsafe fn enum_val(buf: i32, info: i32) -> i32 {
    // SAFETY: info is the game's enum descriptor; buf its token buffer.
    unsafe {
        let count = rd(info);
        let names = rd(ad(info, 4));
        let vals = rd(ad(info, 8));
        let mut k: i32 = 0;
        if 0 < count {
            loop {
                if _strcmpi(pt(buf), pt(rd(ad(names, k.wrapping_mul(4))))) == 0 {
                    break;
                }
                k += 1;
                if !(k < count) {
                    break;
                }
            }
        }
        if count == k {
            atoi(pt(buf))
        } else {
            rd(ad(vals, k.wrapping_mul(4)))
        }
    }
}

unsafe fn quoted(f: *mut c_void, pp12: &mut i32, l38: &mut i32) -> bool {
    // SAFETY: reads from the game's file; writes where the original does.
    unsafe {
        loop {
            let c = fgetc(f) as i8;
            if c == -1 {
                printf((&raw mut DAT_0051be14) as *const i8);
                return false;
            }
            *l38 = l38.wrapping_add(1);
            if c == b'"' as i8 {
                break;
            }
            if *pp12 != 0 {
                wb(*pp12, c as u8);
                *pp12 = ad(*pp12, 1);
            }
        }
        if *pp12 != 0 {
            wb(*pp12, 0);
            *pp12 = ad(*pp12, 1);
        }
    }
    true
}

unsafe fn set3(p: i32, ptr: i32, cnt: i32, ty: i32) {
    // SAFETY: p is the node's member slot.
    unsafe {
        wr(ad(p, 8), ptr);
        wr(ad(p, 4), cnt);
        wr(p, ty);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00412120(
    param_1: *mut c_void,
    param_2: *mut i32,
    param_3: *mut u32,
) -> u32 {
    // SAFETY: the globals, callees and buffers are the game's own; every
    // pointer is read and written where the original does.
    unsafe {
        let bufp = (&raw mut DAT_005506f8) as *mut i8;
        let buf = bufp as usize as i32;
        let flags = (&raw mut DAT_005505f8) as usize as i32;
        let table = (&raw mut DAT_0051bd08) as usize as i32;
        let end = (&raw mut DAT_0051bd70) as usize as i32;
        let f256 = (&raw mut DAT_0051bd80) as *const i8;
        let e_s = (&raw mut DAT_0051be14) as *const i8;
        let w_s = (&raw mut DAT_0051bdd8) as *const i8;
        let f_d = (&raw mut DAT_0051bdf0) as *const i8;
        let f_f = (&raw mut DAT_0051bdec) as *const i8;
        let f_3 = (&raw mut DAT_0051bdcc) as *const i8;
        let f_2 = (&raw mut DAT_0051bdc4) as *const i8;
        let f_str = (&raw mut DAT_0051be10) as *const i8;

        let mut p2 = param_2 as usize as i32;
        if p2 != 0 {
            param_3.write_unaligned(p2 as u32);
        }
        let mut local_38: i32 = 0;
        if fscanf(param_1, f256, bufp) != 1 {
            return 0xffffffff;
        }
        if _strnicmp(bufp, (&raw mut DAT_0051be28) as *const i8, 1) == 0 {
            FUN_00413c60(param_1);
        }
        let mut ti = find_type(buf, table, end);
        let mut local_2c: i32 = 0;
        let mut local_c: i32 = 0;
        if ti == 8 {
            if fscanf(param_1, f256, bufp) != 1 {
                printf(e_s);
                return 0;
            }
            local_38 = ad(sl(buf), 1);
            if p2 != 0 {
                cpy(p2, buf, local_38);
                local_2c = p2;
                p2 = ad(p2, local_38);
                param_3.write_unaligned(p2 as u32);
            }
            let local_20 = local_38;
            if fscanf(param_1, f256, bufp) != 1 {
                printf(e_s);
                return local_20 as u32;
            }
            ti = find_type(buf, table, end);
        }
        if 0x19 < ti {
            printf((&raw mut DAT_0051bda0) as *const i8);
            if FUN_00413b10(param_1) != 0 {
                FUN_004138a0(param_1);
                FUN_004139b0(param_1);
            }
            return local_38 as u32;
        }
        let local_30 = ad(table, ti.wrapping_mul(4));
        let desc = rd(local_30);
        let count = rd(ad(desc, 4));
        let memb = rd(ad(desc, 8));

        let mut piv6 = p2;
        if p2 != 0 {
            wr(p2, local_2c);
            wr(ad(p2, 4), ti);
            piv6 = ad(p2, 24);
            wr(ad(p2, 16), count);
            local_c = p2;
        }
        let mut pp12 = piv6;
        if piv6 != 0 {
            wr(ad(local_c, 20), piv6);
            pp12 = ad(piv6, count.wrapping_mul(12));
            local_2c = piv6;
        }
        local_38 = ad(ad(local_38, 0x18), count.wrapping_mul(0xc));

        if ti == 7 {
            if FUN_00413a30(param_1) == 0 {
                if fscanf(param_1, f256, bufp) != 1 {
                    printf(e_s);
                    return local_38 as u32;
                }
                let len1 = ad(sl(buf), 1);
                if pp12 != 0 {
                    cpy(pp12, buf, len1);
                    wr(ad(local_2c, 8), pp12);
                    pp12 = ad(pp12, len1);
                    wr(ad(local_2c, 4), 1);
                    wr(local_2c, 1);
                }
                local_38 = ad(local_38, len1);
            } else {
                FUN_00413970(param_1);
                if pp12 != 0 {
                    wr(ad(local_2c, 8), pp12);
                    wr(ad(local_2c, 4), 1);
                    wr(local_2c, 1);
                }
                if !quoted(param_1, &mut pp12, &mut local_38) {
                    return local_38 as u32;
                }
            }
            if pp12 != 0 {
                wr(ad(local_c, 12), 0);
                wr(ad(local_c, 8), 0);
            }
            return local_38 as u32;
        }

        FUN_004138a0(param_1);
        let mut k: i32 = 0;
        if 0 < count {
            loop {
                wb(ad(flags, k), 0);
                k += 1;
                if !(k < count) {
                    break;
                }
            }
        }
        let mut local_10: i32 = 0;
        let mut local_34: i32 = 0;
        let mut local_24: i32 = 0;
        let mut local_18: i32 = 0;
        let mut local_14: i32 = 0;
        let mut local_8: i32 = 0;
        if 0 < count {
            'outer: loop {
                if FUN_00413aa0(param_1) != 0 {
                    break;
                }
                let local_20 = ftell(param_1);
                if fscanf(param_1, f256, bufp) != 1 {
                    printf(e_s);
                    return local_38 as u32;
                }
                let mut local_1c: i32 = 0;
                if 0 < count {
                    let mut o: i32 = 0;
                    loop {
                        let len = sl(buf);
                        let r = _strnicmp(pt(buf), pt(rd(rd(ad(memb, o)))), len as u32);
                        if r == 0 {
                            break;
                        }
                        o = ad(o, 4);
                        local_1c += 1;
                        if !(local_1c < count) {
                            break;
                        }
                    }
                }
                if count == local_1c {
                    fseek(param_1, local_20, 0);
                    break 'outer;
                }
                wb(ad(flags, local_1c), 1);
                let local_28 = local_1c.wrapping_mul(4);
                let piv6 = ad(local_2c, local_1c.wrapping_mul(12));
                let mdesc = rd(ad(memb, local_28));
                let mtype = rd(ad(mdesc, 4));
                'sw: {
                    match mtype {
                        1 => {
                            if FUN_00413a30(param_1) == 0 {
                                if pp12 == 0 {
                                    if fscanf(param_1, f256, bufp) != 1 {
                                        printf(e_s);
                                        return local_38 as u32;
                                    }
                                    local_38 = ad(local_38, ad(sl(buf), 1));
                                } else {
                                    set3(piv6, pp12, 1, 1);
                                    if fscanf(param_1, f_str, pt(pp12)) != 1 {
                                        printf(e_s);
                                        return local_38 as u32;
                                    }
                                    let l1 = ad(sl(pp12), 1);
                                    pp12 = ad(pp12, l1);
                                    local_38 = ad(local_38, l1);
                                }
                            } else {
                                FUN_00413970(param_1);
                                if pp12 != 0 {
                                    set3(piv6, pp12, 1, 1);
                                }
                                if !quoted(param_1, &mut pp12, &mut local_38) {
                                    return local_38 as u32;
                                }
                            }
                        }
                        3 => {
                            if pp12 == 0 {
                                let u = rec(param_1, 0, 0);
                                local_38 = ad(local_38, u);
                            } else {
                                let u = rec(param_1, pp12, ad(piv6, 8));
                                local_38 = ad(local_38, u);
                                pp12 = ad(pp12, u);
                                wr(ad(piv6, 4), 1);
                                wr(piv6, 3);
                            }
                        }
                        4 => {
                            if FUN_00413bf0(param_1) == 0 {
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                    pp12 = ad(pp12, 4);
                                    wr(ad(piv6, 4), 1);
                                    wr(piv6, 4);
                                    if pp12 != 0 {
                                        let u = rec(param_1, pp12, ad(rd(ad(piv6, 8)), 4));
                                        local_38 = ad(ad(local_38, 4), u);
                                        pp12 = ad(pp12, u);
                                        break 'sw;
                                    }
                                }
                                let u = rec(param_1, 0, 0);
                                local_38 = ad(ad(local_38, 4), u);
                            } else {
                                FUN_004138e0(param_1);
                                let mut n: i32 = 0;
                                let l20 = ftell(param_1);
                                while FUN_00413b80(param_1) == 0 {
                                    let u = rec(param_1, 0, 0);
                                    if u != 0 {
                                        n += 1;
                                        FUN_00413920(param_1);
                                    }
                                }
                                fseek(param_1, l20, 0);
                                local_38 = ad(local_38, n.wrapping_mul(4));
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                    pp12 = ad(pp12, n.wrapping_mul(4));
                                    wr(ad(piv6, 4), n);
                                    wr(piv6, 4);
                                }
                                n = 0;
                                while FUN_00413b80(param_1) == 0 {
                                    if pp12 == 0 {
                                        let u = rec(param_1, 0, 0);
                                        if u != 0 {
                                            local_38 = ad(local_38, u);
                                            FUN_00413920(param_1);
                                        }
                                    } else {
                                        let u = rec(
                                            param_1,
                                            pp12,
                                            ad(rd(ad(piv6, 8)), n.wrapping_mul(4)),
                                        );
                                        if u != 0 {
                                            pp12 = ad(pp12, u);
                                            n += 1;
                                            local_38 = ad(local_38, u);
                                            FUN_00413920(param_1);
                                        }
                                    }
                                }
                                FUN_004139f0(param_1);
                            }
                        }
                        5 => {
                            if fscanf(param_1, f_d, &raw mut local_34) != 1 {
                                printf(e_s);
                                return local_38 as u32;
                            }
                            if pp12 != 0 {
                                set3(piv6, pp12, 1, 5);
                                wr(pp12, local_34);
                                pp12 = ad(pp12, 4);
                            }
                            local_38 = ad(local_38, 4);
                        }
                        6 => {
                            if FUN_00413bf0(param_1) == 0 {
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                }
                                if fscanf(param_1, f_d, &raw mut local_34) != 1 {
                                    printf(e_s);
                                    return local_38 as u32;
                                }
                                if pp12 != 0 {
                                    wr(pp12, local_34);
                                    pp12 = ad(pp12, 4);
                                }
                                local_38 = ad(local_38, 4);
                                if pp12 != 0 {
                                    wr(ad(piv6, 4), 1);
                                    wr(piv6, 6);
                                }
                            } else {
                                FUN_004138e0(param_1);
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                }
                                let mut n: i32 = 0;
                                while FUN_00413b80(param_1) == 0 {
                                    if fscanf(param_1, f_d, &raw mut local_34) != 1 {
                                        printf(e_s);
                                        return local_38 as u32;
                                    }
                                    if pp12 != 0 {
                                        wr(pp12, local_34);
                                        pp12 = ad(pp12, 4);
                                    }
                                    n += 1;
                                    local_38 = ad(local_38, 4);
                                    FUN_00413920(param_1);
                                }
                                if pp12 != 0 {
                                    wr(ad(piv6, 4), n);
                                    wr(piv6, 6);
                                }
                                FUN_004139f0(param_1);
                            }
                        }
                        7 => {
                            if fscanf(param_1, f_f, &raw mut local_24) != 1 {
                                printf(w_s);
                            }
                            if pp12 != 0 {
                                set3(piv6, pp12, 0, 5);
                                wr(pp12, local_24);
                                pp12 = ad(pp12, 4);
                            }
                            local_38 = ad(local_38, 4);
                        }
                        8 => {
                            if FUN_00413bf0(param_1) == 0 {
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                }
                                if fscanf(param_1, f_f, &raw mut local_24) != 1 {
                                    printf(w_s);
                                }
                                if pp12 != 0 {
                                    wr(pp12, local_24);
                                    pp12 = ad(pp12, 4);
                                }
                                local_38 = ad(local_38, 4);
                                if pp12 != 0 {
                                    wr(ad(piv6, 4), 1);
                                    wr(piv6, 8);
                                }
                            } else {
                                FUN_004138e0(param_1);
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                }
                                let mut n: i32 = 0;
                                while FUN_00413b80(param_1) == 0 {
                                    if fscanf(param_1, f_f, &raw mut local_24) != 1 {
                                        printf(w_s);
                                    }
                                    if pp12 != 0 {
                                        wr(pp12, local_24);
                                        pp12 = ad(pp12, 4);
                                    }
                                    n += 1;
                                    local_38 = ad(local_38, 4);
                                    FUN_00413920(param_1);
                                }
                                if pp12 != 0 {
                                    wr(ad(piv6, 4), n);
                                    wr(piv6, 8);
                                }
                                FUN_004139f0(param_1);
                            }
                        }
                        9 => {
                            if fscanf(param_1, f256, bufp) != 1 {
                                printf(e_s);
                                return local_38 as u32;
                            }
                            let uv2: u8;
                            if _strcmpi(bufp, (&raw mut DAT_0051be08) as *const i8) == 0
                                || _strcmpi(bufp, (&raw mut DAT_0051be04) as *const i8) == 0
                            {
                                uv2 = 1;
                            } else {
                                if _strcmpi(bufp, (&raw mut DAT_0051bdfc) as *const i8) != 0
                                    && _strcmpi(bufp, (&raw mut DAT_0051bdf8) as *const i8) != 0
                                {
                                    printf(e_s);
                                    return local_38 as u32;
                                }
                                uv2 = 0;
                            }
                            local_38 = ad(local_38, 1);
                            if pp12 != 0 {
                                set3(piv6, pp12, 1, 9);
                                wb(pp12, uv2);
                                pp12 = ad(pp12, 1);
                            }
                        }
                        0xb | 0xd => {
                            if fscanf(
                                param_1,
                                f_3,
                                &raw mut local_18,
                                &raw mut local_14,
                                &raw mut local_8,
                            ) != 3
                            {
                                printf(w_s);
                            }
                            if pp12 != 0 {
                                set3(piv6, pp12, 1, mtype);
                                wr(pp12, local_18);
                                wr(ad(pp12, 4), local_14);
                                wr(ad(pp12, 8), local_8);
                                pp12 = ad(pp12, 12);
                            }
                            local_38 = ad(local_38, 12);
                        }
                        0xc | 0xe => {
                            if FUN_00413bf0(param_1) == 0 {
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                }
                                if fscanf(
                                    param_1,
                                    f_3,
                                    &raw mut local_18,
                                    &raw mut local_14,
                                    &raw mut local_8,
                                ) != 3
                                {
                                    printf(w_s);
                                }
                                if pp12 != 0 {
                                    wr(pp12, local_18);
                                    wr(ad(pp12, 4), local_14);
                                    wr(ad(pp12, 8), local_8);
                                    pp12 = ad(pp12, 12);
                                }
                                local_38 = ad(local_38, 12);
                                if pp12 != 0 {
                                    wr(ad(piv6, 4), 1);
                                    wr(piv6, mtype);
                                }
                            } else {
                                FUN_004138e0(param_1);
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                }
                                let mut n: i32 = 0;
                                while FUN_00413b80(param_1) == 0 {
                                    if fscanf(
                                        param_1,
                                        f_3,
                                        &raw mut local_18,
                                        &raw mut local_14,
                                        &raw mut local_8,
                                    ) != 3
                                    {
                                        printf(w_s);
                                    }
                                    if pp12 != 0 {
                                        wr(pp12, local_18);
                                        wr(ad(pp12, 4), local_14);
                                        wr(ad(pp12, 8), local_8);
                                        pp12 = ad(pp12, 12);
                                    }
                                    n += 1;
                                    local_38 = ad(local_38, 12);
                                    FUN_00413920(param_1);
                                }
                                if pp12 != 0 {
                                    wr(ad(piv6, 4), n);
                                    wr(piv6, mtype);
                                }
                                FUN_004139f0(param_1);
                            }
                        }
                        0xf => {
                            if pp12 != 0 {
                                set3(piv6, pp12, 1, mtype);
                            }
                            let mut n: i32 = 0x10;
                            local_38 = ad(local_38, 0x40);
                            loop {
                                if fscanf(param_1, f_f, &raw mut local_24) != 1 {
                                    printf(w_s);
                                }
                                if pp12 != 0 {
                                    wr(pp12, local_24);
                                    pp12 = ad(pp12, 4);
                                }
                                n -= 1;
                                if n == 0 {
                                    break;
                                }
                            }
                        }
                        0x10 | 0x14 => {
                            let lim: i32 = if mtype == 0x10 { 0x10 } else { 4 };
                            if FUN_00413bf0(param_1) == 0 {
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                }
                                local_34 = 0;
                                loop {
                                    if fscanf(param_1, f_f, &raw mut local_24) != 1 {
                                        printf(w_s);
                                    }
                                    if pp12 != 0 {
                                        wr(pp12, local_24);
                                        pp12 = ad(pp12, 4);
                                    }
                                    local_38 = ad(local_38, 4);
                                    local_34 += 1;
                                    if !(local_34 < lim) {
                                        break;
                                    }
                                }
                                if pp12 != 0 {
                                    wr(ad(piv6, 4), 1);
                                    wr(piv6, mtype);
                                }
                            } else {
                                FUN_004138e0(param_1);
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                }
                                let mut n: i32 = 0;
                                while FUN_00413b80(param_1) == 0 {
                                    local_34 = 0;
                                    loop {
                                        if fscanf(param_1, f_f, &raw mut local_24) != 1 {
                                            printf(w_s);
                                        }
                                        if pp12 != 0 {
                                            wr(pp12, local_24);
                                            pp12 = ad(pp12, 4);
                                        }
                                        local_38 = ad(local_38, 4);
                                        local_34 += 1;
                                        if !(local_34 < lim) {
                                            break;
                                        }
                                    }
                                    n += 1;
                                    FUN_00413920(param_1);
                                }
                                if pp12 != 0 {
                                    wr(ad(piv6, 4), n);
                                    wr(piv6, mtype);
                                }
                                FUN_004139f0(param_1);
                            }
                        }
                        0x11 => {
                            if fscanf(
                                param_1,
                                f_2,
                                &raw mut local_18,
                                &raw mut local_14,
                                &raw mut local_8,
                            ) != 2
                            {
                                printf(w_s);
                            }
                            if pp12 != 0 {
                                set3(piv6, pp12, 1, mtype);
                                wr(pp12, local_18);
                                wr(ad(pp12, 4), local_14);
                                pp12 = ad(pp12, 8);
                            }
                            local_38 = ad(local_38, 8);
                        }
                        0x12 => {
                            if FUN_00413bf0(param_1) == 0 {
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                }
                                if fscanf(param_1, f_2, &raw mut local_18, &raw mut local_14) != 2 {
                                    printf(w_s);
                                }
                                if pp12 != 0 {
                                    wr(pp12, local_18);
                                    wr(ad(pp12, 4), local_14);
                                    pp12 = ad(pp12, 8);
                                }
                                local_38 = ad(local_38, 8);
                                if pp12 != 0 {
                                    wr(ad(piv6, 4), 1);
                                    wr(piv6, mtype);
                                }
                            } else {
                                FUN_004138e0(param_1);
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                }
                                let mut n: i32 = 0;
                                while FUN_00413b80(param_1) == 0 {
                                    if fscanf(param_1, f_2, &raw mut local_18, &raw mut local_14)
                                        != 2
                                    {
                                        printf(w_s);
                                    }
                                    if pp12 != 0 {
                                        wr(pp12, local_18);
                                        wr(ad(pp12, 4), local_14);
                                        pp12 = ad(pp12, 8);
                                    }
                                    n += 1;
                                    local_38 = ad(local_38, 8);
                                    FUN_00413920(param_1);
                                }
                                if pp12 != 0 {
                                    wr(ad(piv6, 4), n);
                                    wr(piv6, mtype);
                                }
                                FUN_004139f0(param_1);
                            }
                        }
                        0x13 => {
                            if pp12 != 0 {
                                set3(piv6, pp12, 1, mtype);
                            }
                            let mut n: i32 = 4;
                            local_38 = ad(local_38, 0x10);
                            loop {
                                if fscanf(param_1, f_f, &raw mut local_24) != 1 {
                                    printf(w_s);
                                }
                                if pp12 != 0 {
                                    wr(pp12, local_24);
                                    pp12 = ad(pp12, 4);
                                }
                                n -= 1;
                                if n == 0 {
                                    break;
                                }
                            }
                        }
                        0x15 => {
                            if FUN_00413aa0(param_1) == 0 {
                                if fscanf(param_1, f256, bufp) != 1 {
                                    printf(e_s);
                                    return local_38 as u32;
                                }
                            } else {
                                let def = (&raw mut DAT_0051add0) as usize as i32;
                                let mut j: i32 = 0;
                                while j < 8 {
                                    wb(ad(buf, j), rb(ad(def, j)));
                                    j += 1;
                                }
                            }
                            local_34 = enum_val(buf, rd(ad(mdesc, 8)));
                            if pp12 != 0 {
                                wr(ad(piv6, 8), pp12);
                                wr(ad(piv6, 4), 1);
                                wr(piv6, 0x15);
                                wr(pp12, local_34);
                                pp12 = ad(pp12, 4);
                            }
                            local_38 = ad(local_38, 4);
                        }
                        0x16 => {
                            if FUN_00413bf0(param_1) == 0 {
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                }
                                if fscanf(param_1, f256, bufp) != 1 {
                                    printf(e_s);
                                    return local_38 as u32;
                                }
                                local_34 = enum_val(buf, rd(ad(mdesc, 8)));
                                if pp12 != 0 {
                                    wr(pp12, local_34);
                                    pp12 = ad(pp12, 4);
                                }
                                local_38 = ad(local_38, 4);
                                if pp12 != 0 {
                                    wr(ad(piv6, 4), 1);
                                    wr(piv6, mtype);
                                }
                            } else {
                                FUN_004138e0(param_1);
                                if pp12 != 0 {
                                    wr(ad(piv6, 8), pp12);
                                }
                                let mut n: i32 = 0;
                                while FUN_00413b80(param_1) == 0 {
                                    if fscanf(param_1, f256, bufp) != 1 {
                                        printf(e_s);
                                        return local_38 as u32;
                                    }
                                    local_34 = enum_val(buf, rd(ad(mdesc, 8)));
                                    if pp12 != 0 {
                                        wr(pp12, local_34);
                                        pp12 = ad(pp12, 4);
                                    }
                                    local_38 = ad(local_38, 4);
                                    n += 1;
                                    FUN_00413920(param_1);
                                }
                                if pp12 != 0 {
                                    wr(ad(piv6, 4), n);
                                    wr(piv6, mtype);
                                }
                                FUN_004139f0(param_1);
                            }
                        }
                        _ => {}
                    }
                }
                local_10 += 1;
                if !(local_10 < count) {
                    break;
                }
            }
        }

        local_10 = 0;
        if 0 < count {
            let mut o: i32 = 0;
            let mut p7 = local_2c;
            loop {
                if rb(ad(flags, local_10)) == 0 {
                    let md = rd(ad(memb, o));
                    if pp12 != 0 {
                        let dp = rd(ad(md, 0xc));
                        wr(p7, rd(dp));
                        wr(ad(p7, 4), rd(ad(dp, 4)));
                        wr(ad(p7, 8), pp12);
                        local_2c = rd(ad(md, 0x14));
                        cpy(pp12, rd(ad(md, 0x10)), local_2c);
                        pp12 = ad(pp12, rd(ad(md, 0x14)));
                    }
                    local_38 = ad(local_38, rd(ad(md, 0x14)));
                }
                local_10 += 1;
                p7 = ad(p7, 12);
                o = ad(o, 4);
                if !(local_10 < count) {
                    break;
                }
            }
        }

        let mut n: i32 = 0;
        let off = ftell(param_1);
        while FUN_00413aa0(param_1) == 0 {
            let u = rec(param_1, 0, 0);
            if u != 0 {
                n += 1;
            }
        }
        fseek(param_1, off, 0);
        if n == 0 {
            if pp12 != 0 {
                wr(ad(local_c, 12), 0);
                wr(ad(local_c, 8), 0);
            }
        } else {
            local_38 = ad(local_38, n.wrapping_mul(4));
            let mut piv6b: i32;
            if pp12 == 0 {
                piv6b = 0;
                pp12 = local_2c;
            } else {
                wr(ad(local_c, 12), pp12);
                piv6b = ad(pp12, n.wrapping_mul(4));
                wr(ad(local_c, 8), n);
            }
            while FUN_00413aa0(param_1) == 0 {
                if piv6b == 0 {
                    let u = rec(param_1, 0, 0);
                    if u != 0 {
                        local_38 = ad(local_38, u);
                    }
                } else {
                    let u = rec(param_1, piv6b, pp12);
                    if u != 0 {
                        piv6b = ad(piv6b, u);
                        local_38 = ad(local_38, u);
                        pp12 = ad(pp12, 4);
                    }
                }
            }
        }
        FUN_004139b0(param_1);
        local_38 as u32
    }
}
