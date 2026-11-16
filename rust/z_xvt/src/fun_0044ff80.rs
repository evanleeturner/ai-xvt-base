// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b743: [i8; 0];
    static mut DAT_0071b73c: [u8; 0];
    static mut DAT_00655a94: u16;
    static mut DAT_00655a9c: u16;
    static mut DAT_00655aa2: u16;
    static mut DAT_00655a90: u16;
    static mut DAT_006551cc: u16;
    static mut DAT_00655188: u16;
    static mut DAT_00655186: u16;
    static mut DAT_006551c8: u16;
    static mut DAT_006da3b0: [i32; 0];
    static mut DAT_00709cec: [u8; 0];
    static mut DAT_00525d18: [u8; 0];
    static mut DAT_00525d30: [u8; 0];
    static mut DAT_006daa00: [u8; 0];
    static mut DAT_006daa01: [u8; 0];
    static mut DAT_006daa02: [u8; 0];
    static mut DAT_006daa03: [u8; 0];
    static mut DAT_006daa29: [u8; 0];
    static mut DAT_0070aee9: [u8; 0];
    static mut DAT_0070aee5: [u8; 0];
    static mut DAT_0070aee6: [u8; 0];
    static mut DAT_0070aee7: [u8; 0];
    static mut DAT_0070aee8: [i8; 0];
    static mut DAT_0070aef3: [u8; 0];
    static mut DAT_006d4b36: [u8; 0];
    static mut DAT_006d49a0: i32;
    static mut DAT_006da9ec: i32;
    static mut DAT_006da538: i32;
    static mut DAT_006db594: i32;
    static mut DAT_00525518: i32;
    static mut DAT_00655184: u16;
    static mut DAT_00656d68: [i16; 0];
    static mut DAT_006559b2: i16;
    static mut DAT_00525508: u16;
    static mut DAT_00703c80: *const c_void;
    static mut DAT_0052f0e0: i16;
    static mut DAT_0052f0e4: i16;
    static mut DAT_007092b0: i32;
    static mut DAT_005f0918: i32;
    static mut DAT_005f08e4: i32;
    static mut DAT_005f08dc: i32;
    static mut DAT_005f08e0: i32;
    static mut DAT_005f0908: i32;
    static mut DAT_005f08e8: [i32; 0];
    static mut DAT_005f0910: i8;
    static mut DAT_005f08d8: i8;
    static mut DAT_00709d0a: [i8; 0];
    static mut DAT_00709d0b: [i8; 0];
    static mut DAT_0070ac4e: [i16; 0];
    static mut DAT_006cf480: *mut i8;
    static mut DAT_006cf470: [*mut i8; 0];
    static mut DAT_006d4870: [u8; 0];
    static mut DAT_00525d48: [u8; 0];
    static mut DAT_00656c34: [u32; 0];
    static mut s___s__ld__00525dd8: [i8; 0];
    static mut s___ld____00525dcc: [i8; 0];
    static mut DAT_0071b020: [u16; 0];
    static mut DAT_0070a1e0: [u16; 0];
    static mut DAT_00709e18: [u16; 0];
    static mut DAT_00709d28: [u16; 0];
    static mut DAT_00732e58: i16;
    static mut DAT_0070acc0: i32;
    static mut DAT_006d4a4e: i16;
    static mut DAT_006d9984: *const c_void;
    static mut DAT_0071eae0: *const c_void;
    static mut DAT_00524900: [i32; 0];
    static mut DAT_005248f8: [i32; 0];
    static mut DAT_007092d4: i32;
    static mut DAT_0070ad5d: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049d8b0(a: u16, b: u16, c: u16, d: u16, e: i32, f: i32) -> i8;
    fn FUN_00461f20(a: i32, b: i32, c: i32, d: i32) -> u32;
    fn FUN_0048eda0(a: u32);
    fn FUN_0048ed00(a: i32, b: i32, c: i32, d: i32);
    fn FUN_0048ee00(a: i32);
    fn FUN_0048ee10(a: i32);
    fn FUN_0048ee20(a: i8);
    fn FUN_0048efd0(a: *mut i8);
    fn FUN_0048f040(a: i32);
    fn FUN_0048f000(a: *mut i8);
    fn FUN_0048ece0(a: i32, b: i32);
    fn FUN_0048ed70(a: u32);
    fn FUN_0048f270(a: *mut u8);
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_0048f140(a: *mut u8);
    fn FUN_0048f4f0(a: u32) -> *mut u8;
    fn FUN_004b4060(a: *mut u8) -> u64;
    fn FUN_004513b0(
        a: u32,
        b: u32,
        c: u32,
        d: u16,
        e: u16,
        f: u32,
        g: u16,
        h: u16,
        i: *mut u8,
        j: u32,
        k: i32,
    ) -> u16;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0043fdf0() -> u32;
    fn FUN_0048f510();
}

#[inline(always)]
unsafe fn rd<T>(p: *const T, i: i32) -> T {
    // SAFETY: the caller passes the game's own table and an in-range index.
    unsafe { p.offset(i as isize).read_unaligned() }
}

#[inline(always)]
unsafe fn wr<T>(p: *mut T, i: i32, v: T) {
    // SAFETY: the caller passes the game's own table and an in-range index.
    unsafe { p.offset(i as isize).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rdb<T>(p: *const u8, off: i32) -> T {
    // SAFETY: the caller passes the game's own table and an in-range byte offset.
    unsafe { p.offset(off as isize).cast::<T>().read_unaligned() }
}

#[inline(always)]
unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide, faulting as the original does.
    unsafe {
        core::arch::asm!(
            "cdq",
            "idiv {b}",
            b = in(reg) b,
            inout("eax") a => q,
            out("edx") _,
            options(nomem, nostack)
        );
    }
    q
}

macro_rules! g {
    ($d:ident, $t:ty, $i:expr) => {
        rd((&raw const $d).cast::<$t>(), ($i) as i32)
    };
}
macro_rules! s {
    ($d:ident, $t:ty, $i:expr, $v:expr) => {
        wr((&raw mut $d).cast::<$t>(), ($i) as i32, $v)
    };
}
macro_rules! gb {
    ($d:ident, $t:ty, $o:expr) => {
        rdb::<$t>((&raw const $d).cast::<u8>(), ($o) as i32)
    };
}
macro_rules! a {
    ($d:ident, $t:ty) => {
        (&raw mut $d).cast::<$t>()
    };
}
macro_rules! cur {
    () => {
        DAT_0071ecf4.wrapping_mul(0x5bd)
    };
}
macro_rules! sh4e {
    () => {
        a!(DAT_0070ac4e, u8)
            .offset(DAT_0071ecf4.wrapping_mul(0x10) as isize)
            .cast::<i16>()
    };
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_labels
)]
pub unsafe extern "stdcall" fn FUN_0044ff80() -> u32 {
    // SAFETY: every global, table and callee is the game's own memory and code,
    // accessed at the indices and in the order the original uses.
    unsafe {
        let mut au50: [u32; 20] = [0; 20];
        let s_8c: i16;
        let l_8e: i16;
        let l_8a: i16;
        let mut s_88: i16;

        if g!(DAT_0071b743, i8, cur!()) == 0 {
            s_8c = (DAT_00655a94 as i32 + 2) as i16;
            l_8e = (DAT_00655a94 as i32 - 2 + DAT_00655a9c as i32) as i16;
            l_8a = (DAT_00655aa2 as i32 + 2) as i16;
            s_88 = (DAT_00655aa2 as i32 - 2 + DAT_00655a90 as i32) as i16;
        } else {
            s_8c = (DAT_006551cc as i32 + 2) as i16;
            l_8e = (DAT_006551cc as i32 - 2 + DAT_00655188 as i32) as i16;
            l_8a = (DAT_00655186 as i32 + 2) as i16;
            s_88 = (DAT_00655186 as i32 - 2 + DAT_006551c8 as i32) as i16;
        }
        let mut uvar10: u32 = gb!(DAT_0071b73c, u16, cur!()) as u32;
        let mut local_7c: i32 = 0;
        let mut local_82: i16 = 0;
        let mut u_86: u16;

        loop {
            let i27 = local_82 as i32;
            u_86 = 0;
            s!(DAT_006da3b0, i32, i27, 0);
            loop {
                let u29 = u_86 as u32;
                let cidx = uvar10.wrapping_mul(3).wrapping_add(u29);
                if g!(DAT_00709cec, u8, cidx) != 0 {
                    let k19 = u29.wrapping_add(i27.wrapping_mul(3) as u32).wrapping_mul(2) as i32;
                    if gb!(DAT_00525d18, u16, k19) == g!(DAT_00709cec, u8, cidx) as u16 {
                        let mut k23: i16 = 0;
                        loop {
                            let mut k24: i16 = 0;
                            loop {
                                let i11 = uvar10
                                    .wrapping_mul(7)
                                    .wrapping_add(u29)
                                    .wrapping_mul(0x2a)
                                    .wrapping_add((k24 as i32 * 4) as u32)
                                    as i32;
                                let i2 = i11.wrapping_add(k23 as i32 * 0xb);
                                let u8v = g!(DAT_006daa00, u8, i2) as u16;
                                if u8v != 10 && u8v != 0 {
                                    let c7 = FUN_0049d8b0(
                                        u8v,
                                        g!(DAT_006daa01, u8, (k23 as i32 * 0xb).wrapping_add(i11))
                                            as u16,
                                        g!(DAT_006daa02, u8, i2) as u16,
                                        g!(DAT_006daa03, u8, i2) as u16,
                                        0,
                                        10,
                                    );
                                    if (gb!(DAT_00525d30, u16, k19) & (c7 as u8 as u16)) != 0 {
                                        let i11b = g!(DAT_006da3b0, i32, i27);
                                        s!(DAT_006da3b0, i32, i27, i11b.wrapping_add(1));
                                        if u_86 == 2 {
                                            s!(DAT_006da3b0, i32, i27, i11b.wrapping_add(2));
                                        }
                                    }
                                }
                                k24 = k24.wrapping_add(1);
                                if !(k24 < 2) {
                                    break;
                                }
                            }
                            k23 = k23.wrapping_add(1);
                            if !(k23 < 2) {
                                break;
                            }
                        }
                    }
                }
                let mut l9a: u16 = 0;
                let u8c = gb!(
                    DAT_00525d18,
                    u16,
                    u29.wrapping_add(i27.wrapping_mul(3) as u32).wrapping_mul(2) as i32
                );
                loop {
                    let mut k23: i16 = 0;
                    if 0 < DAT_006d49a0 {
                        loop {
                            let u21 = (l9a as u32)
                                .wrapping_mul(0x4e)
                                .wrapping_add((k23 as i32).wrapping_mul(0x566) as u32);
                            'skip: {
                                if g!(DAT_0070aee9, u8, u21.wrapping_add(uvar10)) == 0 {
                                    break 'skip;
                                }
                                if g!(DAT_0070aee5, u8, u21) as u32 != u29 {
                                    break 'skip;
                                }
                                let ix = (l9a as u32)
                                    .wrapping_add(uvar10.wrapping_mul(8))
                                    .wrapping_add((k23 as i32).wrapping_mul(0x126) as u32);
                                if g!(DAT_006d4b36, u8, ix) as u16 != u8c {
                                    break 'skip;
                                }
                                let k19b: i32;
                                if u_86 == 2 {
                                    if local_82 == 0
                                        && (g!(DAT_0070aee8, i8, u21) as i32 * 0xfa) < 0
                                    {
                                        k19b = g!(DAT_006da3b0, i32, 0).wrapping_add(2);
                                    } else {
                                        if local_82 != 3
                                            || (g!(DAT_0070aee8, i8, u21) as i32 * 0xfa) < 0
                                        {
                                            break 'skip;
                                        }
                                        k19b = g!(DAT_006da3b0, i32, 3).wrapping_add(2);
                                    }
                                } else {
                                    k19b = g!(DAT_006da3b0, i32, i27).wrapping_add(1);
                                }
                                s!(DAT_006da3b0, i32, i27, k19b);
                            }
                            k23 = k23.wrapping_add(1);
                            if !((k23 as i32) < DAT_006d49a0) {
                                break;
                            }
                        }
                    }
                    l9a = l9a.wrapping_add(1);
                    if !(l9a < 8) {
                        break;
                    }
                }
                if g!(DAT_006da3b0, i32, i27) != 0 {
                    s!(
                        DAT_006da3b0,
                        i32,
                        i27,
                        g!(DAT_006da3b0, i32, i27).wrapping_add(1)
                    );
                }
                u_86 = u_86.wrapping_add(1);
                local_7c = local_7c.wrapping_add(g!(DAT_006da3b0, i32, i27));
                if !(u_86 < 3) {
                    break;
                }
            }
            local_82 = local_82.wrapping_add(1);
            if !(local_82 < 4) {
                break;
            }
        }

        FUN_00461f20(
            DAT_006da9ec,
            DAT_006da538,
            DAT_006db594,
            (DAT_00525518 as u32).wrapping_mul(DAT_006da538 as u32) as i32,
        );

        if g!(DAT_00656d68, i16, DAT_00655184) != DAT_006559b2 {
            let u29: u32 = if g!(DAT_0071b743, i8, cur!()) == 0 {
                DAT_00525508 as u32
            } else {
                0x34
            };
            FUN_0048eda0(u29);
            FUN_0048ed00(
                s_8c as i32 - 2,
                l_8a as i32 - 2,
                l_8e as i32 + 2,
                s_88 as i32 + 2,
            );
            core::mem::transmute::<*const c_void, unsafe extern "C" fn()>(DAT_00703c80)();
            if DAT_006559b2 == -2 {
                if DAT_0052f0e0 == 1 {
                    DAT_0052f0e0 = DAT_0052f0e4;
                    let u21 = FUN_0043fdf0();
                    DAT_0052f0e4 = u21 as i16;
                }
                let u21 = FUN_00461f20(0, 0x140, 200, 0);
                return u21 & 0xffff0000;
            }
        }
        FUN_0048ed00(s_8c as i32, l_8a as i32, l_8e as i32, s_88 as i32);
        FUN_0048ee00(1);
        FUN_0048ee10(1);
        FUN_0048ee20(0);
        let u29: u32 = if g!(DAT_0071b743, i8, cur!()) == 0 {
            DAT_00525508 as u32
        } else {
            0x34
        };
        FUN_0048eda0(u29);
        let s23: i16 = (DAT_006d4a4e as i32 + 2) as i16;

        if g!(DAT_00656d68, i16, DAT_00655184) == DAT_006559b2 {
            if DAT_0052f0e0 == 1 {
                if DAT_007092b0 == 0xa2 {
                    if 0 < DAT_005f0918 {
                        DAT_005f08e4 = 1;
                        DAT_005f0918 = DAT_005f0918.wrapping_sub(1);
                    }
                } else if DAT_007092b0 == 0xa3
                    && DAT_005f0918
                        <= DAT_005f08dc
                            .wrapping_add(idiv((l_8a as i32).wrapping_sub(s_88 as i32), s23 as i32))
                {
                    DAT_005f08e4 = 1;
                    DAT_005f0918 = DAT_005f0918.wrapping_add(1);
                }
            }
            if DAT_005f08e0 != local_7c {
                DAT_005f08e4 = 1;
                DAT_005f08e0 = local_7c;
            }
            let mut s24: i16 = 0;
            loop {
                let i27 = s24 as i32;
                if g!(DAT_005f08e8, i32, i27) != g!(DAT_006da3b0, i32, s24) {
                    DAT_005f08e4 = 1;
                    break;
                }
                s24 = s24.wrapping_add(1);
                if !(s24 < 8) {
                    break;
                }
            }
            let u29 = gb!(DAT_0071b73c, u16, cur!()) as u32;
            if g!(DAT_00709d0a, i8, u29.wrapping_mul(3)) != DAT_005f0910
                || g!(DAT_00709d0b, i8, u29.wrapping_mul(3)) != DAT_005f08d8
            {
                DAT_005f08e4 = 1;
            }
        } else {
            DAT_005f0918 = 0;
            DAT_005f0908 = 0;
            DAT_005f08e0 = 0;
            DAT_005f08dc = 0;
            sh4e!().write_unaligned(0x1d8);
            DAT_005f08e4 = 1;
            DAT_005f0910 = -1;
            DAT_005f08d8 = -1;
        }
        if sh4e!().read_unaligned() < 1 {
            DAT_005f08e4 = 1;
            sh4e!().write_unaligned(0x1d8);
        }
        let mut u94: u32 = (DAT_005f0918 as u32)
            .wrapping_add(0xffffffff)
            .wrapping_add(idiv((s_88 as i32).wrapping_sub(l_8a as i32), s23 as i32) as u32)
            .wrapping_mul(0x10000);

        if DAT_005f08e4 != 0 {
            FUN_0048ed00(
                s_8c as i32 - 2,
                l_8a as i32 - 2,
                l_8e as i32 + 2,
                s_88 as i32 + 2,
            );
            core::mem::transmute::<*const c_void, unsafe extern "C" fn()>(DAT_00703c80)();
            let mut i19: i32 = 0;
            FUN_0048ed00(s_8c as i32, l_8a as i32, l_8e as i32, s_88 as i32);
            let mut idx24: i16 = 4;
            FUN_0048efd0(DAT_006cf480);
            let u29 = gb!(DAT_0071b73c, u16, cur!()) as u32;
            let c7 = g!(DAT_00709d0a, i8, u29.wrapping_mul(3));
            let i27x = u29.wrapping_mul(3) as i32;
            if c7 == 0 {
                let c = g!(DAT_00709d0b, i8, i27x);
                if c == 0 || c == 2 {
                    idx24 = 7;
                    au50[0] = 0x4e;
                } else if c == 1 {
                    idx24 = 6;
                    au50[0] = 0x4a;
                }
            } else if c7 == 1 {
                let c = g!(DAT_00709d0b, i8, i27x);
                if c == 0 || c == 2 {
                    idx24 = 5;
                    au50[0] = 0x52;
                } else if c == 1 {
                    idx24 = 8;
                    au50[0] = 0x46;
                }
            } else if c7 == 2 {
                idx24 = 6;
                au50[0] = 0x4a;
            }
            FUN_0048f040(0x20);
            FUN_0048f000(g!(DAT_006cf470, *mut i8, idx24));
            FUN_0048ece0(s_8c as i32, l_8a as i32);
            FUN_0048ed70(au50[0]);
            FUN_0048f270(a!(DAT_006d4870, u8));
            local_82 = 0;
            let mut s_84: i16 = l_8a.wrapping_add(s23);
            loop {
                let i27 = local_82 as i32;
                FUN_0048ed70(g!(DAT_00525d48, u8, i27) as u32);
                u_86 = 0;
                let mut b6: i32 = 1;
                loop {
                    if g!(DAT_006da3b0, i32, i27) != 0 {
                        let u29 = u_86 as u32;
                        let i11 = u29.wrapping_add(uvar10.wrapping_mul(3)) as i32;
                        'lab: {
                            if g!(DAT_00709cec, u8, i11) != 0 {
                                let pv17 =
                                    (i27.wrapping_mul(3).wrapping_add(u29 as i32)).wrapping_mul(2);
                                if gb!(DAT_00525d18, u16, pv17) == g!(DAT_00709cec, u8, i11) as u16
                                {
                                    let mut s96: i16 = 0;
                                    let i16v = uvar10.wrapping_mul(7).wrapping_add(u29) as i32;
                                    let mut l9a: u16 = 0;
                                    let i2 = i16v.wrapping_mul(0x2a);
                                    loop {
                                        let mut s98: i16 = 0;
                                        loop {
                                            let mut i12 = (s96 as i32 * 0xb)
                                                .wrapping_add(i2)
                                                .wrapping_add(s98 as i32 * 4);
                                            let b4 = g!(DAT_006daa01, u8, i12);
                                            let u9 = b4 as u16;
                                            let b5 = g!(DAT_006daa02, u8, i12);
                                            let u8v = g!(DAT_006daa03, u8, i12) as u16;
                                            u94 = (u94 & 0xffff0000) | u8v as u32;
                                            let u20 = g!(DAT_006daa00, u8, i12) as u16;
                                            if u20 != 10 && u20 != 0 {
                                                let c7 =
                                                    FUN_0049d8b0(u20, u9, b5 as u16, u8v, 0, 10);
                                                if (gb!(DAT_00525d30, u16, pv17)
                                                    & (c7 as u8 as u16))
                                                    != 0
                                                {
                                                    if u_86 == 2 {
                                                        i12 = (g!(DAT_006daa29, u8, i2) as i8
                                                            as i32)
                                                            * 0xfa;
                                                        if (local_82 == 0 && -1 < i12)
                                                            || (local_82 == 3 && i12 < 0)
                                                        {
                                                            break 'lab;
                                                        }
                                                        if b6 != 0 {
                                                            if DAT_005f0918 <= i19
                                                                && i19 < (u94 >> 16) as i16 as i32
                                                            {
                                                                FUN_0048ece0(
                                                                    s_8c as i32,
                                                                    s_84 as i32,
                                                                );
                                                                FUN_0048f270(g!(
                                                                    DAT_006cf470,
                                                                    *mut i8,
                                                                    i27
                                                                )
                                                                    as *mut u8);
                                                                s_84 = s_84.wrapping_add(s23);
                                                            }
                                                            b6 = 0;
                                                            i19 = i19.wrapping_add(1);
                                                        }
                                                        FUN_0048ece0(s_8c as i32, s_84 as i32);
                                                        _sprintf(
                                                            au50.as_mut_ptr().cast::<i8>(),
                                                            (&raw const s___s__ld__00525dd8)
                                                                .cast::<i8>(),
                                                            g!(DAT_00656c34, u32, (i12 < 0) as u32),
                                                            i12,
                                                        );
                                                        let mut u13: u32 = 0x52;
                                                        if i12 < 0 {
                                                            u13 = 0x4a;
                                                        }
                                                        FUN_0048ed70(u13);
                                                        FUN_0048f140(
                                                            au50.as_mut_ptr().cast::<u8>(),
                                                        );
                                                        FUN_0048ed70(
                                                            g!(DAT_00525d48, u8, i27) as u32
                                                        );
                                                    }
                                                    if b6 != 0 {
                                                        if DAT_005f0918 <= i19
                                                            && i19 < (u94 >> 16) as i16 as i32
                                                        {
                                                            FUN_0048ece0(s_8c as i32, s_84 as i32);
                                                            FUN_0048f270(g!(
                                                                DAT_006cf470,
                                                                *mut i8,
                                                                i27
                                                            )
                                                                as *mut u8);
                                                            s_84 = s_84.wrapping_add(s23);
                                                        }
                                                        b6 = 0;
                                                        i19 = i19.wrapping_add(1);
                                                    }
                                                    let mut la0: u16 = 0;
                                                    if DAT_005f0918 <= i19
                                                        && i19 <= (u94 >> 16) as i16 as i32
                                                    {
                                                        let mut i12b = (l9a as i16) as i32;
                                                        let mut pb25: *mut u8 =
                                                            core::ptr::null_mut();
                                                        let tix = i12b
                                                            .wrapping_add(i16v.wrapping_mul(4))
                                                            .wrapping_mul(3)
                                                            .wrapping_add(
                                                                (gb!(DAT_00525d18, u16, pv17) & 3)
                                                                    as i32,
                                                            );
                                                        if g!(DAT_0071b020, u16, tix) != 0 {
                                                            pb25 = FUN_0048f4f0(g!(
                                                                DAT_0071b020,
                                                                u16,
                                                                tix
                                                            )
                                                                as u32);
                                                        }
                                                        FUN_0048ece0(s_8c as i32, s_84 as i32);
                                                        if local_82 == 2 && u_86 == 1 {
                                                            la0 = 5;
                                                        } else {
                                                            la0 = gb!(DAT_00525d18, u16, pv17);
                                                        }
                                                        if u_86 == 2 {
                                                            let u28 = FUN_004b4060(
                                                                au50.as_mut_ptr().cast::<u8>(),
                                                            );
                                                            DAT_00732e58 = DAT_00732e58
                                                                .wrapping_add(u28 as i16);
                                                        }
                                                        if pb25.is_null() {
                                                            i12b = i12b
                                                                .wrapping_add(i11.wrapping_mul(4));
                                                            let u13: u32;
                                                            if g!(DAT_00709e18, u16, i12b) < 2
                                                                || u20 == 6
                                                                || u20 == 7
                                                                || u20 == 0x1e
                                                                || u20 == 0x1f
                                                                || u20 == 0x20
                                                                || u20 == 0x21
                                                            {
                                                                u13 = 0xffffffff;
                                                            } else {
                                                                u13 = (g!(DAT_00709d28, u16, i12b)
                                                                    as u32
                                                                    * 100)
                                                                    / g!(DAT_00709e18, u16, i12b)
                                                                        as u32;
                                                            }
                                                            let u14 =
                                                                ((b4 as u32) << 16) | la0 as u32;
                                                            la0 = FUN_004513b0(
                                                                ((b5 as u32) << 16) | u9 as u32,
                                                                u14,
                                                                ((l9a as u32) << 16) | b5 as u32,
                                                                u20,
                                                                u9,
                                                                u14,
                                                                u8v,
                                                                0,
                                                                core::ptr::null_mut(),
                                                                u13,
                                                                i27,
                                                            );
                                                            s_84 = s_84.wrapping_add(la0 as i16);
                                                            if la0
                                                                <= (DAT_006d4a4e as i32 + 2) as u16
                                                            {
                                                                la0 = 0;
                                                            }
                                                        } else {
                                                            FUN_0048f140(pb25);
                                                            i12b = i12b
                                                                .wrapping_add(i11.wrapping_mul(4));
                                                            if g!(DAT_00709e18, u16, i12b) < 2
                                                                || u20 == 6
                                                                || u20 == 7
                                                                || u20 == 0x1e
                                                                || u20 == 0x1f
                                                                || u20 == 0x20
                                                                || u20 == 0x21
                                                            {
                                                                core::mem::transmute::<
                                                                    *const c_void,
                                                                    unsafe extern "C" fn(i32),
                                                                >(
                                                                    DAT_006d9984
                                                                )(
                                                                    10
                                                                );
                                                                let mut skip_zero = pb25.is_null();
                                                                if !skip_zero {
                                                                    let u28 = FUN_004b4060(pb25);
                                                                    skip_zero = ((u28 as u32
                                                                        & 0xffff)
                                                                        as i32)
                                                                        .wrapping_add(
                                                                            DAT_00732e58 as i32,
                                                                        )
                                                                        <= DAT_0070acc0
                                                                            .wrapping_sub(0xb);
                                                                }
                                                                if skip_zero {
                                                                    la0 = 0;
                                                                } else {
                                                                    la0 = (DAT_006d4a4e as i32 + 1)
                                                                        as u16;
                                                                }
                                                                s_84 = s_84
                                                                    .wrapping_add(DAT_006d4a4e)
                                                                    .wrapping_add(la0 as i16)
                                                                    .wrapping_add(2);
                                                            } else {
                                                                _sprintf(
                                                                    au50.as_mut_ptr().cast::<i8>(),
                                                                    (&raw const s___ld____00525dcc)
                                                                        .cast::<i8>(),
                                                                    (g!(DAT_00709d28, u16, i12b)
                                                                        as u32
                                                                        * 100)
                                                                        / g!(
                                                                            DAT_00709e18,
                                                                            u16,
                                                                            i12b
                                                                        )
                                                                            as u32,
                                                                );
                                                                let u13: u32 = if local_82 == 0
                                                                    || local_82 == 2
                                                                {
                                                                    0x4a
                                                                } else if local_82 == 3
                                                                    || local_82 == 1
                                                                {
                                                                    0x52
                                                                } else {
                                                                    0x43
                                                                };
                                                                FUN_0048ed70(u13);
                                                                FUN_0048efd0(pb25.cast::<i8>());
                                                                FUN_0048f000(
                                                                    au50.as_mut_ptr().cast::<i8>(),
                                                                );
                                                                FUN_0048f140(
                                                                    au50.as_mut_ptr().cast::<u8>(),
                                                                );
                                                                FUN_0048ed70(g!(
                                                                    DAT_00525d48,
                                                                    u8,
                                                                    i27
                                                                )
                                                                    as u32);
                                                                core::mem::transmute::<
                                                                    *const c_void,
                                                                    unsafe extern "C" fn(i32),
                                                                >(
                                                                    DAT_006d9984
                                                                )(
                                                                    10
                                                                );
                                                                let u28 = FUN_004b4060(a!(
                                                                    DAT_006d4870,
                                                                    u8
                                                                ));
                                                                if DAT_0070acc0.wrapping_sub(0xb)
                                                                    < ((u28 as u32 & 0xffff) as i32)
                                                                        .wrapping_add(
                                                                            DAT_00732e58 as i32,
                                                                        )
                                                                {
                                                                    la0 = (DAT_006d4a4e as i32 + 1)
                                                                        as u16;
                                                                } else {
                                                                    la0 = 0;
                                                                }
                                                                s_84 = s_84
                                                                    .wrapping_add(DAT_006d4a4e)
                                                                    .wrapping_add(la0 as i16)
                                                                    .wrapping_add(2);
                                                            }
                                                        }
                                                        if !pb25.is_null() {
                                                            FUN_0048f510();
                                                        }
                                                    }
                                                    if la0 == 0 {
                                                        i19 = i19.wrapping_add(1);
                                                    } else {
                                                        i19 = i19.wrapping_add(2);
                                                    }
                                                }
                                            }
                                            s98 = s98.wrapping_add(1);
                                            l9a = l9a.wrapping_add(1);
                                            if !(s98 < 2) {
                                                break;
                                            }
                                        }
                                        s96 = s96.wrapping_add(1);
                                        if !(s96 < 2) {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        let mut l9a: u16 = 0;
                        let pu3 = a!(DAT_00525d18, u8)
                            .offset(
                                (i27.wrapping_mul(3).wrapping_add(u29 as i32)).wrapping_mul(2)
                                    as isize,
                            )
                            .cast::<u16>();
                        let u8c = pu3.read_unaligned();
                        loop {
                            let mut s9c: i16 = 0;
                            if 0 < DAT_006d49a0 {
                                let u13 = l9a as u32;
                                loop {
                                    let u26 = s9c as i32 as u32;
                                    let i11b = u13
                                        .wrapping_mul(0x4e)
                                        .wrapping_add(u26.wrapping_mul(0x566))
                                        as i32;
                                    'skip2: {
                                        if g!(DAT_0070aee9, u8, (i11b as u32).wrapping_add(uvar10))
                                            == 0
                                        {
                                            break 'skip2;
                                        }
                                        if (g!(DAT_00524900, i32, DAT_007092d4)
                                            & g!(
                                                DAT_005248f8,
                                                i32,
                                                g!(DAT_0070ad5d, u8, u26.wrapping_mul(0x566))
                                                    as u32
                                            ))
                                            == 0
                                        {
                                            break 'skip2;
                                        }
                                        if g!(DAT_0070aee5, u8, i11b) as u32 != u29 {
                                            break 'skip2;
                                        }
                                        let ix = u13
                                            .wrapping_add(uvar10.wrapping_mul(8))
                                            .wrapping_add(u26.wrapping_mul(0x126));
                                        if g!(DAT_006d4b36, u8, ix) as u16 != u8c {
                                            break 'skip2;
                                        }
                                        let mut u9 = g!(DAT_0070aee6, u8, i11b) as u16;
                                        if u9 == 10 || u9 == 0 {
                                            break 'skip2;
                                        }
                                        if u_86 == 2 {
                                            let v = g!(DAT_0070aee8, i8, i11b) as i32 * 0xfa;
                                            if !((local_82 != 0 || v < 0)
                                                && (local_82 != 3 || -1 < v))
                                            {
                                                break 'skip2;
                                            }
                                        }
                                        if b6 != 0 {
                                            if DAT_005f0918 <= i19
                                                && i19 < (u94 >> 16) as i16 as i32
                                            {
                                                FUN_0048ece0(s_8c as i32, s_84 as i32);
                                                FUN_0048f270(
                                                    g!(DAT_006cf470, *mut i8, i27) as *mut u8
                                                );
                                                s_84 = s_84.wrapping_add(s23);
                                            }
                                            b6 = 0;
                                            i19 = i19.wrapping_add(1);
                                        }
                                        let mut la2: u16 = 0;
                                        let b4 = g!(DAT_0070aee7, u8, i11b);
                                        u94 = (u94 & 0xffff0000) | b4 as u32;
                                        let b5 = g!(DAT_0070aef3, u8, i11b);
                                        if DAT_005f0918 <= i19 && i19 < (u94 >> 16) as i16 as i32 {
                                            let mut pb25: *mut u8 = core::ptr::null_mut();
                                            let tix = u13
                                                .wrapping_add(u26.wrapping_mul(8))
                                                .wrapping_mul(3)
                                                .wrapping_add((pu3.read_unaligned() & 3) as u32);
                                            if g!(DAT_0070a1e0, u16, tix) != 0 {
                                                pb25 =
                                                    FUN_0048f4f0(g!(DAT_0070a1e0, u16, tix) as u32);
                                            }
                                            FUN_0048ece0(s_8c as i32, s_84 as i32);
                                            let u26b: u32 = if local_82 == 2 && u_86 == 1 {
                                                5
                                            } else {
                                                pu3.read_unaligned() as u32
                                            };
                                            if u_86 == 2 {
                                                let u21 = (g!(DAT_0070aee8, i8, i11b) as i32 * 0xfa)
                                                    as u32;
                                                _sprintf(
                                                    au50.as_mut_ptr().cast::<i8>(),
                                                    (&raw const s___s__ld__00525dd8).cast::<i8>(),
                                                    g!(
                                                        DAT_00656c34,
                                                        u32,
                                                        ((u21 as i32) < 0) as u32
                                                    ),
                                                    u21,
                                                );
                                                let mut u14: u32 = 0x52;
                                                if (u21 as i32) < 0 {
                                                    u14 = 0x4a;
                                                }
                                                FUN_0048ed70(u14);
                                                FUN_0048f140(au50.as_mut_ptr().cast::<u8>());
                                                FUN_0048ed70(g!(DAT_00525d48, u8, i27) as u32);
                                            }
                                            if pb25.is_null() {
                                                u9 = FUN_004513b0(
                                                    ((s9c as u16 as u32) << 16) | u9 as u32,
                                                    u94,
                                                    ((l9a as u32) << 16) | s9c as u16 as u32,
                                                    u9,
                                                    1,
                                                    u26b,
                                                    b4 as u16,
                                                    b5 as u16,
                                                    core::ptr::null_mut(),
                                                    0xffffffff,
                                                    i27,
                                                );
                                                s_84 = s_84.wrapping_add(u9 as i16);
                                                la2 = u9;
                                                if u9 <= (DAT_006d4a4e as i32 + 2) as u16 {
                                                    la2 = 0;
                                                }
                                            } else {
                                                FUN_0048f140(pb25);
                                                core::mem::transmute::<
                                                    *const c_void,
                                                    unsafe extern "C" fn(i32),
                                                >(
                                                    DAT_006d9984
                                                )(
                                                    10
                                                );
                                                let mut skip_zero = pb25.is_null();
                                                if !skip_zero {
                                                    let u28 = FUN_004b4060(pb25);
                                                    skip_zero = ((u28 as u32 & 0xffff) as i32)
                                                        .wrapping_add(DAT_00732e58 as i32)
                                                        <= DAT_0070acc0.wrapping_sub(0xb);
                                                }
                                                if skip_zero {
                                                    la2 = 0;
                                                } else {
                                                    la2 = ((DAT_006d4a4e as i32 & 0xff) + 1) as u16;
                                                }
                                                s_84 = s_84
                                                    .wrapping_add(DAT_006d4a4e)
                                                    .wrapping_add(la2 as i16)
                                                    .wrapping_add(2);
                                            }
                                            if !pb25.is_null() {
                                                FUN_0048f510();
                                            }
                                        }
                                        if la2 as i16 == 0 {
                                            i19 = i19.wrapping_add(1);
                                        } else {
                                            i19 = i19.wrapping_add(2);
                                        }
                                    }
                                    s9c = s9c.wrapping_add(1);
                                    if !((s9c as i32) < DAT_006d49a0) {
                                        break;
                                    }
                                }
                            }
                            l9a = l9a.wrapping_add(1);
                            if !(l9a < 8) {
                                break;
                            }
                        }
                    }
                    u_86 = u_86.wrapping_add(1);
                    if !(u_86 < 3) {
                        break;
                    }
                }
                local_82 = local_82.wrapping_add(1);
                if !(local_82 < 4) {
                    break;
                }
            }
            DAT_005f08dc = i19;
            let mut s24: i16 = 0;
            loop {
                let s18 = s24.wrapping_add(1);
                s!(DAT_005f08e8, i32, s24, g!(DAT_006da3b0, i32, s24));
                s24 = s18;
                if !(s18 < 8) {
                    break;
                }
            }
            let u21 = gb!(DAT_0071b73c, u16, cur!()) as u32;
            DAT_005f0910 = g!(DAT_00709d0a, i8, u21.wrapping_mul(3));
            DAT_005f08d8 = g!(DAT_00709d0b, i8, u21.wrapping_mul(3));
        }

        if g!(DAT_0071b743, i8, cur!()) == 0 {
            s_88 = (((DAT_005f08dc as i16) as i32 + 1) * s23 as i32 + DAT_00655aa2 as i32) as i16;
            if ((DAT_00655a90 as u32)
                .wrapping_add(DAT_00655aa2 as u32)
                .wrapping_sub(2) as i32)
                < s_88 as i32
            {
                s_88 = ((DAT_00655aa2 as i32 - 2) + DAT_00655a90 as i32) as i16;
            }
            if DAT_0052f0e0 == -1 {
                DAT_0052f0e0 = 1;
            }
        }
        'end: {
            let u21: u32;
            if DAT_0052f0e0 == 1 {
                u21 = 0x46;
            } else {
                if DAT_0052f0e4 != 1 {
                    break 'end;
                }
                if g!(DAT_0071b743, i8, cur!()) == 0 {
                    u21 = DAT_00525508 as u32;
                } else {
                    u21 = 0x34;
                }
            }
            FUN_0048eda0(u21);
            FUN_0048ed00(
                s_8c as i32 - 2,
                l_8a as i32 - 2,
                l_8e as i32 + 2,
                s_88 as i32 + 2,
            );
            core::mem::transmute::<*const c_void, unsafe extern "C" fn(i32, i32, i32, i32, i32)>(
                DAT_0071eae0,
            )(
                s_8c as i32 - 2,
                l_8a as i32 - 2,
                l_8e as i32 + 2,
                s_88 as i32 + 2,
                1,
            );
        }
        DAT_005f08e4 = 0;
        FUN_0048ee00(0);
        let u21 = FUN_00461f20(0, 0x140, 200, 0);
        u21 & 0xffff0000
    }
}
