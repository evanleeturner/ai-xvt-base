// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00780c82: [u8; 4];
    static mut DAT_00780e86: [u8; 4];
    static mut DAT_00780e8a: [u8; 4];
    static mut DAT_00780e8e: [u8; 4];
    static mut DAT_00781092: [u8; 4];
    static mut DAT_00781096: [u8; 4];
    static mut DAT_0078072a: [u8; 4];
    static mut DAT_00780c7a: [u8; 4];
    static mut DAT_007806fa: [u8; 4];
    static mut DAT_0081b5be: [u8; 4];
    static mut DAT_008158b2: [u8; 4];
    static mut DAT_008050a6: [u8; 4];
    static mut DAT_008050ae: [u8; 4];

    static mut DAT_00780c86: [u8; 1];
    static mut DAT_00780e92: [u8; 1];
    static mut DAT_00815ace: [u8; 1];
    static mut DAT_00815ad6: [u8; 1];
    static mut DAT_00815ade: [u8; 1];
    static mut DAT_00815ae2: [u8; 1];
    static mut DAT_00815ae6: [u8; 1];
    static mut DAT_00815ae7: [u8; 1];
    static mut DAT_00815ce6: [u8; 1];
    static mut DAT_008050b2: [u8; 1];
    static mut DAT_008050b6: [u8; 1];
    static mut DAT_008050ba: [u8; 1];
    static mut DAT_008050be: [u8; 1];
    static mut DAT_008050bf: [u8; 1];
    static mut DAT_008050c0: [u8; 1];
    static mut DAT_008050c2: [u8; 1];
    static mut DAT_007810a6: [u8; 1];
    static mut DAT_007810aa: [u8; 1];
    static mut DAT_007810ae: [u8; 1];
    static mut DAT_007810b2: [u8; 1];
    static mut DAT_007810b3: [u8; 1];
    static mut DAT_007810b4: [u8; 1];
    static mut DAT_007810b5: [u8; 1];
    static mut DAT_007810b6: [u8; 1];

    static mut DAT_00532468: [i8; 1];
    static mut DAT_0053245c: [i8; 1];
    static mut DAT_00532450: [i8; 1];
    static mut DAT_00532438: [i8; 1];

    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00433150() -> i32;
    fn FUN_004cee90(param_1: i32) -> u32;
}

macro_rules! p {
    ($n:ident) => {
        (&raw mut $n).cast::<u8>()
    };
}

unsafe fn rd(p: *const u8) -> u32 {
    // SAFETY: caller passes a valid address of at least 4 bytes.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn wr(p: *mut u8, v: u32) {
    // SAFETY: caller passes a valid address of at least 4 bytes.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

unsafe fn cpy(dst: *mut u8, src: *const u8, n: u32) {
    let mut i: u32 = 0;
    while i < n {
        // SAFETY: caller guarantees both ranges are valid for n bytes.
        unsafe {
            *dst.add(i as usize) = *src.add(i as usize);
        }
        i = i.wrapping_add(1);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cb8b0(param_1: i32, param_2: *mut u32, param_3: u32) -> u8 {
    // SAFETY: all globals and callees are the game's own memory; param_2
    // points at the caller's packet, read where the original reads it.
    unsafe {
        let mut local_554: i32 = 0;
        let mut local_550: [u32; 20] = [0; 20];
        let mut local_500: [u8; 0x400] = [0; 0x400];
        let mut local_100: [i8; 256] = [0; 256];
        let lp = local_500.as_mut_ptr();
        let mut local_564: i32;
        let mut uvar7: u16;
        let mut utmp_a: u32;
        let mut utmp_b: u32;
        let mut utmp_c: u32;

        let uvar6_0 = param_2.read();
        let bvar4 = (uvar6_0 as u8) & 0x7f;
        uvar7 = bvar4 as u16;
        local_550[0] = if uvar6_0 < 0x3c || 0x3f < uvar6_0 {
            1
        } else {
            0
        };
        let svar1 = param_3 as i16;
        let psrc = param_2.add(1).cast::<u8>();

        if param_1 == 0 {
            utmp_a = rd(p!(DAT_00780c82));
            _sprintf(
                local_100.as_mut_ptr(),
                p!(DAT_00532468).cast::<i8>(),
                utmp_a,
            );
            uvar7 = bvar4 as u16 | (((utmp_a as u16) & 0x7f) << 8);
            utmp_a = utmp_a.wrapping_add(1);
            wr(p!(DAT_00780c82), utmp_a);
            if 0x7f < utmp_a {
                wr(p!(DAT_00780c82), 0);
            }
            let mut puvar8 = lp.add(2);
            local_564 = 2;
            lp.cast::<u16>().write_unaligned(uvar7);
            if local_550[0] != 0 && FUN_00433150() == 0 {
                lp.add(2)
                    .cast::<i16>()
                    .write_unaligned(svar1.wrapping_sub(4));
                puvar8 = lp.add(4);
                local_564 = 4;
            }
            let uvar6 = param_3.wrapping_sub(4);
            cpy(puvar8, psrc, uvar6);
            let mut puvar11 = puvar8.add(uvar6 as usize);
            local_564 = (uvar6 as i32).wrapping_add(local_564);
            if local_550[0] != 0 {
                utmp_a = rd(p!(DAT_00780e8a));
                if utmp_a == 0 {
                    utmp_b = rd(p!(DAT_00780e86));
                    local_550[0] = utmp_b;
                    cpy(puvar11, p!(DAT_00780c86), utmp_b);
                    utmp_b = rd(p!(DAT_00780e86));
                    local_564 = local_564.wrapping_add(utmp_b as i32);
                } else {
                    local_564 = local_564.wrapping_add(1);
                    *puvar11 = 0x39;
                    wr(p!(DAT_00780e8a), 0);
                }
            }
            let _ = &mut puvar11;
            *p!(DAT_00780c86) = bvar4;
            cpy(p!(DAT_00780c86).add(1), psrc, uvar6);
            wr(p!(DAT_00780e86), param_3.wrapping_sub(3));
        } else {
            utmp_a = rd(p!(DAT_0078072a));
            if utmp_a as i32 == param_1 {
                utmp_a = rd(p!(DAT_00780c82));
                _sprintf(
                    local_100.as_mut_ptr(),
                    p!(DAT_0053245c).cast::<i8>(),
                    utmp_a,
                );
                utmp_a = rd(p!(DAT_00780e8e));
                uvar7 = bvar4 as u16 | (((utmp_a as u16) & 0x7f) << 8) | 0x8080;
                utmp_a = utmp_a.wrapping_add(1);
                wr(p!(DAT_00780e8e), utmp_a);
                if 0x7f < utmp_a {
                    wr(p!(DAT_00780e8e), 0);
                }
                let mut puvar8 = lp.add(2);
                local_564 = 2;
                lp.cast::<u16>().write_unaligned(uvar7);
                if local_550[0] != 0 && FUN_00433150() == 0 {
                    lp.add(2)
                        .cast::<i16>()
                        .write_unaligned(svar1.wrapping_sub(4));
                    puvar8 = lp.add(4);
                    local_564 = 4;
                }
                let uvar6 = param_3.wrapping_sub(4);
                cpy(puvar8, psrc, uvar6);
                let puvar11 = puvar8.add(uvar6 as usize);
                local_564 = (uvar6 as i32).wrapping_add(local_564);
                if local_550[0] != 0 {
                    utmp_a = rd(p!(DAT_00781096));
                    if utmp_a == 0 {
                        utmp_b = rd(p!(DAT_00781092));
                        local_550[0] = utmp_b;
                        cpy(puvar11, p!(DAT_00780e92), utmp_b);
                        utmp_b = rd(p!(DAT_00781092));
                        local_564 = local_564.wrapping_add(utmp_b as i32);
                    } else {
                        local_564 = local_564.wrapping_add(1);
                        *puvar11 = 0x39;
                        wr(p!(DAT_00781096), 0);
                    }
                }
                *p!(DAT_00780e92) = bvar4;
                cpy(p!(DAT_00780e92).add(1), psrc, uvar6);
                wr(p!(DAT_00781092), param_3.wrapping_sub(3));
            } else {
                let uvar6 = FUN_004cee90(param_1);
                utmp_b = rd(p!(DAT_0081b5be));
                let off = uvar6.wrapping_mul(0x238) as usize;
                if uvar6 < utmp_b && uvar6 < 0x28 {
                    let iv2 = rd(p!(DAT_00815ae2).add(off)) as i32;
                    _sprintf(
                        local_100.as_mut_ptr(),
                        p!(DAT_00532450).cast::<i8>(),
                        iv2 as u32,
                    );
                    let iv2b = rd(p!(DAT_00815ae2).add(off)) as i32;
                    let iv3 = iv2b.wrapping_add(1);
                    wr(p!(DAT_00815ae2).add(off), iv3 as u32);
                    uvar7 = bvar4 as u16 | (((iv2b as u16) & 0x7f) << 8);
                    if 0x7f < iv3 {
                        wr(p!(DAT_00815ae2).add(off), 0);
                    }
                }
                uvar7 |= 0x8000;
                let mut local_55c = lp.add(2);
                local_564 = 2;
                lp.cast::<u16>().write_unaligned(uvar7);
                if local_550[0] != 0 && FUN_00433150() == 0 {
                    lp.add(2)
                        .cast::<i16>()
                        .write_unaligned(svar1.wrapping_sub(4));
                    local_55c = lp.add(4);
                    local_564 = 4;
                }
                let n = param_3.wrapping_sub(4);
                cpy(local_55c, psrc, n);
                local_564 = (n as i32).wrapping_add(local_564);
                if local_550[0] != 0 {
                    utmp_a = rd(p!(DAT_00815ce6).add(off));
                    local_550[0] = utmp_a;
                    cpy(
                        local_55c.add(n as usize),
                        p!(DAT_00815ae6).add(off),
                        local_550[0],
                    );
                    utmp_a = rd(p!(DAT_00815ce6).add(off));
                    local_564 = local_564.wrapping_add(utmp_a as i32);
                }
                *p!(DAT_00815ae6).add(off) = bvar4;
                cpy(p!(DAT_00815ae7).add(off), psrc, n);
                wr(p!(DAT_00815ce6).add(off), param_3.wrapping_sub(3));
            }
        }

        utmp_a = rd(p!(DAT_00780c7a));
        if utmp_a as i32 != param_1 {
            utmp_b = rd(p!(DAT_008158b2));
            let off = utmp_b.wrapping_mul(0x210) as usize;
            cpy(p!(DAT_008050c2).add(off), param_2.cast::<u8>(), param_3);
            wr(p!(DAT_008050b2).add(off), param_1 as u32);
            wr(p!(DAT_008050b6).add(off), param_3);
            wr(p!(DAT_008050ba).add(off), 0);
            *p!(DAT_008050be).add(off) = 0;
            if param_1 == 0 {
                *p!(DAT_008050bf).add(off) = 0;
            } else {
                utmp_a = rd(p!(DAT_0078072a));
                if utmp_a as i32 == param_1 {
                    *p!(DAT_008050bf).add(off) = 2;
                } else {
                    *p!(DAT_008050bf).add(off) = 1;
                }
            }
            *p!(DAT_008050c0).add(off) = local_500[1] & 0x7f;
            utmp_b = utmp_b.wrapping_add(1);
            wr(p!(DAT_008158b2), utmp_b);
            uvar7 = lp.cast::<u16>().read_unaligned();
            if 0x7f < utmp_b {
                utmp_b = 0;
                wr(p!(DAT_008158b2), utmp_b);
            }
        }

        utmp_a = rd(p!(DAT_00780c7a));
        let mut piface = p!(DAT_007806fa).cast::<*mut i32>().read_unaligned();
        let cond = utmp_a == param_1 as u32 || param_1 == 0 || piface.is_null() || {
            utmp_c = rd(p!(DAT_0078072a));
            utmp_c as i32 == param_1
        };
        if cond {
            utmp_b = rd(p!(DAT_008050ae));
            if utmp_b < 0x400 {
                utmp_c = rd(p!(DAT_008050a6));
                let off = utmp_c.wrapping_mul(0x210) as usize;
                cpy(p!(DAT_007810b6).add(off), param_2.cast::<u8>(), param_3);
                utmp_a = rd(p!(DAT_00780c7a));
                wr(p!(DAT_007810a6).add(off), utmp_a);
                wr(p!(DAT_007810aa).add(off), param_3);
                wr(p!(DAT_007810ae).add(off), 0);
                *p!(DAT_007810b2).add(off) = 0;
                *p!(DAT_007810b5).add(off) = 0;
                utmp_a = rd(p!(DAT_00780c7a));
                let uvar6 = FUN_004cee90(utmp_a as i32);
                utmp_b = rd(p!(DAT_0081b5be));
                let off2 = uvar6.wrapping_mul(0x238) as usize;
                let val = ((uvar7 & 0x7f00) >> 8) as u32;
                if param_1 == 0 {
                    *p!(DAT_007810b3).add(off) = 0;
                    if uvar6 < utmp_b && uvar6 < 0x28 {
                        wr(p!(DAT_00815ad6).add(off2), val);
                    }
                } else {
                    utmp_a = rd(p!(DAT_0078072a));
                    if utmp_a as i32 == param_1 {
                        *p!(DAT_007810b3).add(off) = 2;
                        if uvar6 < utmp_b && uvar6 < 0x28 {
                            wr(p!(DAT_00815ade).add(off2), val);
                        }
                    } else {
                        *p!(DAT_007810b3).add(off) = 1;
                        if uvar6 < utmp_b && uvar6 < 0x28 {
                            wr(p!(DAT_00815ace).add(off2), val);
                        }
                    }
                }
                *p!(DAT_007810b4).add(off) = local_500[1] & 0x7f;
                utmp_c = utmp_c.wrapping_add(1);
                wr(p!(DAT_008050a6), utmp_c);
                utmp_b = rd(p!(DAT_008050ae));
                utmp_b = utmp_b.wrapping_add(1);
                wr(p!(DAT_008050ae), utmp_b);
                if 0x3ff < utmp_c {
                    wr(p!(DAT_008050a6), 0);
                }
            }
        }

        piface = p!(DAT_007806fa).cast::<*mut i32>().read_unaligned();
        if !piface.is_null() {
            utmp_a = rd(p!(DAT_00780c7a));
            if utmp_a as i32 != param_1 {
                let vt = piface.read() as u32;
                let f = ((vt.wrapping_add(0x68)) as *const u32).read_unaligned();
                let func: unsafe extern "stdcall" fn(
                    *mut i32,
                    u32,
                    i32,
                    i32,
                    *mut core::ffi::c_void,
                    i32,
                ) -> i32 = core::mem::transmute(f as usize as *const core::ffi::c_void);
                local_554 = func(
                    piface,
                    utmp_a,
                    param_1,
                    0,
                    lp.cast::<core::ffi::c_void>(),
                    local_564,
                );
            }
            if local_554 != 0 {
                _sprintf(
                    local_550.as_mut_ptr().cast::<i8>(),
                    p!(DAT_00532438).cast::<i8>(),
                    local_554,
                );
            }
            return (local_554 == 0) as u8;
        }
        1
    }
}
