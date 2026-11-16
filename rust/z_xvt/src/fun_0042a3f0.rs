// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00732e54: i32;
    static mut DAT_006d9d50: u16;
    static mut DAT_006d9d56: i32;
    static mut DAT_006d499c: u32;
    static mut DAT_00732a24: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_00703c88: i32;
    static mut DAT_00703ca0: [i8; 1];
    static mut DAT_0052687a: [u8; 1];
    static mut DAT_00524948: [i8; 1];
    static mut s_stationaryldrpln_00524934: [i8; 1];
    static mut s_stationaryflwpln_00524920: [i8; 1];
    static mut s_boardtocapturepln_00524f90: [i8; 1];
    static mut s_boardtodestroypln_00524f7c: [i8; 1];
    static mut s_boardtogivepln_00523c18: [i8; 1];
    static mut s_boardtotakepln_00524f6c: [i8; 1];
    static mut s_boardtoexchangepln_00524f58: [i8; 1];
    static mut s_boardtopickuppln_00524f44: [i8; 1];
    static mut s_boardtocontactpln_00524f30: [i8; 1];
    static mut s_boardtorepairpln_00524f1c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049f010(param_1: u32, param_2: u16, param_3: u16) -> u8;
    fn FUN_0042b150(param_1: i32) -> i32;
    fn FUN_00421c70(param_1: u32, param_2: i32);
    fn FUN_0049ec30(param_1: u16, param_2: u16, param_3: u16) -> u16;
}

unsafe fn r8(a: i32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn r16(a: i32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn r32(a: i32) -> i32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

unsafe fn eq(a: *const i8, b: *const i8, n: i32) -> bool {
    let mut i = 0;
    while i < n as usize {
        // SAFETY: game strings, compared as the original's repe cmpsb does.
        unsafe {
            if *a.add(i) != *b.add(i) {
                return false;
            }
        }
        i += 1;
    }
    true
}

unsafe fn is_board(p: *const i8, pickup: bool) -> bool {
    // SAFETY: pointers are the game's own strings.
    unsafe {
        if eq(p, &raw const s_boardtogivepln_00523c18 as *const i8, 0xf) {
            return true;
        }
        if eq(p, &raw const s_boardtotakepln_00524f6c as *const i8, 0xf) {
            return true;
        }
        if eq(
            p,
            &raw const s_boardtoexchangepln_00524f58 as *const i8,
            0x13,
        ) {
            return true;
        }
        if eq(
            p,
            &raw const s_boardtocapturepln_00524f90 as *const i8,
            0x12,
        ) {
            return true;
        }
        if eq(
            p,
            &raw const s_boardtodestroypln_00524f7c as *const i8,
            0x12,
        ) {
            return true;
        }
        if pickup && eq(p, &raw const s_boardtopickuppln_00524f44 as *const i8, 0x11) {
            return true;
        }
        if eq(
            p,
            &raw const s_boardtocontactpln_00524f30 as *const i8,
            0x12,
        ) {
            return true;
        }
        eq(p, &raw const s_boardtorepairpln_00524f1c as *const i8, 0x11)
    }
}

unsafe fn upd(local_e: u16, best: &mut u32, idx: &mut u16) {
    // SAFETY: the callee and globals are the game's own.
    unsafe {
        FUN_00421c70(DAT_006d9d50 as u32, 0);
        let mut dx = DAT_00732a2c;
        let mut dy = DAT_00732a28;
        let mut dz = DAT_00732a24;
        FUN_00421c70(local_e as u32, 0);
        dx = dx.wrapping_sub(DAT_00732a2c);
        dy = dy.wrapping_sub(DAT_00732a28);
        dz = dz.wrapping_sub(DAT_00732a24);
        if dx < 0 {
            dx = dx.wrapping_neg();
        }
        if dy < 0 {
            dy = dy.wrapping_neg();
        }
        if dz < 0 {
            dz = dz.wrapping_neg();
        }
        if dy < dx {
            DAT_006d499c = ((dy >> 1) as u32).wrapping_add(dx as u32);
        } else {
            DAT_006d499c = ((dx >> 1) as u32).wrapping_add(dy as u32);
        }
        if dz < DAT_006d499c as i32 {
            dz >>= 1;
        } else {
            DAT_006d499c = ((DAT_006d499c as i32) >> 1) as u32;
        }
        DAT_006d499c = DAT_006d499c.wrapping_add(dz as u32);
        if DAT_006d499c < *best {
            *idx = local_e;
            *best = DAT_006d499c;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042a3f0(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) -> u32 {
    // SAFETY: all globals, callees and pointers are the game's own memory.
    unsafe {
        let mut local_e: u16 = DAT_00732a3c;
        let mut local_4: u32 = 0xffff_ffff;
        let mut local_12: u16 = 0xffff;
        let mut i_sum: i32;
        if (DAT_00732a3c as i32) < DAT_006d4980 {
            loop {
                let iv13 = (local_e as u32).wrapping_mul(0x23) as i32;
                let us_c = iv13 as u16;
                if r8(DAT_006d4988.wrapping_add(3).wrapping_add(iv13)) != 0 {
                    let arg = ((us_c as u32) << 16) | local_e as u32;
                    let mut u9 = FUN_0049f010(arg, param_1, param_2) as u16;
                    let u3 = FUN_0049f010(arg, param_4, param_5) as u16;
                    if param_3 == 1 {
                        u9 |= u3;
                    } else {
                        u9 &= u3;
                    }
                    if u9 != 0 {
                        let tmp = r32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(iv13));
                        let iv5 = r32(tmp.wrapping_add(0xa7));
                        let iv4 = FUN_0042b150(iv5);
                        let p11 = (&raw const DAT_00703ca0 as *const i8)
                            .wrapping_add((r8(iv4.wrapping_add(0xb)) as usize) * 0x55);
                        let b2: bool;
                        if eq(p11, &raw const DAT_00524948 as *const i8, 8)
                            || eq(
                                p11,
                                &raw const s_stationaryldrpln_00524934 as *const i8,
                                0x11,
                            )
                            || eq(
                                p11,
                                &raw const s_stationaryflwpln_00524920 as *const i8,
                                0x11,
                            )
                            || r8(DAT_006d4988.wrapping_add(2).wrapping_add(iv13)) == 5
                        {
                            b2 = true;
                        } else {
                            let tmpb = (&raw const DAT_006d9d56).read_unaligned();
                            let p = (&raw const DAT_00703ca0 as *const i8)
                                .wrapping_add((r8(tmpb.wrapping_add(0xb)) as usize) * 0x55);
                            if eq(
                                p,
                                &raw const s_boardtocapturepln_00524f90 as *const i8,
                                0x12,
                            ) {
                                b2 = r16(iv5.wrapping_add(0x1df)) == 0;
                            } else {
                                let tmpb = (&raw const DAT_006d9d56).read_unaligned();
                                let p = (&raw const DAT_00703ca0 as *const i8)
                                    .wrapping_add((r8(tmpb.wrapping_add(0xb)) as usize) * 0x55);
                                if eq(
                                    p,
                                    &raw const s_boardtodestroypln_00524f7c as *const i8,
                                    0x12,
                                ) {
                                    b2 = r16(iv5.wrapping_add(0x1df)) == 0;
                                } else {
                                    let s = r16(iv5.wrapping_add(0x1df));
                                    let c = r8(iv4.wrapping_add(0x34));
                                    b2 = s == 0 || c == 0x13 || c == 0x19;
                                }
                            }
                        }
                        if b2 {
                            let mut local_10: i16 = 0;
                            let mut uv9: u16 = DAT_00732a3c;
                            if (uv9 as i32) < DAT_006d4980 {
                                loop {
                                    let q = ((uv9 as u32).wrapping_mul(0x23))
                                        .wrapping_add(DAT_006d4988 as u32)
                                        as i32;
                                    if r8(q.wrapping_add(3)) != 0 && uv9 != DAT_006d9d50 {
                                        let t = r32(q.wrapping_add(0x1f));
                                        let i5 = r32(t.wrapping_add(0xa7));
                                        let i4 = FUN_0042b150(i5);
                                        let p = (&raw const DAT_00703ca0 as *const i8)
                                            .wrapping_add(
                                                (r8(i4.wrapping_add(0xb)) as usize) * 0x55,
                                            );
                                        if is_board(p, true)
                                            && (r16(i4.wrapping_add(0x18)) == local_e
                                                || r16(i5.wrapping_add(0x66)) == local_e)
                                        {
                                            local_10 = local_10.wrapping_add(1);
                                        }
                                    }
                                    uv9 = uv9.wrapping_add(1);
                                    if !((uv9 as i32) < DAT_006d4980) {
                                        break;
                                    }
                                }
                            }
                            let mut uv7: u32 = 0;
                            if r8(DAT_00732e54.wrapping_add(0x7d)) != 0 {
                                loop {
                                    if r16(DAT_00732e54
                                        .wrapping_add(0x7e)
                                        .wrapping_add((uv7 as i32).wrapping_mul(2)))
                                        == r16(DAT_006d4988.wrapping_add(iv13))
                                    {
                                        local_10 = local_10.wrapping_add(1);
                                    }
                                    uv7 = (uv7 as u16).wrapping_add(1) as u32;
                                    if !(uv7 < r8(DAT_00732e54.wrapping_add(0x7d)) as u32) {
                                        break;
                                    }
                                }
                            }
                            if local_10 == 0 {
                                upd(local_e, &mut local_4, &mut local_12);
                            }
                        }
                    }
                }
                local_e = local_e.wrapping_add(1);
                if !((local_e as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
        local_e = DAT_00732a38 as u16;
        i_sum = (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32;
        if (DAT_00732a38 & 0xffff) < i_sum {
            loop {
                let iv13 = (((local_e as u32).wrapping_mul(8)).wrapping_sub(local_e as u32))
                    .wrapping_mul(5) as i32;
                let b1 = r8(DAT_006d4988.wrapping_add(iv13).wrapping_add(3));
                if b1 != 0
                    && ((&raw const DAT_0052687a as *const u8)
                        .wrapping_add((b1 as usize) * 0x16)
                        .read()
                        & 2)
                        != 0
                {
                    let f = r8(DAT_006d4988.wrapping_add(iv13).wrapping_add(0x16)) as u16;
                    let mut u9 = FUN_0049ec30(f, param_1, param_2);
                    let u3 = FUN_0049ec30(f, param_4, param_5);
                    if param_3 == 1 {
                        u9 |= u3;
                    } else {
                        u9 &= u3;
                    }
                    if u9 != 0 {
                        let mut s8: i16 = 0;
                        let mut uv9: u16 = DAT_00732a3c;
                        if (uv9 as i32) < DAT_006d4980 {
                            loop {
                                let q = ((uv9 as u32).wrapping_mul(0x23))
                                    .wrapping_add(DAT_006d4988 as u32)
                                    as i32;
                                if r8(q.wrapping_add(3)) != 0 && uv9 != DAT_006d9d50 {
                                    let t = r32(q.wrapping_add(0x1f));
                                    let tb = r32(t.wrapping_add(0xa7));
                                    let i5 = FUN_0042b150(tb);
                                    let p = (&raw const DAT_00703ca0 as *const i8)
                                        .wrapping_add((r8(i5.wrapping_add(0xb)) as usize) * 0x55);
                                    if is_board(p, false) && r16(i5.wrapping_add(0x18)) == local_e {
                                        s8 = s8.wrapping_add(1);
                                    }
                                }
                                uv9 = uv9.wrapping_add(1);
                                if !((uv9 as i32) < DAT_006d4980) {
                                    break;
                                }
                            }
                        }
                        let mut uv7: u32 = 0;
                        if r8(DAT_00732e54.wrapping_add(0x7d)) != 0 {
                            loop {
                                if r16(DAT_00732e54
                                    .wrapping_add(0x7e)
                                    .wrapping_add((uv7 as i32).wrapping_mul(2)))
                                    == r16(DAT_006d4988.wrapping_add(iv13))
                                {
                                    s8 = s8.wrapping_add(1);
                                }
                                uv7 = (uv7 as u16).wrapping_add(1) as u32;
                                if !(uv7 < r8(DAT_00732e54.wrapping_add(0x7d)) as u32) {
                                    break;
                                }
                            }
                        }
                        if s8 == 0 {
                            upd(local_e, &mut local_4, &mut local_12);
                        }
                    }
                }
                local_e = local_e.wrapping_add(1);
                i_sum = (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32;
                if !((local_e as i32) < i_sum) {
                    break;
                }
            }
        }
        (((i_sum as u32) >> 16) << 16) | local_12 as u32
    }
}
