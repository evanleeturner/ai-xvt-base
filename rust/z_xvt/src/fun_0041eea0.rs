// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070ad8b: u8;
    static mut DAT_0070ad8a: u8;
    static mut DAT_0071f2d8: u8;
    static mut DAT_0071f2dc: u8;
    static mut DAT_0071f2dd: u8;
    static mut DAT_007092c1: u8;
    static mut DAT_0073b374: u16;
    static mut DAT_007394ca: u8;
    static mut DAT_007394cc: u8;
    static mut DAT_006d4a66: u8;
    static mut DAT_0070b156: u8;
    static mut DAT_0070b182: u8;
    static mut DAT_0070b1ae: u8;
    static mut DAT_0070b15e: u8;
    static mut DAT_0070b18a: u8;
    static mut DAT_0070b1b6: u8;
    static mut DAT_0070b1e2: u8;
    static mut DAT_0070b252: u8;
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_0073abc4: i32;
    static mut DAT_0073abc8: i32;
    static mut DAT_0073abcc: i32;
    static mut DAT_006d4a02: u8;
    static mut DAT_007394c8: u16;
    static mut DAT_00732e40: u16;
    static mut DAT_006da9f0: u16;
    static mut DAT_007394ce: u16;
    static mut DAT_006d4a50: i32;
    static mut DAT_006ffbec: i32;
    static mut DAT_0072f4e8: i32;
    static mut DAT_0073b372: u8;
    static mut DAT_0070ad4d: u8;
    static mut DAT_007394c5: u8;
    static mut DAT_0070ad4e: u8;
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00732e54: i32;
    static mut DAT_0052b14b: u8;
    static mut DAT_0052b14d: u8;
    static mut DAT_0052b14f: u8;
    static mut DAT_0052b151: u8;
    static mut DAT_0052b153: u8;
    static mut DAT_0052b155: u8;
    static mut DAT_0070ab18: i32;
    static mut DAT_0070ab14: i32;
    static mut DAT_0070ac20: i32;
    static mut DAT_0070ad43: u8;
    static mut DAT_007394c4: u8;
    static mut DAT_0070ad47: u8;
    static mut DAT_007394c3: u8;
    static mut DAT_0070ad48: u8;
    static mut DAT_007394cb: u8;
    static mut DAT_0070ad44: u8;
    static mut DAT_0073abc1: u8;
    static mut DAT_0070b215: u8;
    static mut DAT_0073abc0: u8;
    static mut DAT_0073b370: u8;
    static mut DAT_0052686b: u8;
    static mut DAT_005247e8: u8;
    static mut DAT_0070ad42: u8;
    static mut DAT_007394c7: u8;
    static mut DAT_0070ad49: u8;
    static mut DAT_007394d0: u16;
    static mut DAT_007394c2: u8;
    static mut DAT_006d4a6c: u8;
    static mut DAT_006d4a6a: u8;
    static mut DAT_0070ad40: u8;
    static mut DAT_006d4aa3: u8;
    static mut DAT_007394d2: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042a230(a: i32, b: i16, c: i32, d: i16);
    fn FUN_00418cd0(a: i32, b: i32, c: i32);
    fn FUN_00478470() -> u16;
    fn FUN_00418b00(a: u16, b: u16, c: u16);
    fn FUN_004a7aa0(a: u32) -> u32;
    fn FUN_004a31b0(a: u32, b: u32) -> u64;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0041f670() -> u64;
}

macro_rules! ab {
    ($n:ident) => {
        (&raw mut $n).cast::<u8>()
    };
}

unsafe fn rd<T>(base: *mut u8, off: u32) -> T {
    // SAFETY: the caller passes an address inside the game's own memory.
    unsafe { base.byte_add(off as usize).cast::<T>().read_unaligned() }
}

unsafe fn wr<T>(base: *mut u8, off: u32, v: T) {
    // SAFETY: the caller passes an address inside the game's own memory.
    unsafe { base.byte_add(off as usize).cast::<T>().write_unaligned(v) }
}

fn addr(a: u32) -> *mut u8 {
    a as usize as *mut u8
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041eea0(param_1: u16) -> u64 {
    // SAFETY: all globals and callees are the game's own; every pointer is
    // formed from the game's tables exactly as the original does.
    unsafe {
        let c_var4 = DAT_0071f2dd;
        let c_var3 = DAT_0071f2dc;
        let c_var2: u8 = rd(ab!(DAT_0071f2d8), 3);
        let mut found = false;
        let mut u5: u32 = DAT_0073b374 as u32;
        let mut u8v: u32;
        let mut u6: u16;
        let mut i10: i32;
        let mut i9: i32;
        let mut u11: u16;
        DAT_007394ca = 0;
        DAT_007394cc = 0;

        let cond = rd::<u8>(ab!(DAT_0070ad8b), u5.wrapping_mul(0x566)) == 0
            || ((rd::<u8>(ab!(DAT_0071f2d8), 3) == 0 && DAT_0071f2dc == 0) && DAT_0071f2dd == 0)
            || DAT_007092c1 != 0
            || param_1 != 0xffff;

        if cond {
            i10 = (rd::<u16>(ab!(DAT_006d4a66), u5.wrapping_mul(0x126)) as u32)
                .wrapping_sub(0x8000)
                .wrapping_add(u5.wrapping_mul(0x2b3)) as i32;
            DAT_00732a2c = ((rd::<i16>(ab!(DAT_0070b156), (i10 as u32).wrapping_mul(2)) as i32
                as u32)
                << 8) as i32;
            i10 = (i10 as u32).wrapping_mul(2) as i32;
            DAT_00732a28 = (rd::<i16>(ab!(DAT_0070b182), i10 as u32) as i32 as u32)
                .wrapping_mul(0xffffff00) as i32;
            DAT_00732a24 = ((rd::<i16>(ab!(DAT_0070b1ae), i10 as u32) as i32 as u32) << 8) as i32;
            DAT_0073abc4 = DAT_00732a2c;
            DAT_0073abc8 = DAT_00732a28;
            DAT_0073abcc = DAT_00732a24;
            let mut v: i32 = rd(ab!(DAT_0070b252), u5.wrapping_mul(0x566));
            if (((rd::<u8>(ab!(DAT_0071f2d8), 3) != 0 || DAT_0071f2dc != 0) || DAT_0071f2dd != 0)
                && DAT_006d4a02 == 2)
                && v != -1
            {
                u11 = FUN_00478470();
                u8v = FUN_00478470() as u32;
                u5 = (u11 & 0x7fff) as u32;
                if (u8v & 1) == 0 {
                    u5 = 0u32.wrapping_sub(u5);
                }
                DAT_0073abc4 = (DAT_0073abc4 as u32).wrapping_add(u5) as i32;
                u11 = FUN_00478470();
                u8v = FUN_00478470() as u32;
                u5 = (u11 & 0x7fff) as u32;
                if (u8v & 1) == 0 {
                    u5 = 0u32.wrapping_sub(u5);
                }
                DAT_0073abc8 = (DAT_0073abc8 as u32).wrapping_add(u5) as i32;
            }
            if rd::<i16>(ab!(DAT_0070b1e2), (DAT_0073b374 as u32).wrapping_mul(0x566)) == 0 {
                DAT_007394c8 = 0;
                DAT_00732e40 = 0x4000;
            } else {
                let o = (DAT_0073b374 as u32).wrapping_mul(0x566);
                DAT_00732a2c =
                    (rd::<i16>(ab!(DAT_0070b15e), o) as i32 as u32).wrapping_mul(0x100) as i32;
                DAT_00732a28 =
                    (rd::<i16>(ab!(DAT_0070b18a), o) as i32 as u32).wrapping_mul(0xffffff00) as i32;
                DAT_00732a24 =
                    (rd::<i16>(ab!(DAT_0070b1b6), o) as i32 as u32).wrapping_mul(0x100) as i32;
                FUN_00418cd0(
                    (DAT_00732a2c as u32).wrapping_sub(DAT_0073abc4 as u32) as i32,
                    (DAT_00732a28 as u32).wrapping_sub(DAT_0073abc8 as u32) as i32,
                    (DAT_00732a24 as u32).wrapping_sub(DAT_0073abcc as u32) as i32,
                );
                DAT_007394c8 = DAT_006da9f0;
            }
            DAT_006da9f0 = DAT_007394c8;
            DAT_007394ce = DAT_00732e40;
            v = rd(ab!(DAT_0070b252), (DAT_0073b374 as u32).wrapping_mul(0x566));
            if ((DAT_007092c1 == 0
                && rd::<u8>(ab!(DAT_0070ad8b), (DAT_0073b374 as u32).wrapping_mul(0x566)) == 0)
                && ((c_var2 != 0 || c_var3 != 0) || c_var4 != 0))
                && (param_1 == 0xffff && v == -1)
            {
                DAT_006da9f0 = (DAT_007394c8 as u32).wrapping_add(0xffff8000) as u16;
                u11 = 0x8000u32.wrapping_sub(DAT_00732e40 as u32) as u16;
                DAT_00732e40 = u11;
                FUN_00418b00(0xffff, DAT_006da9f0, u11);
                DAT_006d4a50 = (DAT_006d4a50 as u32).wrapping_mul(8) as i32;
                DAT_006ffbec = (DAT_006ffbec as u32).wrapping_mul(8) as i32;
                DAT_0072f4e8 = (DAT_0072f4e8 as u32).wrapping_mul(8) as i32;
                DAT_0073abc4 = (DAT_0073abc4 as u32).wrapping_add(DAT_006d4a50 as u32) as i32;
                DAT_0073abc8 = (DAT_0073abc8 as u32).wrapping_add(DAT_006ffbec as u32) as i32;
                DAT_0073abcc = (DAT_0073abcc as u32).wrapping_add(DAT_0072f4e8 as u32) as i32;
                DAT_007394ca = 1;
            }
            DAT_0073b372 = rd(ab!(DAT_0070ad4d), (DAT_0073b374 as u32).wrapping_mul(0x566));
            DAT_007394c5 = rd(ab!(DAT_0070ad4e), (DAT_0073b374 as u32).wrapping_mul(0x566));
        } else {
            let mut local_2: u16 = 0;
            u8v = DAT_00732a3c as u32;
            if (u8v as i32) < DAT_006d4980 {
                local_2 = 0;
                u8v = 0;
                u11 = DAT_00732a3c;
                loop {
                    let p = (u11 as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(DAT_006d4988 as u32);
                    let pp = addr(p);
                    if rd::<u8>(pp, 3) != 0 {
                        let t32: u32 = rd(pp, 0x1f);
                        DAT_00732e54 = rd(addr(t32), 0xa7);
                        if rd::<u8>(pp, 0x16) == rd::<u8>(ab!(DAT_0070ad8a), u5.wrapping_mul(0x566))
                            && rd::<i8>(addr(DAT_00732e54 as u32), 5) == -1
                        {
                            found = true;
                            local_2 = u11;
                            break;
                        }
                    }
                    u11 = u11.wrapping_add(1);
                    if !((u11 as i32) < DAT_006d4980) {
                        break;
                    }
                }
            }
            if !found {
                return (((u8v as u64) << 32) | (u5.wrapping_mul(0x2b3) as u64))
                    & 0xffffffffffff0000u64;
            }
            i10 = (local_2 as u32).wrapping_mul(0x23) as i32;
            let mut i7 = (DAT_006d4988 as u32).wrapping_add(i10 as u32) as i32;
            let t32: u32 = rd(addr(i7 as u32), 0x1f);
            DAT_00732e54 = rd(addr(t32), 0xa7);
            i9 = (rd::<u8>(addr(DAT_00732e54 as u32), 4) as u32).wrapping_mul(0xef) as i32;
            let s_a: i16 = rd(ab!(DAT_0052b14b), i9 as u32);
            let u_a: u16 = rd(ab!(DAT_0052b14d), i9 as u32);
            let s_b: i16 = rd(ab!(DAT_0052b14f), i9 as u32);
            FUN_0042a230(i7, s_a, u_a as u32 as i32, s_b);
            i7 = (DAT_006d4988 as u32).wrapping_add(i10 as u32) as i32;
            let mut p = addr(i7 as u32);
            let mut v: i32 = rd(p, 4);
            DAT_0073abc4 = (v as u32).wrapping_add(DAT_0070ab18 as u32) as i32;
            v = rd(p, 8);
            DAT_0073abc8 = (v as u32).wrapping_add(DAT_0070ab14 as u32) as i32;
            v = rd(p, 0xc);
            DAT_0073abcc = (v as u32).wrapping_add(DAT_0070ac20 as u32) as i32;
            let s_a: i16 = rd(ab!(DAT_0052b151), i9 as u32);
            let s_b: i16 = rd(ab!(DAT_0052b153), i9 as u32);
            let s_c: i16 = rd(ab!(DAT_0052b155), i9 as u32);
            FUN_0042a230(i7, s_a, s_b as i32, s_c);
            i10 = (DAT_006d4988 as u32).wrapping_add(i10 as u32) as i32;
            p = addr(i10 as u32);
            v = rd(p, 4);
            DAT_00732a2c = (v as u32).wrapping_add(DAT_0070ab18 as u32) as i32;
            v = rd(p, 8);
            DAT_00732a28 = (v as u32).wrapping_add(DAT_0070ab14 as u32) as i32;
            v = rd(p, 0xc);
            DAT_00732a24 = (v as u32).wrapping_add(DAT_0070ac20 as u32) as i32;
            FUN_00418cd0(
                (DAT_00732a2c as u32).wrapping_sub(DAT_0073abc4 as u32) as i32,
                (DAT_00732a28 as u32).wrapping_sub(DAT_0073abc8 as u32) as i32,
                (DAT_00732a24 as u32).wrapping_sub(DAT_0073abcc as u32) as i32,
            );
            DAT_007394cc = 1;
            DAT_007394c8 = DAT_006da9f0;
            DAT_007394ce = DAT_00732e40;
            DAT_0073b372 = 0;
            if 3 < rd::<u8>(ab!(DAT_0070ad43), (DAT_0073b374 as u32).wrapping_mul(0x566)) {
                DAT_0073b372 = 6;
            }
            DAT_007394c5 = 0;
        }

        u8v = DAT_0073b374 as u32;
        u5 = u8v.wrapping_mul(0x566);
        DAT_007394c4 = rd(ab!(DAT_0070ad47), u5);
        DAT_007394c3 = rd(ab!(DAT_0070ad48), u5);
        DAT_007394cb = rd(ab!(DAT_0070ad44), u5);
        DAT_0073abc1 = rd(ab!(DAT_0070b215), u5);
        i9 = 0;
        DAT_0073abc0 = 0;
        let idx42: u8 = rd(ab!(DAT_0070ad42), u5);
        let t5247: u8 = rd(ab!(DAT_005247e8), idx42 as u32);
        DAT_0073b370 = rd(ab!(DAT_0052686b), (t5247 as u32).wrapping_mul(0x16));
        DAT_007394c7 = rd(ab!(DAT_0070ad49), u5);
        let t5247b: u8 = rd(ab!(DAT_005247e8), idx42 as u32);
        i10 = ((((t5247b as u32).wrapping_mul(0xb) >> 8) << 8) | (DAT_007394c7 as u32)) as i32;

        if param_1 == 0xffff {
            DAT_007394d0 = 0;
            DAT_007394c2 = 0xff;
            if rd::<u8>(ab!(DAT_0070ad43), u5) != 0 {
                loop {
                    let b = DAT_0073b374 as u32;
                    if rd::<u16>(ab!(DAT_006d4a6c), b.wrapping_mul(0x126))
                        < rd::<u16>(ab!(DAT_006d4a6a), b.wrapping_mul(0x93).wrapping_mul(2))
                    {
                        let r = FUN_0041f670();
                        u11 = r as u16;
                        if u11 == 0xffff {
                            return r & 0xffffffffffff0000u64;
                        }
                        i9 = i9.wrapping_add(1);
                        u5 = DAT_0073b374 as u32;
                        let o = u5.wrapping_mul(0x126);
                        let c: u16 = rd(ab!(DAT_006d4a6c), o);
                        wr(ab!(DAT_006d4a6c), o, c.wrapping_add(1));
                        if (rd::<u8>(ab!(DAT_0070ad40), u5.wrapping_mul(0x566)) as u32)
                            == (DAT_007394d0 as u32)
                        {
                            let q: u8 = rd(ab!(DAT_006d4aa3), o);
                            wr(ab!(DAT_006d4aa3), o, q.wrapping_add(1));
                        }
                    }
                    DAT_007394d0 = DAT_007394d0.wrapping_add(1);
                    i10 = (DAT_0073b374 as u32).wrapping_mul(0x2b3) as i32;
                    u5 = DAT_007394d0 as u32;
                    if !(u5
                        < rd::<u8>(ab!(DAT_0070ad43), (DAT_0073b374 as u32).wrapping_mul(0x566))
                            as u32)
                    {
                        break;
                    }
                }
            }
        } else {
            u5 = u8v.wrapping_mul(0x126);
            if rd::<u16>(ab!(DAT_006d4a6c), u5)
                < rd::<u16>(ab!(DAT_006d4a6a), u8v.wrapping_mul(0x93).wrapping_mul(2))
            {
                DAT_007394d0 = param_1;
                let r = FUN_0041f670();
                u11 = r as u16;
                if u11 == 0xffff {
                    return r & 0xffffffffffff0000u64;
                }
                i9 = 1;
                u8v = DAT_0073b374 as u32;
                let o = u8v.wrapping_mul(0x126);
                let c: u16 = rd(ab!(DAT_006d4a6c), o);
                wr(ab!(DAT_006d4a6c), o, c.wrapping_add(1));
                i10 = u8v.wrapping_mul(0x2b3) as i32;
                u5 = DAT_007394d0 as u32;
                if (rd::<u8>(ab!(DAT_0070ad40), u8v.wrapping_mul(0x566)) as u32) == u5 {
                    let q: u8 = rd(ab!(DAT_006d4aa3), o);
                    wr(ab!(DAT_006d4aa3), o, q.wrapping_add(1));
                }
            }
        }
        u6 = (u5 >> 0x10) as u16;
        if (((c_var2 != 0 || c_var3 != 0) || c_var4 != 0) && param_1 == 0xffff) && i9 != 0 {
            u5 = FUN_004a7aa0((((u5 >> 8) << 8) | (DAT_007394d2 as u32)) & 0xffff00ff);
            let r = FUN_004a31b0((((u5 >> 16) & 0xffff) << 16) | (DAT_0073b374 as u32), u5);
            i10 = (r >> 32) as u32 as i32;
            u6 = ((r >> 16) & 0xffff) as u16;
        }
        ((i10 as u32 as u64) << 32) | ((((u6 as u32) << 16) | 1u32) as u64)
    }
}
