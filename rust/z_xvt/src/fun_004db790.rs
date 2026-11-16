// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0077dc66: u8;
    static mut DAT_0077dc6e: u8;
    static mut DAT_0077dc72: u8;
    static mut DAT_0077dc76: u8;
    static mut DAT_0077dc7a: u8;
    static mut DAT_0077dc7e: u8;
    static mut DAT_0077e082: u8;
    static mut DAT_0077e092: u8;
    static mut DAT_0077e096: u8;
    static mut DAT_0077e09a: u8;
    static mut DAT_0077e09e: u8;
    static mut DAT_0077e0a2: u8;
    static mut DAT_0077e0aa: u8;
    static mut DAT_007806f2: u8;
    static mut DAT_007596b7: u8;
    static mut DAT_007596bb: u8;
    static mut DAT_007596bf: u8;
    static mut DAT_007596c3: u8;
    static mut DAT_00759657: u8;
    static mut DAT_0075964b: u8;
    static mut DAT_00759645: u8;
    static mut DAT_0075965c: u8;
    static mut DAT_00758830: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _malloc(size: u32) -> *mut core::ffi::c_void;
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dd000(param_1: *mut u32, param_2: *mut i32);
    fn FUN_004d4d00() -> u32;
    fn FUN_004d52b0();
    fn FUN_004d5350(param_1: i32);
    fn FUN_004d4d90();
    fn FUN_004d3070(param_1: *mut i32) -> u32;
    fn FUN_004dbd20();
    fn FUN_004d51b0(param_2: *mut i32);
}

#[inline(always)]
fn off(p: *mut u8, o: i32) -> *mut u8 {
    p.wrapping_offset(o as isize)
}

#[inline(always)]
unsafe fn rd(p: *mut u8) -> u32 {
    // SAFETY: caller passes a valid address in the game's memory.
    unsafe { p.cast::<u32>().read_unaligned() }
}

#[inline(always)]
unsafe fn wr(p: *mut u8, v: u32) {
    // SAFETY: caller passes a valid address in the game's memory.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004db790(param_1: u32, param_2: *mut i32) -> u32 {
    let mut param_2 = param_2;
    let mut local_10: [i32; 4] = [0; 4];
    let mut local_20: *mut u8 = core::ptr::null_mut();
    let mut uvar9: u32 = 0;
    let mut ivar4: i32 = 0;

    // SAFETY: all globals and callees are the game's own; pointers refer to
    // the game's buffers and are accessed as the original does.
    unsafe {
        let ivar2 = rd(&raw mut DAT_0077dc66) as i32;
        if 8 < ivar2 {
            return 0;
        }
        let ivar3 = (ivar2 as u32).wrapping_mul(0x440) as i32;

        wr(off(&raw mut DAT_0077e0aa, ivar3), rd(&raw mut DAT_007806f2));
        wr(off(&raw mut DAT_0077e092, ivar3), rd(&raw mut DAT_007596b7));
        wr(off(&raw mut DAT_0077e09a, ivar3), rd(&raw mut DAT_007596bb));
        wr(off(&raw mut DAT_0077e096, ivar3), rd(&raw mut DAT_007596bf));
        wr(off(&raw mut DAT_0077e09e, ivar3), rd(&raw mut DAT_007596c3));

        'out: {
            if param_2.is_null() {
                wr(off(&raw mut DAT_0077dc6e, ivar3), 0);
                wr(off(&raw mut DAT_0077dc72, ivar3), 0);
                wr(off(&raw mut DAT_0077dc7e, ivar3), 0);
                wr(off(&raw mut DAT_0077dc7a, ivar3), 0);
                FUN_004dcfd0(local_10.as_mut_ptr(), 0, 0, 0x27f, 0x1df);
                param_2 = local_10.as_mut_ptr();
                break 'out;
            }
            if *param_2 < 0 {
                *param_2 = 0;
            }
            if *param_2.add(1) < 0 {
                *param_2.add(1) = 0;
            }
            if 0x27f < *param_2.add(2) {
                *param_2.add(2) = 0x27f;
            }
            if 0x1df < *param_2.add(3) {
                *param_2.add(3) = 0x1df;
            }
            if *param_2.add(2) < *param_2 || *param_2.add(3) < *param_2.add(1) {
                return 0;
            }
            FUN_004dd000(off(&raw mut DAT_0077e082, ivar3).cast::<u32>(), param_2);
            uvar9 = ((*param_2.add(2)).wrapping_sub(*param_2) as u32).wrapping_add(1);
            ivar4 = (*param_2.add(3))
                .wrapping_sub(*param_2.add(1))
                .wrapping_add(1);
            let mut local_1c = ivar4;
            let bpp = rd(&raw mut DAT_00759657) as i32;
            if bpp == 8 {
                local_20 = _malloc((ivar4 as u32).wrapping_mul(uvar9)).cast::<u8>();
                let cvar1 = DAT_00759645;
                if local_20.is_null() {
                    return 0;
                }
                if DAT_0075965c == 0 {
                    DAT_00758830 = FUN_004d4d00();
                } else {
                    FUN_004d52b0();
                }
                let pitch = rd(&raw mut DAT_0075964b);
                let mut puvar5 = (*param_2.add(1) as u32)
                    .wrapping_mul(pitch)
                    .wrapping_add(*param_2 as u32)
                    .wrapping_add(DAT_00758830) as usize
                    as *mut u8;
                let mut puvar8 = local_20;
                if 0 < ivar4 {
                    loop {
                        let mut p10 = puvar5;
                        let mut p12 = puvar8;
                        let mut n = uvar9 >> 2;
                        while n != 0 {
                            wr(p12, rd(p10));
                            p10 = p10.wrapping_add(4);
                            p12 = p12.wrapping_add(4);
                            n -= 1;
                        }
                        let mut n = uvar9 & 3;
                        while n != 0 {
                            *p12 = *p10;
                            p10 = p10.wrapping_add(1);
                            p12 = p12.wrapping_add(1);
                            n -= 1;
                        }
                        let pitch = rd(&raw mut DAT_0075964b) as i32;
                        puvar5 = off(puvar5, pitch);
                        local_1c = local_1c.wrapping_sub(1);
                        puvar8 = puvar8.wrapping_add(uvar9 as usize);
                        if local_1c == 0 {
                            break;
                        }
                    }
                }
                if DAT_0075965c != 0 {
                    FUN_004d5350(1);
                }
                if cvar1 == 0 {
                    FUN_004d4d90();
                }
            } else if bpp == 0x10 {
                local_20 = _malloc((ivar4 as u32).wrapping_mul(uvar9).wrapping_mul(2)).cast::<u8>();
                let cvar1 = DAT_00759645;
                if local_20.is_null() {
                    return 0;
                }
                if DAT_0075965c == 0 {
                    DAT_00758830 = FUN_004d4d00();
                } else {
                    FUN_004d52b0();
                }
                let pitch = rd(&raw mut DAT_0075964b);
                let mut puvar7 = (*param_2.add(1) as u32)
                    .wrapping_mul(pitch)
                    .wrapping_add((*param_2 as u32).wrapping_mul(2))
                    .wrapping_add(DAT_00758830) as usize
                    as *mut u8;
                let mut local_18 = local_20;
                if 0 < ivar4 {
                    loop {
                        let mut uvar6 = uvar9;
                        let mut p8 = local_18;
                        let mut p11 = puvar7;
                        if 0 < uvar9 as i32 {
                            loop {
                                p8.cast::<u16>()
                                    .write_unaligned(p11.cast::<u16>().read_unaligned());
                                uvar6 = uvar6.wrapping_sub(1);
                                p8 = p8.wrapping_add(2);
                                p11 = p11.wrapping_add(2);
                                if uvar6 == 0 {
                                    break;
                                }
                            }
                        }
                        let pitch = rd(&raw mut DAT_0075964b);
                        puvar7 = puvar7.wrapping_add((pitch & 0xffff_fffe) as usize);
                        local_1c = local_1c.wrapping_sub(1);
                        local_18 = local_18.wrapping_add(uvar9.wrapping_mul(2) as usize);
                        if local_1c == 0 {
                            break;
                        }
                    }
                }
                if DAT_0075965c != 0 {
                    FUN_004d5350(1);
                }
                if cvar1 == 0 {
                    FUN_004d4d90();
                }
            }
            wr(off(&raw mut DAT_0077dc6e, ivar3), uvar9);
            wr(off(&raw mut DAT_0077dc72, ivar3), ivar4 as u32);
            wr(off(&raw mut DAT_0077dc7e, ivar3), local_20 as usize as u32);
            let bpp = rd(&raw mut DAT_00759657) as i32;
            if bpp == 8 {
                wr(
                    off(&raw mut DAT_0077dc7a, ivar3),
                    (ivar4 as u32).wrapping_mul(uvar9),
                );
                wr(off(&raw mut DAT_0077dc76, ivar3), 0);
                FUN_004d3070(off(&raw mut DAT_0077dc6e, ivar3).cast::<i32>());
            } else if bpp == 0x10 {
                wr(
                    off(&raw mut DAT_0077dc7a, ivar3),
                    (ivar4 as u32).wrapping_mul(uvar9).wrapping_mul(2),
                );
                wr(off(&raw mut DAT_0077dc76, ivar3), 0);
            }
            if DAT_0075965c != 0 {
                FUN_004dbd20();
            }
        }
        let _ = (uvar9, ivar4);
        FUN_004d51b0(param_2);
        let cnt = rd(&raw mut DAT_0077dc66).wrapping_add(1);
        wr(&raw mut DAT_0077dc66, cnt);
        wr(
            off(&raw mut DAT_0077e0a2, cnt.wrapping_mul(0x440) as i32),
            param_1,
        );
        wr(&raw mut DAT_007806f2, 0xffff_ffff);
    }
    1
}
