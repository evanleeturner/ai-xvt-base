// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081bcd0: u32;
    static mut DAT_0081b6fc: u16;
    static mut DAT_0081bfba: [u8; 4];
    static mut DAT_0081bfb6: [u8; 4];
    static mut DAT_00757cee: [u8; 1];
    static mut DAT_00743860: [i32; 8];
    static mut DAT_007579d0: [i32; 1];
    static mut DAT_00757870: [i32; 1];
    static mut DAT_00757680: [i32; 1];
    static mut DAT_00757a28: i32;
    static mut DAT_0081b850: i32;
    static mut DAT_0081bc80: [u8; 1];
    static mut DAT_0081b740: [i8; 1];
    static mut DAT_005321c4: [i8; 1];
    static mut s__c_s__c_s_00531334: [i8; 1];
    static mut s__s__s_00533618: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "sprintf"]
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004cde00() -> i32;
    fn FUN_004dcfa0(param_1: i32) -> *mut i8;
    fn FUN_004da7c0(param_1: i32, param_2: *mut i8, param_3: i32, param_4: i32, param_5: u32);
    fn FUN_004cdbc0() -> i32;
    fn FUN_004dcfd0(param_1: *mut u32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004d5180(param_1: *mut u32);
    fn FUN_004d51b0(param_1: *mut u32);
    fn FUN_004ecce0(param_1: i32) -> u32;
    fn FUN_004ec8d0(param_1: i32) -> i32;
    fn FUN_004eca20(param_1: i32) -> i8;
    fn FUN_004ecba0(param_1: i32) -> i8;
    fn FUN_004daac0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut u32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
}

unsafe fn color(flagp: *const i32) -> u32 {
    // SAFETY: globals and the flag pointer belong to the game.
    unsafe {
        let mut u9 = DAT_0081b6fc;
        if *flagp == 0 {
            u9 = DAT_0081bcd0 as u16;
        }
        u9 as u32
    }
}

unsafe fn head(
    rec: *mut u8,
    flagp: *const i32,
    ival0: i32,
    ival1: i32,
    y: i32,
    u5: u32,
    param_1: i32,
) {
    // SAFETY: game functions and globals; rec points at a game record.
    unsafe {
        let mut local_30 = [0u32; 4];
        let mut local_20 = [0u32; 4];
        FUN_004dcfd0(local_30.as_mut_ptr(), 0x58, y, 0xe2, y.wrapping_add(0xf));
        FUN_004d5180(local_20.as_mut_ptr());
        FUN_004d51b0(local_30.as_mut_ptr());
        let p_name = rec.wrapping_sub(0xe) as *mut i8;
        let mut u6 = u5;
        let buf = (&raw mut DAT_0081b740).cast::<i8>();
        if *flagp == 0 {
            let pc = FUN_004dcfa0(ival1.wrapping_add(0x8e));
            _sprintf(
                buf,
                (&raw const s__c_s__c_s_00531334).cast::<i8>(),
                6i32,
                pc,
                1i32,
                p_name,
            );
            let r = FUN_004cdbc0();
            if r == ival0 || DAT_0081b850 == 2 {
                let m = ((param_1 % 0x18) as u32) & 0xffff_fffe;
                let off = m.wrapping_mul(2);
                let p = (&raw mut DAT_0081bc80)
                    .cast::<u8>()
                    .wrapping_add(off as usize);
                u6 = p.cast::<u32>().read_unaligned();
            }
        } else {
            let pc = FUN_004dcfa0(ival1.wrapping_add(0x8e));
            _sprintf(buf, (&raw const s__s__s_00533618).cast::<i8>(), pc, p_name);
        }
        FUN_004da7c0(0xc, buf, 0x58, y, u6);
        FUN_004d51b0(local_20.as_mut_ptr());
    }
}

unsafe fn cols(idx: i32, y: i32, c: u32) {
    // SAFETY: game functions.
    unsafe {
        let u4 = FUN_004ecce0(idx);
        let pc = FUN_004dcfa0(u4.wrapping_add(0x15) as i32);
        FUN_004da7c0(0xc, pc, 0xe4, y, c);
        let v = FUN_004ec8d0(idx);
        let pc = FUN_004dcfa0(v.wrapping_add(0x236));
        FUN_004da7c0(0xc, pc, 0x152, y, c);
        let c1 = FUN_004eca20(idx);
        let pc = FUN_004dcfa0((c1 as u8 as u32).wrapping_add(0x240) as i32);
        FUN_004da7c0(0xc, pc, 0x17a, y, c);
        let c2 = FUN_004ecba0(idx);
        let pc = FUN_004dcfa0((c2 as u8 as u32).wrapping_add(0x245) as i32);
        FUN_004da7c0(0xc, pc, 0x193, y, c);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004eaed0(param_1: i32) {
    // SAFETY: all globals and callees are the game's own; record pointers
    // walk the game's tables exactly as the original does.
    unsafe {
        let base = (&raw mut DAT_00743860).cast::<i32>();
        let end = base.wrapping_add(8);
        let recs = (&raw mut DAT_00757cee).cast::<u8>();
        let a870 = (&raw mut DAT_00757870).cast::<i32>();
        let a9d0 = (&raw mut DAT_007579d0).cast::<i32>();
        let a680 = (&raw mut DAT_00757680).cast::<i32>();

        let n = FUN_004cde00();
        let mut iv2: i32 = 0i32.wrapping_sub(n.wrapping_mul(0xf) >> 1);
        let iv8 = iv2.wrapping_add(0x155);

        let u6 = DAT_0081bcd0;
        let pc = FUN_004dcfa0(0xae);
        FUN_004da7c0(0xc, pc, 0x58, iv8, u6);
        let u6 = DAT_0081bcd0;
        let pc = FUN_004dcfa0(0x232);
        FUN_004da7c0(0xc, pc, 0xe4, iv8, u6);
        let u6 = DAT_0081bcd0;
        let pc = FUN_004dcfa0(0x233);
        FUN_004da7c0(0xc, pc, 0x152, iv8, u6);
        let u6 = DAT_0081bcd0;
        let pc = FUN_004dcfa0(0x234);
        FUN_004da7c0(0xc, pc, 0x17a, iv8, u6);
        let u6 = DAT_0081bcd0;
        let pc = FUN_004dcfa0(0x235);
        iv2 = iv2.wrapping_add(0x164);
        FUN_004da7c0(0xc, pc, 0x193, iv8, u6);
        FUN_004cde00();

        let bfba = (&raw mut DAT_0081bfba).cast::<i32>().read_unaligned();
        let first = bfba == 0 || bfba == 3 || bfba == 4;

        // loop 1 (both branches)
        {
            let mut idx: i32 = 0;
            let mut rec = recs;
            let mut flag = base;
            loop {
                let ival0 = rec.cast::<i32>().read_unaligned();
                if ival0 != 0 && FUN_004cdbc0() == ival0 {
                    let ival1 = rec.add(4).cast::<i32>().read_unaligned();
                    let c = color(flag);
                    head(rec, flag, ival0, ival1, iv2, c, param_1);
                    cols(idx, iv2, c);
                    iv2 = iv2.wrapping_add(0xf);
                }
                rec = rec.add(0x2a);
                flag = flag.add(1);
                idx = idx.wrapping_add(1);
                if !(flag < end) {
                    break;
                }
            }
        }

        if first {
            // loop 2
            {
                let mut idx: i32 = 0;
                let mut rec = recs;
                let mut flag = base;
                loop {
                    let ival0 = rec.cast::<i32>().read_unaligned();
                    if ival0 != 0 && FUN_004cdbc0() != ival0 {
                        let bfb6 = (&raw mut DAT_0081bfb6).cast::<i32>().read_unaligned();
                        let mut i12: i32 = 0;
                        let i8v = a9d0.offset(bfb6 as isize).read();
                        if 0 < i8v {
                            loop {
                                let k = i12.wrapping_add((bfb6 as u32).wrapping_mul(8) as i32);
                                if a870.offset(k as isize).read() == ival0 {
                                    break;
                                }
                                i12 = i12.wrapping_add(1);
                                if !(i12 < i8v) {
                                    break;
                                }
                            }
                        }
                        if i8v != i12 {
                            let ival1 = rec.add(4).cast::<i32>().read_unaligned();
                            let c = color(flag);
                            head(rec, flag, ival0, ival1, iv2, c, param_1);
                            cols(idx, iv2, c);
                            iv2 = iv2.wrapping_add(0xf);
                        }
                    }
                    rec = rec.add(0x2a);
                    flag = flag.add(1);
                    idx = idx.wrapping_add(1);
                    if !(flag < end) {
                        break;
                    }
                }
            }

            // loop 3
            let mut flag = base;
            let mut rec = recs;
            loop {
                let ival0 = rec.cast::<i32>().read_unaligned();
                if ival0 != 0 && FUN_004cdbc0() != ival0 {
                    let bfb6 = (&raw mut DAT_0081bfb6).cast::<i32>().read_unaligned();
                    let mut i8v: i32 = 0;
                    let i13 = a9d0.offset(bfb6 as isize).read();
                    let mut b10 = i13 == 0;
                    if 0 < i13 {
                        loop {
                            let k = i8v.wrapping_add((bfb6 as u32).wrapping_mul(8) as i32);
                            if a870.offset(k as isize).read() == ival0 {
                                break;
                            }
                            i8v = i8v.wrapping_add(1);
                            if !(i8v < i13) {
                                break;
                            }
                        }
                        b10 = i13 == i8v;
                    }
                    if b10 {
                        let ival1 = rec.add(4).cast::<i32>().read_unaligned();
                        let c = color(flag);
                        head(rec, flag, ival0, ival1, iv2, c, param_1);
                        let mut local_10 = [0u32; 4];
                        FUN_004dcfd0(
                            local_10.as_mut_ptr(),
                            0xe4,
                            iv2,
                            0x193,
                            iv2.wrapping_add(0xe),
                        );
                        let pc = FUN_004dcfa0(0x249);
                        iv2 = iv2.wrapping_add(0xf);
                        FUN_004daac0(0xc, pc, local_10.as_mut_ptr(), 1, 0, c);
                    }
                }
                rec = rec.add(0x2a);
                flag = flag.add(1);
                if 0x74387fusize < flag as usize {
                    return;
                }
            }
        }

        // else branch: loop 2
        let mut idx: i32 = 0;
        let mut rec = recs;
        let mut flag = base;
        loop {
            let ival0 = rec.cast::<i32>().read_unaligned();
            let mut i13 = iv2;
            if ival0 != 0 && FUN_004cdbc0() != ival0 {
                let ival1 = rec.add(4).cast::<i32>().read_unaligned();
                let c = color(flag);
                i13 = 0;
                let mut b10 = false;
                let mut l34: i32 = 0;
                if 0 < DAT_00757a28 {
                    let mut l3c = a870;
                    let mut l38 = a9d0;
                    loop {
                        let mut i12: i32 = 0;
                        let i8v = l38.read();
                        if 0 < i8v {
                            let mut ps = l3c;
                            loop {
                                if ps.read() == ival0 {
                                    if a680.offset(i13.wrapping_add(i12) as isize).read() == -1 {
                                        b10 = true;
                                    }
                                    break;
                                }
                                ps = ps.wrapping_add(1);
                                i12 = i12.wrapping_add(1);
                                if !(i12 < i8v) {
                                    break;
                                }
                            }
                            if i12 < i8v {
                                break;
                            }
                        }
                        i13 = i13.wrapping_add(8);
                        l3c = l3c.wrapping_add(8);
                        l38 = l38.wrapping_add(1);
                        l34 = l34.wrapping_add(1);
                        if !(l34 < DAT_00757a28) {
                            break;
                        }
                    }
                }
                i13 = iv2.wrapping_add(0xf);
                head(rec, flag, ival0, ival1, iv2, c, param_1);
                if b10 {
                    let s = (&raw mut DAT_005321c4).cast::<i8>();
                    FUN_004da7c0(0xc, s, 0xe4, iv2, c);
                    FUN_004da7c0(0xc, s, 0x152, iv2, c);
                    FUN_004da7c0(0xc, s, 0x17a, iv2, c);
                    FUN_004da7c0(0xc, s, 0x193, iv2, c);
                } else {
                    cols(idx, iv2, c);
                }
            }
            rec = rec.add(0x2a);
            flag = flag.add(1);
            idx = idx.wrapping_add(1);
            iv2 = i13;
            if 0x74387fusize < flag as usize {
                return;
            }
        }
    }
}
