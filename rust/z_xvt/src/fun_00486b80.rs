// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052a348: i32;
    static mut _DAT_005254e0: i32;
    static mut DAT_005254e4: i32;
    static mut DAT_005254e8: f32;
    static mut DAT_006db7b4: i32;
    static mut _DAT_005201dc: f32;
    static mut _DAT_005201f4: f32;
    static mut DAT_0052a46c: i32;
    static mut DAT_00602678: i32;
    static mut DAT_00602690: u32;
    static mut DAT_006026a0: i32;
    static mut DAT_006026a4: *mut i32;
    static mut _DAT_006026a8: i32;
    static mut DAT_006026bc: i32;
    static mut _DAT_006026c0: *mut f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(a: *mut i8, b: *mut i8) -> i32;
    fn FUN_0048e520(a: *mut u32, b: *mut i8) -> *mut u32;
    fn FUN_00485db0(a: *mut u32);
    fn FUN_004150a0(a: *mut f32, b: *mut f32);
    fn FUN_00414fe0(a: *mut f32, b: *mut f32);
    fn FUN_004151d0(a: *mut f32, b: *mut f32);
    fn FUN_00415040(a: *mut f32, b: *mut f32) -> f64;
    fn FUN_00415060(a: *mut f32, b: i32) -> f64;
    fn FUN_00415080(a: *mut f32, b: i32) -> f64;
    fn FUN_00415300(a: *mut f32, b: *mut f32);
}

#[inline(always)]
unsafe fn rd(p: *mut u32, i: usize) -> u32 {
    // SAFETY: the game's own structure memory, read where the original reads.
    unsafe { p.wrapping_add(i).read_unaligned() }
}

#[inline(always)]
unsafe fn wr(p: *mut u32, i: usize, v: u32) {
    // SAFETY: the game's own structure memory, written where the original writes.
    unsafe { p.wrapping_add(i).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rf(p: *mut u32, i: usize) -> f32 {
    // SAFETY: the game's own structure memory, read where the original reads.
    unsafe { p.wrapping_add(i).cast::<f32>().read_unaligned() }
}

#[inline(always)]
unsafe fn wf(p: *mut u32, i: usize, v: f32) {
    // SAFETY: the game's own structure memory, written where the original writes.
    unsafe { p.wrapping_add(i).cast::<f32>().write_unaligned(v) }
}

#[inline(always)]
unsafe fn fr(p: *mut f32, i: usize) -> f32 {
    // SAFETY: the game's own float data, read where the original reads.
    unsafe { p.wrapping_add(i).read_unaligned() }
}

#[inline(always)]
unsafe fn ri(a: i32) -> i32 {
    // SAFETY: an address inside the game's own memory.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

#[inline(always)]
fn fadd(p: *mut f32, n: u32) -> *mut f32 {
    (p as usize as u32).wrapping_add(n.wrapping_mul(4)) as usize as *mut f32
}

#[inline(always)]
unsafe fn case10(p2: *mut u32, p3: *mut u32) {
    // SAFETY: the game's own globals and structures.
    unsafe {
        let i5 = rd(p2, 4) as i32;
        if i5 == 8 || i5 == 7 {
            wr(p3, 0x22, DAT_00602690);
        } else if i5 == 6 || i5 == 5 {
            wr(p3, 0x27, DAT_00602690);
        } else {
            wr(p3, 0x1d, DAT_00602690);
        }
    }
}

#[inline(always)]
unsafe fn case14(p2: *mut u32, p3: *mut u32) {
    // SAFETY: the game's own globals and structures.
    unsafe {
        wr(p3, 0x29, rd(p2, 0));
        DAT_006026a4 = rd(p2, 5) as usize as *mut i32;
        wr(p3, 0x2a, DAT_006026a4 as usize as u32);
        wr(p3, 0x2b, rd(p3, 0x2a));
        let i5 = rd(p3, 0x2a).wrapping_add(0x18) as i32;
        wr(p3, 0x2b, i5 as u32);
        let p12 = p3.wrapping_add(0x2c) as *mut i32;
        if DAT_006026a4.wrapping_add(1).read_unaligned() == 0 {
            p12.write_unaligned(DAT_006026a4.read_unaligned());
        } else {
            p12.write_unaligned(i5);
            let i10 = rd(p3, 0x2a) as i32;
            let mut i9 = ri(i10.wrapping_add(0x14)).wrapping_mul(ri(i10.wrapping_add(0x10)));
            if ri(i10.wrapping_add(8)) == i9 {
                i9 = ri(i10.wrapping_add(0xc));
            }
            p12.write_unaligned(i9.wrapping_add(i5));
        }
        let i5b = p12.read_unaligned();
        p12.write_unaligned(i5b.wrapping_add(0x1000));
        wr(p3, 0x2d, i5b.wrapping_add(0x1000) as u32);
        p12.write_unaligned(p12.read_unaligned().wrapping_sub(0x1000));
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00486b80(param_1: i32, param_2: *mut u32, param_3: *mut u32) {
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    if param_2.is_null() {
        return;
    }
    // SAFETY: the callees and globals are the game's own; all pointers point
    // into the game's own structures and are accessed as the original does.
    unsafe {
        loop {
            let mut i_stack_134: i32 = 0;
            let mut i_stack_130: i32 = 0;
            while rd(param_2, 1) == 7 {
                'found: {
                    if DAT_0052a348 == 0 {
                        let mut i_stack_128: i32 = 0;
                        let pc = rd(param_2, 5) as usize as *mut i8;
                        let mut i_stack_13c = ri(param_1.wrapping_add(6));
                        if 0 < i_stack_13c {
                            let mut i_stack_12c: i32 = 0;
                            loop {
                                let i_stack_140 = ri(param_1.wrapping_add(10));
                                let mut p7 = ri(i_stack_140.wrapping_add(i_stack_12c)) as u32
                                    as usize
                                    as *mut u32;
                                if p7.is_null() {
                                    p7 = core::ptr::null_mut();
                                } else if rd(p7, 0) == 0
                                    || __strcmpi(rd(p7, 0) as usize as *mut i8, pc) != 0
                                {
                                    let mut i5: i32 = 0;
                                    if 0 < rd(p7, 2) as i32 {
                                        let mut i10: i32 = 0;
                                        loop {
                                            let a = ri((rd(p7, 3) as i32).wrapping_add(i10)) as u32
                                                as usize
                                                as *mut u32;
                                            param_2 = FUN_0048e520(a, pc);
                                            if !param_2.is_null() {
                                                break 'found;
                                            }
                                            i10 = i10.wrapping_add(4);
                                            i5 = i5.wrapping_add(1);
                                            if !(i5 < rd(p7, 2) as i32) {
                                                break;
                                            }
                                        }
                                    }
                                    p7 = core::ptr::null_mut();
                                }
                                param_2 = p7;
                                if !p7.is_null() {
                                    break 'found;
                                }
                                i_stack_12c = i_stack_12c.wrapping_add(4);
                                i_stack_128 = i_stack_128.wrapping_add(1);
                                i_stack_13c = ri(param_1.wrapping_add(6));
                                if !(i_stack_128 < i_stack_13c) {
                                    break;
                                }
                            }
                        }
                        param_2 = core::ptr::null_mut();
                    } else {
                        let pc = rd(param_2, 5) as usize as *mut i8;
                        if *pc == 0 {
                            param_2 = rd(param_2, 0) as usize as *mut u32;
                        } else {
                            let mut i_stack_124: i32 = 0;
                            let mut i_stack_13c = ri(param_1.wrapping_add(6));
                            'cb3: {
                                if 0 < i_stack_13c {
                                    let mut i_stack_128: i32 = 0;
                                    loop {
                                        let i_stack_140 = ri(param_1.wrapping_add(10));
                                        let p7 = ri(i_stack_140.wrapping_add(i_stack_128)) as u32
                                            as usize
                                            as *mut u32;
                                        let mut p6: *mut u32 = core::ptr::null_mut();
                                        if !p7.is_null() {
                                            let mut need = true;
                                            if rd(p7, 0) != 0 {
                                                let r =
                                                    __strcmpi(rd(p7, 0) as usize as *mut i8, pc);
                                                p6 = p7;
                                                need = r != 0;
                                            }
                                            if need {
                                                let mut i_stack_12c: i32 = 0;
                                                p6 = core::ptr::null_mut();
                                                if 0 < rd(p7, 2) as i32 {
                                                    let mut i5: i32 = 0;
                                                    loop {
                                                        let a =
                                                            ri((rd(p7, 3) as i32).wrapping_add(i5))
                                                                as u32
                                                                as usize
                                                                as *mut u32;
                                                        p6 = FUN_0048e520(a, pc);
                                                        if !p6.is_null() {
                                                            break;
                                                        }
                                                        i5 = i5.wrapping_add(4);
                                                        i_stack_12c = i_stack_12c.wrapping_add(1);
                                                        if !(i_stack_12c < rd(p7, 2) as i32) {
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        if !p6.is_null() {
                                            wr(param_2, 0, p6 as usize as u32);
                                            break 'cb3;
                                        }
                                        i_stack_128 = i_stack_128.wrapping_add(4);
                                        i_stack_124 = i_stack_124.wrapping_add(1);
                                        i_stack_13c = ri(param_1.wrapping_add(6));
                                        if !(i_stack_124 < i_stack_13c) {
                                            break;
                                        }
                                    }
                                }
                                wr(param_2, 0, 0);
                            }
                            *(rd(param_2, 5) as usize as *mut u8) = 0;
                            param_2 = rd(param_2, 0) as usize as *mut u32;
                        }
                    }
                }
                if param_2.is_null() {
                    return;
                }
            }

            let mut pf13 = rd(param_2, 5) as usize as *mut f32;
            if pf13.is_null() {
                match rd(param_2, 1) {
                    10 => case10(param_2, param_3),
                    0x14 => case14(param_2, param_3),
                    0x18 => {
                        i_stack_130 = (_DAT_005254e0 as u32).wrapping_add(1) as i32;
                        let i5 = rd(param_2, 2) as i32;
                        if i5 < i_stack_130 {
                            i_stack_130 = i5;
                        }
                    }
                    _ => {}
                }
            } else {
                match rd(param_2, 1) {
                    1 | 0xf | 0x10 | 0x11 => {
                        let mut pf11 = pf13.wrapping_add(1);
                        wr(param_3, 0x23, rd(param_2, 4));
                        wf(param_3, 0x24, fr(pf13, 0));
                        wr(param_3, 0x28, pf11 as usize as u32);
                        pf11 = fadd(pf11, rd(param_2, 4).wrapping_mul(0x10));
                        wr(param_3, 0x25, pf11 as usize as u32);
                        let i5 = rd(param_2, 4);
                        wr(
                            param_3,
                            0x26,
                            fadd(pf11, i5.wrapping_mul(3)) as usize as u32,
                        );
                        let mut p12 = DAT_006026a4;
                        let i10 = rd(param_2, 4);
                        if rd(param_3, 0x2a) == 0 {
                            wr(param_3, 0x2a, DAT_006026a4 as usize as u32);
                            wr(param_3, 0x2b, p12 as usize as u32);
                            p12 = p12.wrapping_add(6);
                            wr(param_3, 0x2b, p12 as usize as u32);
                            let p2c = param_3.wrapping_add(0x2c) as *mut i32;
                            if DAT_006026a4.wrapping_add(1).read_unaligned() == 0 {
                                p2c.write_unaligned(DAT_006026a4.read_unaligned());
                            } else {
                                p2c.write_unaligned(p12 as usize as i32);
                                let i9 = rd(param_3, 0x2a) as i32;
                                let mut i8 = ri(i9.wrapping_add(0x14))
                                    .wrapping_mul(ri(i9.wrapping_add(0x10)));
                                if ri(i9.wrapping_add(8)) == i8 {
                                    i8 = ri(i9.wrapping_add(0xc));
                                }
                                p2c.write_unaligned(i8.wrapping_add(p12 as usize as i32));
                            }
                            let i9 = p2c.read_unaligned();
                            p2c.write_unaligned(i9.wrapping_add(0x1000));
                            wr(param_3, 0x2d, i9.wrapping_add(0x1000) as u32);
                            p2c.write_unaligned(p2c.read_unaligned().wrapping_sub(0x1000));
                        }
                        if rd(param_3, 0x21) == 0 {
                            let a = fadd(fadd(pf11, i5.wrapping_mul(3)), i10.wrapping_mul(6));
                            wr(param_3, 0x21, a as usize as u32);
                            FUN_00485db0(param_3);
                            wr(param_3, 0x21, 0);
                        } else {
                            FUN_00485db0(param_3);
                        }
                    }
                    2 => {
                        let pf11 = pf13.wrapping_add(3);
                        let mut pf1 = param_3.wrapping_add(2) as *mut f32;
                        FUN_004150a0(param_3.wrapping_add(5) as *mut f32, pf11);
                        FUN_00414fe0(pf1, pf11);
                        pf1.write_unaligned(fr(pf13, 0) + pf1.read_unaligned());
                        pf1 = param_3.wrapping_add(0x11) as *mut f32;
                        wf(param_3, 3, fr(pf13, 1) + rf(param_3, 3));
                        wf(param_3, 4, fr(pf13, 2) + rf(param_3, 4));
                        FUN_004151d0(pf1, pf11);
                        let mut v = FUN_00415040(pf13, pf1);
                        wf(param_3, 0xe, (rf(param_3, 0xe) as f64 - v) as f32);
                        v = FUN_00415060(pf13, pf1 as usize as i32);
                        wf(param_3, 0xf, (rf(param_3, 0xf) as f64 - v) as f32);
                        v = FUN_00415080(pf13, pf1 as usize as i32);
                        wf(param_3, 0x10, (rf(param_3, 0x10) as f64 - v) as f32);
                    }
                    3 => {
                        wr(param_3, 0x1e, rd(param_2, 4));
                        wr(param_3, 0x1f, pf13 as usize as u32);
                    }
                    4 => {
                        wf(param_3, 2, fr(pf13, 0) + rf(param_3, 2));
                        let pf11 = param_3.wrapping_add(0x11) as *mut f32;
                        wf(param_3, 3, fr(pf13, 1) + rf(param_3, 3));
                        wf(param_3, 4, fr(pf13, 2) + rf(param_3, 4));
                        let mut v = FUN_00415040(pf13, pf11);
                        wf(param_3, 0xe, (rf(param_3, 0xe) as f64 - v) as f32);
                        v = FUN_00415060(pf13, pf11 as usize as i32);
                        wf(param_3, 0xf, (rf(param_3, 0xf) as f64 - v) as f32);
                        v = FUN_00415080(pf13, pf11 as usize as i32);
                        wf(param_3, 0x10, (rf(param_3, 0x10) as f64 - v) as f32);
                    }
                    5 => {
                        FUN_004150a0(param_3.wrapping_add(5) as *mut f32, pf13);
                        FUN_00414fe0(param_3.wrapping_add(2) as *mut f32, pf13);
                        FUN_004151d0(param_3.wrapping_add(0x11) as *mut f32, pf13);
                    }
                    6 => {
                        let a = fr(pf13, 0);
                        let b = fr(pf13, 1);
                        let c = fr(pf13, 2);
                        wf(param_3, 5, a * rf(param_3, 5));
                        wf(param_3, 6, b * rf(param_3, 6));
                        wf(param_3, 7, c * rf(param_3, 7));
                        wf(param_3, 8, a * rf(param_3, 8));
                        wf(param_3, 9, b * rf(param_3, 9));
                        wf(param_3, 10, c * rf(param_3, 10));
                        wf(param_3, 0xb, a * rf(param_3, 0xb));
                        wf(param_3, 0xc, b * rf(param_3, 0xc));
                        wf(param_3, 0xd, c * rf(param_3, 0xd));
                        wf(param_3, 2, a * rf(param_3, 2));
                        wf(param_3, 3, b * rf(param_3, 3));
                        wf(param_3, 4, c * rf(param_3, 4));
                        let mut f4 = _DAT_005201dc / a;
                        wf(param_3, 0x11, rf(param_3, 0x11) * f4);
                        wf(param_3, 0x12, rf(param_3, 0x12) * f4);
                        wf(param_3, 0x13, f4 * rf(param_3, 0x13));
                        f4 = _DAT_005201dc / b;
                        wf(param_3, 0x14, rf(param_3, 0x14) * f4);
                        wf(param_3, 0x15, rf(param_3, 0x15) * f4);
                        wf(param_3, 0x16, f4 * rf(param_3, 0x16));
                        f4 = _DAT_005201dc / c;
                        wf(param_3, 0x17, rf(param_3, 0x17) * f4);
                        wf(param_3, 0x18, rf(param_3, 0x18) * f4);
                        wf(param_3, 0x19, f4 * rf(param_3, 0x19));
                    }
                    10 => case10(param_2, param_3),
                    0xb => {
                        _DAT_006026c0 = pf13;
                        wr(param_3, 0x21, pf13 as usize as u32);
                    }
                    0xd => {
                        wr(param_3, 0x20, pf13 as usize as u32);
                    }
                    0x13 => {
                        wf(param_3, 0x1a, fr(pf13, 0));
                        wf(param_3, 0x1b, fr(pf13, 1));
                        wf(param_3, 0x1c, fr(pf13, 2));
                    }
                    0x14 => case14(param_2, param_3),
                    0x15 => {
                        if DAT_006db7b4 < 1 || DAT_005254e4 != 0 {
                            i_stack_134 = DAT_005254e4;
                            if DAT_005254e4 == 0 {
                                i_stack_134 = 1;
                            } else if (rd(param_2, 2) as i32) < DAT_005254e4 {
                                i_stack_134 = -1;
                            }
                        } else {
                            let mut f_stack_138: f32 = 1.0;
                            if 0 < (DAT_005254e8 as i32) {
                                f_stack_138 =
                                    _DAT_005201dc / ((DAT_006db7b4 as f32) * DAT_005254e8);
                            }
                            i_stack_134 = 1;
                            let i5 = rd(param_2, 2) as i32;
                            if 0 < i5 {
                                loop {
                                    if fr(pf13, 0) <= f_stack_138 {
                                        break;
                                    }
                                    pf13 = pf13.wrapping_add(1);
                                    i_stack_134 = i_stack_134.wrapping_add(1);
                                    if !(i_stack_134 <= i5) {
                                        break;
                                    }
                                }
                            }
                            if i5 < i_stack_134 {
                                i_stack_134 = -1;
                            }
                        }
                    }
                    0x17 => {
                        if (rd(param_3, 1) & 0x7fffffff) != 0 {
                            wf(param_3, 0xe, rf(param_3, 0xe) - fr(pf13, 0));
                            let pf11 = param_3.wrapping_add(5) as *mut f32;
                            wf(param_3, 0xf, rf(param_3, 0xf) - fr(pf13, 1));
                            wf(param_3, 0x10, rf(param_3, 0x10) - fr(pf13, 2));
                            let mut v = FUN_00415040(pf13, pf11);
                            wf(param_3, 2, (v + rf(param_3, 2) as f64) as f32);
                            v = FUN_00415060(pf13, pf11 as usize as i32);
                            wf(param_3, 3, (v + rf(param_3, 3) as f64) as f32);
                            v = FUN_00415080(pf13, pf11 as usize as i32);
                            wf(param_3, 4, (v + rf(param_3, 4) as f64) as f32);
                            let mut af120: [f32; 4] = [0.0; 4];
                            let mut af110: [f32; 16] = [0.0; 16];
                            af120[0] = fr(pf13, 3) * _DAT_005201f4;
                            af120[1] = fr(pf13, 4) * _DAT_005201f4;
                            af120[2] = fr(pf13, 5) * _DAT_005201f4;
                            af120[3] = f32::from_bits(rd(param_3, 1));
                            FUN_00415300(af110.as_mut_ptr(), af120.as_mut_ptr());
                            FUN_004150a0(
                                param_3.wrapping_add(0x11) as *mut f32,
                                af110.as_mut_ptr(),
                            );
                            FUN_00414fe0(param_3.wrapping_add(0xe) as *mut f32, af110.as_mut_ptr());
                            FUN_004151d0(pf11, af110.as_mut_ptr());
                            wf(param_3, 0xe, fr(pf13, 0) + rf(param_3, 0xe));
                            wf(param_3, 0xf, fr(pf13, 1) + rf(param_3, 0xf));
                            wf(param_3, 0x10, fr(pf13, 2) + rf(param_3, 0x10));
                            v = FUN_00415040(pf13, pf11);
                            wf(param_3, 2, (rf(param_3, 2) as f64 - v) as f32);
                            v = FUN_00415060(pf13, pf11 as usize as i32);
                            wf(param_3, 3, (rf(param_3, 3) as f64 - v) as f32);
                            v = FUN_00415080(pf13, pf11 as usize as i32);
                            wf(param_3, 4, (rf(param_3, 4) as f64 - v) as f32);
                        }
                    }
                    0x18 => {
                        let i5 = rd(param_2, 2) as i32;
                        i_stack_130 = (_DAT_005254e0 as u32).wrapping_add(1) as i32;
                        if i5 < i_stack_130 {
                            i_stack_130 = i5;
                        }
                    }
                    _ => {}
                }
            }

            if rd(param_2, 2) == 0 {
                return;
            }
            let i5: i32;
            if i_stack_130 == 0 {
                if i_stack_134 == 0 {
                    let mut au_d0: [u32; 52] = [0; 52];
                    let mut dst = au_d0.as_mut_ptr();
                    let mut cnt: i32 = 0x34;
                    while cnt != 0 {
                        wr(dst, 0, rd(param_3, 0));
                        param_3 = param_3.wrapping_add(1);
                        dst = dst.wrapping_add(1);
                        cnt -= 1;
                    }
                    let mut i10: i32 = 0;
                    DAT_00602678 = 0;
                    DAT_006026a0 = 0;
                    let mut i5b: i32 = 0;
                    _DAT_006026c0 = core::ptr::null_mut();
                    _DAT_006026a8 = 0;
                    DAT_00602690 = 0;
                    DAT_006026bc = 0;
                    if (rd(param_2, 2) as i32) < 1 {
                        DAT_00602678 = 0;
                        DAT_00602690 = 0;
                        DAT_006026a0 = 0;
                        _DAT_006026a8 = 0;
                        DAT_006026bc = 0;
                        _DAT_006026c0 = core::ptr::null_mut();
                        return;
                    }
                    loop {
                        DAT_0052a46c = DAT_0052a46c.wrapping_add(1);
                        i10 = i10.wrapping_add(4);
                        i5b = i5b.wrapping_add(1);
                        let child = ri((rd(param_2, 3) as i32).wrapping_add(-4).wrapping_add(i10))
                            as u32 as usize as *mut u32;
                        FUN_00486b80(param_1, child, au_d0.as_mut_ptr());
                        if !(i5b < rd(param_2, 2) as i32) {
                            break;
                        }
                    }
                    return;
                }
                if i_stack_134 == -1 {
                    return;
                }
                i5 = rd(param_2, 3) as i32;
                i_stack_130 = i_stack_134;
            } else {
                i5 = rd(param_2, 3) as i32;
            }
            DAT_0052a46c = DAT_0052a46c.wrapping_add(1);
            param_2 = ri(i5
                .wrapping_add(-4)
                .wrapping_add((i_stack_130 as u32).wrapping_mul(4) as i32))
                as u32 as usize as *mut u32;
            if param_2.is_null() {
                return;
            }
        }
    }
}
