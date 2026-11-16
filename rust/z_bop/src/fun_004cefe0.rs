// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052b960: u8;
    static mut DAT_0052b954: u8;
    static mut DAT_0052b948: u8;
    static mut DAT_0052b930: u8;

    static mut DAT_00acf14d: u8;
    static mut DAT_00acf151: u8;
    static mut DAT_00acf351: u8;
    static mut DAT_00acf355: u8;
    static mut DAT_00acf359: u8;
    static mut DAT_00acf35d: u8;
    static mut DAT_00acf55d: u8;
    static mut DAT_00acf561: u8;
    static mut DAT_00acebf5: u8;
    static mut DAT_00acebc5: u8;
    static mut DAT_00acf145: u8;
    static mut DAT_00b53571: u8;
    static mut DAT_00b53579: u8;
    static mut DAT_00b63d7d: u8;
    static mut DAT_00b69a89: u8;
    static mut DAT_00b5357d: u8;
    static mut DAT_00b53581: u8;
    static mut DAT_00b53585: u8;
    static mut DAT_00b53589: u8;
    static mut DAT_00b5358a: u8;
    static mut DAT_00b5358b: u8;
    static mut DAT_00b5358d: u8;
    static mut DAT_00acf571: u8;
    static mut DAT_00acf575: u8;
    static mut DAT_00acf579: u8;
    static mut DAT_00acf57d: u8;
    static mut DAT_00acf57e: u8;
    static mut DAT_00acf57f: u8;
    static mut DAT_00acf580: u8;
    static mut DAT_00acf581: u8;
    static mut DAT_00b63f99: u8;
    static mut DAT_00b63fa1: u8;
    static mut DAT_00b63fa9: u8;
    static mut DAT_00b63fad: u8;
    static mut DAT_00b63fb1: u8;
    static mut DAT_00b641b1: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_0046fad0() -> i32;
    fn FUN_004d23b0(param_1: i32) -> u32;
}

macro_rules! a {
    ($d:ident) => {
        (&raw mut $d).cast::<u8>()
    };
}

unsafe fn rd(p: *const u8) -> u32 {
    // SAFETY: caller passes a readable address.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn wr(p: *mut u8, v: u32) {
    // SAFETY: caller passes a writable address.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

fn off(p: *mut u8, n: u32) -> *mut u8 {
    p.wrapping_add(n as usize)
}

unsafe fn copy(mut dst: *mut u8, mut src: *const u8, n: u32) {
    // SAFETY: caller passes valid regions, copied forward as the original.
    unsafe {
        let mut c = n >> 2;
        while c != 0 {
            wr(dst, rd(src));
            src = src.wrapping_add(4);
            dst = dst.wrapping_add(4);
            c = c.wrapping_sub(1);
        }
        let mut c = n & 3;
        while c != 0 {
            *dst = *src;
            src = src.wrapping_add(1);
            dst = dst.wrapping_add(1);
            c = c.wrapping_sub(1);
        }
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn branch12(
    param_2: *mut u32,
    param_3: u32,
    b4: u8,
    fmt: *const i8,
    sctr: *mut u8,
    ctr: *mut u8,
    mask: u16,
    flag: *mut u8,
    len: *mut u8,
    buf: *mut u8,
    loc500: *mut u16,
    loc100: *mut i8,
    loc550: *mut u32,
    loc564: &mut i32,
    s_var1: i16,
) -> u16 {
    // SAFETY: all pointers are the game's own memory or the caller's locals.
    unsafe {
        let mut t = rd(sctr);
        sprintf(loc100, fmt, t);
        t = rd(ctr);
        let u8v = t as u16;
        t = t.wrapping_add(1);
        wr(ctr, t);
        let u_var8: u16 = (b4 as u16) | ((u8v & 0x7f) << 8) | mask;
        if 0x7f < t {
            wr(ctr, 0);
        }
        let mut p7 = loc500.add(1).cast::<u8>();
        *loc564 = 2;
        *loc500 = u_var8;
        if *loc550 != 0 && FUN_0046fad0() == 0 {
            *loc500.add(1) = s_var1.wrapping_add(-4) as u16;
            p7 = loc500.add(2).cast::<u8>();
            *loc564 = 4;
        }
        let u6 = param_3.wrapping_sub(4);
        let src = param_2.add(1).cast::<u8>();
        copy(p7, src, u6);
        let p11 = off(p7, param_3.wrapping_sub(4));
        *loc564 = param_3.wrapping_sub(4).wrapping_add(*loc564 as u32) as i32;
        if *loc550 != 0 {
            let mut t2 = rd(flag);
            if t2 == 0 {
                *loc550 = rd(len);
                t2 = rd(len);
                copy(p11, buf, t2);
                t2 = rd(len);
                *loc564 = (*loc564 as u32).wrapping_add(t2) as i32;
            } else {
                *p11 = 0x39;
                *loc564 = (*loc564 as u32).wrapping_add(1) as i32;
                wr(flag, 0);
            }
        }
        *buf = b4;
        copy(buf.add(1), src, u6);
        wr(len, param_3.wrapping_sub(3));
        u_var8
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cefe0(param_1: i32, param_2: *mut u32, param_3: u32) -> u8 {
    // SAFETY: globals and callees are the game's own; param_2 points at the
    // caller's packet; locals are used as the original's stack buffers.
    unsafe {
        let mut local_500: [u16; 512] = [0; 512];
        let mut local_550: [u32; 20] = [0; 20];
        let mut local_100: [i8; 256] = [0; 256];
        let l500 = local_500.as_mut_ptr();
        let l550 = local_550.as_mut_ptr();
        let l100 = local_100.as_mut_ptr();
        let zero: u32 = 0;
        let mut local_554: i32 = 0;
        let mut local_564: i32 = 0;
        let u_var6 = *param_2;
        let b4 = (u_var6 as u8) & 0x7f;
        let mut u_var8: u16 = b4 as u16;
        local_550[0] = if u_var6 < 0x3c || 0x3f < u_var6 { 1 } else { 0 };
        let s_var1 = param_3 as i16;

        if param_1 == 0 {
            u_var8 = branch12(
                param_2,
                param_3,
                b4,
                a!(DAT_0052b960).cast::<i8>(),
                a!(DAT_00acf14d),
                a!(DAT_00acf14d),
                0,
                a!(DAT_00acf355),
                a!(DAT_00acf351),
                a!(DAT_00acf151),
                l500,
                l100,
                l550,
                &mut local_564,
                s_var1,
            );
        } else {
            let mut t = rd(a!(DAT_00acebf5));
            if t as i32 == param_1 {
                u_var8 = branch12(
                    param_2,
                    param_3,
                    b4,
                    a!(DAT_0052b954).cast::<i8>(),
                    a!(DAT_00acf14d),
                    a!(DAT_00acf359),
                    0x8080,
                    a!(DAT_00acf561),
                    a!(DAT_00acf55d),
                    a!(DAT_00acf35d),
                    l500,
                    l100,
                    l550,
                    &mut local_564,
                    s_var1,
                );
            } else {
                let u6 = FUN_004d23b0(param_1);
                t = rd(a!(DAT_00b69a89));
                let o = u6.wrapping_mul(0x238);
                if u6 < t && u6 < 0x28 {
                    t = rd(off(a!(DAT_00b63fad), o));
                    sprintf(l100, a!(DAT_0052b948).cast::<i8>(), t);
                    let i2 = t as i32;
                    let i3 = (i2 as u32).wrapping_add(1) as i32;
                    wr(off(a!(DAT_00b63fad), o), i3 as u32);
                    let us = i2 as u16;
                    u_var8 = (b4 as u16) | ((us & 0x7f) << 8);
                    if 0x7f < i3 {
                        wr(off(a!(DAT_00b63fad), o), zero);
                    }
                }
                u_var8 |= 0x8000;
                let mut local_55c = l500.add(1).cast::<u8>();
                local_564 = 2;
                local_500[0] = u_var8;
                if local_550[0] != 0 && FUN_0046fad0() == 0 {
                    local_500[1] = s_var1.wrapping_add(-4) as u16;
                    local_55c = l500.add(2).cast::<u8>();
                    local_564 = 4;
                }
                let src = param_2.add(1).cast::<u8>();
                copy(local_55c, src, param_3.wrapping_sub(4));
                local_564 = param_3.wrapping_sub(4).wrapping_add(local_564 as u32) as i32;
                if local_550[0] != 0 {
                    local_550[0] = rd(off(a!(DAT_00b641b1), o));
                    let s11 = off(a!(DAT_00b63fb1), o);
                    let p10 = off(local_55c, param_3.wrapping_sub(4));
                    copy(p10, s11, local_550[0]);
                    let t2 = rd(off(a!(DAT_00b641b1), o));
                    local_564 = (local_564 as u32).wrapping_add(t2) as i32;
                }
                *off(a!(DAT_00b63fb1), o) = b4;
                let p9 = off(a!(DAT_00b63fb1), o).add(1);
                copy(p9, src, param_3.wrapping_sub(4));
                wr(off(a!(DAT_00b641b1), o), param_3.wrapping_sub(3));
            }
        }

        let mut t = rd(a!(DAT_00acf145));
        if t as i32 != param_1 {
            let mut idx = rd(a!(DAT_00b63d7d));
            let o = idx.wrapping_mul(0x210);
            copy(off(a!(DAT_00b5358d), o), param_2.cast::<u8>(), param_3);
            wr(off(a!(DAT_00b5357d), o), param_1 as u32);
            wr(off(a!(DAT_00b53581), o), param_3);
            wr(off(a!(DAT_00b53585), o), zero);
            *off(a!(DAT_00b53589), o) = 0;
            let t2 = rd(a!(DAT_00acebf5));
            if param_1 == 0 {
                *off(a!(DAT_00b5358a), o) = 0;
            } else if t2 as i32 == param_1 {
                *off(a!(DAT_00b5358a), o) = 2;
            } else {
                *off(a!(DAT_00b5358a), o) = 1;
            }
            *off(a!(DAT_00b5358b), o) = ((local_500[0] >> 8) as u8) & 0x7f;
            idx = idx.wrapping_add(1);
            wr(a!(DAT_00b63d7d), idx);
            u_var8 = local_500[0];
            if 0x7f < idx {
                wr(a!(DAT_00b63d7d), zero);
            }
        }

        t = rd(a!(DAT_00acf145));
        let mut t2 = rd(a!(DAT_00acebc5));
        let cond = t as i32 == param_1 || param_1 == 0 || t2 == 0 || {
            t2 = rd(a!(DAT_00acebf5));
            t2 as i32 == param_1
        };
        if cond {
            t2 = rd(a!(DAT_00b53579));
            if t2 < 0x400 {
                let mut idx = rd(a!(DAT_00b53571));
                let o = idx.wrapping_mul(0x210);
                copy(off(a!(DAT_00acf581), o), param_2.cast::<u8>(), param_3);
                t = rd(a!(DAT_00acf145));
                wr(off(a!(DAT_00acf571), o), t);
                wr(off(a!(DAT_00acf575), o), param_3);
                wr(off(a!(DAT_00acf579), o), zero);
                *off(a!(DAT_00acf57d), o) = 0;
                *off(a!(DAT_00acf580), o) = 0;
                t = rd(a!(DAT_00acf145));
                let u6 = FUN_004d23b0(t as i32);
                t = rd(a!(DAT_00b69a89));
                t2 = rd(a!(DAT_00acebf5));
                let o6 = u6.wrapping_mul(0x238);
                if param_1 == 0 {
                    *off(a!(DAT_00acf57e), o) = 0;
                    if u6 < t && u6 < 0x28 {
                        let us = (u_var8 & 0x7f00) >> 8;
                        wr(off(a!(DAT_00b63fa1), o6), us as u32);
                    }
                } else if t2 as i32 == param_1 {
                    *off(a!(DAT_00acf57e), o) = 2;
                    if u6 < t && u6 < 0x28 {
                        wr(off(a!(DAT_00b63fa9), o6), ((u_var8 & 0x7f00) >> 8) as u32);
                    }
                } else {
                    *off(a!(DAT_00acf57e), o) = 1;
                    if u6 < t && u6 < 0x28 {
                        wr(off(a!(DAT_00b63f99), o6), ((u_var8 & 0x7f00) >> 8) as u32);
                    }
                }
                *off(a!(DAT_00acf57f), o) = ((local_500[0] >> 8) as u8) & 0x7f;
                t2 = rd(a!(DAT_00b53579));
                t2 = t2.wrapping_add(1);
                wr(a!(DAT_00b53579), t2);
                idx = idx.wrapping_add(1);
                wr(a!(DAT_00b53571), idx);
                if 0x3ff < idx {
                    wr(a!(DAT_00b53571), zero);
                }
            }
        }

        t2 = rd(a!(DAT_00acebc5));
        if t2 != 0 {
            t = rd(a!(DAT_00acf145));
            if t as i32 != param_1 {
                let vt = rd(t2 as usize as *const u8);
                let f = rd((vt.wrapping_add(0x68)) as usize as *const u8);
                let func = core::mem::transmute::<
                    usize,
                    unsafe extern "stdcall" fn(
                        u32,
                        u32,
                        i32,
                        u32,
                        *mut core::ffi::c_void,
                        i32,
                    ) -> i32,
                >(f as usize);
                local_554 = func(
                    t2,
                    t,
                    param_1,
                    0,
                    l500.cast::<core::ffi::c_void>(),
                    local_564,
                );
            }
            if local_554 != 0 {
                sprintf(l550.cast::<i8>(), a!(DAT_0052b930).cast::<i8>(), local_554);
            }
            return (local_554 == 0) as u8;
        }
        1
    }
}
