// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0073b374: u16;
    static mut DAT_006db6c0: u16;
    static mut DAT_00536e8c: u8;
    static mut DAT_006d49a0: i16;
    static mut DAT_006d4a60: u8;
    static mut DAT_006d4a61: u8;
    static mut DAT_006d4a62: u8;
    static mut DAT_006d4a64: u8;
    static mut DAT_006d4a68: u16;
    static mut DAT_006d4a6a: u16;
    static mut DAT_006d4aa2: u8;
    static mut DAT_006d4abe: u8;
    static mut DAT_006d4ac8: u8;
    static mut DAT_006d4ad2: u8;
    static mut DAT_006d4adc: u8;
    static mut DAT_006d4ae6: u8;
    static mut DAT_006d4af0: u8;
    static mut DAT_006d4afa: u8;
    static mut DAT_006d4b04: u8;
    static mut DAT_006d4b0e: u8;
    static mut DAT_006d4b18: u8;
    static mut DAT_006d4b22: u8;
    static mut DAT_006d4b2c: u8;
    static mut DAT_006d4b36: u8;
    static mut DAT_006d4b3a: u8;
    static mut DAT_006d4a02: u8;
    static mut DAT_0070ad04: u8;
    static mut DAT_0070ad05: u8;
    static mut DAT_0070ad06: u8;
    static mut DAT_0070ad07: u8;
    static mut DAT_0070ad08: u8;
    static mut DAT_0070ad40: u8;
    static mut DAT_0070ad41: u8;
    static mut DAT_0070ad42: u8;
    static mut DAT_0070ad43: u8;
    static mut DAT_0070ad48: u8;
    static mut DAT_0070ad51: u8;
    static mut DAT_0070ad54: u8;
    static mut DAT_0070ad55: u8;
    static mut DAT_0070ad5d: u8;
    static mut DAT_0070ad5e: u8;
    static mut DAT_0070ad76: u8;
    static mut DAT_0070ad77: u8;
    static mut DAT_0070b252: u8;
    static mut DAT_005247e8: u8;
    static mut DAT_0052686b: u8;
    static mut DAT_0052687a: u8;
    static mut DAT_005248f8: u8;
    static mut DAT_00524900: u8;
    static mut DAT_007092d4: u8;
    static mut DAT_00709f30: u8;
    static mut DAT_00709f3a: u8;
    static mut DAT_007092e8: u8;
    static mut DAT_00709310: u8;
    static mut DAT_00709338: u8;
    static mut DAT_00709388: u8;
    static mut DAT_00709cec: u8;
    static mut DAT_00709d0a: u8;
    static mut DAT_00709d28: u8;
    static mut DAT_00709b08: u8;
    static mut DAT_0072f508: u8;
    static mut DAT_0071b741: u8;
    static mut DAT_0071b744: u8;
    static mut DAT_0071b745: u8;
    static mut DAT_0071b749: u8;
    static mut DAT_0071b74a: u8;
    static mut DAT_0071b74c: u8;
    static mut DAT_0071b74e: u8;
    static mut DAT_0071b752: u8;
    static mut DAT_0071b759: u8;
    static mut DAT_0071b75b: u8;
    static mut DAT_0071b75d: u8;
    static mut DAT_0071b75f: u8;
    static mut DAT_0071b77b: u8;
    static mut DAT_0071b77d: u8;
    static mut DAT_0071b77f: u8;
    static mut DAT_0071b781: u8;
    static mut DAT_0071b783: u8;
    static mut DAT_0071bae7: u8;
    static mut DAT_0071bb19: u8;
    static mut DAT_0071bb1a: u8;
    static mut DAT_006db586: u16;
    static mut DAT_006db7a4: u16;
    static mut DAT_0070acd0: i32;
    static mut _DAT_0070a13c: u16;
    static mut DAT_0071e6a0: u32;
    static mut _DAT_00732e5a: u16;
    static mut _DAT_0071e6a4: u32;
    static mut DAT_00525580: u8;
    static mut DAT_006fedd0: u16;
    static mut DAT_0071e6a8: u32;
    static mut DAT_0071e6ac: u32;
    static mut DAT_0071e6b0: u32;
    static mut DAT_0071e6b4: u16;
    static mut DAT_0070ac40: u8;
    static mut DAT_0070ac50: u8;
    static mut DAT_0071bcdd: u8;
    static mut DAT_0071eac5: u8;
    static mut DAT_0071bb39: u8;
    static mut DAT_0071e921: u8;
    static mut DAT_0071b775: u8;
    static mut DAT_0071e55d: u8;
    static mut DAT_00703ac4: u8;
    static mut DAT_006db592: u16;
    static mut DAT_006ffbe4: u16;
    static mut DAT_007327e0: u8;
    static mut DAT_0070a1d2: u16;
    static mut DAT_0071e6be: u16;
    static mut _DAT_006db584: u16;
    static mut DAT_0070a178: u8;
    static mut DAT_006db5a8: u32;
    static mut DAT_00709f44: u8;
    static mut DAT_00709f84: u8;
    static mut _DAT_00709ce9: u8;
    static mut DAT_007092c0: u8;
    static mut DAT_007092e4: u32;
    static mut DAT_00709ce8: u8;
    static mut DAT_00709ceb: u8;

    fn FUN_0041eea0(param_1: i32);
    fn FUN_004215c0(param_1: i32);
    fn FUN_004b1bc0(param_1: i32, param_2: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004a6f90();
    fn FUN_004b1730();
    fn FUN_0048f530();
    fn FUN_004a6a80();
}

unsafe fn s8(base: *mut u8, i: isize, v: u8) {
    // SAFETY: base points into the game's own table; offset as in the original.
    unsafe { base.wrapping_offset(i).write(v) }
}

unsafe fn l8(base: *mut u8, i: isize) -> u8 {
    // SAFETY: reads the game's own table at the original's offset.
    unsafe { base.wrapping_offset(i).read() }
}

unsafe fn s16(base: *mut u8, i: isize, v: u16) {
    // SAFETY: unaligned store into the game's own memory.
    unsafe { base.wrapping_offset(i).cast::<u16>().write_unaligned(v) }
}

unsafe fn s32(base: *mut u8, i: isize, v: u32) {
    // SAFETY: unaligned store into the game's own memory.
    unsafe { base.wrapping_offset(i).cast::<u32>().write_unaligned(v) }
}

unsafe fn sw(base: *mut u16, i: isize, v: u16) {
    // SAFETY: store into the game's own u16 table.
    unsafe { base.wrapping_offset(i).write_unaligned(v) }
}

unsafe fn lw(base: *mut u16, i: isize) -> u16 {
    // SAFETY: read of the game's own u16 table.
    unsafe { base.wrapping_offset(i).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_variables
)]
pub unsafe extern "stdcall" fn FUN_0041d620() {
    // SAFETY: all globals, tables and callees are the game's own memory and
    // code; every access mirrors the original's offsets and order.
    unsafe {
        FUN_004a6f90();
        DAT_0073b374 = 0;
        DAT_006db6c0 = 1;
        DAT_00536e8c = 1;
        if 0 < DAT_006d49a0 as i32 {
            loop {
                let u5 = DAT_0073b374 as u32;
                let p21 = (&raw mut DAT_006d4a60).wrapping_offset(u5.wrapping_mul(0x126) as isize);
                let p11 = (&raw mut DAT_006d4a61).wrapping_offset(u5.wrapping_mul(0x126) as isize);
                let mut i7 = u5.wrapping_mul(0x126) as i32;
                p21.write(0);
                s8(&raw mut DAT_006d4a64, i7 as isize, 0);
                p11.write(0);
                sw(&raw mut DAT_006d4a68, u5.wrapping_mul(0x93) as isize, 0);
                s16(&raw mut DAT_006d4a62, i7 as isize, 0);
                let i20 = u5.wrapping_mul(0x566) as i32 as isize;
                let mut local_12 = l8(&raw mut DAT_0070ad43, i20) as u16;
                let mut local_4 = l8(&raw mut DAT_0070ad42, i20) as u32;
                let i18 = ((l8(&raw mut DAT_005247e8, local_4 as isize) as u32).wrapping_mul(0x16))
                    as i32 as isize;
                if l8(&raw mut DAT_0052686b, i18) == 8 {
                    local_12 = local_12.wrapping_mul(local_12);
                }
                local_4 = DAT_007092d4 as u32;
                let b1 = l8(
                    &raw mut DAT_005248f8,
                    l8(&raw mut DAT_0070ad5d, i20) as isize,
                );
                let b17 = l8(&raw mut DAT_00524900, local_4 as isize);
                let mut i_tmp = (&raw mut DAT_0070b252)
                    .wrapping_offset(i20)
                    .cast::<i32>()
                    .read_unaligned();
                if (b1 & b17) == 0 && i_tmp == -1 {
                    sw(&raw mut DAT_006d4a6a, u5.wrapping_mul(0x93) as isize, 0);
                    s8(&raw mut DAT_006d4aa2, i7 as isize, 0);
                } else {
                    let c2 = l8(&raw mut DAT_0070ad41, i20);
                    sw(
                        &raw mut DAT_006d4a6a,
                        u5.wrapping_mul(0x93) as isize,
                        ((l8(&raw mut DAT_0070ad51, i20) as u32)
                            .wrapping_add(1)
                            .wrapping_mul(local_12 as u32)) as u16,
                    );
                    if c2 == 0 && l8(&raw mut DAT_0070ad43, i20) <= l8(&raw mut DAT_0070ad40, i20) {
                        s8(&raw mut DAT_006d4aa2, i7 as isize, 0);
                    } else {
                        s8(
                            &raw mut DAT_006d4aa2,
                            i7 as isize,
                            l8(&raw mut DAT_0070ad51, i20).wrapping_add(1),
                        );
                    }
                }
                let mut i14: i32 = 1;
                loop {
                    let i13 = i14 + 1;
                    sw(
                        &raw mut DAT_006d4a6a,
                        (i14 as u32).wrapping_add(u5.wrapping_mul(0x93)) as isize,
                        0,
                    );
                    s8(&raw mut DAT_006d4aa2, (i7 + i14) as isize, 0);
                    i14 = i13;
                    if !(i13 < 0x1c) {
                        break;
                    }
                }
                i14 = 0;
                loop {
                    let k = (i7 + i14) as isize;
                    s8(&raw mut DAT_006d4abe, k, 0);
                    let i13 = i14 + 1;
                    s8(&raw mut DAT_006d4ac8, k, 0);
                    s8(&raw mut DAT_006d4ad2, k, 0);
                    s8(&raw mut DAT_006d4adc, k, 0);
                    s8(&raw mut DAT_006d4ae6, k, 0);
                    s8(&raw mut DAT_006d4af0, k, 0);
                    s8(&raw mut DAT_006d4afa, k, 0);
                    s8(&raw mut DAT_006d4b04, k, 0);
                    s8(&raw mut DAT_006d4b0e, k, 0);
                    s8(&raw mut DAT_006d4b18, k, 0);
                    s8(&raw mut DAT_006d4b22, k, 0);
                    s8(&raw mut DAT_006d4b2c, k, 0);
                    i14 = i13;
                    if !(i13 < 10) {
                        break;
                    }
                }
                i14 = 10;
                loop {
                    i14 -= 1;
                    s32(&raw mut DAT_006d4b36, i7 as isize, 0x4040404);
                    s32(&raw mut DAT_006d4b3a, i7 as isize, 0x4040404);
                    i7 = i7.wrapping_add(8);
                    if i14 == 0 {
                        break;
                    }
                }
                i_tmp = (&raw mut DAT_0070b252)
                    .wrapping_offset(i20)
                    .cast::<i32>()
                    .read_unaligned();
                if l8(&raw mut DAT_0070ad42, i20) != 0
                    && (((b1 & b17) != 0 || i_tmp != -1)
                        && lw(&raw mut DAT_006d4a6a, u5.wrapping_mul(0x93) as isize) != 0)
                {
                    i_tmp = (&raw mut DAT_0070b252)
                        .wrapping_offset(i20)
                        .cast::<i32>()
                        .read_unaligned();
                    if i_tmp == -1 {
                        if l8(&raw mut DAT_0070ad5e, i20) == 0
                            && l8(&raw mut DAT_0070ad76, i20) == 0
                            && l8(&raw mut DAT_0070ad77, i20) == 0
                            && (l8(&raw mut DAT_0070ad54, i20) == 0
                                || l8(&raw mut DAT_0070ad55, i20) == 0)
                        {
                            p21.write(1);
                            if (l8(&raw mut DAT_0052687a, i18) & 0x80) != 0 {
                                if DAT_006d4a02 != 2 {
                                    p11.write(0);
                                } else {
                                    p11.write(l8(&raw mut DAT_0070ad51, i20));
                                }
                                FUN_004215c0(0xffff);
                            } else {
                                p11.write(l8(&raw mut DAT_0070ad51, i20));
                                FUN_0041eea0(0xffff);
                            }
                        }
                    } else {
                        p21.write(1);
                        if (l8(&raw mut DAT_0052687a, i18) & 0x80) == 0 {
                            p11.write(l8(&raw mut DAT_0070ad51, i20));
                            FUN_0041eea0(0xffff);
                        } else {
                            if DAT_006d4a02 == 2 {
                                p11.write(l8(&raw mut DAT_0070ad51, i20));
                            } else {
                                p11.write(0);
                            }
                            FUN_004215c0(0xffff);
                        }
                    }
                    let q = (DAT_0073b374 as u32).wrapping_mul(0x566) as isize;
                    let idx = (l8(&raw mut DAT_005247e8, l8(&raw mut DAT_0070ad42, q) as isize)
                        as u32)
                        .wrapping_mul(0x16) as isize;
                    if (l8(&raw mut DAT_0052687a, idx) & 0x20) == 0 {
                        s8(
                            &raw mut DAT_00709f30,
                            l8(&raw mut DAT_0070ad48, q) as isize,
                            1,
                        );
                    }
                }
                DAT_0073b374 = DAT_0073b374.wrapping_add(1);
                if !((DAT_0073b374 as i32) < DAT_006d49a0 as i32) {
                    break;
                }
            }
        }
        let mut i7: i32 = 0;
        let mut p15 = (&raw mut DAT_00709388).cast::<u16>();
        let mut local_4: u32 = 0;
        let mut local_10 = (&raw mut DAT_00709338).cast::<u16>();
        let mut local_c = (&raw mut DAT_007092e8).cast::<u32>();
        let mut i20: i32 = 0;
        loop {
            s8(&raw mut DAT_00709f3a, local_4 as isize, 0);
            local_c.write_unaligned(0);
            local_c.wrapping_add(10).write_unaligned(0);
            local_c.wrapping_add(0x308).write_unaligned(0);
            let mut i18: i32 = 0;
            loop {
                let i14 = i18 + i20;
                s8(&raw mut DAT_00709cec, i14 as isize, 4);
                s8(&raw mut DAT_00709d0a, i14 as isize, 0);
                let mut i13 = 4;
                let mut p16 = (&raw mut DAT_00709d28)
                    .cast::<u16>()
                    .wrapping_offset((i14 as isize) * 4);
                loop {
                    p16.write_unaligned(0);
                    p16.wrapping_add(0x78).write_unaligned(0);
                    i13 -= 1;
                    p16 = p16.wrapping_add(1);
                    if i13 == 0 {
                        break;
                    }
                }
                i18 += 1;
                if !(i18 < 3) {
                    break;
                }
            }
            local_10.write_unaligned(0);
            local_10.wrapping_add(10).write_unaligned(0);
            local_10.wrapping_add(0x14).write_unaligned(0);
            local_10.wrapping_add(0x1e).write_unaligned(0);
            let mut p16 = p15;
            i18 = 0;
            loop {
                p16.write_unaligned(0);
                p15 = p16.wrapping_add(1);
                p16.wrapping_add(0x1e0).write_unaligned(0);
                let i14 = i18 + 1;
                s8(&raw mut DAT_00709b08, (i7 + i18) as isize, 0);
                p16 = p15;
                i18 = i14;
                if !(i14 < 0x30) {
                    break;
                }
            }
            i7 = i7.wrapping_add(0x30);
            local_10 = local_10.wrapping_add(1);
            i20 += 3;
            local_c = local_c.wrapping_add(1);
            local_4 = local_4.wrapping_add(1);
            if !((local_c as usize) < ((&raw mut DAT_00709310) as usize)) {
                break;
            }
        }
        DAT_0073b374 = 0;
        if 0 < DAT_006d49a0 as i32 {
            let i7 = DAT_006d49a0 as i32;
            loop {
                let u5 = DAT_0073b374 as u32;
                let i20 = u5.wrapping_mul(0x566) as i32 as isize;
                let mut c2 = l8(&raw mut DAT_0070ad04, i20);
                let u8v = l8(&raw mut DAT_0070ad48, i20) as u32;
                let mut i18: isize = 0;
                while c2 != 0 && i18 < 0x10 {
                    let mut b17: u8 = 0xff;
                    let b1 = l8(&raw mut DAT_0070ad04, i20 + i18);
                    if b1 == 0x41 {
                        b17 = 10;
                    } else if b1 == 0x4f {
                        b17 = 0xb;
                    } else if b1 == 0x46 {
                        b17 = 0xc;
                    } else if b1 == 0x48 {
                        b17 = 0xd;
                    } else if 0x30 < b1 && b1 < 0x3a {
                        b17 = b1.wrapping_sub(0x31);
                    }
                    if b17 != 0xff {
                        c2 = l8(&raw mut DAT_0070ad05, i20 + i18);
                        let c3 = l8(&raw mut DAT_0070ad06, i20 + i18);
                        let c4 = l8(&raw mut DAT_0070ad07, i20 + i18);
                        let mut c12: u8 = 0;
                        if (c2, c3, c4) == (b'C', b'O', b'M') {
                            c12 = 1;
                        } else if (c2, c3, c4) == (b'B', b'A', b'S') {
                            c12 = 2;
                        } else if (c2, c3, c4) == (b'S', b'T', b'A') {
                            c12 = 3;
                        } else if (c2, c3, c4) == (b'M', b'I', b'S') {
                            c12 = 4;
                        } else if (c2, c3, c4) == (b'C', b'O', b'N') {
                            c12 = 5;
                        } else if (c2, c3, c4) == (b'S', b'T', b'R') {
                            c12 = 6;
                        } else if (c2, c3, c4) == (b'R', b'E', b'L') {
                            c12 = 7;
                        } else if (c2, c3, c4) == (b'P', b'R', b'I') {
                            c12 = 8;
                        } else if (c2, c3, c4) == (b'S', b'E', b'C') {
                            c12 = 9;
                        } else if (c2, c3, c4) == (b'T', b'E', b'R') {
                            c12 = 10;
                        } else if (c2, c3, c4) == (b'R', b'E', b'S') {
                            c12 = 11;
                        } else if (c2, c3, c4) == (b'M', b'A', b'N') {
                            c12 = 12;
                        }
                        if c12 != 0 {
                            if b17 < 10 {
                                s8(
                                    &raw mut DAT_00709b08,
                                    u5.wrapping_add((b17 as u32).wrapping_mul(0x30)) as isize,
                                    c12,
                                );
                            } else {
                                let mut u19: u32 = 0;
                                let mut pc6 = (&raw mut DAT_00709b08).wrapping_offset(u5 as isize);
                                loop {
                                    let mut write = false;
                                    if b17 == 10 {
                                        write = true;
                                    } else if b17 == 0xb {
                                        if u8v != u19 {
                                            write = true;
                                        }
                                    } else if b17 == 0xc {
                                        if l8(
                                            &raw mut DAT_0072f508,
                                            u19.wrapping_add(u8v.wrapping_mul(0x1e5)) as isize,
                                        ) != 0
                                        {
                                            write = true;
                                        }
                                    } else if b17 == 0xd
                                        && l8(
                                            &raw mut DAT_0072f508,
                                            u19.wrapping_add(u8v.wrapping_mul(0x1e5)) as isize,
                                        ) == 0
                                    {
                                        write = true;
                                    }
                                    if write {
                                        pc6.write(c12);
                                    }
                                    pc6 = pc6.wrapping_add(0x30);
                                    u19 = u19.wrapping_add(1);
                                    if !((u19 as i32) < 10) {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    c2 = l8(&raw mut DAT_0070ad08, i20 + i18);
                    i18 += 4;
                }
                DAT_0073b374 = DAT_0073b374.wrapping_add(1);
                if !((DAT_0073b374 as i32) < i7) {
                    break;
                }
            }
        }
        let mut i7: isize = 0;
        loop {
            s8(&raw mut DAT_0071b741, i7, 0);
            s8(&raw mut DAT_0071b749, i7, 1);
            s16(&raw mut DAT_0071b74a, i7, 0xffff);
            s16(&raw mut DAT_0071b74c, i7, 0xffff);
            s16(&raw mut DAT_0071b759, i7, 0xffff);
            s16(&raw mut DAT_0071b75b, i7, 0xffff);
            s32(&raw mut DAT_0071b74e, i7, 0xffffffff);
            s32(&raw mut DAT_0071b752, i7, 0xffffffff);
            s16(&raw mut DAT_0071b75d, i7, 0xffff);
            s16(&raw mut DAT_0071b75f, i7, 0);
            s16(&raw mut DAT_0071b77b, i7, 0);
            s16(&raw mut DAT_0071b77d, i7, 0);
            s16(&raw mut DAT_0071b77f, i7, 0);
            s16(&raw mut DAT_0071b781, i7, 0);
            s16(&raw mut DAT_0071b783, i7, 0);
            s8(&raw mut DAT_0071b744, i7, 0);
            s32(&raw mut DAT_0071b745, i7, 0);
            let mut p9 = (&raw mut DAT_0071bae7).wrapping_offset(i7);
            let mut n = 0xc;
            while n != 0 {
                p9.cast::<u32>().write_unaligned(0);
                p9 = p9.wrapping_add(4);
                n -= 1;
            }
            p9.cast::<u16>().write_unaligned(0);
            s8(&raw mut DAT_0071bb19, i7, 0);
            let n7 = i7 + 0x5bd;
            s8(&raw mut DAT_0071bb1a, i7, 0);
            i7 = n7;
            if !(n7 < 0x2de8) {
                break;
            }
        }
        DAT_006db586 = DAT_006db7a4;
        if DAT_0070acd0 == 0 {
            FUN_004b1730();
        }
        _DAT_0070a13c = 0;
        DAT_0071e6a0 = 0;
        _DAT_00732e5a = 0;
        _DAT_0071e6a4 = 0;
        DAT_00525580 = 1;
        DAT_006fedd0 = 0xffff;
        DAT_0071e6a8 = 0;
        DAT_0071e6ac = 0;
        DAT_0071e6b0 = 0;
        DAT_0071e6b4 = 0;
        let mut p15 = (&raw mut DAT_0070ac40).cast::<u16>();
        loop {
            let mut n = 8;
            let mut p16 = p15;
            loop {
                p16.write_unaligned(0);
                p16 = p16.wrapping_add(8);
                n -= 1;
                if n == 0 {
                    break;
                }
            }
            p15 = p15.wrapping_add(1);
            if !((p15 as usize) < ((&raw mut DAT_0070ac50) as usize)) {
                break;
            }
        }
        let mut p9 = &raw mut DAT_0071bcdd;
        loop {
            p9.cast::<u32>().write_unaligned(0);
            let p10 = p9.wrapping_add(0x5bd);
            p9.wrapping_add(4).cast::<u32>().write_unaligned(0);
            p9 = p10;
            if !((p10 as usize) < ((&raw mut DAT_0071eac5) as usize)) {
                break;
            }
        }
        let mut p21 = &raw mut DAT_0071bb39;
        loop {
            p21.write(0);
            let p11 = p21.wrapping_add(0x5bd);
            s16(p21, -6, 0);
            s16(p21, -4, 0);
            s16(p21, 0xb, 0);
            s32(p21, 0xd, 0x400);
            s16(p21, 7, 0);
            s16(p21, 0x11, 0);
            let t32 = p21.wrapping_offset(-0x409).cast::<u32>().read_unaligned();
            s16(p21, -0x12, t32 as u16);
            p21 = p11;
            if !((p11 as usize) < ((&raw mut DAT_0071e921) as usize)) {
                break;
            }
        }
        let mut p21 = &raw mut DAT_0071b775;
        DAT_00703ac4 = 0xff;
        let mut i7: i32 = 0;
        loop {
            p21.write(0);
            p21 = p21.wrapping_add(0x5bd);
            FUN_004b1bc0(0, i7);
            i7 = i7.wrapping_add(1);
            if !((p21 as usize) < ((&raw mut DAT_0071e55d) as usize)) {
                break;
            }
        }
        DAT_006db592 = 0;
        DAT_006ffbe4 = 1;
        DAT_007327e0 = 1;
        DAT_0070a1d2 = 0xf;
        DAT_0071e6be = 0xf;
        _DAT_006db584 = 0xf;
        FUN_0048f530();
        DAT_0070a178 = 0;
        DAT_006db5a8 = 0;
        let mut i7: i32 = 0;
        loop {
            s8(&raw mut DAT_00709f44, i7 as isize, 0);
            let i20 = i7 + 1;
            s8(&raw mut DAT_00709f84, i7 as isize, 0);
            i7 = i20;
            if !(i20 < 0x40) {
                break;
            }
        }
        s16(&raw mut _DAT_00709ce9, 0, 0);
        DAT_007092c0 = 0;
        DAT_007092e4 = 0;
        DAT_00709ce8 = 0;
        DAT_00709ceb = 0;
        DAT_00536e8c = 0;
        FUN_004a6a80();
    }
}
