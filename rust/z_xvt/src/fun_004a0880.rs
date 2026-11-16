// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_006d4a02: i8;
    static mut DAT_007092d4: i8;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0070aee5: u8;
    static mut DAT_0070aee9: u8;
    static mut DAT_0070ad48: u8;
    static mut DAT_0070ad42: u8;
    static mut DAT_005247e8: u8;
    static mut DAT_0052686a: u8;
    static mut DAT_0072f508: u8;
    static mut DAT_0071b73c: u8;
    static mut DAT_0071b73a: u8;
    static mut DAT_0071b79d: u8;
    static mut DAT_0071b7a1: u8;
    static mut DAT_0071b7c5: u8;
    static mut DAT_0071b825: u8;
    static mut DAT_0071b885: u8;
    static mut DAT_0071b8e5: u8;
    static mut DAT_0071b917: u8;
    static mut DAT_0071b949: u8;
    static mut DAT_0071b97b: u8;
    static mut DAT_0071b987: u8;
    static mut DAT_0071b993: u8;
    static mut DAT_0071b99f: u8;
    static mut DAT_0071b9af: u8;
    static mut DAT_0071b9bf: u8;
    static mut DAT_0071b9c9: u8;
    static mut DAT_0071b9d9: u8;
    static mut DAT_00732fe2: u16;
    static mut DAT_00732950: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a1980(a: u32) -> i32;
    fn FUN_004a1590(a: i32, b: u32, c: i32, d: u16, e: i32, f: u32) -> i32;
    fn FUN_0040b730(a: i32, b: u32, c: i32, d: i32, e: u32, f: u16);
    fn FUN_004a1d70(a: i32, b: i32);
    fn FUN_0040c070(a: i32, b: i32, c: u32, d: u32);
}

#[inline(always)]
fn ad(base: *mut u8, off: u32) -> u32 {
    (base as usize as u32).wrapping_add(off)
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u8).read() }
}

#[inline(always)]
unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn inc16(a: u32) {
    // SAFETY: address inside the game's own memory.
    unsafe {
        let p = a as usize as *mut u16;
        p.write_unaligned(p.read_unaligned().wrapping_add(1));
    }
}

#[inline(always)]
unsafe fn add32(a: u32, d: u32) {
    // SAFETY: address inside the game's own memory.
    unsafe {
        let p = a as usize as *mut u32;
        p.write_unaligned(p.read_unaligned().wrapping_add(d));
    }
}

#[inline(always)]
unsafe fn dmg(uvar: u32, uvar10: u32) -> i32 {
    // SAFETY: game globals and callee.
    unsafe {
        let _ = uvar;
        let uvar4 = u32::from(rd8(ad(
            &raw mut DAT_005247e8,
            u32::from(rd8(ad(
                &raw mut DAT_0070ad42,
                u32::from(rd8(uvar10
                    .wrapping_mul(0x23)
                    .wrapping_add(0x16)
                    .wrapping_add(DAT_006d4988 as u32)))
                .wrapping_mul(0x566),
            ))),
        )));
        if rd8(ad(&raw mut DAT_0052686a, uvar4.wrapping_mul(0x16))) == 0 {
            FUN_004a1980(uvar10)
        } else {
            let mut r = if (uvar4 < 0x46 || 0x4a < uvar4) && (uvar4 < 0x50 || 0x54 < uvar4) {
                0x28i32
            } else {
                500
            };
            if DAT_006d4a02 == 2 {
                r = r.wrapping_mul(3);
            }
            r
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a0880(
    param_1: u32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    // SAFETY: all globals, callees and computed addresses are the game's own memory.
    unsafe {
        let uvar10 = param_1 & 0xffff;
        let mut u_stack_12: u16 = 0;
        let mut local_10: u16 = 1;
        let uvar3: u16 = u16::from(rd8(uvar10
            .wrapping_mul(0x23)
            .wrapping_add(0x16)
            .wrapping_add(DAT_006d4988 as u32)));
        let uvar7 = u32::from(uvar3);
        let ivar5 = uvar7.wrapping_mul(0x566);
        let mut pc8 = ad(&raw mut DAT_0070aee5, ivar5);
        let ivar2 = (param_4 as u32).wrapping_mul(0x5bd);
        let w = rd16(ad(&raw mut DAT_0071b73c, ivar2));
        let mut pc9 = ad(&raw mut DAT_0070aee9, u32::from(w)).wrapping_add(ivar5);
        let mut local_4 = 8i32;
        loop {
            if rd8(pc8) == 0 && rd8(pc9) == 1 {
                let c = rd8(pc8.wrapping_add(1)) as i8;
                if c == 0 {
                    u_stack_12 = 0x6000;
                } else if c == 2 {
                    u_stack_12 = 0xf000;
                } else if c == 10 {
                    u_stack_12 = 0x6000;
                }
            }
            pc8 = pc8.wrapping_add(0x4e);
            pc9 = pc9.wrapping_add(0x4e);
            local_4 -= 1;
            if local_4 == 0 {
                break;
            }
        }
        let w = rd16(ad(&raw mut DAT_0071b73c, ivar2));
        let idx = u32::from(w)
            .wrapping_mul(0x1e5)
            .wrapping_add(u32::from(rd8(ad(&raw mut DAT_0070ad48, ivar5))));
        if rd8(ad(&raw mut DAT_0072f508, idx)) == 0 {
            let mut v5 = dmg(uvar7, uvar10);
            let p6 = (param_6 as u32).wrapping_mul(2);
            if param_3 == 1 {
                inc16(ad(
                    &raw mut DAT_0071b885,
                    uvar7.wrapping_mul(2).wrapping_add(ivar2),
                ));
                if param_5 == -1 {
                    inc16(ad(&raw mut DAT_0071b993, p6.wrapping_add(ivar2)));
                } else {
                    inc16(ad(&raw mut DAT_0071b949, p6.wrapping_add(ivar2)));
                }
                v5 /= 10;
                local_10 = 10;
            } else if param_3 == 2 {
                inc16(ad(
                    &raw mut DAT_0071b825,
                    uvar7.wrapping_mul(2).wrapping_add(ivar2),
                ));
                if param_5 == -1 {
                    inc16(ad(&raw mut DAT_0071b987, p6.wrapping_add(ivar2)));
                } else {
                    inc16(ad(
                        &raw mut DAT_0071b9af,
                        (param_5 as u32).wrapping_mul(2).wrapping_add(ivar2),
                    ));
                    inc16(ad(&raw mut DAT_0071b917, p6.wrapping_add(ivar2)));
                    inc16(ad(
                        &raw mut DAT_0071b9d9,
                        (param_4 as u32)
                            .wrapping_mul(2)
                            .wrapping_add((param_5 as u32).wrapping_mul(0x5bd)),
                    ));
                }
                local_10 = 6;
                v5 /= 2;
            } else if param_3 == 3 {
                inc16(ad(
                    &raw mut DAT_0071b7c5,
                    uvar7.wrapping_mul(2).wrapping_add(ivar2),
                ));
                if param_5 == -1 {
                    inc16(ad(&raw mut DAT_0071b97b, p6.wrapping_add(ivar2)));
                } else {
                    inc16(ad(
                        &raw mut DAT_0071b99f,
                        (param_5 as u32).wrapping_mul(2).wrapping_add(ivar2),
                    ));
                    inc16(ad(&raw mut DAT_0071b8e5, p6.wrapping_add(ivar2)));
                    inc16(ad(
                        &raw mut DAT_0071b9c9,
                        (param_4 as u32)
                            .wrapping_mul(2)
                            .wrapping_add((param_5 as u32).wrapping_mul(0x5bd)),
                    ));
                }
                local_10 = 1;
            }
            if DAT_007092d4 == 2 {
                v5 = v5.wrapping_mul(2);
            } else if DAT_007092d4 == 0 {
                v5 /= 2;
            }
            let w = rd16(ad(&raw mut DAT_0071b73c, ivar2));
            let ivar6 = FUN_004a1590(
                2,
                (u32::from(u_stack_12) << 16) | u32::from(uvar3),
                param_4,
                local_10,
                param_2,
                u32::from(w),
            );
            if -1 < ivar6 {
                add32(ad(&raw mut DAT_0071b79d, ivar2), v5 as u32);
            }
            if param_3 == 3 && DAT_0071ecf4 == param_4 {
                FUN_0040b730(DAT_0071ecf4, 0xffff_ffff, 0x15, -1, 0xffff_ffff, u_stack_12);
                return;
            }
        } else if param_3 == 2 || param_3 == 3 {
            let dm = dmg(uvar7, uvar10);
            add32(ad(&raw mut DAT_0071b79d, ivar2), (dm as u32).wrapping_neg());
            inc16(ad(&raw mut DAT_0071b9bf, ivar2));
            add32(ad(&raw mut DAT_0071b7a1, ivar2), (-500i32) as u32);
            let s = rd16(ad(&raw mut DAT_0071b73a, ivar2));
            DAT_00732fe2 = s;
            FUN_004a1d70(0x10f, param_4);
            if dm != 0 {
                DAT_00732950 = dm as u16;
                FUN_004a1d70(199, param_4);
            }
            if DAT_0071ecf4 == param_4 {
                FUN_0040c070(2, 0x39, uvar10, 0xffff);
            }
        }
    }
}
