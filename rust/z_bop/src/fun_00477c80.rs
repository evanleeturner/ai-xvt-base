// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0060f1c0: *mut i32;
    static mut DAT_0060f1d8: i32;
    static mut DAT_0060f1e8: *mut i32;
    static mut DAT_0060f1f0: i32;
    static mut DAT_0060f204: i32;
    static mut DAT_0060f208: *mut i32;
    static mut DAT_0099947e: i32;
    static mut DAT_00999488: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004792f0(a: i32, b: i32, c: i32) -> i32;
    fn FUN_004794c0(a: i32, b: *mut i32, c: i32) -> *mut i32;
    fn FUN_00421300(a: *mut f32, b: *mut f32);
    fn FUN_00420fe0(a: *mut f32, b: *mut f32);
    fn FUN_00479e80(a: *mut i32, b: *mut i8) -> *mut u8;
}

fn ptr(a: i32) -> *mut i32 {
    a as u32 as usize as *mut i32
}

fn pa(p: *mut i32, n: u32) -> *mut i32 {
    p.wrapping_add(n as usize)
}

unsafe fn rd(a: i32) -> i32 {
    // SAFETY: the address is a game-memory address the original reads.
    unsafe { ptr(a).read_unaligned() }
}

unsafe fn wr(a: i32, v: i32) {
    // SAFETY: the address is a game-memory address the original writes.
    unsafe { ptr(a).write_unaligned(v) }
}

unsafe fn g(p: *mut i32, i: i32) -> i32 {
    // SAFETY: the original reads this element of the caller's buffer.
    unsafe { p.wrapping_offset(i as isize).read_unaligned() }
}

unsafe fn s(p: *mut i32, i: i32, v: i32) {
    // SAFETY: the original writes this element of the caller's buffer.
    unsafe { p.wrapping_offset(i as isize).write_unaligned(v) }
}

unsafe fn fg(p: *mut i32, i: i32) -> f32 {
    // SAFETY: same element read as `g`.
    unsafe { f32::from_bits(g(p, i) as u32) }
}

unsafe fn fs(p: *mut i32, i: i32, v: f32) {
    // SAFETY: same element write as `s`.
    unsafe { s(p, i, v.to_bits() as i32) }
}

unsafe fn cp(mut dst: *mut i32, mut src: *mut i32, mut n: u32) {
    while n != 0 {
        // SAFETY: the original copies these dwords between its buffers.
        unsafe { s(dst, 0, g(src, 0)) };
        src = src.wrapping_add(1);
        dst = dst.wrapping_add(1);
        n = n.wrapping_sub(1);
    }
}

unsafe fn strcopy(src: *const i8, dst: *mut u8) -> u32 {
    let mut n: u32 = 0xffff_ffff;
    let mut p = src;
    loop {
        if n == 0 {
            break;
        }
        n = n.wrapping_sub(1);
        // SAFETY: the original scans this NUL-terminated string.
        let c = unsafe { *p };
        p = p.wrapping_add(1);
        if c == 0 {
            break;
        }
    }
    let len = !n;
    let mut sp = src as *const u8;
    let mut dp = dst;
    let mut k = len >> 2;
    while k != 0 {
        // SAFETY: the original copies the string to the output buffer.
        unsafe { (dp as *mut u32).write_unaligned((sp as *const u32).read_unaligned()) };
        sp = sp.wrapping_add(4);
        dp = dp.wrapping_add(4);
        k -= 1;
    }
    let mut k = len & 3;
    while k != 0 {
        // SAFETY: as above, the trailing bytes.
        unsafe { *dp = *sp };
        sp = sp.wrapping_add(1);
        dp = dp.wrapping_add(1);
        k -= 1;
    }
    len
}

unsafe fn edge(pf4: *mut i32, p5: *mut i32, slot: i32, a: i32, b: i32, fv13: &mut i32) {
    // SAFETY: the callee is the game's own; p5 is the output buffer.
    unsafe {
        let v = FUN_004792f0(pf4 as i32, a, b);
        s(p5, slot, v);
        if v == -1 {
            s(p5, slot, *fv13);
            *fv13 = fv13.wrapping_add(1);
        }
    }
}

unsafe fn blk3(iv16: i32, co: i32, dof: i32, fv13: i32, p4: *mut i32) -> *mut i32 {
    // SAFETY: reads and writes the original's descriptor and output buffers.
    unsafe {
        let n = (rd(iv16.wrapping_add(co)) as u32).wrapping_mul(0xc) >> 2;
        cp(p4, ptr(rd(iv16.wrapping_add(dof))), n);
        let iv14 = rd(iv16.wrapping_add(co));
        let mut p5 = pa(p4, (iv14 as u32).wrapping_mul(3));
        let mut out = p5;
        if iv14 < fv13 {
            let p15 = ptr(rd(iv16.wrapping_add(dof))
                .wrapping_sub(0xc)
                .wrapping_add(iv14.wrapping_mul(0xc)));
            let mut iv7 = fv13.wrapping_sub(iv14);
            out = pa(p5, (iv7 as u32).wrapping_mul(3));
            loop {
                s(p5, 0, g(p15, 0));
                iv7 = iv7.wrapping_sub(1);
                s(p5, 1, g(p15, 1));
                s(p5, 2, g(p15, 2));
                p5 = p5.wrapping_add(3);
                if iv7 == 0 {
                    break;
                }
            }
        }
        out
    }
}

unsafe fn blk1(iv16: i32, co: i32, dof: i32, fv13: i32, p4: *mut i32) -> *mut i32 {
    // SAFETY: reads and writes the original's descriptor and output buffers.
    unsafe {
        let n = (rd(iv16.wrapping_add(co)) as u32) & 0x3fff_ffff;
        cp(p4, ptr(rd(iv16.wrapping_add(dof))), n);
        let iv14 = rd(iv16.wrapping_add(co));
        let mut p5 = pa(p4, iv14 as u32);
        let mut out = p5;
        if iv14 < fv13 {
            let iv7 = rd(iv16.wrapping_add(dof));
            let mut iv23 = fv13.wrapping_sub(iv14);
            out = pa(p5, iv23 as u32);
            loop {
                iv23 = iv23.wrapping_sub(1);
                s(
                    p5,
                    0,
                    rd(iv7.wrapping_sub(4).wrapping_add(iv14.wrapping_mul(4))),
                );
                p5 = p5.wrapping_add(1);
                if iv23 == 0 {
                    break;
                }
            }
        }
        out
    }
}

unsafe fn copy_quad(p: *mut i32) {
    // SAFETY: duplicates the first four dwords into slots 8..0xf.
    unsafe {
        s(p, 8, g(p, 0));
        s(p, 9, g(p, 1));
        s(p, 10, g(p, 2));
        s(p, 0xb, g(p, 3));
        s(p, 0xc, g(p, 0));
        s(p, 0xd, g(p, 1));
        s(p, 0xe, g(p, 2));
        s(p, 0xf, g(p, 3));
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00477c80(
    param_1: u32,
    param_2: *mut i32,
    param_3: i32,
    param_4: *mut i32,
    param_5: *mut u32,
    param_6: *mut u8,
) -> i32 {
    let _ = (param_5, param_6);
    let mut param_3 = param_3;
    let mut param_4 = param_4;
    let unaff_ebp: *mut u32 = core::ptr::null_mut();
    let unaff_edi: *mut u8 = core::ptr::null_mut();
    let mut local_d0: [i32; 52] = [0; 52];
    if param_2.is_null() {
        return 0;
    }
    // SAFETY: all pointers and globals belong to the game; the function reads
    // and writes its buffers exactly where the original does.
    unsafe {
        let pf4 = param_4;
        let pf5 = pa(param_4, 6);
        let mut fv13 = g(param_2, 1);
        s(param_4, 1, fv13);
        if g(param_2, 0) == 0 {
            s(param_4, 0, 0);
            param_4 = pf5;
        } else {
            s(param_4, 0, pf5 as i32);
            let len = strcopy(ptr(g(param_2, 0)) as *const i8, pf5 as *mut u8);
            param_4 = (pf5 as u32).wrapping_add(len) as usize as *mut i32;
        }
        s(pf4, 4, 1);
        s(pf4, 5, param_4 as i32);
        s(pf4, 2, 0);
        s(pf4, 3, 0);
        let mut iv16 = g(param_2, 5);
        let mut p5 = param_4;
        if iv16 != 0 && g(param_2, 4) != 0 {
            let mut ld8 = ptr(rd(iv16.wrapping_add(8)));
            if !ld8.is_null() {
                let sw = fv13;
                match sw {
                    1 => {
                        let mut iv7: i32 = 0;
                        fv13 = 0;
                        s(pf4, 4, 0);
                        let mut iv14 = rd(iv16.wrapping_add(4));
                        let ldc = if rd(iv16.wrapping_add(0x28)) == iv14 {
                            rd(iv16.wrapping_add(0x2c))
                        } else {
                            0
                        };
                        let mut iv23 = 0;
                        if rd(iv16.wrapping_add(0x1c)) == iv14 {
                            iv23 = rd(iv16.wrapping_add(0x20));
                        }
                        p5 = pa(param_4, 1);
                        if 0 < iv14 {
                            loop {
                                iv14 = iv7.wrapping_add(1);
                                loop {
                                    let iv20 = iv14;
                                    iv14 = iv20.wrapping_add(2);
                                    s(p5, 0, g(ld8, iv7));
                                    s(p5, 1, g(ld8, iv20));
                                    s(p5, 2, g(ld8, iv20.wrapping_add(1)));
                                    s(p5, 3, g(ld8, iv14));
                                    let fv10;
                                    if ldc == 0 {
                                        s(p5, 8, g(p5, 0));
                                        s(p5, 9, g(p5, 1));
                                        s(p5, 10, g(p5, 2));
                                        fv10 = g(p5, 3);
                                    } else {
                                        s(p5, 8, rd(ldc.wrapping_add(iv7.wrapping_mul(4))));
                                        s(
                                            p5,
                                            9,
                                            rd(ldc
                                                .wrapping_sub(8)
                                                .wrapping_add(iv14.wrapping_mul(4))),
                                        );
                                        s(
                                            p5,
                                            10,
                                            rd(ldc
                                                .wrapping_sub(4)
                                                .wrapping_add(iv14.wrapping_mul(4))),
                                        );
                                        fv10 = rd(ldc.wrapping_add(iv14.wrapping_mul(4)));
                                    }
                                    s(p5, 0xb, fv10);
                                    if iv23 == 0 {
                                        s(p5, 0xc, g(p5, 0));
                                        s(p5, 0xd, g(p5, 1));
                                        s(p5, 0xe, g(p5, 2));
                                        s(p5, 0xf, g(p5, 3));
                                    } else {
                                        s(p5, 0xc, rd(iv23.wrapping_add(iv7.wrapping_mul(4))));
                                        s(
                                            p5,
                                            0xd,
                                            rd(iv23
                                                .wrapping_sub(8)
                                                .wrapping_add(iv14.wrapping_mul(4))),
                                        );
                                        s(
                                            p5,
                                            0xe,
                                            rd(iv23
                                                .wrapping_sub(4)
                                                .wrapping_add(iv14.wrapping_mul(4))),
                                        );
                                        s(p5, 0xf, rd(iv23.wrapping_add(iv14.wrapping_mul(4))));
                                    }
                                    edge(pf4, p5, 4, g(p5, 0), g(p5, 1), &mut fv13);
                                    edge(pf4, p5, 5, g(p5, 1), g(p5, 2), &mut fv13);
                                    if g(p5, 3) == -1 {
                                        edge(pf4, p5, 6, g(p5, 2), g(p5, 0), &mut fv13);
                                        s(p5, 7, -1);
                                    } else {
                                        edge(pf4, p5, 6, g(p5, 2), g(p5, 3), &mut fv13);
                                        edge(pf4, p5, 7, g(p5, 3), g(p5, 0), &mut fv13);
                                    }
                                    p5 = p5.wrapping_add(0x10);
                                    s(pf4, 4, g(pf4, 4).wrapping_add(1));
                                    if g(ld8, iv14) == -1 {
                                        iv7 = iv20.wrapping_add(3);
                                        break;
                                    }
                                    if g(ld8, iv20.wrapping_add(3)) == -1 {
                                        iv7 = iv20.wrapping_add(4);
                                        break;
                                    }
                                }
                                if !(iv7 < rd(iv16.wrapping_add(4))) {
                                    break;
                                }
                            }
                        }
                        p5 = FUN_004794c0(pf4 as i32, pa(param_4, 1), param_3);
                        wr(g(pf4, 5), fv13);
                        if DAT_00999488 < fv13 {
                            DAT_00999488 = fv13;
                        }
                    }
                    2 => {
                        p5 = pa(param_4, 0xc);
                        let mut p15 = ptr(rd(iv16.wrapping_add(0x38)));
                        if !p15.is_null() {
                            let f3 = -fg(p15, 0);
                            fs(param_4, 0, f3);
                            fs(param_4, 1, -fg(p15, 1));
                            let f13 = fg(param_4, 1);
                            fs(param_4, 2, -fg(p15, 2));
                            let f10 = fg(param_4, 2);
                            if rd(iv16.wrapping_add(0x2c)) != 0 {
                                p15 = pa(param_4, 3);
                                FUN_00421300(
                                    p15.cast::<f32>(),
                                    ptr(rd(iv16.wrapping_add(0x2c))).cast::<f32>(),
                                );
                                FUN_00420fe0(param_4.cast::<f32>(), p15.cast::<f32>());
                                fs(param_4, 0, fg(param_4, 0) - f3);
                                fs(param_4, 1, fg(param_4, 1) - f13);
                                fs(param_4, 2, fg(param_4, 2) - f10);
                                let p17 = ptr(rd(iv16.wrapping_add(0x20)));
                                if !p17.is_null() {
                                    fs(param_4, 3, fg(p17, 0) * fg(param_4, 3));
                                    fs(param_4, 4, fg(p17, 1) * fg(param_4, 4));
                                    fs(param_4, 5, fg(param_4, 5) * fg(p17, 2));
                                    fs(param_4, 6, fg(p17, 0) * fg(param_4, 6));
                                    fs(param_4, 7, fg(param_4, 7) * fg(p17, 1));
                                    fs(param_4, 8, fg(param_4, 8) * fg(p17, 2));
                                    fs(param_4, 9, fg(p17, 0) * fg(param_4, 9));
                                    fs(param_4, 10, fg(param_4, 10) * fg(p17, 1));
                                    fs(param_4, 0xb, fg(param_4, 0xb) * fg(p17, 2));
                                    fs(param_4, 0, fg(p17, 0) * fg(param_4, 0));
                                    fs(param_4, 1, fg(param_4, 1) * fg(p17, 1));
                                    fs(param_4, 2, fg(param_4, 2) * fg(p17, 2));
                                    let p17 = ptr(rd(iv16.wrapping_add(0x14)));
                                    if !p17.is_null() {
                                        fs(param_4, 0, fg(param_4, 0) + f3);
                                        fs(param_4, 1, fg(param_4, 1) + f13);
                                        fs(param_4, 2, fg(param_4, 2) + f10);
                                        FUN_00421300(p15.cast::<f32>(), p17.cast::<f32>());
                                        FUN_00420fe0(param_4.cast::<f32>(), p15.cast::<f32>());
                                        fs(param_4, 0, fg(param_4, 0) - f3);
                                        fs(param_4, 1, fg(param_4, 1) - f13);
                                        fs(param_4, 2, fg(param_4, 2) - f10);
                                        let p15b = ptr(rd(iv16.wrapping_add(8)));
                                        if !p15b.is_null() {
                                            fs(param_4, 0, fg(param_4, 0) - fg(p15b, 0));
                                            fs(param_4, 1, fg(param_4, 1) - fg(p15b, 1));
                                            fs(param_4, 2, fg(param_4, 2) - fg(p15b, 2));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    3 => {
                        s(pf4, 4, rd(iv16.wrapping_add(4)));
                        let n = (rd(iv16.wrapping_add(4)) as u32).wrapping_mul(0xc) >> 2;
                        cp(param_4, ptr(rd(iv16.wrapping_add(8))), n);
                        DAT_0060f1c0 = param_4;
                        DAT_0060f204 = rd(iv16.wrapping_add(4));
                        p5 = pa(param_4, (DAT_0060f204 as u32).wrapping_mul(3));
                        let iv7 = (&raw mut DAT_0099947e).read_unaligned();
                        if iv7 < g(pf4, 4) {
                            (&raw mut DAT_0099947e).write_unaligned(g(pf4, 4));
                        }
                    }
                    4 | 6 => {
                        s(param_4, 0, g(ld8, 0));
                        s(param_4, 1, g(ld8, 1));
                        s(param_4, 2, g(ld8, 2));
                        p5 = pa(param_4, 3);
                    }
                    5 => {
                        FUN_00421300(param_4.cast::<f32>(), ld8.cast::<f32>());
                        p5 = pa(param_4, 9);
                    }
                    7 => {
                        s(pf4, 4, 1);
                        s(pf4, 5, param_4 as i32);
                        let len = strcopy(
                            ptr(rd(iv16.wrapping_add(8))) as *const i8,
                            param_4 as *mut u8,
                        );
                        let _ = len;
                        // second scan of the same string
                        let mut n: u32 = 0xffff_ffff;
                        let mut q = ptr(rd(iv16.wrapping_add(8))) as *const i8;
                        loop {
                            if n == 0 {
                                break;
                            }
                            n = n.wrapping_sub(1);
                            let c = *q;
                            q = q.wrapping_add(1);
                            if c == 0 {
                                break;
                            }
                        }
                        p5 = (param_4 as u32).wrapping_add(!n) as usize as *mut i32;
                    }
                    9 => {
                        fv13 = rd(iv16.wrapping_add(4));
                        if rd(iv16.wrapping_add(4)) < rd(iv16.wrapping_add(0x10)) {
                            fv13 = rd(iv16.wrapping_add(0x10));
                        }
                        if fv13 < rd(iv16.wrapping_add(0x1c)) {
                            fv13 = rd(iv16.wrapping_add(0x1c));
                        }
                        if fv13 < rd(iv16.wrapping_add(0x28)) {
                            fv13 = rd(iv16.wrapping_add(0x28));
                        }
                        if fv13 < rd(iv16.wrapping_add(0x34)) {
                            fv13 = rd(iv16.wrapping_add(0x34));
                        }
                        if fv13 < rd(iv16.wrapping_add(0x40)) {
                            fv13 = rd(iv16.wrapping_add(0x40));
                        }
                        s(pf4, 4, fv13);
                        param_4 = blk3(iv16, 4, 8, fv13, param_4);
                        p5 = param_4;
                        if rd(iv16.wrapping_add(0x14)) != 0 {
                            param_4 = blk3(iv16, 0x10, 0x14, fv13, param_4);
                            p5 = param_4;
                            if rd(iv16.wrapping_add(0x20)) != 0 {
                                param_4 = blk3(iv16, 0x1c, 0x20, fv13, param_4);
                                p5 = param_4;
                                if rd(iv16.wrapping_add(0x2c)) != 0 {
                                    param_4 = blk3(iv16, 0x28, 0x2c, fv13, param_4);
                                    p5 = param_4;
                                    if rd(iv16.wrapping_add(0x38)) != 0 {
                                        param_4 = blk1(iv16, 0x34, 0x38, fv13, param_4);
                                        p5 = param_4;
                                        if rd(iv16.wrapping_add(0x44)) != 0 {
                                            p5 = blk1(iv16, 0x40, 0x44, fv13, param_4);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    10 | 14 => {
                        s(pf4, 4, g(ld8, 0));
                    }
                    11 => {
                        s(pf4, 4, rd(iv16.wrapping_add(4)));
                        let n = (rd(iv16.wrapping_add(4)) as u32).wrapping_mul(0xc) >> 2;
                        cp(param_4, ptr(rd(iv16.wrapping_add(8))), n);
                        DAT_0060f208 = param_4;
                        wr((param_3 as u32).wrapping_add(0x84) as i32, param_4 as i32);
                        p5 = pa(param_4, (rd(iv16.wrapping_add(4)) as u32).wrapping_mul(3));
                    }
                    13 => {
                        s(pf4, 4, rd(iv16.wrapping_add(4)));
                        let n = ((rd(iv16.wrapping_add(4)) as u32) & 0x1fff_ffff) << 1;
                        cp(param_4, ptr(rd(iv16.wrapping_add(8))), n);
                        DAT_0060f1e8 = param_4;
                        p5 = pa(param_4, (rd(iv16.wrapping_add(4)) as u32).wrapping_mul(2));
                    }
                    15 => {
                        let mut local_e4 = g(ld8, 0);
                        p5 = pa(param_4, 1);
                        let mut iv14: i32 = 0;
                        if rd(iv16.wrapping_add(0x14)) != 0 {
                            let iv7 = rd(rd(iv16.wrapping_add(0x14)));
                            if rd(iv16.wrapping_add(0x20)) != 0 {
                                iv16 = rd(rd(iv16.wrapping_add(0x20)));
                                fv13 = 0;
                                let mut p15 = p5;
                                let mut ld8i = local_e4;
                                if iv16 != 1 && iv16.wrapping_sub(1) > -1 {
                                    loop {
                                        ld8i = ld8i.wrapping_add(iv7);
                                        let mut iv23: i32 = 0;
                                        if iv7 != 1 && iv7.wrapping_sub(1) > -1 {
                                            loop {
                                                s(p15, 0, local_e4.wrapping_add(iv23));
                                                s(
                                                    p15,
                                                    1,
                                                    local_e4.wrapping_add(iv23).wrapping_add(1),
                                                );
                                                s(p15, 2, ld8i.wrapping_add(iv23).wrapping_add(1));
                                                s(p15, 3, ld8i.wrapping_add(iv23));
                                                copy_quad(p15);
                                                let mut fv10;
                                                if iv14 == 0 {
                                                    s(p15, 4, fv13);
                                                    fv10 = fv13.wrapping_add(1);
                                                } else {
                                                    if iv14 == 1 {
                                                        fv10 = fv13
                                                            .wrapping_add(iv7.wrapping_mul(-3))
                                                            .wrapping_add(iv23)
                                                            .wrapping_add(4);
                                                    } else {
                                                        fv10 =
                                                            fv13.wrapping_add(iv7.wrapping_mul(-2));
                                                    }
                                                    s(p15, 4, fv10);
                                                    fv10 = fv13;
                                                }
                                                s(p15, 5, fv10);
                                                s(p15, 6, fv10.wrapping_add(1));
                                                let p17 = pa(p15, 7);
                                                fv13 = fv10.wrapping_add(2);
                                                if iv23 == 0 {
                                                    s(p17, 0, fv13);
                                                    fv13 = fv10.wrapping_add(3);
                                                } else {
                                                    s(p17, 0, fv10.wrapping_sub(2));
                                                    if iv14 == 0 {
                                                        s(p17, 0, fv10.wrapping_sub(3));
                                                    }
                                                    if iv23 == 1 {
                                                        s(p17, 0, g(p17, 0).wrapping_sub(1));
                                                    }
                                                }
                                                p15 = p15.wrapping_add(0x10);
                                                iv23 = iv23.wrapping_add(1);
                                                if !(iv23 < iv7.wrapping_sub(1)) {
                                                    break;
                                                }
                                            }
                                        }
                                        local_e4 = local_e4.wrapping_add(iv7);
                                        iv14 = iv14.wrapping_add(1);
                                        if !(iv14 < iv16.wrapping_sub(1)) {
                                            break;
                                        }
                                    }
                                }
                                s(
                                    pf4,
                                    4,
                                    iv7.wrapping_sub(1).wrapping_mul(iv16.wrapping_sub(1)),
                                );
                                p5 = FUN_004794c0(pf4 as i32, p5, param_3);
                                wr(g(pf4, 5), fv13);
                                if DAT_00999488 < fv13 {
                                    DAT_00999488 = fv13;
                                }
                            }
                        }
                    }
                    16 => {
                        fv13 = 0;
                        let mut fv10: i32 = 0;
                        let mut local_e4: i32 = 0;
                        s(pf4, 4, 0);
                        p5 = pa(param_4, 1);
                        if 0 < rd(iv16.wrapping_add(4)) {
                            loop {
                                let iv3 = g(ld8, 0);
                                let mut fv12 = fv10;
                                if 2 < iv3 {
                                    let mut iv14 = fv10.wrapping_add(1).wrapping_sub(fv10);
                                    fv12 = fv10.wrapping_add(1);
                                    let mut fv18 = fv13.wrapping_add(1);
                                    let mut fv19 = fv13;
                                    loop {
                                        fv13 = fv18;
                                        if !(iv14 < iv3) {
                                            break;
                                        }
                                        s(pf4, 4, g(pf4, 4).wrapping_add(1));
                                        s(p5, 0, fv10);
                                        s(p5, 1, fv12);
                                        s(p5, 2, fv12.wrapping_add(1));
                                        let mut fv11 = fv12.wrapping_add(2);
                                        if fv11.wrapping_sub(iv3) == fv10 {
                                            s(p5, 3, -1);
                                            s(p5, 4, fv19);
                                            s(p5, 5, fv13);
                                            s(p5, 6, fv13.wrapping_add(1));
                                            fv18 = fv13.wrapping_add(2);
                                            s(p5, 7, -1);
                                        } else {
                                            s(p5, 3, fv11);
                                            fv11 = fv12.wrapping_add(3);
                                            if fv11.wrapping_sub(iv3) != fv10 {
                                                fv11 = fv12.wrapping_add(2);
                                            }
                                            s(p5, 4, fv19);
                                            s(p5, 5, fv13);
                                            s(p5, 6, fv13.wrapping_add(1));
                                            fv19 = fv13.wrapping_add(2);
                                            s(p5, 7, fv19);
                                            fv18 = fv13.wrapping_add(3);
                                        }
                                        copy_quad(p5);
                                        p5 = p5.wrapping_add(0x10);
                                        fv12 = fv11;
                                        iv14 = fv11.wrapping_sub(fv10);
                                    }
                                }
                                ld8 = ld8.wrapping_add(1);
                                local_e4 = local_e4.wrapping_add(1);
                                fv10 = fv12;
                                if !(local_e4 < rd(iv16.wrapping_add(4))) {
                                    break;
                                }
                            }
                        }
                        p5 = FUN_004794c0(pf4 as i32, pa(param_4, 1), param_3);
                        wr(g(pf4, 5), fv13);
                        if DAT_00999488 < fv13 {
                            DAT_00999488 = fv13;
                        }
                    }
                    17 => {
                        fv13 = g(ld8, 0);
                        p5 = pa(param_4, 1);
                        let mut pi24 = ptr(rd(iv16.wrapping_add(0x14)));
                        if !pi24.is_null() {
                            let mut local_e4: i32 = 0;
                            s(pf4, 4, 0);
                            let mut p15 = p5;
                            let mut fv10 = local_d0[0];
                            if 0 < rd(iv16.wrapping_add(0x10)) {
                                loop {
                                    let mut uv8: u32 = 2;
                                    if 2 < g(pi24, 0) {
                                        loop {
                                            fv10 = uv8.wrapping_add(fv13 as u32) as i32;
                                            if (uv8 & 1) == 0 {
                                                s(p15, 0, fv10.wrapping_sub(2));
                                                s(p15, 1, fv10.wrapping_sub(1));
                                                s(p15, 2, fv10);
                                                s(p15, 3, -1);
                                                if uv8 == 2 {
                                                    s(p15, 4, local_d0[0]);
                                                    local_d0[0] = local_d0[0].wrapping_add(1);
                                                } else {
                                                    s(p15, 4, local_d0[0].wrapping_sub(2));
                                                }
                                                s(p15, 5, local_d0[0]);
                                            } else {
                                                s(p15, 0, fv10);
                                                s(p15, 1, fv10.wrapping_sub(1));
                                                s(p15, 2, fv10.wrapping_sub(2));
                                                s(p15, 3, -1);
                                                s(p15, 4, local_d0[0]);
                                                s(p15, 5, local_d0[0].wrapping_sub(2));
                                            }
                                            s(p15, 6, local_d0[0].wrapping_add(1));
                                            local_d0[0] = local_d0[0].wrapping_add(2);
                                            uv8 = uv8.wrapping_add(1);
                                            s(p15, 7, -1);
                                            copy_quad(p15);
                                            p15 = p15.wrapping_add(0x10);
                                            s(pf4, 4, g(pf4, 4).wrapping_add(1));
                                            if !((uv8 as i32) < g(pi24, 0)) {
                                                break;
                                            }
                                        }
                                    }
                                    let iv14 = g(pi24, 0);
                                    pi24 = pi24.wrapping_add(1);
                                    fv13 = fv13.wrapping_add(iv14);
                                    local_e4 = local_e4.wrapping_add(1);
                                    fv10 = local_d0[0];
                                    if !(local_e4 < rd(iv16.wrapping_add(0x10))) {
                                        break;
                                    }
                                }
                            }
                            p5 = FUN_004794c0(pf4 as i32, p5, param_3);
                            wr(g(pf4, 5), fv10);
                            if DAT_00999488 < fv10 {
                                DAT_00999488 = fv10;
                            }
                        }
                    }
                    20 => {
                        s(pf4, 5, param_4 as i32);
                        let pb6 = FUN_00479e80(param_4, ptr(rd(iv16.wrapping_add(8))) as *mut i8);
                        p5 = (param_4 as u32).wrapping_add(pb6 as u32) as usize as *mut i32;
                    }
                    21 => {
                        let mut iv14: i32 = 0;
                        let mut iv7: i32 = 0;
                        s(pf4, 4, 0);
                        if 0 < rd(iv16.wrapping_add(4)) {
                            loop {
                                if rd(g(param_2, 3).wrapping_add(iv14)) != 0
                                    && (iv14 == 0 || fg(ld8, -1) != fg(ld8, 0))
                                {
                                    s(param_4, 0, g(ld8, 0));
                                    wr(
                                        g(param_2, 3).wrapping_add(g(pf4, 4).wrapping_mul(4)),
                                        rd(g(param_2, 3).wrapping_add(iv14)),
                                    );
                                    s(pf4, 4, g(pf4, 4).wrapping_add(1));
                                    param_4 = param_4.wrapping_add(1);
                                }
                                iv7 = iv7.wrapping_add(1);
                                ld8 = ld8.wrapping_add(1);
                                iv14 = iv14.wrapping_add(4);
                                if !(iv7 < rd(iv16.wrapping_add(4))) {
                                    break;
                                }
                            }
                        }
                        s(param_2, 2, g(pf4, 4));
                        p5 = param_4;
                    }
                    22 => {
                        s(param_4, 0, g(ld8, 0));
                        let q = ptr(rd(iv16.wrapping_add(0x14)));
                        s(param_4, 1, g(q, 0));
                        s(param_4, 2, g(q, 1));
                        s(param_4, 3, g(q, 2));
                        p5 = pa(param_4, 4);
                    }
                    23 => {
                        s(param_4, 0, g(ld8, 0));
                        s(param_4, 1, g(ld8, 1));
                        s(param_4, 2, g(ld8, 2));
                        let q = ptr(rd(iv16.wrapping_add(0x14)));
                        s(param_4, 3, g(q, 0));
                        s(param_4, 4, g(q, 1));
                        s(param_4, 5, g(q, 2));
                        let q = ptr(rd(iv16.wrapping_add(0x20)));
                        s(param_4, 6, g(q, 0));
                        s(param_4, 7, g(q, 1));
                        s(param_4, 8, g(q, 2));
                        let q = ptr(rd(iv16.wrapping_add(0x2c)));
                        s(param_4, 9, g(q, 0));
                        s(param_4, 10, g(q, 1));
                        s(param_4, 0xb, g(q, 2));
                        p5 = pa(param_4, 0xc);
                    }
                    25 => {
                        s(param_4, 0, g(ld8, 0));
                        s(param_4, 1, rd(rd(iv16.wrapping_add(0x14))));
                        let q = ptr(rd(iv16.wrapping_add(0x20)));
                        s(param_4, 2, g(q, 0));
                        s(param_4, 3, g(q, 1));
                        s(param_4, 4, g(q, 2));
                        let q = ptr(rd(iv16.wrapping_add(0x2c)));
                        s(param_4, 5, g(q, 0));
                        s(param_4, 6, g(q, 1));
                        s(param_4, 7, g(q, 2));
                        let q = ptr(rd(iv16.wrapping_add(0x38)));
                        s(param_4, 8, g(q, 0));
                        s(param_4, 9, g(q, 1));
                        s(param_4, 10, g(q, 2));
                        let q = ptr(rd(iv16.wrapping_add(0x44)));
                        s(param_4, 0xb, g(q, 0));
                        s(param_4, 0xc, g(q, 1));
                        s(param_4, 0xd, g(q, 2));
                        s(param_4, 0xe, rd(rd(iv16.wrapping_add(0x50))));
                        let q = ptr(rd(iv16.wrapping_add(0x5c)));
                        s(param_4, 0xf, g(q, 0));
                        s(param_4, 0x10, g(q, 1));
                        s(param_4, 0x11, g(q, 2));
                        p5 = pa(param_4, 0x12);
                    }
                    _ => {}
                }
            }
        }
        param_4 = p5;
        iv16 = 0;
        if g(param_2, 2) != 0 {
            let mut pd = local_d0.as_mut_ptr();
            let mut k: i32 = 0x34;
            while k != 0 {
                *pd = rd(param_3);
                param_3 = (param_3 as u32).wrapping_add(4) as i32;
                pd = pd.wrapping_add(1);
                k = k.wrapping_sub(1);
            }
            DAT_0060f208 = core::ptr::null_mut();
            DAT_0060f1f0 = 0;
            DAT_0060f1d8 = 0;
            let mut iv14: i32 = 0;
            s(pf4, 2, g(param_2, 2));
            s(pf4, 3, param_4 as i32);
            param_4 = pa(param_4, g(param_2, 2) as u32);
            if 0 < g(param_2, 2) {
                loop {
                    iv16 = iv16.wrapping_add(4);
                    iv14 = iv14.wrapping_add(1);
                    wr(g(pf4, 3).wrapping_sub(4).wrapping_add(iv16), param_4 as i32);
                    let iv7 = FUN_00477c80(
                        param_1,
                        ptr(rd(g(param_2, 3).wrapping_sub(4).wrapping_add(iv16))),
                        local_d0.as_mut_ptr() as i32,
                        param_4,
                        unaff_ebp,
                        unaff_edi,
                    );
                    param_4 = (param_4 as u32).wrapping_add(iv7 as u32) as usize as *mut i32;
                    if !(iv14 < g(param_2, 2)) {
                        break;
                    }
                }
            }
        }
        (param_4 as u32).wrapping_sub(pf4 as u32) as i32
    }
}
