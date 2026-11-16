// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006026b8: *mut i32;
    static mut DAT_006026bc: i32;
    static mut DAT_0052a54c: i32;
    static mut DAT_006026b0: *mut i32;
    static mut DAT_00602694: *mut i32;
    static mut DAT_00602680: *mut i32;
    static mut DAT_00602678: i32;
    static mut DAT_00602690: i32;
    static mut DAT_006026c0: i32;
    static mut DAT_00602684: *mut i32;
    static mut DAT_006026a0: i32;
    static mut DAT_006026ac: *mut i32;
    static mut DAT_006026a8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(a: *mut i8, b: *mut i8) -> i32;
    fn FUN_00489fb0(a: i32, b: *mut i32, c: *mut u32, d: i32, e: i32);
    fn FUN_0048e520(a: *mut u32, b: *mut i8) -> *mut i32;
    fn FUN_00489840(a: i32, b: *mut i32, c: i32, d: *mut u32);
    fn FUN_004899d0(a: i32, b: *mut i32, c: i32, d: *mut u32);
    fn FUN_00489b40(a: i32, b: *mut i32, c: i32, d: i32);
    fn FUN_00488a90(a: *mut i8, b: i32, c: i32) -> i32;
}

#[inline(always)]
unsafe fn rd(p: *mut i32, i: i32) -> i32 {
    // SAFETY: the caller passes a pointer into the game's own buffers.
    unsafe { p.wrapping_offset(i as isize).read_unaligned() }
}

#[inline(always)]
unsafe fn wr(p: *mut i32, i: i32, v: i32) {
    // SAFETY: the caller passes a pointer into the game's own buffers.
    unsafe { p.wrapping_offset(i as isize).write_unaligned(v) }
}

#[inline(always)]
fn ad(p: *mut i32, i: i32) -> *mut i32 {
    p.wrapping_offset(i as isize)
}

#[inline(always)]
unsafe fn rda(a: i32) -> i32 {
    // SAFETY: the address is one the original dereferences.
    unsafe { (a as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn wra(a: i32, v: i32) {
    // SAFETY: the address is one the original writes.
    unsafe { (a as *mut i32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rf(p: *mut f32, i: usize) -> f32 {
    // SAFETY: the pointer addresses the game's float triples.
    unsafe { p.wrapping_add(i).read_unaligned() }
}

#[inline(always)]
unsafe fn wf(p: *mut f32, i: usize, v: f32) {
    // SAFETY: the pointer addresses the game's float triples.
    unsafe { p.wrapping_add(i).write_unaligned(v) }
}

#[inline(always)]
unsafe fn cpy(dst: *mut i32, src: *const i32, n: u32) {
    let mut i: u32 = 0;
    while i != n {
        // SAFETY: forward dword copy between the game's buffers, as the original does.
        unsafe {
            dst.wrapping_add(i as usize)
                .write_unaligned(src.wrapping_add(i as usize).read_unaligned());
        }
        i = i.wrapping_add(1);
    }
}

#[inline(always)]
unsafe fn slen(a: i32) -> u32 {
    let mut p = a as *const u8;
    let mut n: u32 = 1;
    // SAFETY: the address is a NUL-terminated string in the game's memory.
    unsafe {
        while p.read() != 0 {
            p = p.wrapping_add(1);
            n = n.wrapping_add(1);
        }
    }
    n
}

#[inline(always)]
unsafe fn name(dst: *mut i32, src: i32) -> *mut i32 {
    // SAFETY: src is a NUL-terminated string, dst has room for it.
    unsafe {
        let n = slen(src);
        let d = dst as *mut u8;
        let s = src as *const u8;
        let mut i: u32 = 0;
        while i != n {
            d.wrapping_add(i as usize)
                .write(s.wrapping_add(i as usize).read());
            i = i.wrapping_add(1);
        }
        d.wrapping_add(n as usize) as *mut i32
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "C" fn FUN_00488bc0(
    mut param_1: *mut i32,
    param_2: *mut i32,
    param_3: i32,
    param_4: i32,
    param_5: *mut u32,
) -> i32 {
    // SAFETY: all pointers, globals and callees are the game's own memory,
    // accessed exactly where and in the order the original does.
    unsafe {
        let p3 = param_1;
        if param_2.is_null() {
            return 0;
        }
        let mut p10: *mut i32 = core::ptr::null_mut();
        let mut uf4: u32 = 0;
        let mut b: i32 = 1;

        match rd(param_2, 1) {
            1 | 0xf | 0x10 | 0x11 => {
                if rda(rd(param_2, 5)) < 0 {
                    b = 0;
                } else {
                    b = DAT_006026b8.is_null() as i32;
                    wr(param_1, 1, rd(param_2, 1));
                    let p4 = ad(param_1, 6);
                    p10 = p3;
                    if b != 0 {
                        if rd(param_2, 0) == 0 {
                            wr(param_1, 0, 0);
                            param_1 = p4;
                        } else {
                            wr(param_1, 0, p4 as i32);
                            param_1 = name(p4, rd(param_2, 0));
                        }
                        wr(p3, 4, rd(param_2, 4));
                        wr(p3, 5, param_1 as i32);
                        let i9 = rd(param_2, 4);
                        cpy(
                            param_1,
                            rd(param_2, 5) as *const i32,
                            (i9 as u32).wrapping_mul(0x30).wrapping_add(4) >> 2,
                        );
                        let p13 = ad(param_1, i9.wrapping_mul(0xc).wrapping_add(1));
                        let u7 = rd(param_2, 4) as u32;
                        cpy(p13, rd(param_2, 5) as *const i32, (u7 & 0xfffffff) << 2);
                        let i9 = rd(param_2, 4);
                        uf4 = (i9 as u32).wrapping_mul(0x24);
                        let p4b = (i9 as u32)
                            .wrapping_mul(0x30)
                            .wrapping_add(4)
                            .wrapping_add(rd(param_2, 5) as u32)
                            as *mut i32;
                        let p15 = ad(p13, u7.wrapping_mul(4) as i32);
                        cpy(p15, p4b, uf4 >> 2);
                        let i5 = DAT_006026bc;
                        param_1 = ad(p15, (i9 as u32).wrapping_mul(9) as i32);
                        if param_5.wrapping_add(0x21).read_unaligned() == 0 {
                            uf4 = (DAT_006026bc as u32).wrapping_mul(0xc);
                            let p4c = ad(p4b, (i9 as u32).wrapping_mul(9) as i32);
                            cpy(
                                param_1,
                                p4c,
                                (DAT_006026bc as u32).wrapping_mul(3) & 0x3fffffff,
                            );
                            param_1 = ad(param_1, (i5 as u32).wrapping_mul(3) as i32);
                        }
                        b = 0;
                    } else {
                        if rd(param_2, 0) == 0 {
                            wr(param_1, 0, 0);
                            param_1 = p4;
                        } else {
                            wr(param_1, 0, p4 as i32);
                            param_1 = name(p4, rd(param_2, 0));
                        }
                        let mut i9: i32 = 0;
                        wr(p3, 4, 0);
                        wr(p3, 5, param_1 as i32);
                        wr(param_1, 0, 0);
                        DAT_0052a54c = 0;
                        DAT_006026b0 = DAT_00602694;
                        if 0 < rd(DAT_00602680, 2) {
                            let mut i16: i32 = 0;
                            loop {
                                FUN_00489fb0(
                                    p3 as i32,
                                    param_2,
                                    rda(rd(DAT_00602680, 3).wrapping_add(i16)) as *mut u32,
                                    param_3,
                                    param_5 as i32,
                                );
                                if DAT_0052a54c != 0 {
                                    break;
                                }
                                i16 = i16.wrapping_add(4);
                                i9 = i9.wrapping_add(1);
                                if !(i9 < rd(DAT_00602680, 2)) {
                                    break;
                                }
                            }
                        }
                        b = 0;
                        param_1 = ad(
                            param_1,
                            (rd(p3, 4) as u32).wrapping_mul(0x19).wrapping_add(1) as i32,
                        );
                    }
                }
            }
            2 => {
                uf4 = 0x30;
            }
            3 => {
                DAT_00602678 = rd(param_2, 5);
                DAT_006026bc = rd(param_2, 4);
                if DAT_006026b8.is_null() {
                    uf4 = (rd(param_2, 4) as u32).wrapping_mul(0xc);
                } else {
                    b = 0;
                }
            }
            4 => {
                uf4 = 0xc;
            }
            5 => {
                uf4 = 0x24;
            }
            6 => {
                uf4 = 0xc;
            }
            7 => {
                uf4 = slen(rd(param_2, 5));
                p10 = param_2;
                loop {
                    if rd(p10, 1) != 7 {
                        break;
                    }
                    let pc14 = rd(p10, 5);
                    let mut e0: i32 = 0;
                    let mut i5 = rda(param_3.wrapping_add(6));
                    'found: {
                        if 0 < i5 {
                            let mut ec: i32 = 0;
                            loop {
                                let p15 = rda(param_3.wrapping_add(10));
                                let mut p4 = rda(p15.wrapping_add(ec)) as *mut i32;
                                if p4.is_null() {
                                } else if rd(p4, 0) == 0
                                    || __strcmpi(rd(p4, 0) as *mut i8, pc14 as *mut i8) != 0
                                {
                                    let mut i9: i32 = 0;
                                    let mut i16: i32 = 0;
                                    if 0 < rd(p4, 2) {
                                        loop {
                                            p10 = FUN_0048e520(
                                                rda(rd(p4, 3).wrapping_add(i9)) as *mut u32,
                                                pc14 as *mut i8,
                                            );
                                            if !p10.is_null() {
                                                break 'found;
                                            }
                                            i9 = i9.wrapping_add(4);
                                            i16 = i16.wrapping_add(1);
                                            if !(i16 < rd(p4, 2)) {
                                                break;
                                            }
                                        }
                                    }
                                    p4 = core::ptr::null_mut();
                                }
                                p10 = p4;
                                if !p4.is_null() {
                                    break 'found;
                                }
                                ec = ec.wrapping_add(4);
                                e0 = e0.wrapping_add(1);
                                i5 = rda(param_3.wrapping_add(6));
                                if !(e0 < i5) {
                                    break;
                                }
                            }
                        }
                        p10 = core::ptr::null_mut();
                    }
                    if p10.is_null() {
                        break;
                    }
                }
                if !p10.is_null() && rd(p10, 1) == 0x14 {
                    DAT_00602694 = p10;
                }
            }
            9 => {
                uf4 = (rd(param_2, 4) as u32).wrapping_mul(0x38);
                DAT_00602690 = rd(param_2, 5);
            }
            0xb => {
                DAT_006026c0 = rd(param_2, 5);
                param_5
                    .wrapping_add(0x21)
                    .write_unaligned(DAT_006026c0 as u32);
                if DAT_00602684.is_null() {
                    uf4 = (rd(param_2, 4) as u32).wrapping_mul(0xc);
                } else {
                    b = 0;
                }
            }
            0xd => {
                DAT_006026a0 = rd(param_2, 5);
                if DAT_006026ac.is_null() {
                    uf4 = (rd(param_2, 4) as u32) << 3;
                } else {
                    b = 0;
                }
            }
            0x13 => {
                uf4 = 0xc;
            }
            0x14 => {
                DAT_00602694 = param_2;
                let p4 = rd(param_2, 5) as *mut i32;
                let i9 = (rd(p4, 5) as u32).wrapping_mul(rd(p4, 4) as u32) as i32;
                let mut i16 = i9;
                if i9.wrapping_sub(rd(p4, 2)) == 0 {
                    i16 = rd(p4, 3);
                }
                uf4 = (i16 as u32).wrapping_add(0x18);
                if rd(p4, 1) == 0 {
                    let mut i9b = i9;
                    if i9.wrapping_sub(rd(p4, 2)) == 0 {
                        i9b = rd(p4, 3);
                    }
                    if rd(p4, 0) == (p4 as u32).wrapping_add(i9b as u32).wrapping_add(0x18) as i32 {
                        uf4 = (i16 as u32).wrapping_add(0x3018);
                    }
                } else {
                    uf4 = uf4.wrapping_add((rd(p4, 1) as u32).wrapping_mul(0x300));
                }
            }
            0x15 => {
                DAT_00602680 = param_2;
                wr(param_1, 1, 0);
                p10 = ad(param_1, 6);
                if rd(param_2, 0) == 0 {
                    wr(param_1, 0, 0);
                    param_1 = p10;
                } else {
                    let end = name(p10, rd(param_2, 0));
                    wr(param_1, 0, p10 as i32);
                    param_1 = end;
                }
                wr(p3, 5, 0);
                wr(p3, 4, 0);
                p10 = ad(param_1, 4);
                wr(p3, 2, 4);
                wr(p3, 3, param_1 as i32);
                wr(param_1, 0, p10 as i32);
                wr(param_1, 5, 3);
                wr(p10, 0, 0);
                wr(param_1, 9, ad(param_1, 10) as i32);
                wr(param_1, 8, 0);
                wr(param_1, 6, 0);
                wr(param_1, 7, 0);
                FUN_00489840(p10 as i32, param_2, param_3, param_5);
                let mut pf6 = rd(param_1, 9) as *mut f32;
                let mut f_ec = rf(pf6, 0);
                let mut f_e8 = rf(pf6, 1);
                let mut f_e4 = rf(pf6, 2);
                let mut i9 = rd(param_1, 8);
                let mut f_e0 = f_ec;
                let mut f_dc = f_e8;
                let mut f_d8 = f_e4;
                if 0 < i9 {
                    loop {
                        if rf(pf6, 0) < f_ec {
                            f_ec = rf(pf6, 0);
                        }
                        if rf(pf6, 1) < f_e8 {
                            f_e8 = rf(pf6, 1);
                        }
                        if rf(pf6, 2) < f_e4 {
                            f_e4 = rf(pf6, 2);
                        }
                        if f_e0 < rf(pf6, 0) {
                            f_e0 = rf(pf6, 0);
                        }
                        if f_dc < rf(pf6, 1) {
                            f_dc = rf(pf6, 1);
                        }
                        if f_d8 < rf(pf6, 2) {
                            f_d8 = rf(pf6, 2);
                        }
                        pf6 = pf6.wrapping_add(3);
                        i9 = i9.wrapping_sub(1);
                        if i9 == 0 {
                            break;
                        }
                    }
                }
                DAT_006026b8 = p10;
                wf(pf6, 0, f_ec);
                wf(pf6, 1, f_e8);
                wf(pf6, 2, f_e4);
                wf(pf6, 3, f_e0);
                wf(pf6, 4, f_dc);
                wf(pf6, 5, f_d8);
                let i9 = rd(param_1, 8);
                wr(param_1, 8, (i9 as u32).wrapping_add(2) as i32);
                p10 = ad(
                    param_1,
                    10i32.wrapping_add(((i9 as u32).wrapping_add(2).wrapping_mul(3)) as i32),
                );
                wra(rd(p3, 3).wrapping_add(4), p10 as i32);
                wr(p10, 1, 0xd);
                wr(p10, 0, 0);
                wr(p10, 5, ad(p10, 6) as i32);
                wr(p10, 4, 0);
                wr(p10, 2, 0);
                wr(p10, 3, 0);
                FUN_004899d0(p10 as i32, param_2, param_3, param_5);
                let p4 = ad(
                    p10,
                    6i32.wrapping_add((rd(p10, 4) as u32).wrapping_mul(2) as i32),
                );
                DAT_006026ac = p10;
                wra(rd(p3, 3).wrapping_add(8), p4 as i32);
                wr(p4, 1, 0xb);
                wr(p4, 0, 0);
                wr(p4, 5, ad(p4, 6) as i32);
                wr(p4, 4, 0);
                wr(p4, 2, 0);
                wr(p4, 3, 0);
                let i9 = DAT_006026bc;
                let u2 = param_5.wrapping_add(0x21).read_unaligned();
                param_5.wrapping_add(0x21).write_unaligned(0);
                FUN_00489b40(p4 as i32, param_2, param_3, param_5 as i32);
                param_5.wrapping_add(0x21).write_unaligned(u2);
                p10 = ad(
                    p4,
                    6i32.wrapping_add((rd(p4, 4) as u32).wrapping_mul(3) as i32),
                );
                DAT_00602684 = p4;
                DAT_006026bc = i9;
                wra(rd(p3, 3).wrapping_add(0xc), p10 as i32);
                let p13 = ad(p10, 6);
                wr(p10, 1, 0x15);
                wr(p10, 0, 0);
                wr(p10, 5, p13 as i32);
                wr(p10, 4, rd(param_2, 4));
                cpy(
                    p13,
                    rd(param_2, 5) as *const i32,
                    (rd(param_2, 4) as u32) & 0x3fffffff,
                );
                b = 0;
                param_1 = ad(p13, rd(param_2, 4));
            }
            0x16 => {
                uf4 = 0x10;
            }
            0x17 => {
                uf4 = 0x30;
            }
            0x19 => {
                uf4 = 0x48;
            }
            _ => {}
        }

        let mut p4 = param_1;
        'done: {
            if b != 0 {
                let p10n = ad(param_1, 6);
                wr(param_1, 1, rd(param_2, 1));
                if rd(param_2, 0) == 0 {
                    wr(param_1, 0, 0);
                    param_1 = p10n;
                } else {
                    wr(param_1, 0, p10n as i32);
                    param_1 = name(p10n, rd(param_2, 0));
                }
                wr(p4, 4, rd(param_2, 4));
                wr(p4, 5, param_1 as i32);
                let src = rd(param_2, 5) as *mut i32;
                cpy(param_1, src, uf4 >> 2);
                let mut d =
                    (param_1 as *mut u8).wrapping_add(((uf4 >> 2) as usize).wrapping_mul(4));
                let mut s = src.wrapping_add((uf4 >> 2) as usize) as *mut u8;
                let mut k = uf4 & 3;
                while k != 0 {
                    d.write(s.read());
                    s = s.wrapping_add(1);
                    d = d.wrapping_add(1);
                    k -= 1;
                }
                param_1 = (param_1 as u32).wrapping_add(uf4) as *mut i32;
                p10 = p4;
                if rd(param_2, 1) == 0x14 {
                    let pu17 = rd(param_2, 5) as *mut i32;
                    if rd(pu17, 1) == 0 {
                        let mut i9 = (rd(pu17, 5) as u32).wrapping_mul(rd(pu17, 4) as u32) as i32;
                        if rd(pu17, 2) == i9 {
                            i9 = rd(pu17, 3);
                        }
                        let pc14 = rd(pu17, 0);
                        if (pu17 as u32).wrapping_add(i9 as u32).wrapping_add(0x18) as i32 == pc14 {
                            p4 = rd(p4, 5) as *mut i32;
                            let mut i9 = (rd(p4, 5) as u32).wrapping_mul(rd(p4, 4) as u32) as i32;
                            if rd(p4, 2) == i9 {
                                i9 = rd(p4, 3);
                            }
                            wr(
                                p4,
                                0,
                                (p4 as u32).wrapping_add(i9 as u32).wrapping_add(0x18) as i32,
                            );
                        } else {
                            let mut i16: i32 = 0;
                            let mut i9: i32 = 0;
                            let mut i5 = rda(param_4.wrapping_add(6));
                            if 0 < i5 {
                                loop {
                                    let p15 = rda(param_4.wrapping_add(10));
                                    let p11 = rda(p15.wrapping_add(i16)) as *mut i32;
                                    if p4 == p11 {
                                        break;
                                    }
                                    i5 = FUN_00488a90(pc14 as *mut i8, p11 as i32, p4 as i32);
                                    if i5 != 0 {
                                        wra(rd(p4, 5), i5);
                                        break 'done;
                                    }
                                    i16 = i16.wrapping_add(4);
                                    i9 = i9.wrapping_add(1);
                                    i5 = rda(param_4.wrapping_add(6));
                                    if !(i9 < i5) {
                                        break;
                                    }
                                }
                            }
                            let p11 = rd(pu17, 0) as *mut i32;
                            wra(rd(p4, 5), param_1 as i32);
                            cpy(param_1, p11, 0xc00);
                            param_1 = ad(param_1, 0xc00);
                        }
                    }
                }
            }
        }

        if p10.is_null() {
            if rd(param_2, 2) == 0 {
                return 0;
            }
            wr(param_1, 0, 0);
            wr(param_1, 1, 0);
            wr(param_1, 4, 0);
            wr(param_1, 5, 0);
            p10 = param_1;
            param_1 = ad(param_1, 6);
        }
        let mut i9: i32 = 0;
        wr(p10, 2, 0);
        wr(p10, 3, 0);
        if rd(param_2, 2) != 0 {
            let mut au_stack_d0: [u32; 52] = [0u32; 52];
            let mut k: usize = 0;
            while k != 0x34 {
                au_stack_d0[k] = param_5.wrapping_add(k).read_unaligned();
                k += 1;
            }
            DAT_00602678 = 0;
            DAT_006026a0 = 0;
            DAT_006026c0 = 0;
            DAT_006026a8 = 0;
            DAT_00602690 = 0;
            wr(p10, 3, param_1 as i32);
            let mut i16: i32 = 0;
            wr(p10, 2, rd(param_2, 2));
            param_1 = ad(param_1, rd(param_2, 2));
            if 0 < rd(param_2, 2) {
                loop {
                    wra(rd(p10, 3).wrapping_add(i9), param_1 as i32);
                    let i5 = FUN_00488bc0(
                        param_1,
                        rda(rd(param_2, 3).wrapping_add(i9)) as *mut i32,
                        param_3,
                        param_4,
                        au_stack_d0.as_mut_ptr(),
                    );
                    if i5 == 0 {
                        wra(rd(p10, 3).wrapping_add(i9), 0);
                    }
                    i9 = i9.wrapping_add(4);
                    param_1 = (param_1 as u32).wrapping_add(i5 as u32) as *mut i32;
                    i16 = i16.wrapping_add(1);
                    if !(i16 < rd(param_2, 2)) {
                        break;
                    }
                }
            }
        }
        (param_1 as u32).wrapping_sub(p3 as u32) as i32
    }
}
