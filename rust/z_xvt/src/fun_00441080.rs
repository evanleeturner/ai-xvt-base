// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_00732a3c: u32;
    static mut DAT_006d4980: u32;
    static mut DAT_006d4988: i32;
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_00732a38: u32;
    static mut DAT_00703c88: u32;
    static mut DAT_006db580: u32;

    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0071b73c: [u8; 0];
    static mut DAT_0071bb1b: [u8; 0];
    static mut DAT_0071bb1f: [u8; 0];
    static mut DAT_0071bb23: [u8; 0];
    static mut DAT_0070ad48: [u8; 0];
    static mut DAT_0070aee5: [i8; 0];
    static mut DAT_0070aee9: [i8; 0];
    static mut DAT_006d4b36: [u8; 0];
    static mut DAT_006daa00: [u8; 0];
    static mut DAT_006daa01: [u8; 0];
    static mut DAT_006daa02: [u8; 0];
    static mut DAT_006daa04: [u8; 0];
    static mut DAT_006daa05: [u8; 0];
    static mut DAT_006daa06: [u8; 0];
    static mut DAT_006daa0b: [u8; 0];
    static mut DAT_006daa0f: [u8; 0];
    static mut DAT_006daa10: [u8; 0];
    static mut DAT_006daa11: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049f010(a: u32, b: u16, c: u16) -> u16;
    fn FUN_00421c70(a: u32, b: i32);
    fn FUN_00418cd0(a: i32, b: i32, c: i32);
    fn FUN_0042a140(a: u32, b: u32);
    fn FUN_004a3830(a: u32, b: i32, c: i32, d: i32, e: u32, f: i32) -> u32;
}

unsafe fn r8(a: i32) -> u8 {
    // SAFETY: the address is inside the game's own memory.
    unsafe { (a as u32 as usize as *const u8).read() }
}

unsafe fn r16(a: i32) -> u16 {
    // SAFETY: the address is inside the game's own memory.
    unsafe { (a as u32 as usize as *const u16).read_unaligned() }
}

unsafe fn r32(a: i32) -> i32 {
    // SAFETY: the address is inside the game's own memory.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

unsafe fn ab(base: *mut u8, off: i32) -> u8 {
    // SAFETY: the array is the game's own table; the offset is the original's.
    unsafe { base.wrapping_offset(off as isize).read() }
}

unsafe fn aq(base: *mut u8, off: i32) -> i32 {
    // SAFETY: the array is the game's own table; the offset is the original's.
    unsafe {
        base.wrapping_offset(off as isize)
            .cast::<i32>()
            .read_unaligned()
    }
}

unsafe fn call_pos(iv2: i32, uvar8: u32) {
    // SAFETY: the callees and globals are the game's own.
    unsafe {
        let b1b = (&raw mut DAT_0071bb1b).cast::<u8>();
        let b1f = (&raw mut DAT_0071bb1f).cast::<u8>();
        let b23 = (&raw mut DAT_0071bb23).cast::<u8>();
        let b30 = (&raw mut DAT_0071b730).cast::<u8>();
        let ui_tmp = aq(b30, iv2) as u32;
        if ui_tmp == 0xffff_ffff {
            FUN_00421c70(uvar8, 0);
            let a = aq(b1b, iv2);
            let b = aq(b1f, iv2);
            let c = aq(b23, iv2);
            FUN_00418cd0(
                DAT_00732a2c.wrapping_sub(a),
                DAT_00732a28.wrapping_sub(b),
                DAT_00732a24.wrapping_sub(c),
            );
        } else {
            FUN_0042a140(ui_tmp, uvar8);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00441080(param_1: i32, param_2: i32) -> u32 {
    let unaff_ebp: u32 = 0;
    let unaff_edi: i32 = 0;
    // SAFETY: the globals, tables and callees are the game's own memory;
    // pointers are read where the original reads them.
    unsafe {
        let b30 = (&raw mut DAT_0071b730).cast::<u8>();
        let b3c = (&raw mut DAT_0071b73c).cast::<u8>();
        let a48 = (&raw mut DAT_0070ad48).cast::<u8>();
        let e5 = (&raw mut DAT_0070aee5).cast::<i8>();
        let e9 = (&raw mut DAT_0070aee9).cast::<i8>();
        let b36 = (&raw mut DAT_006d4b36).cast::<u8>();
        let s00 = (&raw mut DAT_006daa00).cast::<u8>();
        let s01 = (&raw mut DAT_006daa01).cast::<u8>();
        let s02 = (&raw mut DAT_006daa02).cast::<u8>();
        let s04 = (&raw mut DAT_006daa04).cast::<u8>();
        let s05 = (&raw mut DAT_006daa05).cast::<u8>();
        let s06 = (&raw mut DAT_006daa06).cast::<u8>();
        let s0b = (&raw mut DAT_006daa0b).cast::<u8>();
        let s0f = (&raw mut DAT_006daa0f).cast::<u8>();
        let s10 = (&raw mut DAT_006daa10).cast::<u8>();
        let s11 = (&raw mut DAT_006daa11).cast::<u8>();

        let mut local_4: u32 = 0xffff_ffff;
        let mut local_14: u32 = 0xffff_ffff;
        let iv2 = (param_2 as u32).wrapping_mul(0x5bd) as i32;
        let u_5 = (b3c
            .wrapping_offset(iv2 as isize)
            .cast::<u16>()
            .read_unaligned()) as u32;
        let mut s_1e: i16 = -1;
        let mut local_20: i16 = -1;
        let mut s_1c: i16;

        if _DAT_00732a3c < DAT_006d4980 {
            let mut local_8: i32 = _DAT_00732a3c.wrapping_mul(0x23) as i32;
            let mut uvar8: u32 = _DAT_00732a3c;
            loop {
                s_1c = uvar8 as i16;
                let mut iv9 = DAT_006d4988.wrapping_add(local_8);
                let ui_tmp = aq(b30, iv2) as u32;
                if r8(iv9.wrapping_add(3)) != 0
                    && ui_tmp != uvar8
                    && r8(iv9.wrapping_add(2)) != 0x0d
                {
                    let b3: bool;
                    if s_1c == -1 || (DAT_006d4980 as i32) <= ((uvar8 & 0xffff) as i32) {
                        b3 = false;
                    } else {
                        let iv6 = (uvar8 & 0xffff)
                            .wrapping_mul(0x23)
                            .wrapping_add(DAT_006d4988 as u32)
                            as i32;
                        let t = r32(iv6.wrapping_add(0x1b));
                        if t == -1 {
                            b3 = false;
                        } else {
                            let t2 = r32(iv6.wrapping_add(0x1f));
                            let iv6b = r32(t2.wrapping_add(0xa7));
                            if iv6b == 0 || (r8(iv6b.wrapping_add(0x1e0)) & 1) == 0 {
                                b3 = false;
                            } else if r8(iv6b.wrapping_add(0x237)) == 0 {
                                b3 = false;
                            } else if r8(iv6b.wrapping_add(0x233)) != 3 {
                                b3 = false;
                            } else {
                                b3 = r16(iv6b.wrapping_add(0x238)) != 0;
                            }
                        }
                    }
                    if !b3 {
                        let iv6 = (r8(iv9.wrapping_add(0x16)) as u32).wrapping_mul(0x566) as i32;
                        if (ab(a48, iv6) as u32) != u_5 {
                            let mut uv11: u32 = 0;
                            let mut pc13 = e5.wrapping_offset(iv6 as isize);
                            let mut bv = false;
                            let mut pc10 =
                                e9.wrapping_offset(u_5.wrapping_add(iv6 as u32) as i32 as isize);
                            loop {
                                if *pc10 != 0
                                    && *pc13 == param_1 as i8
                                    && -1 < *pc13.wrapping_offset(3)
                                {
                                    let idx = uv11.wrapping_add(u_5.wrapping_mul(8)).wrapping_add(
                                        (r8(iv9.wrapping_add(0x16)) as u32).wrapping_mul(0x126),
                                    ) as i32;
                                    if ab(b36, idx) == 4 {
                                        bv = true;
                                    }
                                }
                                pc13 = pc13.wrapping_offset(0x4e);
                                pc10 = pc10.wrapping_offset(0x4e);
                                uv11 = uv11.wrapping_add(1);
                                if uv11 >= 8 {
                                    break;
                                }
                            }
                            iv9 = u_5
                                .wrapping_mul(7)
                                .wrapping_add(param_1 as u32)
                                .wrapping_mul(0x2a) as i32;
                            if ab(s00, iv9) != 0x0a && ab(s00, iv9) != 0 {
                                if FUN_0049f010(uvar8, ab(s01, iv9) as u16, ab(s02, iv9) as u16)
                                    != 0
                                {
                                    bv = true;
                                }
                            }
                            if ab(s04, iv9) != 0x0a && ab(s04, iv9) != 0 {
                                if FUN_0049f010(uvar8, ab(s05, iv9) as u16, ab(s06, iv9) as u16)
                                    != 0
                                {
                                    bv = true;
                                }
                            }
                            if ab(s0b, iv9) != 0x0a && ab(s0b, iv9) != 0 {
                                if FUN_0049f010(uvar8, ab(s01, iv9) as u16, ab(s02, iv9) as u16)
                                    != 0
                                {
                                    bv = true;
                                }
                            }
                            if ab(s0f, iv9) != 0x0a && ab(s0f, iv9) != 0 {
                                if FUN_0049f010(uvar8, ab(s10, iv9) as u16, ab(s11, iv9) as u16)
                                    != 0
                                {
                                    bv = true;
                                }
                            }
                            if bv {
                                let t = r32((DAT_006d4988 as u32)
                                    .wrapping_add(0x1f)
                                    .wrapping_add(local_8 as u32)
                                    as i32);
                                let t = r32(t.wrapping_add(0xa7));
                                let c1 = r8(t.wrapping_add(7));
                                if c1 == 0 || c1 == 6 {
                                    call_pos(iv2, uvar8);
                                    let uv11 =
                                        FUN_004a3830(uvar8, param_2, 0, 1, unaff_ebp, unaff_edi);
                                    if uv11 == 0 {
                                        if DAT_006db580 < local_14 {
                                            local_14 = DAT_006db580;
                                            local_20 = s_1c;
                                        }
                                    } else if DAT_006db580 < local_4 {
                                        local_4 = DAT_006db580;
                                        s_1e = s_1c;
                                    }
                                }
                            }
                        }
                    }
                }
                local_8 = local_8.wrapping_add(0x23);
                uvar8 = uvar8.wrapping_add(1);
                if uvar8 >= DAT_006d4980 {
                    break;
                }
            }
        }

        s_1c = DAT_00732a38 as i16;
        if DAT_00732a38 < DAT_00703c88.wrapping_add(DAT_00732a38) {
            local_14 = DAT_00732a38.wrapping_mul(0x23);
            let mut uvar8: u32 = DAT_00732a38;
            loop {
                s_1c = uvar8 as i16;
                let base = local_14.wrapping_add(DAT_006d4988 as u32) as i32;
                if r8(base.wrapping_add(3)) != 0 {
                    let uv11 = r8(base.wrapping_add(0x16)) as u32;
                    let iv9 = uv11.wrapping_mul(0x566) as i32;
                    let us_tmp = b3c
                        .wrapping_offset(iv2 as isize)
                        .cast::<u16>()
                        .read_unaligned();
                    if (ab(a48, iv9) as u16) != us_tmp {
                        let mut uv12: u32 = 0;
                        let mut pc13 = e5.wrapping_offset(iv9 as isize);
                        let mut lc =
                            e9.wrapping_offset(u_5.wrapping_add(iv9 as u32) as i32 as isize);
                        loop {
                            if *lc != 0 && *pc13 == param_1 as i8 && -1 < *pc13.wrapping_offset(3) {
                                let idx = uv12
                                    .wrapping_add(u_5.wrapping_mul(8))
                                    .wrapping_add(uv11.wrapping_mul(0x126))
                                    as i32;
                                if ab(b36, idx) == 4 {
                                    call_pos(iv2, uvar8);
                                    if DAT_006db580 < local_4 {
                                        local_4 = DAT_006db580;
                                        s_1e = s_1c;
                                    }
                                }
                            }
                            pc13 = pc13.wrapping_offset(0x4e);
                            lc = lc.wrapping_offset(0x4e);
                            uv12 = uv12.wrapping_add(1);
                            if uv12 >= 8 {
                                break;
                            }
                        }
                    }
                }
                local_14 = local_14.wrapping_add(0x23);
                uvar8 = uvar8.wrapping_add(1);
                s_1c = uvar8 as i16;
                if uvar8 >= DAT_00703c88.wrapping_add(DAT_00732a38) {
                    break;
                }
            }
        }

        if s_1e == -1 {
            return 0xffff_0000 | (local_20 as u16 as u32);
        }
        ((s_1c as u16 as u32) << 16) | (s_1e as u16 as u32)
    }
}
