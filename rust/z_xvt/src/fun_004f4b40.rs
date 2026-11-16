// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757870: [i32; 0];
    static mut DAT_007579d0: [i32; 0];
    static mut DAT_0081bfb6: [u8; 4];
    static mut DAT_00757680: [i32; 0];
    static mut DAT_00757ce0: [u8; 0];
    static mut DAT_007579f8: u32;
    static mut DAT_00757a00: [u32; 0];
    static mut DAT_00757a20: i32;
    static mut DAT_0081b850: i32;
    static mut DAT_0081b854: u32;
    static mut DAT_0081bcb0: u32;
    static mut DAT_0081bcd0: u32;
    static mut DAT_0081b6fc: u32;
    static mut DAT_0081bc80: [u8; 0];
    static mut DAT_0081b740: [i8; 0];
    static mut DAT_00758620: u32;
    static mut DAT_00758624: u32;
    static mut DAT_00758628: i32;
    static mut DAT_0075862c: i32;
    static mut DAT_0075766c: i16;
    static mut DAT_00743935: i8;
    static mut DAT_0085e7c5: i8;
    static DAT_00531334: [i8; 0];
    static DAT_00533618: [i8; 0];
    static DAT_00533edc: [i8; 0];
    static DAT_0053309c: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(p: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004dcfa0(a: u32) -> *mut i8;
    fn FUN_004daac0(a: i32, s: *mut i8, r: *mut i32, b: i32, c: i32, d: u32);
    fn FUN_004cdbc0() -> i32;
    fn FUN_004da920(a: i32, s: *mut i8, r: *mut i32, d: u32);
    fn FUN_004d5180(p: *mut i32);
    fn FUN_004d51b0(p: *mut i32);
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004db630(a: i32) -> u8;
    fn FUN_004ddc70(a: *mut i32, b: *mut i32);
    fn FUN_004db530() -> u8;
    fn FUN_004db550() -> u8;
    fn FUN_004dd7d0(p: *mut i32, a: i32, b: i32) -> i32;
    fn FUN_004db510(a: i32);
    fn FUN_004cb830(a: i32, p: *mut u32, b: i32);
    fn FUN_004dd030(p: *mut i32, a: i32, b: i32);
    fn FUN_004db5f0(a: i32) -> u8;
    fn FUN_004de190(s: *const i8, a: i32, b: i32, c: i32);
    fn FUN_004db610(a: i32) -> u8;
    fn FUN_004db520();
}

unsafe fn lb() -> u32 {
    // SAFETY: the game's own 4 bytes.
    unsafe { (&raw const DAT_0081bfb6).cast::<u32>().read_unaligned() }
}
unsafe fn r870(i: u32) -> i32 {
    // SAFETY: game array.
    unsafe {
        (&raw mut DAT_00757870)
            .cast::<i32>()
            .wrapping_add(i as usize)
            .read_unaligned()
    }
}
unsafe fn r680(i: u32) -> i32 {
    // SAFETY: game array.
    unsafe {
        (&raw mut DAT_00757680)
            .cast::<i32>()
            .wrapping_add(i as usize)
            .read_unaligned()
    }
}
unsafe fn w680(i: u32, v: i32) {
    // SAFETY: game array.
    unsafe {
        (&raw mut DAT_00757680)
            .cast::<i32>()
            .wrapping_add(i as usize)
            .write_unaligned(v)
    }
}
unsafe fn cnt() -> i32 {
    // SAFETY: game array indexed by lb().
    unsafe {
        (&raw mut DAT_007579d0)
            .cast::<i32>()
            .wrapping_add(lb() as usize)
            .read_unaligned()
    }
}
unsafe fn ga00(i: u32) -> *mut u32 {
    // SAFETY: address computation only.
    unsafe {
        (&raw mut DAT_00757a00)
            .cast::<u32>()
            .wrapping_add(i as usize)
    }
}
unsafe fn ce0(off: u32) -> *mut u8 {
    // SAFETY: address computation only.
    unsafe {
        (&raw mut DAT_00757ce0)
            .cast::<u8>()
            .wrapping_add(off as usize)
    }
}
unsafe fn rce(off: u32) -> u32 {
    // SAFETY: game table read.
    unsafe { ce0(off).cast::<u32>().read_unaligned() }
}
unsafe fn buf() -> *mut i8 {
    // SAFETY: address computation only.
    unsafe { (&raw mut DAT_0081b740).cast::<i8>() }
}
unsafe fn sel(param_1: i32) -> u32 {
    // SAFETY: game table read.
    unsafe {
        (&raw const DAT_0081bc80)
            .cast::<u8>()
            .wrapping_add((((param_1 % 0x18) as u32 & 0xfffffffe).wrapping_mul(2)) as usize)
            .cast::<u32>()
            .read_unaligned()
    }
}
unsafe fn p620() -> *mut u32 {
    // SAFETY: address computation only.
    unsafe { &raw mut DAT_00758620 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f4b40(param_1: i32) -> u32 {
    let mut l34: [i32; 4] = [0; 4];
    let mut l20: [i32; 4] = [0; 4];
    let mut l10: [i32; 4] = [0; 4];
    let mut l38: i32 = 0;
    let mut l3c: i32 = 0;
    let mut u18: u32;
    let mut u15: u32;
    let mut iv14: i32;
    let mut iv16: i32;
    let mut iv12: i32;
    // SAFETY: all globals and callees are the game's own; pointers are
    // derived from those globals and local arrays.
    unsafe {
        FUN_004dcfd0(l34.as_mut_ptr(), 0x58, 0x14f, 0xe5, 0x15d);
        let pc = FUN_004dcfa0(0x265);
        FUN_004daac0(0xc, pc, l34.as_mut_ptr(), 0, 1, 0xffff);
        FUN_004dcfd0(l34.as_mut_ptr(), 0xe8, 0x14f, 0x150, 0x15d);
        let pc = FUN_004dcfa0(0x266);
        FUN_004daac0(0xc, pc, l34.as_mut_ptr(), 0, 1, 0xffff);
        FUN_004dcfd0(l34.as_mut_ptr(), 0x153, 0x14f, 0x1bb, 0x15d);
        let pc = FUN_004dcfa0(0x29f);
        FUN_004daac0(0xc, pc, l34.as_mut_ptr(), 0, 1, 0xffff);
        iv14 = FUN_004cdbc0();
        let l24: u32 = (iv14 == r870(lb().wrapping_mul(8))) as u32;
        if 1 < cnt() {
            FUN_004dcfd0(l34.as_mut_ptr(), 0x58, 0x1a6, 0x1bb, 0x1b4);
            if l24 == 0 && DAT_0081b850 != 2 {
                u15 = 0x268;
                u18 = DAT_0081b854;
            } else {
                u15 = 0x267;
                u18 = DAT_0081bcb0;
            }
            let pc = FUN_004dcfa0(u15);
            FUN_004da920(0xc, pc, l34.as_mut_ptr(), u18);
        }
        let mut count: i32 = 0;
        iv16 = 0;
        FUN_004dcfd0(l34.as_mut_ptr(), 0x153, 0x160, 0x1bb, 0x170);
        iv14 = DAT_00757a20;
        if 0 < cnt() {
            'outer: loop {
                let mut iv8: i32 = 0;
                let mut p13 = ce0(0xe);
                iv12 = iv16.wrapping_add(lb().wrapping_mul(8) as i32);
                loop {
                    let t = p13.cast::<i32>().read_unaligned();
                    if t == r870(iv12 as u32) {
                        break;
                    }
                    p13 = p13.wrapping_add(0x2a);
                    iv8 = iv8.wrapping_add(1);
                    if !((p13 as usize) < (ce0(0x15e) as usize)) {
                        break;
                    }
                }
                if iv8 != 8 && r680(iv12 as u32) == -1 {
                    let iv9: i32 = iv8.wrapping_mul(0x2a);
                    u18 = rce((iv9 as u32).wrapping_add(0xe));
                    if u18 != DAT_007579f8 && r870(iv12 as u32) != 0 {
                        iv12 = 0;
                        if 0 < iv14 {
                            let mut p5 = ga00(0);
                            loop {
                                if p5.read() == u18 {
                                    break;
                                }
                                p5 = p5.wrapping_add(1);
                                iv12 = iv12.wrapping_add(1);
                                if !(iv12 < iv14) {
                                    break;
                                }
                            }
                        }
                        FUN_004d5180(l10.as_mut_ptr());
                        FUN_004d51b0(l34.as_mut_ptr());
                        iv14 = ce0(iv9 as u32) as usize as i32;
                        if iv12 == DAT_00757a20 {
                            let u17: u32 = 1;
                            let t = rce((iv9 as u32).wrapping_add(0x12));
                            let pc = FUN_004dcfa0(t.wrapping_add(0x8e));
                            sprintf(buf(), (&raw const DAT_00531334).cast::<i8>(), 6i32, pc, u17);
                            iv14 = FUN_004cdbc0();
                            let t = rce((iv9 as u32).wrapping_add(0xe)) as i32;
                            if iv14 == t {
                                u18 = sel(param_1);
                            } else {
                                u18 = DAT_0081bcd0;
                                if DAT_0081b850 == 2 {
                                    u18 = sel(param_1);
                                }
                            }
                        } else {
                            let t = rce((iv9 as u32).wrapping_add(0x12));
                            let pc = FUN_004dcfa0(t.wrapping_add(0x8e));
                            sprintf(buf(), (&raw const DAT_00533618).cast::<i8>(), pc, iv14);
                            u18 = DAT_0081b6fc;
                        }
                        FUN_004daac0(0xc, buf(), l34.as_mut_ptr(), 0, 1, u18);
                        FUN_004d51b0(l10.as_mut_ptr());
                        count = count.wrapping_add(1);
                        iv14 = FUN_004cdbc0();
                        if (iv14 == r870(lb().wrapping_mul(8)) || DAT_0081b850 == 2)
                            && FUN_004db630(3) == 0
                        {
                            FUN_004ddc70(&raw mut l38, &raw mut l3c);
                            if (FUN_004db530() != 0 || FUN_004db550() != 0)
                                && FUN_004dd7d0(l34.as_mut_ptr(), l38, l3c) != 0
                            {
                                if DAT_0081b850 == 2 {
                                    DAT_007579f8 = rce((iv9 as u32).wrapping_add(0xe));
                                    FUN_004db510(3);
                                } else {
                                    iv14 = 0;
                                    if 0 < DAT_00757a20 {
                                        let mut p11 = ga00(0).cast::<i32>();
                                        loop {
                                            let t = rce((iv9 as u32).wrapping_add(0xe)) as i32;
                                            if p11.read() == t {
                                                break;
                                            }
                                            p11 = p11.wrapping_add(1);
                                            iv14 = iv14.wrapping_add(1);
                                            if !(iv14 < DAT_00757a20) {
                                                break;
                                            }
                                        }
                                    }
                                    if iv14 == DAT_00757a20 {
                                        DAT_00758624 = rce((iv9 as u32).wrapping_add(0xe));
                                        DAT_00757a20 = DAT_00757a20.wrapping_add(1);
                                        ga00(iv14 as u32).write(DAT_00758624);
                                        DAT_00758620 = 0x6b;
                                        DAT_00758628 = FUN_004cdbc0();
                                        FUN_004cb830(0, p620(), 0xc);
                                        DAT_007579f8 = rce((iv9 as u32).wrapping_add(0xe));
                                        iv14 = 0;
                                        FUN_004db510(3);
                                        u18 = DAT_007579f8;
                                        if 0 < cnt() {
                                            loop {
                                                iv12 =
                                                    iv14.wrapping_add(lb().wrapping_mul(8) as i32);
                                                if r870(iv12 as u32) as u32 == u18
                                                    && r680(iv12 as u32) != -1
                                                {
                                                    DAT_00758620 = 0x50;
                                                    DAT_00758624 = lb();
                                                    DAT_0075862c = -1;
                                                    DAT_00758628 = iv14;
                                                    FUN_004cb830(0, p620(), 0x14);
                                                    u18 = DAT_007579f8;
                                                }
                                                iv14 = iv14.wrapping_add(1);
                                                if !(iv14 < cnt()) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        FUN_004dd030(l34.as_mut_ptr(), 0, 0x11);
                        iv14 = DAT_00757a20;
                        if 3 < count {
                            break 'outer;
                        }
                    }
                }
                iv16 = iv16.wrapping_add(1);
                if !(iv16 < cnt()) {
                    break;
                }
            }
        }
        iv16 = 0;
        FUN_004ddc70(&raw mut l38, &raw mut l3c);
        FUN_004dcfd0(l34.as_mut_ptr(), 0x58, 0x160, 0xe5, 0x170);
        iv14 = 0;
        FUN_004dcfd0(l20.as_mut_ptr(), 0xe8, 0x160, 0x14c, 0x170);
        if 0 < DAT_0075766c as i32 {
            let mut p48: *mut i8 = &raw mut DAT_00743935;
            loop {
                if p48.offset(-0x51).read() != 0 && (p48.offset(-0x5d).read() as u8 as u32) == lb()
                {
                    let b10: u8 = match p48.offset(-0x5e).read() {
                        0 => 2,
                        1 | 4 => 3,
                        2 => 5,
                        3 => 4,
                        5 => 6,
                        _ => 1,
                    };
                    iv16 = iv16.wrapping_add(1);
                    let u17: u32 = 1;
                    let pc4 = p48.offset(-0xb5);
                    let pc6 =
                        FUN_004dcfa0((p48.offset(-99).read() as u8 as u32).wrapping_add(0x253));
                    sprintf(
                        buf(),
                        (&raw const DAT_00533edc).cast::<i8>(),
                        iv16,
                        b10 as u32,
                        pc6,
                        pc4,
                        u17,
                    );
                    let mut p4 = p48;
                    if p48.read() == 0 {
                        p4 = FUN_004dcfa0(0x264);
                    }
                    u18 = 0xffffffff;
                    let mut p6: *mut i8;
                    loop {
                        p6 = p4;
                        if u18 == 0 {
                            break;
                        }
                        u18 = u18.wrapping_sub(1);
                        p6 = p4.wrapping_add(1);
                        let c = p4.read();
                        p4 = p6;
                        if c == 0 {
                            break;
                        }
                    }
                    u18 = !u18;
                    iv12 = -1;
                    p4 = buf();
                    let mut p13: *mut i8;
                    loop {
                        p13 = p4;
                        if iv12 == 0 {
                            break;
                        }
                        iv12 = iv12.wrapping_sub(1);
                        p13 = p4.wrapping_add(1);
                        let c = p4.read();
                        p4 = p13;
                        if c == 0 {
                            break;
                        }
                    }
                    p4 = p6.wrapping_sub(u18 as usize);
                    p6 = p13.wrapping_sub(1);
                    u15 = u18 >> 2;
                    while u15 != 0 {
                        let t = p4.cast::<u32>().read_unaligned();
                        p6.cast::<u32>().write_unaligned(t);
                        p4 = p4.wrapping_add(4);
                        p6 = p6.wrapping_add(4);
                        u15 = u15.wrapping_sub(1);
                    }
                    u18 &= 3;
                    while u18 != 0 {
                        p6.write(p4.read());
                        p4 = p4.wrapping_add(1);
                        p6 = p6.wrapping_add(1);
                        u18 = u18.wrapping_sub(1);
                    }
                    iv12 = -1;
                    let mut iv8: i32 = 0;
                    FUN_004daac0(0xc, buf(), l34.as_mut_ptr(), 0, 1, 0xffff);
                    if 0 < cnt() {
                        loop {
                            let mut iv9: i32 = iv8.wrapping_add(lb().wrapping_mul(8) as i32);
                            if r680(iv9 as u32) == iv14 {
                                let mut iv7: i32 = 0;
                                let mut p13 = ce0(0xe);
                                loop {
                                    iv12 = iv8;
                                    let t = p13.cast::<i32>().read_unaligned();
                                    if t == r870(iv9 as u32) {
                                        let off = (iv7 as u32).wrapping_mul(0x2a);
                                        iv9 = ce0(off) as usize as i32;
                                        let u17: u32 = 1;
                                        let t = rce(off.wrapping_add(0x12));
                                        let pc = FUN_004dcfa0(t.wrapping_add(0x8e));
                                        sprintf(
                                            buf(),
                                            (&raw const DAT_00531334).cast::<i8>(),
                                            6i32,
                                            pc,
                                            u17,
                                            iv9,
                                        );
                                        FUN_004d5180(l10.as_mut_ptr());
                                        FUN_004d51b0(l20.as_mut_ptr());
                                        iv9 = FUN_004cdbc0();
                                        let t = rce(off.wrapping_add(0xe)) as i32;
                                        if iv9 == t {
                                            u18 = sel(param_1);
                                        } else {
                                            u18 = DAT_0081bcd0;
                                            if DAT_0081b850 == 2 {
                                                u18 = sel(param_1);
                                            }
                                        }
                                        FUN_004daac0(0xc, buf(), l20.as_mut_ptr(), 0, 1, u18);
                                        FUN_004d51b0(l10.as_mut_ptr());
                                        break;
                                    }
                                    p13 = p13.wrapping_add(0x2a);
                                    iv7 = iv7.wrapping_add(1);
                                    if !((p13 as usize) < (ce0(0x15e) as usize)) {
                                        break;
                                    }
                                }
                            }
                            iv8 = iv8.wrapping_add(1);
                            if !(iv8 < cnt()) {
                                break;
                            }
                        }
                    }
                    if 1 < cnt() {
                        if iv12 == -1 {
                            if FUN_004db630(3) != 0
                                && (FUN_004db5f0(3) != 0 || FUN_004db5f0(3) != 0)
                                && FUN_004dd7d0(l20.as_mut_ptr(), l38, l3c) != 0
                            {
                                if DAT_0081b850 == 2 {
                                    if DAT_0085e7c5 != 0 {
                                        FUN_004de190(
                                            (&raw const DAT_0053309c).cast::<i8>(),
                                            1,
                                            0,
                                            0xff,
                                        );
                                    }
                                    iv12 = 0;
                                    if 0 < cnt() {
                                        loop {
                                            let ix = lb().wrapping_mul(8).wrapping_add(iv12 as u32);
                                            if r870(ix) as u32 == DAT_007579f8 {
                                                w680(ix, iv14);
                                                break;
                                            }
                                            iv12 = iv12.wrapping_add(1);
                                            if !(iv12 < cnt()) {
                                                break;
                                            }
                                        }
                                    }
                                } else {
                                    DAT_00758628 = 0;
                                    if 0 < cnt() {
                                        loop {
                                            let ix = lb()
                                                .wrapping_mul(8)
                                                .wrapping_add(DAT_00758628 as u32);
                                            if r870(ix) as u32 == DAT_007579f8 {
                                                break;
                                            }
                                            DAT_00758628 = DAT_00758628.wrapping_add(1);
                                            if !(DAT_00758628 < cnt()) {
                                                break;
                                            }
                                        }
                                    }
                                    DAT_00758624 = lb();
                                    DAT_00758620 = 0x50;
                                    DAT_0075862c = iv14;
                                    FUN_004cb830(0, p620(), 0x10);
                                    DAT_00758620 = 0x6a;
                                    DAT_00758624 = DAT_007579f8;
                                    FUN_004cb830(0, p620(), 8);
                                }
                                DAT_007579f8 = 0;
                                FUN_004db520();
                            }
                        } else if FUN_004db630(3) == 0 {
                            if (l24 != 0 || DAT_0081b850 == 2)
                                && (FUN_004db530() != 0 || FUN_004db550() != 0)
                                && FUN_004dd7d0(l20.as_mut_ptr(), l38, l3c) != 0
                            {
                                if DAT_0081b850 == 2 {
                                    let ix = (iv12 as u32).wrapping_add(lb().wrapping_mul(8));
                                    DAT_007579f8 = r870(ix) as u32;
                                    FUN_004db510(3);
                                    let ix = (iv12 as u32).wrapping_add(lb().wrapping_mul(8));
                                    w680(ix, -1);
                                } else {
                                    let mut iv8: i32 = 0;
                                    if 0 < DAT_00757a20 {
                                        let mut p11 = ga00(0).cast::<i32>();
                                        loop {
                                            let ix =
                                                (iv12 as u32).wrapping_add(lb().wrapping_mul(8));
                                            if p11.read() == r870(ix) {
                                                break;
                                            }
                                            p11 = p11.wrapping_add(1);
                                            iv8 = iv8.wrapping_add(1);
                                            if !(iv8 < DAT_00757a20) {
                                                break;
                                            }
                                        }
                                    }
                                    if iv8 == DAT_00757a20 {
                                        DAT_00757a20 = DAT_00757a20.wrapping_add(1);
                                        DAT_00758620 = 0x6b;
                                        let ix = (iv12 as u32).wrapping_add(lb().wrapping_mul(8));
                                        DAT_00758624 = r870(ix) as u32;
                                        ga00(iv8 as u32).write(DAT_00758624);
                                        DAT_00758628 = FUN_004cdbc0();
                                        FUN_004cb830(0, p620(), 0xc);
                                        iv8 = 0;
                                        let ix = (iv12 as u32).wrapping_add(lb().wrapping_mul(8));
                                        DAT_007579f8 = r870(ix) as u32;
                                        FUN_004db510(3);
                                        u18 = DAT_007579f8;
                                        if 0 < cnt() {
                                            loop {
                                                iv12 =
                                                    iv8.wrapping_add(lb().wrapping_mul(8) as i32);
                                                if r870(iv12 as u32) as u32 == u18
                                                    && r680(iv12 as u32) != -1
                                                {
                                                    DAT_00758620 = 0x50;
                                                    DAT_00758624 = lb();
                                                    DAT_0075862c = -1;
                                                    DAT_00758628 = iv8;
                                                    FUN_004cb830(0, p620(), 0x14);
                                                    u18 = DAT_007579f8;
                                                }
                                                iv8 = iv8.wrapping_add(1);
                                                if !(iv8 < cnt()) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else if (FUN_004db5f0(3) != 0 || FUN_004db610(3) != 0)
                            && FUN_004dd7d0(l20.as_mut_ptr(), l38, l3c) != 0
                        {
                            if DAT_0081b850 == 2 {
                                if DAT_0085e7c5 != 0 {
                                    FUN_004de190(
                                        (&raw const DAT_0053309c).cast::<i8>(),
                                        1,
                                        0,
                                        0xff,
                                    );
                                }
                                let mut iv7: i32 = 0;
                                let iv9: u32 = lb().wrapping_mul(8);
                                let iv8: i32 = cnt();
                                w680((iv12 as u32).wrapping_add(iv9), -1);
                                if 0 < iv8 {
                                    loop {
                                        let ix = iv9.wrapping_add(iv7 as u32);
                                        if r870(ix) as u32 == DAT_007579f8 {
                                            w680(ix, iv14);
                                            break;
                                        }
                                        iv7 = iv7.wrapping_add(1);
                                        if !(iv7 < iv8) {
                                            break;
                                        }
                                    }
                                }
                            } else {
                                DAT_00758624 = lb();
                                DAT_00758620 = 0x50;
                                DAT_0075862c = -1;
                                DAT_00758628 = iv12;
                                FUN_004cb830(0, p620(), 0x14);
                                DAT_00758628 = 0;
                                if 0 < cnt() {
                                    loop {
                                        let ix =
                                            lb().wrapping_mul(8).wrapping_add(DAT_00758628 as u32);
                                        if r870(ix) as u32 == DAT_007579f8 {
                                            break;
                                        }
                                        DAT_00758628 = DAT_00758628.wrapping_add(1);
                                        if !(DAT_00758628 < cnt()) {
                                            break;
                                        }
                                    }
                                }
                                DAT_00758624 = lb();
                                DAT_00758620 = 0x50;
                                DAT_0075862c = iv14;
                                FUN_004cb830(0, p620(), 0x10);
                                DAT_00758620 = 0x6a;
                                DAT_00758624 = DAT_007579f8;
                                FUN_004cb830(0, p620(), 8);
                            }
                            DAT_007579f8 = 0;
                            FUN_004db520();
                        }
                    }
                    FUN_004dd030(l20.as_mut_ptr(), 0, 0x11);
                    FUN_004dd030(l34.as_mut_ptr(), 0, 0x11);
                    if 3 < iv16 {
                        return 1;
                    }
                }
                iv14 = iv14.wrapping_add(1);
                p48 = p48.wrapping_add(0x562);
                if !(iv14 < DAT_0075766c as i32) {
                    break;
                }
            }
        }
    }
    1
}
