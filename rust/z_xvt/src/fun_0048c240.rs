// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0060268c: i32;
    static mut DAT_006a8884: i32;
    static mut DAT_006a887a: [u8; 4];
    static mut DAT_00602678: *mut f32;
    static mut DAT_006026bc: i32;
    static mut DAT_006026c0: *mut f32;
    static mut DAT_006026a0: *mut f32;
    static mut DAT_006026a8: i32;
    static mut DAT_00602690: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048d980(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_0048dba0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_00415300(param_1: *mut f32, param_2: *mut f32);
    fn FUN_00414fe0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_0048e580(param_1: *mut i32, param_2: *mut i8) -> *mut u8;
}

unsafe fn g(p: *const i32, i: i32) -> i32 {
    // SAFETY: the game's own memory, read where the original reads.
    unsafe { p.wrapping_offset(i as isize).read_unaligned() }
}

unsafe fn s(p: *mut i32, i: i32, v: i32) {
    // SAFETY: the game's own memory, written where the original writes.
    unsafe { p.wrapping_offset(i as isize).write_unaligned(v) }
}

unsafe fn rd(a: i32, off: i32) -> i32 {
    // SAFETY: an address held as an integer by the original, read as an int.
    unsafe { (a.wrapping_add(off) as *const i32).read_unaligned() }
}

unsafe fn gf(p: *const f32, i: i32) -> f32 {
    // SAFETY: the game's own memory, read where the original reads.
    unsafe { p.wrapping_offset(i as isize).read_unaligned() }
}

unsafe fn sf(p: *mut f32, i: i32, v: f32) {
    // SAFETY: the game's own memory, written where the original writes.
    unsafe { p.wrapping_offset(i as isize).write_unaligned(v) }
}

fn pa(p: *mut i32, n: i32) -> *mut i32 {
    p.wrapping_offset(n as isize)
}

unsafe fn slen(p: *const i8) -> u32 {
    let mut n: u32 = 0xffff_ffff;
    let mut q = p;
    loop {
        if n == 0 {
            break;
        }
        n = n.wrapping_sub(1);
        // SAFETY: the string is nul-terminated, as the original assumes.
        let c = unsafe { q.read() };
        q = q.wrapping_add(1);
        if c == 0 {
            break;
        }
    }
    !n
}

unsafe fn cpys(dst: *mut i32, src: *const i8, n: u32) {
    let mut d = dst;
    let mut sp = src as *const i32;
    let mut k = n >> 2;
    while k != 0 {
        // SAFETY: copies the caller's buffers as the original does.
        unsafe { d.write_unaligned(sp.read_unaligned()) };
        sp = sp.wrapping_add(1);
        d = d.wrapping_add(1);
        k -= 1;
    }
    let mut db = d as *mut i8;
    let mut sb = sp as *const i8;
    let mut r = n & 3;
    while r != 0 {
        // SAFETY: byte tail of the same copy.
        unsafe { db.write(sb.read()) };
        sb = sb.wrapping_add(1);
        db = db.wrapping_add(1);
        r -= 1;
    }
}

unsafe fn cpyw(dst: *mut i32, src: *const i32, n: u32) {
    let mut d = dst;
    let mut sp = src;
    let mut k = n;
    while k != 0 {
        // SAFETY: copies the caller's buffers as the original does.
        unsafe { d.write_unaligned(sp.read_unaligned()) };
        sp = sp.wrapping_add(1);
        d = d.wrapping_add(1);
        k -= 1;
    }
}

unsafe fn sec3(param_4: *mut i32, src: i32, cnt: i32, f10: i32) -> *mut i32 {
    // SAFETY: the game's own buffers, copied as the original does.
    unsafe {
        cpyw(
            param_4,
            src as *const i32,
            (cnt.wrapping_mul(0xc) as u32) >> 2,
        );
        let mut p9 = pa(param_4, cnt.wrapping_mul(3));
        let mut p4 = p9;
        if cnt < f10 {
            let p15 = src.wrapping_sub(0xc).wrapping_add(cnt.wrapping_mul(0xc)) as *mut i32;
            let mut i6 = f10.wrapping_sub(cnt);
            p4 = pa(p9, i6.wrapping_mul(3));
            loop {
                s(p9, 0, g(p15, 0));
                i6 = i6.wrapping_sub(1);
                s(p9, 1, g(p15, 1));
                s(p9, 2, g(p15, 2));
                p9 = pa(p9, 3);
                if i6 == 0 {
                    break;
                }
            }
        }
        p4
    }
}

unsafe fn sec1(param_4: *mut i32, src: i32, cnt: i32, f10: i32) -> *mut i32 {
    // SAFETY: the game's own buffers, copied as the original does.
    unsafe {
        cpyw(param_4, src as *const i32, (cnt as u32) & 0x3fff_ffff);
        let mut p9 = pa(param_4, cnt);
        let mut p4 = p9;
        if cnt < f10 {
            let mut i16v = f10.wrapping_sub(cnt);
            p4 = pa(p9, i16v);
            let sp = src.wrapping_sub(4).wrapping_add(cnt.wrapping_mul(4)) as *const i32;
            loop {
                i16v = i16v.wrapping_sub(1);
                s(p9, 0, g(sp, 0));
                p9 = pa(p9, 1);
                if i16v == 0 {
                    break;
                }
            }
        }
        p4
    }
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_assignments,
    unused_mut,
    unused_variables,
    unused_unsafe
)]
pub unsafe extern "C" fn FUN_0048c240(
    param_1: u32,
    param_2: *mut i32,
    mut param_3: i32,
    mut param_4: *mut i32,
    param_5: *mut u32,
    param_6: *mut u8,
) -> i32 {
    let pfVar3: *mut i32 = param_4;
    if param_2.is_null() {
        return 0;
    }
    let mut pfVar9: *mut i32;
    let mut pfVar15: *mut i32;
    let mut pfVar22: *mut i32;
    let mut piVar19: *mut i32;
    let mut local_d8: *mut i32;
    let mut local_dc: i32;
    let mut local_e0: i32;
    let mut local_d0: [i32; 52] = [0; 52];
    let mut fVar10: i32;
    let mut fVar12: i32;
    let mut fVar4: i32;
    let mut fVar14: i32;
    let mut fVar13: i32;
    let mut fVar23: i32 = 0;
    let mut fVar24: i32 = 0;
    let mut iVar6: i32;
    let mut iVar11: i32;
    let mut iVar16: i32;
    let mut iVar17: i32;
    let mut iVar18: i32;
    let mut uVar7: u32;

    // SAFETY: every pointer is the game's own memory; reads, writes and calls
    // happen in the order and at the offsets the original uses.
    unsafe {
        pfVar9 = pa(param_4, 6);
        fVar10 = g(param_2, 1);
        s(param_4, 1, fVar10);
        if g(param_2, 0) == 0 {
            s(param_4, 0, 0);
            param_4 = pfVar9;
        } else {
            s(param_4, 0, pfVar9 as i32);
            uVar7 = slen(g(param_2, 0) as *const i8);
            cpys(pfVar9, g(param_2, 0) as *const i8, uVar7);
            uVar7 = slen(g(param_2, 0) as *const i8);
            param_4 = pfVar9
                .cast::<u8>()
                .wrapping_add(uVar7 as usize)
                .cast::<i32>();
        }
        s(pfVar3, 4, 1);
        s(pfVar3, 5, param_4 as i32);
        s(pfVar3, 2, 0);
        s(pfVar3, 3, 0);
        iVar17 = g(param_2, 5);
        pfVar9 = param_4;
        if iVar17 != 0 && g(param_2, 4) != 0 {
            local_d8 = rd(iVar17, 8) as *mut i32;
            if !local_d8.is_null() {
                match fVar10 {
                    1 => {
                        iVar6 = 0;
                        fVar10 = 0;
                        pfVar9 = pa(param_4, 1);
                        s(pfVar3, 4, 0);
                        iVar11 = rd(iVar17, 4);
                        if rd(iVar17, 0x28) == iVar11 {
                            local_dc = rd(iVar17, 0x2c);
                        } else {
                            local_dc = 0;
                        }
                        iVar16 = 0;
                        if rd(iVar17, 0x1c) == iVar11 {
                            iVar16 = rd(iVar17, 0x20);
                        }
                        pfVar15 = pfVar9;
                        if 0 < iVar11 {
                            loop {
                                iVar11 = iVar6.wrapping_add(1);
                                loop {
                                    iVar18 = iVar11;
                                    iVar11 = iVar18.wrapping_add(2);
                                    s(pfVar15, 0, g(local_d8, iVar6));
                                    s(pfVar15, 1, g(local_d8, iVar18));
                                    s(pfVar15, 2, g(local_d8, iVar18.wrapping_add(1)));
                                    s(pfVar15, 3, g(local_d8, iVar11));
                                    if local_dc == 0 {
                                        s(pfVar15, 8, g(pfVar15, 0));
                                        s(pfVar15, 9, g(pfVar15, 1));
                                        s(pfVar15, 10, g(pfVar15, 2));
                                        s(pfVar15, 11, g(pfVar15, 3));
                                    } else {
                                        let l = local_dc as *const i32;
                                        s(pfVar15, 8, g(l, iVar6));
                                        s(pfVar15, 9, g(l, iVar11.wrapping_sub(2)));
                                        s(pfVar15, 10, g(l, iVar11.wrapping_sub(1)));
                                        s(pfVar15, 11, g(l, iVar11));
                                    }
                                    if iVar16 == 0 {
                                        s(pfVar15, 12, g(pfVar15, 0));
                                        s(pfVar15, 13, g(pfVar15, 1));
                                        s(pfVar15, 14, g(pfVar15, 2));
                                        s(pfVar15, 15, g(pfVar15, 3));
                                    } else {
                                        let l = iVar16 as *const i32;
                                        s(pfVar15, 12, g(l, iVar6));
                                        s(pfVar15, 13, g(l, iVar11.wrapping_sub(2)));
                                        s(pfVar15, 14, g(l, iVar11.wrapping_sub(1)));
                                        s(pfVar15, 15, g(l, iVar11));
                                    }
                                    fVar12 =
                                        FUN_0048d980(pfVar3 as i32, g(pfVar15, 0), g(pfVar15, 1));
                                    s(pfVar15, 4, fVar12);
                                    if fVar12 == -1 {
                                        s(pfVar15, 4, fVar10);
                                        fVar10 = fVar10.wrapping_add(1);
                                    }
                                    fVar12 =
                                        FUN_0048d980(pfVar3 as i32, g(pfVar15, 1), g(pfVar15, 2));
                                    s(pfVar15, 5, fVar12);
                                    if fVar12 == -1 {
                                        s(pfVar15, 5, fVar10);
                                        fVar10 = fVar10.wrapping_add(1);
                                    }
                                    if g(pfVar15, 3) == -1 {
                                        fVar12 = FUN_0048d980(
                                            pfVar3 as i32,
                                            g(pfVar15, 2),
                                            g(pfVar15, 0),
                                        );
                                        s(pfVar15, 6, fVar12);
                                        if fVar12 == -1 {
                                            s(pfVar15, 6, fVar10);
                                            fVar10 = fVar10.wrapping_add(1);
                                        }
                                        s(pfVar15, 7, -1);
                                    } else {
                                        fVar12 = FUN_0048d980(
                                            pfVar3 as i32,
                                            g(pfVar15, 2),
                                            g(pfVar15, 3),
                                        );
                                        s(pfVar15, 6, fVar12);
                                        if fVar12 == -1 {
                                            s(pfVar15, 6, fVar10);
                                            fVar10 = fVar10.wrapping_add(1);
                                        }
                                        fVar12 = FUN_0048d980(
                                            pfVar3 as i32,
                                            g(pfVar15, 3),
                                            g(pfVar15, 0),
                                        );
                                        s(pfVar15, 7, fVar12);
                                        if fVar12 == -1 {
                                            s(pfVar15, 7, fVar10);
                                            fVar10 = fVar10.wrapping_add(1);
                                        }
                                    }
                                    pfVar15 = pa(pfVar15, 0x10);
                                    s(pfVar3, 4, g(pfVar3, 4).wrapping_add(1));
                                    if g(local_d8, iVar11) == -1 {
                                        iVar6 = iVar18.wrapping_add(3);
                                        break;
                                    }
                                    if g(local_d8, iVar18.wrapping_add(3)) == -1 {
                                        iVar6 = iVar18.wrapping_add(4);
                                        break;
                                    }
                                }
                                if !(iVar6 < rd(iVar17, 4)) {
                                    break;
                                }
                            }
                        }
                        pfVar9 = pa(pfVar9, g(pfVar3, 4).wrapping_mul(0x10));
                        FUN_0048dba0(pfVar9, g(pfVar3, 5), g(pfVar3, 4), param_3);
                        fVar12 = g(pfVar3, 4);
                        iVar17 = DAT_0060268c.wrapping_mul(3);
                        s(g(pfVar3, 5) as *mut i32, 0, fVar10);
                        pfVar9 = pa(pfVar9, fVar12.wrapping_mul(9));
                        pfVar9 = pa(pfVar9, iVar17);
                        if DAT_006a8884 < fVar10 {
                            DAT_006a8884 = fVar10;
                        }
                    }
                    2 => {
                        let fp4 = param_4 as *mut f32;
                        let mut fp15: *mut f32;
                        pfVar9 = pa(param_4, 0xc);
                        fp15 = rd(iVar17, 0x38) as *mut f32;
                        if !fp15.is_null() {
                            let fF4 = -gf(fp15, 0);
                            sf(fp4, 0, fF4);
                            sf(fp4, 1, -gf(fp15, 1));
                            let fF10 = gf(fp4, 1);
                            sf(fp4, 2, -gf(fp15, 2));
                            let fF12 = gf(fp4, 2);
                            if !(rd(iVar17, 0x2c) as *mut f32).is_null() {
                                fp15 = fp4.wrapping_add(3);
                                FUN_00415300(fp15, rd(iVar17, 0x2c) as *mut f32);
                                FUN_00414fe0(fp4, fp15);
                                sf(fp4, 0, gf(fp4, 0) - fF4);
                                sf(fp4, 1, gf(fp4, 1) - fF10);
                                sf(fp4, 2, gf(fp4, 2) - fF12);
                                let fpS = rd(iVar17, 0x20) as *mut f32;
                                if !fpS.is_null() {
                                    sf(fp4, 3, gf(fp4, 3) * gf(fpS, 0));
                                    sf(fp4, 4, gf(fpS, 1) * gf(fp4, 4));
                                    sf(fp4, 5, gf(fp4, 5) * gf(fpS, 2));
                                    sf(fp4, 6, gf(fp4, 6) * gf(fpS, 0));
                                    sf(fp4, 7, gf(fp4, 7) * gf(fpS, 1));
                                    sf(fp4, 8, gf(fp4, 8) * gf(fpS, 2));
                                    sf(fp4, 9, gf(fp4, 9) * gf(fpS, 0));
                                    sf(fp4, 10, gf(fp4, 10) * gf(fpS, 1));
                                    sf(fp4, 11, gf(fp4, 11) * gf(fpS, 2));
                                    sf(fp4, 0, gf(fp4, 0) * gf(fpS, 0));
                                    sf(fp4, 1, gf(fp4, 1) * gf(fpS, 1));
                                    sf(fp4, 2, gf(fp4, 2) * gf(fpS, 2));
                                    let fpS2 = rd(iVar17, 0x14) as *mut f32;
                                    if !fpS2.is_null() {
                                        sf(fp4, 0, gf(fp4, 0) + fF4);
                                        sf(fp4, 1, gf(fp4, 1) + fF10);
                                        sf(fp4, 2, gf(fp4, 2) + fF12);
                                        FUN_00415300(fp15, fpS2);
                                        FUN_00414fe0(fp4, fp15);
                                        sf(fp4, 0, gf(fp4, 0) - fF4);
                                        sf(fp4, 1, gf(fp4, 1) - fF10);
                                        sf(fp4, 2, gf(fp4, 2) - fF12);
                                        fp15 = rd(iVar17, 8) as *mut f32;
                                        if !fp15.is_null() {
                                            sf(fp4, 0, gf(fp4, 0) - gf(fp15, 0));
                                            sf(fp4, 1, gf(fp4, 1) - gf(fp15, 1));
                                            sf(fp4, 2, gf(fp4, 2) - gf(fp15, 2));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    3 => {
                        s(pfVar3, 4, rd(iVar17, 4));
                        cpyw(
                            param_4,
                            rd(iVar17, 8) as *const i32,
                            (rd(iVar17, 4).wrapping_mul(0xc) as u32) >> 2,
                        );
                        DAT_00602678 = param_4 as *mut f32;
                        DAT_006026bc = rd(iVar17, 4);
                        pfVar9 = pa(param_4, DAT_006026bc.wrapping_mul(3));
                        let tp = (&raw mut DAT_006a887a).cast::<i32>();
                        let mut tmp = tp.read_unaligned();
                        if tmp < g(pfVar3, 4) {
                            tmp = g(pfVar3, 4);
                            tp.write_unaligned(tmp);
                        }
                    }
                    4 | 6 => {
                        s(param_4, 0, g(local_d8, 0));
                        s(param_4, 1, g(local_d8, 1));
                        s(param_4, 2, g(local_d8, 2));
                        pfVar9 = pa(param_4, 3);
                    }
                    5 => {
                        FUN_00415300(param_4 as *mut f32, local_d8 as *mut f32);
                        pfVar9 = pa(param_4, 9);
                    }
                    7 => {
                        if iVar17 != 0 {
                            s(pfVar3, 4, 1);
                            s(pfVar3, 5, param_4 as i32);
                            uVar7 = slen(rd(iVar17, 8) as *const i8);
                            cpys(param_4, rd(iVar17, 8) as *const i8, uVar7);
                            uVar7 = slen(rd(iVar17, 8) as *const i8);
                            pfVar9 = param_4
                                .cast::<u8>()
                                .wrapping_add(uVar7 as usize)
                                .cast::<i32>();
                        }
                    }
                    9 => {
                        fVar10 = rd(iVar17, 4);
                        if rd(iVar17, 4) < rd(iVar17, 0x10) {
                            fVar10 = rd(iVar17, 0x10);
                        }
                        if fVar10 < rd(iVar17, 0x1c) {
                            fVar10 = rd(iVar17, 0x1c);
                        }
                        if fVar10 < rd(iVar17, 0x28) {
                            fVar10 = rd(iVar17, 0x28);
                        }
                        if fVar10 < rd(iVar17, 0x34) {
                            fVar10 = rd(iVar17, 0x34);
                        }
                        if fVar10 < rd(iVar17, 0x40) {
                            fVar10 = rd(iVar17, 0x40);
                        }
                        s(pfVar3, 4, fVar10);
                        param_4 = sec3(param_4, rd(iVar17, 8), rd(iVar17, 4), fVar10);
                        'a: {
                            if rd(iVar17, 0x14) == 0 {
                                break 'a;
                            }
                            param_4 = sec3(param_4, rd(iVar17, 0x14), rd(iVar17, 0x10), fVar10);
                            if rd(iVar17, 0x20) == 0 {
                                break 'a;
                            }
                            param_4 = sec3(param_4, rd(iVar17, 0x20), rd(iVar17, 0x1c), fVar10);
                            if rd(iVar17, 0x2c) == 0 {
                                break 'a;
                            }
                            param_4 = sec3(param_4, rd(iVar17, 0x2c), rd(iVar17, 0x28), fVar10);
                            if rd(iVar17, 0x38) == 0 {
                                break 'a;
                            }
                            param_4 = sec1(param_4, rd(iVar17, 0x38), rd(iVar17, 0x34), fVar10);
                            if rd(iVar17, 0x44) == 0 {
                                break 'a;
                            }
                            param_4 = sec1(param_4, rd(iVar17, 0x44), rd(iVar17, 0x40), fVar10);
                        }
                        pfVar9 = param_4;
                    }
                    10 | 14 => {
                        s(pfVar3, 4, g(local_d8, 0));
                    }
                    11 => {
                        s(pfVar3, 4, rd(iVar17, 4));
                        cpyw(
                            param_4,
                            rd(iVar17, 8) as *const i32,
                            (rd(iVar17, 4).wrapping_mul(0xc) as u32) >> 2,
                        );
                        DAT_006026c0 = param_4 as *mut f32;
                        s(param_3.wrapping_add(0x84) as *mut i32, 0, param_4 as i32);
                        pfVar9 = pa(param_4, rd(iVar17, 4).wrapping_mul(3));
                    }
                    13 => {
                        s(pfVar3, 4, rd(iVar17, 4));
                        iVar11 = (((rd(iVar17, 4) as u32) & 0x1fff_ffff) << 1) as i32;
                        cpyw(param_4, rd(iVar17, 8) as *const i32, iVar11 as u32);
                        DAT_006026a0 = param_4 as *mut f32;
                        pfVar9 = pa(param_4, (rd(iVar17, 4) as u32).wrapping_mul(2) as i32);
                    }
                    15 => {
                        fVar10 = g(local_d8, 0);
                        pfVar9 = pa(param_4, 1);
                        iVar11 = 0;
                        if rd(iVar17, 0x14) != 0 {
                            iVar6 = rd(rd(iVar17, 0x14), 0);
                            if rd(iVar17, 0x20) != 0 {
                                iVar17 = rd(rd(iVar17, 0x20), 0);
                                fVar12 = 0;
                                if iVar17 != 1 && -1 < iVar17.wrapping_sub(1) {
                                    local_dc = 0;
                                    pfVar15 = pfVar9;
                                    local_e0 = iVar6;
                                    loop {
                                        iVar16 = 0;
                                        if iVar6 != 1 && -1 < iVar6.wrapping_sub(1) {
                                            loop {
                                                fVar4 = fVar10
                                                    .wrapping_add(local_dc)
                                                    .wrapping_add(iVar16);
                                                s(pfVar15, 0, fVar4);
                                                s(pfVar15, 1, fVar4.wrapping_add(1));
                                                fVar4 = fVar10
                                                    .wrapping_add(local_e0)
                                                    .wrapping_add(iVar16);
                                                s(pfVar15, 2, fVar4.wrapping_add(1));
                                                s(pfVar15, 3, fVar4);
                                                s(pfVar15, 8, g(pfVar15, 0));
                                                s(pfVar15, 9, g(pfVar15, 1));
                                                s(pfVar15, 10, g(pfVar15, 2));
                                                s(pfVar15, 11, g(pfVar15, 3));
                                                s(pfVar15, 12, g(pfVar15, 0));
                                                s(pfVar15, 13, g(pfVar15, 1));
                                                s(pfVar15, 14, g(pfVar15, 2));
                                                s(pfVar15, 15, g(pfVar15, 3));
                                                if iVar11 == 0 {
                                                    s(pfVar15, 4, fVar12);
                                                    fVar4 = fVar12.wrapping_add(1);
                                                } else {
                                                    fVar4 = fVar12;
                                                    if iVar11 == 1 {
                                                        s(
                                                            pfVar15,
                                                            4,
                                                            iVar16
                                                                .wrapping_add(
                                                                    iVar6.wrapping_mul(-3),
                                                                )
                                                                .wrapping_add(fVar12)
                                                                .wrapping_add(4),
                                                        );
                                                    } else {
                                                        s(
                                                            pfVar15,
                                                            4,
                                                            fVar12.wrapping_add(
                                                                iVar6.wrapping_mul(-2),
                                                            ),
                                                        );
                                                    }
                                                }
                                                s(pfVar15, 5, fVar4);
                                                s(pfVar15, 6, fVar4.wrapping_add(1));
                                                fVar12 = fVar4.wrapping_add(2);
                                                if iVar16 == 0 {
                                                    s(pfVar15, 7, fVar12);
                                                    fVar12 = fVar4.wrapping_add(3);
                                                } else {
                                                    s(pfVar15, 7, fVar4.wrapping_sub(2));
                                                    if iVar11 == 0 {
                                                        s(pfVar15, 7, fVar4.wrapping_sub(3));
                                                    }
                                                    if iVar16 == 1 {
                                                        s(
                                                            pfVar15,
                                                            7,
                                                            g(pfVar15, 7).wrapping_sub(1),
                                                        );
                                                    }
                                                }
                                                pfVar15 = pa(pfVar15, 0x10);
                                                iVar16 = iVar16.wrapping_add(1);
                                                if !(iVar16 < iVar6.wrapping_sub(1)) {
                                                    break;
                                                }
                                            }
                                        }
                                        local_e0 = local_e0.wrapping_add(iVar6);
                                        local_dc = local_dc.wrapping_add(iVar6);
                                        iVar11 = iVar11.wrapping_add(1);
                                        if !(iVar11 < iVar17.wrapping_sub(1)) {
                                            break;
                                        }
                                    }
                                }
                                fVar10 = iVar6.wrapping_sub(1).wrapping_mul(iVar17.wrapping_sub(1));
                                s(pfVar3, 4, fVar10);
                                pfVar9 = pa(pfVar9, fVar10.wrapping_mul(0x10));
                                FUN_0048dba0(pfVar9, g(pfVar3, 5), fVar10, param_3);
                                fVar10 = g(pfVar3, 4);
                                iVar17 = DAT_0060268c.wrapping_mul(3);
                                s(g(pfVar3, 5) as *mut i32, 0, fVar12);
                                pfVar9 = pa(pfVar9, fVar10.wrapping_mul(9));
                                pfVar9 = pa(pfVar9, iVar17);
                                if DAT_006a8884 < fVar12 {
                                    DAT_006a8884 = fVar12;
                                }
                            }
                        }
                    }
                    16 => {
                        fVar10 = 0;
                        pfVar15 = pa(param_4, 1);
                        fVar12 = 0;
                        local_e0 = 0;
                        s(pfVar3, 4, 0);
                        pfVar9 = pfVar15;
                        if 0 < rd(iVar17, 4) {
                            loop {
                                fVar4 = g(local_d8, 0);
                                fVar14 = fVar12;
                                if 2 < fVar4 {
                                    iVar11 = fVar12.wrapping_add(1).wrapping_sub(fVar12);
                                    fVar14 = fVar12.wrapping_add(1);
                                    fVar23 = fVar10.wrapping_add(1);
                                    fVar24 = fVar10;
                                    loop {
                                        fVar10 = fVar23;
                                        if !(iVar11 < fVar4) {
                                            break;
                                        }
                                        s(pfVar3, 4, g(pfVar3, 4).wrapping_add(1));
                                        s(pfVar9, 0, fVar12);
                                        s(pfVar9, 1, fVar14);
                                        s(pfVar9, 2, fVar14.wrapping_add(1));
                                        fVar13 = fVar14.wrapping_add(2);
                                        if fVar13.wrapping_sub(fVar4) == fVar12 {
                                            s(pfVar9, 3, -1);
                                            s(pfVar9, 4, fVar24);
                                            s(pfVar9, 5, fVar10);
                                            s(pfVar9, 6, fVar10.wrapping_add(1));
                                            fVar23 = fVar10.wrapping_add(2);
                                            s(pfVar9, 7, -1);
                                        } else {
                                            s(pfVar9, 3, fVar13);
                                            fVar13 = fVar14.wrapping_add(3);
                                            if fVar13.wrapping_sub(fVar4) != fVar12 {
                                                fVar13 = fVar14.wrapping_add(2);
                                            }
                                            s(pfVar9, 4, fVar24);
                                            s(pfVar9, 5, fVar10);
                                            s(pfVar9, 6, fVar10.wrapping_add(1));
                                            fVar24 = fVar10.wrapping_add(2);
                                            s(pfVar9, 7, fVar24);
                                            fVar23 = fVar10.wrapping_add(3);
                                        }
                                        s(pfVar9, 8, g(pfVar9, 0));
                                        s(pfVar9, 9, g(pfVar9, 1));
                                        s(pfVar9, 10, g(pfVar9, 2));
                                        s(pfVar9, 11, g(pfVar9, 3));
                                        s(pfVar9, 12, g(pfVar9, 0));
                                        s(pfVar9, 13, g(pfVar9, 1));
                                        s(pfVar9, 14, g(pfVar9, 2));
                                        s(pfVar9, 15, g(pfVar9, 3));
                                        pfVar9 = pa(pfVar9, 0x10);
                                        fVar14 = fVar13;
                                        iVar11 = fVar13.wrapping_sub(fVar12);
                                    }
                                }
                                local_d8 = pa(local_d8, 1);
                                local_e0 = local_e0.wrapping_add(1);
                                fVar12 = fVar14;
                                if !(local_e0 < rd(iVar17, 4)) {
                                    break;
                                }
                            }
                        }
                        pfVar15 = pa(pfVar15, g(pfVar3, 4).wrapping_mul(0x10));
                        FUN_0048dba0(pfVar15, g(pfVar3, 5), g(pfVar3, 4), param_3);
                        fVar12 = g(pfVar3, 4);
                        iVar17 = DAT_0060268c.wrapping_mul(3);
                        s(g(pfVar3, 5) as *mut i32, 0, fVar10);
                        pfVar9 = pa(pfVar15, fVar12.wrapping_mul(9));
                        pfVar9 = pa(pfVar9, iVar17);
                        if DAT_006a8884 < fVar10 {
                            DAT_006a8884 = fVar10;
                        }
                    }
                    17 => {
                        fVar10 = g(local_d8, 0);
                        pfVar9 = pa(param_4, 1);
                        piVar19 = rd(iVar17, 0x14) as *mut i32;
                        if !piVar19.is_null() {
                            local_e0 = 0;
                            s(pfVar3, 4, 0);
                            pfVar15 = pfVar9;
                            fVar12 = local_d0[0];
                            if 0 < rd(iVar17, 0x10) {
                                loop {
                                    uVar7 = 2;
                                    if 2 < g(piVar19, 0) {
                                        loop {
                                            fVar12 = fVar10.wrapping_add(uVar7 as i32);
                                            if (uVar7 & 1) == 0 {
                                                s(pfVar15, 0, fVar12.wrapping_sub(2));
                                                s(pfVar15, 1, fVar12.wrapping_sub(1));
                                                s(pfVar15, 2, fVar12);
                                                s(pfVar15, 3, -1);
                                                if uVar7 == 2 {
                                                    s(pfVar15, 4, local_d0[0]);
                                                    local_d0[0] = local_d0[0].wrapping_add(1);
                                                } else {
                                                    s(pfVar15, 4, local_d0[0].wrapping_sub(2));
                                                }
                                                s(pfVar15, 5, local_d0[0]);
                                            } else {
                                                s(pfVar15, 0, fVar12);
                                                s(pfVar15, 1, fVar12.wrapping_sub(1));
                                                s(pfVar15, 2, fVar12.wrapping_sub(2));
                                                s(pfVar15, 3, -1);
                                                s(pfVar15, 4, local_d0[0]);
                                                s(pfVar15, 5, local_d0[0].wrapping_sub(2));
                                            }
                                            s(pfVar15, 6, local_d0[0].wrapping_add(1));
                                            local_d0[0] = local_d0[0].wrapping_add(2);
                                            uVar7 = uVar7.wrapping_add(1);
                                            s(pfVar15, 7, -1);
                                            s(pfVar15, 8, g(pfVar15, 0));
                                            s(pfVar15, 9, g(pfVar15, 1));
                                            s(pfVar15, 10, g(pfVar15, 2));
                                            s(pfVar15, 11, g(pfVar15, 3));
                                            s(pfVar15, 12, g(pfVar15, 0));
                                            s(pfVar15, 13, g(pfVar15, 1));
                                            s(pfVar15, 14, g(pfVar15, 2));
                                            s(pfVar15, 15, g(pfVar15, 3));
                                            pfVar15 = pa(pfVar15, 0x10);
                                            s(pfVar3, 4, g(pfVar3, 4).wrapping_add(1));
                                            if !((uVar7 as i32) < g(piVar19, 0)) {
                                                break;
                                            }
                                        }
                                    }
                                    iVar11 = g(piVar19, 0);
                                    piVar19 = pa(piVar19, 1);
                                    fVar10 = fVar10.wrapping_add(iVar11);
                                    local_e0 = local_e0.wrapping_add(1);
                                    fVar12 = local_d0[0];
                                    if !(local_e0 < rd(iVar17, 0x10)) {
                                        break;
                                    }
                                }
                            }
                            pfVar9 = pa(pfVar9, g(pfVar3, 4).wrapping_mul(0x10));
                            FUN_0048dba0(pfVar9, g(pfVar3, 5), g(pfVar3, 4), param_3);
                            fVar10 = g(pfVar3, 4);
                            iVar17 = DAT_0060268c.wrapping_mul(3);
                            s(g(pfVar3, 5) as *mut i32, 0, fVar12);
                            pfVar9 = pa(pfVar9, fVar10.wrapping_mul(9));
                            pfVar9 = pa(pfVar9, iVar17);
                            if DAT_006a8884 < fVar12 {
                                DAT_006a8884 = fVar12;
                            }
                        }
                    }
                    20 => {
                        s(pfVar3, 5, param_4 as i32);
                        let pbVar5 = FUN_0048e580(param_4, rd(iVar17, 8) as *mut i8);
                        pfVar9 = param_4
                            .cast::<u8>()
                            .wrapping_add(pbVar5 as usize)
                            .cast::<i32>();
                    }
                    21 => {
                        iVar11 = 0;
                        iVar6 = 0;
                        s(pfVar3, 4, 0);
                        if 0 < rd(iVar17, 4) {
                            loop {
                                if rd(g(param_2, 3), iVar11) != 0
                                    && (iVar11 == 0 || g(local_d8, -1) != g(local_d8, 0))
                                {
                                    s(param_4, 0, g(local_d8, 0));
                                    s(
                                        g(param_2, 3).wrapping_add(g(pfVar3, 4).wrapping_mul(4))
                                            as *mut i32,
                                        0,
                                        rd(iVar11, g(param_2, 3)),
                                    );
                                    s(pfVar3, 4, g(pfVar3, 4).wrapping_add(1));
                                    param_4 = pa(param_4, 1);
                                }
                                iVar6 = iVar6.wrapping_add(1);
                                local_d8 = pa(local_d8, 1);
                                iVar11 = iVar11.wrapping_add(4);
                                if !(iVar6 < rd(iVar17, 4)) {
                                    break;
                                }
                            }
                        }
                        s(param_2, 2, g(pfVar3, 4));
                        pfVar9 = param_4;
                    }
                    22 => {
                        s(param_4, 0, g(local_d8, 0));
                        pfVar9 = rd(iVar17, 0x14) as *mut i32;
                        s(param_4, 1, g(pfVar9, 0));
                        s(param_4, 2, g(pfVar9, 1));
                        s(param_4, 3, g(pfVar9, 2));
                        pfVar9 = pa(param_4, 4);
                    }
                    23 => {
                        s(param_4, 0, g(local_d8, 0));
                        s(param_4, 1, g(local_d8, 1));
                        s(param_4, 2, g(local_d8, 2));
                        pfVar9 = rd(iVar17, 0x14) as *mut i32;
                        s(param_4, 3, g(pfVar9, 0));
                        s(param_4, 4, g(pfVar9, 1));
                        s(param_4, 5, g(pfVar9, 2));
                        pfVar9 = rd(iVar17, 0x20) as *mut i32;
                        s(param_4, 6, g(pfVar9, 0));
                        s(param_4, 7, g(pfVar9, 1));
                        s(param_4, 8, g(pfVar9, 2));
                        pfVar9 = rd(iVar17, 0x2c) as *mut i32;
                        s(param_4, 9, g(pfVar9, 0));
                        s(param_4, 10, g(pfVar9, 1));
                        s(param_4, 11, g(pfVar9, 2));
                        pfVar9 = pa(param_4, 0xc);
                    }
                    25 => {
                        s(param_4, 0, g(local_d8, 0));
                        s(param_4, 1, g(rd(iVar17, 0x14) as *mut i32, 0));
                        pfVar9 = rd(iVar17, 0x20) as *mut i32;
                        s(param_4, 2, g(pfVar9, 0));
                        s(param_4, 3, g(pfVar9, 1));
                        s(param_4, 4, g(pfVar9, 2));
                        pfVar9 = rd(iVar17, 0x2c) as *mut i32;
                        s(param_4, 5, g(pfVar9, 0));
                        s(param_4, 6, g(pfVar9, 1));
                        s(param_4, 7, g(pfVar9, 2));
                        pfVar9 = rd(iVar17, 0x38) as *mut i32;
                        s(param_4, 8, g(pfVar9, 0));
                        s(param_4, 9, g(pfVar9, 1));
                        s(param_4, 10, g(pfVar9, 2));
                        pfVar9 = rd(iVar17, 0x44) as *mut i32;
                        s(param_4, 11, g(pfVar9, 0));
                        s(param_4, 12, g(pfVar9, 1));
                        s(param_4, 13, g(pfVar9, 2));
                        s(param_4, 14, g(rd(iVar17, 0x50) as *mut i32, 0));
                        pfVar9 = rd(iVar17, 0x5c) as *mut i32;
                        s(param_4, 15, g(pfVar9, 0));
                        s(param_4, 16, g(pfVar9, 1));
                        s(param_4, 17, g(pfVar9, 2));
                        pfVar9 = pa(param_4, 0x12);
                    }
                    _ => {}
                }
            }
        }
        param_4 = pfVar9;
        iVar17 = 0;
        if g(param_2, 2) != 0 {
            iVar11 = 0x34;
            let mut k: usize = 0;
            while iVar11 != 0 {
                local_d0[k] = rd(param_3, 0);
                param_3 = param_3.wrapping_add(4);
                k += 1;
                iVar11 = iVar11.wrapping_sub(1);
            }
            DAT_006026c0 = core::ptr::null_mut();
            DAT_006026a8 = 0;
            DAT_00602690 = 0;
            iVar11 = 0;
            s(pfVar3, 2, g(param_2, 2));
            s(pfVar3, 3, param_4 as i32);
            param_4 = pa(param_4, g(param_2, 2));
            if 0 < g(param_2, 2) {
                loop {
                    iVar17 = iVar17.wrapping_add(4);
                    iVar11 = iVar11.wrapping_add(1);
                    s(
                        g(pfVar3, 3).wrapping_sub(4).wrapping_add(iVar17) as *mut i32,
                        0,
                        param_4 as i32,
                    );
                    iVar6 = FUN_0048c240(
                        param_1,
                        rd(g(param_2, 3).wrapping_sub(4), iVar17) as *mut i32,
                        local_d0.as_mut_ptr() as i32,
                        param_4,
                        param_5,
                        param_6,
                    );
                    param_4 = param_4
                        .cast::<u8>()
                        .wrapping_offset(iVar6 as isize)
                        .cast::<i32>();
                    if !(iVar11 < g(param_2, 2)) {
                        break;
                    }
                }
            }
        }
        (param_4 as i32).wrapping_sub(pfVar3 as i32)
    }
}
