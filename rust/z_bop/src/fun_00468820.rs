// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b0: u16;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a73b6: [u8; 4];
    static mut DAT_009a73d2: u16;
    static mut DAT_009a73d0: u16;
    static mut DAT_00a08104: u32;
    static mut DAT_005243c8: [i32; 1];
    static mut DAT_005243b8: [i32; 1];
    static mut DAT_005243d8: [u8; 1];
    static mut DAT_009e9640: u16;
    static mut DAT_00a00854: i32;
    static mut DAT_009a8070: u16;
    static mut DAT_009a73c0: u16;
    static mut DAT_009d8c84: [u8; 1];
    static mut DAT_009d9155: [u8; 1];
    static mut DAT_009d8c88: [u8; 1];
    static mut DAT_009fd458: [u8; 1];
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_00a080f0: u16;
    static mut DAT_009a8c00: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00403360(param_1: u32, param_2: i32) -> u8;
    fn FUN_004034e0(param_1: u32, param_2: u32);
    fn FUN_0049f360();
    fn FUN_00407090(param_1: u32, param_2: u32) -> u32;
    fn FUN_004a4d70(param_1: u32, param_2: u32) -> u32;
}

unsafe fn rb(a: u32) -> u8 {
    // SAFETY: the address is a game-memory location the original reads.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn rh(a: u32) -> u16 {
    // SAFETY: the address is a game-memory location the original reads.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn rw(a: u32) -> u32 {
    // SAFETY: the address is a game-memory location the original reads.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn wb(a: u32, v: u8) {
    // SAFETY: the address is a game-memory location the original writes.
    unsafe { (a as usize as *mut u8).write(v) }
}

unsafe fn wh(a: u32, v: u16) {
    // SAFETY: the address is a game-memory location the original writes.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00468820() -> u32 {
    // SAFETY: all globals and callees are the game's own; every pointer
    // access is at the offsets the original uses.
    unsafe {
        let mut cvar1: i8 = rb((DAT_009a73b0 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(2)
            .wrapping_add(DAT_009a1fe8)) as i8;
        if cvar1 == 3 || cvar1 == 4 {
            return DAT_009a1fe8 & 0xffff0000;
        }
        let mut uvar10: u32 = DAT_009a73d2 as u32;
        let mut ubase: u32 = rw((&raw const DAT_009a73b6) as usize as u32);
        if rb(ubase.wrapping_add(0x34)) as u32 == DAT_009a73d2 as u32 {
            if rh(DAT_00a08104.wrapping_add(0x6a)) as i16 == -1 {
                let mut ivar3: i32 = (&raw const DAT_005243c8)
                    .cast::<i32>()
                    .add(DAT_009a73d0 as usize)
                    .read();
                let mut uvar9: u16 = DAT_009e9640;
                if (uvar9 as i32) < DAT_00a00854 {
                    loop {
                        let ivar6 = (uvar9 as u32).wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
                        cvar1 = rb(ivar6.wrapping_add(3)) as i8;
                        if cvar1 != 0 {
                            let utmp = rw(ivar6.wrapping_add(0x1f));
                            let utmp2 = rw(utmp.wrapping_add(0xa5));
                            let iv = utmp2;
                            if rb(iv.wrapping_add(1)) != 0 && rh(iv.wrapping_add(4)) == DAT_009a73b0
                            {
                                let mut iv6: i32 = ivar3;
                                if cvar1 == -0x70 || cvar1 == -0x6b {
                                    iv6 = ivar3.wrapping_mul(3);
                                }
                                let bvar4 = FUN_00403360(uvar9 as u32, iv6);
                                if bvar4 == 1 {
                                    wh(DAT_00a08104.wrapping_add(0x6a), uvar9);
                                    FUN_004034e0(DAT_009a73b0 as u32, uvar9 as u32);
                                    let stmp = rh((DAT_009a73b0 as u32)
                                        .wrapping_mul(0x23)
                                        .wrapping_add(0x10)
                                        .wrapping_add(DAT_009a1fe8));
                                    let idx = (DAT_009a8070.wrapping_sub(stmp) >> 0xd) as usize;
                                    if (&raw const DAT_005243d8).cast::<u8>().add(idx).read() == 0 {
                                        ubase = rw((&raw const DAT_009a73b6) as usize as u32);
                                        wb(ubase.wrapping_add(0x34), 1);
                                    } else {
                                        ubase = rw((&raw const DAT_009a73b6) as usize as u32);
                                        wb(ubase.wrapping_add(0x34), 0x20);
                                    }
                                    FUN_0049f360();
                                    uvar10 = DAT_00a08104;
                                    if rb(DAT_00a08104.wrapping_add(0x247)) != 0 {
                                        if rb(DAT_00a08104.wrapping_add(0x246)) == 1
                                            && (rb(DAT_00a08104.wrapping_add(0x1e5)) & 2) != 0
                                        {
                                            let svar2 = rh(DAT_00a08104.wrapping_add(0x248));
                                            uvar10 = (DAT_00a08104 & 0xffff0000) | svar2 as u32;
                                            if svar2 == 0 {
                                                wh(DAT_00a08104.wrapping_add(0x248), 10);
                                                let uvar7 = DAT_00a08104;
                                                uvar10 = (DAT_009a73c0 as u32).wrapping_mul(0x566);
                                                if (&raw const DAT_009d8c84)
                                                    .cast::<u8>()
                                                    .add(uvar10 as usize)
                                                    .read()
                                                    != 0x15
                                                    && (&raw const DAT_009d9155)
                                                        .cast::<u8>()
                                                        .add(uvar10 as usize)
                                                        .read()
                                                        != 0x15
                                                {
                                                    let a = DAT_00a08104.wrapping_add(0x247);
                                                    wb(a, rb(a).wrapping_sub(1));
                                                    return uvar7 & 0xffff0000;
                                                }
                                            }
                                        } else if rb(DAT_00a08104.wrapping_add(0x246)) == 2
                                            && rh(DAT_00a08104.wrapping_add(0x24a)) == 0
                                        {
                                            uvar10 = FUN_00407090(DAT_009a73b0 as u32, 0x9b);
                                        }
                                    }
                                    return uvar10 & 0xffff0000;
                                }
                            }
                        }
                        uvar9 = uvar9.wrapping_add(1);
                        if (uvar9 as i32) >= DAT_00a00854 {
                            break;
                        }
                    }
                }
                ivar3 = (&raw const DAT_005243b8)
                    .cast::<i32>()
                    .add(DAT_009a73d0 as usize)
                    .read();
                if rh(DAT_00a08104.wrapping_add(0x6a)) as i16 == -1 {
                    uvar9 = DAT_00a07cec;
                    if (uvar9 as i32) < DAT_009a1fe0 {
                        loop {
                            uvar10 = uvar9 as u32;
                            let ivar6 = DAT_009a1fe8.wrapping_add(uvar10.wrapping_mul(0x23));
                            if rb(ivar6.wrapping_add(3)) != 0 {
                                let utmp = rw((DAT_009a73b0 as u32)
                                    .wrapping_mul(0x23)
                                    .wrapping_add(0x1f)
                                    .wrapping_add(DAT_009a1fe8));
                                let uvar7 = rb(utmp.wrapping_add(0x89)) as u32;
                                let idx1 =
                                    (rb(ivar6.wrapping_add(0x16)) as u32).wrapping_mul(0x566);
                                let x = (&raw const DAT_009d8c88)
                                    .cast::<u8>()
                                    .add(idx1 as usize)
                                    .read() as u32;
                                let bvar4: u8 = if x == uvar7 {
                                    0
                                } else {
                                    let i2 = x.wrapping_add(uvar7.wrapping_mul(0x1e5));
                                    ((&raw const DAT_009fd458)
                                        .cast::<u8>()
                                        .add(i2 as usize)
                                        .read()
                                        == 0) as u8
                                };
                                if bvar4 != 0
                                    && rb(DAT_00a08104.wrapping_add(7)) == 0
                                    && rb(ivar6.wrapping_add(2)) == 0
                                    && FUN_00403360(uvar10, ivar3) == 1
                                {
                                    FUN_004034e0(uvar10, DAT_009a73b0 as u32);
                                    let iv6 = DAT_009a1fe8.wrapping_add(uvar10.wrapping_mul(0x23));
                                    let stmp = rh(iv6.wrapping_add(0x10));
                                    let mut uvar5 = DAT_009a8070.wrapping_sub(stmp);
                                    if 0x7fff < uvar5 {
                                        uvar5 = 0u16.wrapping_sub(uvar5);
                                    }
                                    let stmp2 = rh(iv6.wrapping_add(0x12));
                                    let mut uvar8 = DAT_00a080f0.wrapping_sub(stmp2);
                                    if 0x7fff < uvar8 {
                                        uvar8 = 0u16.wrapping_sub(uvar8);
                                    }
                                    if uvar5 < 0x2000 && uvar8 < 0x2000 {
                                        wh(DAT_00a08104.wrapping_add(0x6a), uvar9);
                                        break;
                                    }
                                }
                            }
                            uvar9 = uvar9.wrapping_add(1);
                            if (uvar9 as i32) >= DAT_009a1fe0 {
                                break;
                            }
                        }
                    }
                }
            }
            uvar10 = DAT_00a08104;
            if rh(DAT_00a08104.wrapping_add(0x6a)) != 0xffff {
                let uvar7 = DAT_009a73b0 as u32;
                cvar1 = rb(uvar7
                    .wrapping_mul(0x23)
                    .wrapping_add(2)
                    .wrapping_add(DAT_009a1fe8)) as i8;
                uvar10 = (((uvar7.wrapping_mul(7) >> 8) & 0xffffff) << 8) | (cvar1 as u8 as u32);
                if cvar1 != 2 {
                    let mut icond = false;
                    if (rb(DAT_00a08104.wrapping_add(0x1e5)) & 1) != 0 {
                        let itmp = rw(DAT_00a08104.wrapping_add(0x211)) as i32;
                        if itmp < 500 {
                            icond = true;
                        }
                    }
                    if !icond {
                        uvar10 = DAT_00a08104;
                        let utmp = rw(DAT_00a08104.wrapping_add(199));
                        let utmp2 = rw(DAT_00a08104.wrapping_add(0xc3));
                        if utmp <= utmp2 {
                            icond = true;
                        }
                    }
                    if icond {
                        FUN_004034e0(rh(DAT_00a08104.wrapping_add(0x6a)) as u32, uvar7);
                        if DAT_009a8c00 < 0x8000
                            && rb(DAT_00a08104.wrapping_add(0x246)) == 2
                            && rb(DAT_00a08104.wrapping_add(0x247)) != 0
                            && rh(DAT_00a08104.wrapping_add(0x24a)) == 0
                        {
                            FUN_00407090(DAT_009a73b0 as u32, 0x9b);
                        }
                        uvar10 = DAT_009a1fe8;
                        let itmp = rw((rh(DAT_00a08104.wrapping_add(0x6a)) as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(0x1b)
                            .wrapping_add(DAT_009a1fe8)) as i32;
                        if itmp != -1 {
                            ubase = rw((&raw const DAT_009a73b6) as usize as u32);
                            wb(ubase.wrapping_add(0x34), 0x20);
                            FUN_0049f360();
                            uvar10 = FUN_004a4d70(DAT_009a73b0 as u32, 0xffff);
                        }
                    }
                }
            }
        }
        uvar10 & 0xffff0000
    }
}
