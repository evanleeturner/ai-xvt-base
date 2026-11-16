// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005201dc: f32;
    static mut DAT_00602678: i32;
    static mut DAT_006026a0: i32;
    static mut _DAT_006026a8: i32;
    static mut DAT_00602690: i32;
    static mut DAT_006026bc: i32;
    static mut _DAT_006026c0: *mut f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(a: *const i8, b: *const i8) -> i32;
    fn FUN_0048e520(a: *mut u32, b: *mut i8) -> *mut u32;
    fn FUN_00487cb0(a: i32, b: *mut f32, c: *mut f32) -> i32;
    fn FUN_004150a0(a: *mut f32, b: *mut f32);
    fn FUN_00414fe0(a: *mut f32, b: *mut f32);
    fn FUN_004151d0(a: *mut f32, b: *mut f32);
    fn FUN_00415040(a: *mut f32, b: *mut f32) -> f64;
    fn FUN_00415060(a: *mut f32, b: i32) -> f64;
    fn FUN_00415080(a: *mut f32, b: i32) -> f64;
}

unsafe fn ru(p: *mut u32, i: usize) -> u32 {
    // SAFETY: caller supplies a valid game structure pointer.
    unsafe { p.add(i).read_unaligned() }
}

unsafe fn wu(p: *mut u32, i: usize, v: u32) {
    // SAFETY: caller supplies a valid game structure pointer.
    unsafe { p.add(i).write_unaligned(v) }
}

unsafe fn rf(p: *mut f32, i: usize) -> f32 {
    // SAFETY: caller supplies a valid game structure pointer.
    unsafe { p.add(i).read_unaligned() }
}

unsafe fn wf(p: *mut f32, i: usize, v: f32) {
    // SAFETY: caller supplies a valid game structure pointer.
    unsafe { p.add(i).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00487800(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: *mut f32,
    param_5: *mut f32,
) -> u32 {
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    let mut local_d0: [u32; 52] = [0; 52];
    // SAFETY: all pointers are the game's own structures, accessed exactly
    // where the original accesses them; globals and callees are the game's.
    unsafe {
        if param_2.is_null() {
            return 0;
        }
        while ru(param_2, 1) == 7 {
            let str2 = ru(param_2, 5) as *mut i8;
            let mut local_d4: i32 = 0;
            let mut cnt: i32 = ((param_1 as u32).wrapping_add(6) as *const i32).read_unaligned();
            'search: {
                if 0 < cnt {
                    let mut local_d8: u32 = 0;
                    loop {
                        let base: u32 =
                            ((param_1 as u32).wrapping_add(10) as *const u32).read_unaligned();
                        let mut p5 = (base.wrapping_add(local_d8) as *const u32).read_unaligned()
                            as *mut u32;
                        if p5.is_null() {
                            p5 = core::ptr::null_mut();
                        } else if ru(p5, 0) == 0
                            || __strcmpi(ru(p5, 0) as *const i8, str2 as *const i8) != 0
                        {
                            let mut iv4: u32 = 0;
                            let mut iv7: i32 = 0;
                            if 0 < ru(p5, 2) as i32 {
                                loop {
                                    let q = ((ru(p5, 3).wrapping_add(iv4)) as *const u32)
                                        .read_unaligned()
                                        as *mut u32;
                                    param_2 = FUN_0048e520(q, str2);
                                    if !param_2.is_null() {
                                        break 'search;
                                    }
                                    iv4 = iv4.wrapping_add(4);
                                    iv7 = iv7.wrapping_add(1);
                                    if !(iv7 < ru(p5, 2) as i32) {
                                        break;
                                    }
                                }
                            }
                            p5 = core::ptr::null_mut();
                        }
                        param_2 = p5;
                        if !p5.is_null() {
                            break 'search;
                        }
                        local_d8 = local_d8.wrapping_add(4);
                        local_d4 = local_d4.wrapping_add(1);
                        cnt = ((param_1 as u32).wrapping_add(6) as *const i32).read_unaligned();
                        if !(local_d4 < cnt) {
                            break;
                        }
                    }
                }
                param_2 = core::ptr::null_mut();
            }
            if param_2.is_null() {
                return 0;
            }
        }
        let f3 = param_3 as *mut f32;
        let pf2 = ru(param_2, 5) as *mut f32;
        if !pf2.is_null() {
            match ru(param_2, 1) {
                1 | 0xf | 0x10 | 0x11 => {
                    let mut p8: u32 = (pf2 as u32).wrapping_add(4);
                    wu(param_3, 0x23, ru(param_2, 4));
                    wu(param_3, 0x24, (pf2 as *const u32).read_unaligned());
                    wu(param_3, 0x28, p8);
                    p8 = p8.wrapping_add(ru(param_2, 4).wrapping_mul(0x10).wrapping_mul(4));
                    wu(param_3, 0x25, p8);
                    let iv4 = ru(param_2, 4) as i32;
                    let t = p8.wrapping_add((iv4 as u32).wrapping_mul(3).wrapping_mul(4));
                    wu(param_3, 0x26, t);
                    if ru(param_3, 0x21) == 0 {
                        wu(
                            param_3,
                            0x21,
                            t.wrapping_add(ru(param_2, 4).wrapping_mul(6).wrapping_mul(4)),
                        );
                        let r = FUN_00487cb0(param_3 as i32, param_4, param_5);
                        if r != 0 {
                            return 1;
                        }
                        wu(param_3, 0x21, 0);
                    } else {
                        let r = FUN_00487cb0(param_3 as i32, param_4, param_5);
                        if r != 0 {
                            return 1;
                        }
                    }
                }
                2 => {
                    let p8 = f3.add(2);
                    FUN_004150a0(f3.add(5), pf2.add(3));
                    FUN_00414fe0(p8, pf2.add(3));
                    wf(p8, 0, rf(pf2, 0) + rf(p8, 0));
                    let p8 = f3.add(0x11);
                    wf(f3, 3, rf(pf2, 1) + rf(f3, 3));
                    wf(f3, 4, rf(pf2, 2) + rf(f3, 4));
                    FUN_004151d0(p8, pf2.add(3));
                    let v = FUN_00415040(pf2, p8);
                    wf(f3, 0xe, (rf(f3, 0xe) as f64 - v) as f32);
                    let v = FUN_00415060(pf2, p8 as i32);
                    wf(f3, 0xf, (rf(f3, 0xf) as f64 - v) as f32);
                    let v = FUN_00415080(pf2, p8 as i32);
                    wf(f3, 0x10, (rf(f3, 0x10) as f64 - v) as f32);
                }
                3 => {
                    wu(param_3, 0x1e, ru(param_2, 4));
                    wu(param_3, 0x1f, pf2 as u32);
                }
                4 => {
                    wf(f3, 2, rf(f3, 2) + rf(pf2, 0));
                    let p8 = f3.add(0x11);
                    wf(f3, 3, rf(f3, 3) + rf(pf2, 1));
                    wf(f3, 4, rf(f3, 4) + rf(pf2, 2));
                    let v = FUN_00415040(pf2, p8);
                    wf(f3, 0xe, (rf(f3, 0xe) as f64 - v) as f32);
                    let v = FUN_00415060(pf2, p8 as i32);
                    wf(f3, 0xf, (rf(f3, 0xf) as f64 - v) as f32);
                    let v = FUN_00415080(pf2, p8 as i32);
                    wf(f3, 0x10, (rf(f3, 0x10) as f64 - v) as f32);
                }
                5 => {
                    FUN_004150a0(f3.add(5), pf2);
                    FUN_00414fe0(f3.add(2), pf2);
                    FUN_004151d0(f3.add(0x11), pf2);
                }
                6 => {
                    let a = rf(pf2, 0);
                    let b = rf(pf2, 1);
                    let c = rf(pf2, 2);
                    wf(f3, 5, rf(f3, 5) * a);
                    wf(f3, 6, rf(f3, 6) * b);
                    wf(f3, 7, rf(f3, 7) * c);
                    wf(f3, 8, rf(f3, 8) * a);
                    wf(f3, 9, rf(f3, 9) * b);
                    wf(f3, 10, rf(f3, 10) * c);
                    wf(f3, 0xb, rf(f3, 0xb) * a);
                    wf(f3, 0xc, b * rf(f3, 0xc));
                    wf(f3, 0xd, c * rf(f3, 0xd));
                    wf(f3, 2, rf(f3, 2) * a);
                    wf(f3, 3, rf(f3, 3) * b);
                    wf(f3, 4, rf(f3, 4) * c);
                    let f = _DAT_005201dc / rf(pf2, 0);
                    wf(f3, 0x11, rf(f3, 0x11) * f);
                    wf(f3, 0x12, rf(f3, 0x12) * f);
                    wf(f3, 0x13, f * rf(f3, 0x13));
                    let f = _DAT_005201dc / rf(pf2, 1);
                    wf(f3, 0x14, rf(f3, 0x14) * f);
                    wf(f3, 0x15, rf(f3, 0x15) * f);
                    wf(f3, 0x16, f * rf(f3, 0x16));
                    let f = _DAT_005201dc / rf(pf2, 2);
                    wf(f3, 0x17, rf(f3, 0x17) * f);
                    wf(f3, 0x18, rf(f3, 0x18) * f);
                    wf(f3, 0x19, f * rf(f3, 0x19));
                }
                0xb => {
                    _DAT_006026c0 = pf2;
                    wu(param_3, 0x21, pf2 as u32);
                }
                _ => {}
            }
        }
        let mut iv4: u32 = 0;
        if ru(param_2, 2) != 0 {
            for k in 0..0x34usize {
                local_d0[k] = ru(param_3, 0);
                param_3 = param_3.add(1);
            }
            DAT_00602678 = 0;
            DAT_006026a0 = 0;
            let mut iv7: i32 = 0;
            _DAT_006026c0 = core::ptr::null_mut();
            _DAT_006026a8 = 0;
            DAT_00602690 = 0;
            DAT_006026bc = 0;
            if 0 < ru(param_2, 2) as i32 {
                loop {
                    let child = ((ru(param_2, 3).wrapping_add(iv4)) as *const u32).read_unaligned()
                        as *mut u32;
                    let r = FUN_00487800(param_1, child, local_d0.as_mut_ptr(), param_4, param_5);
                    if r != 0 {
                        return 1;
                    }
                    iv4 = iv4.wrapping_add(4);
                    iv7 = iv7.wrapping_add(1);
                    if !(iv7 < ru(param_2, 2) as i32) {
                        break;
                    }
                }
            }
        }
        0
    }
}
