// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d8ccb: [u8; 0];
    static mut DAT_009d8cca: [u8; 0];
    static mut DAT_009ed22c: u8;
    static mut DAT_009ed22d: u8;
    static mut DAT_009ed228: [u8; 0];
    static mut DAT_009d6940: [u8; 0];
    static mut DAT_009a1834: u16;
    static mut DAT_0099f98a: u8;
    static mut DAT_0099f98c: u8;
    static mut DAT_009a1084: i32;
    static mut DAT_009a1088: i32;
    static mut DAT_009a108c: i32;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_009a2062: i8;
    static mut DAT_009d8c94: [u8; 0];
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009d8c83: [u8; 0];
    static mut DAT_009d8c80: [u8; 0];
    static mut DAT_009d8c82: [u8; 0];
    static mut DAT_009d8c84: [u8; 0];
    static mut DAT_009d8c87: [u8; 0];
    static mut DAT_009d8c89: [u8; 0];
    static mut DAT_009d8c8d: [u8; 0];
    static mut DAT_009d8c8e: [u8; 0];
    static mut DAT_009d9155: [u8; 0];
    static mut DAT_009a2000: i16;
    static mut DAT_009a20c8: [u8; 0];
    static mut DAT_009d9192: [u8; 0];
    static mut DAT_009d911c: [u8; 0];
    static mut DAT_009d9122: [u8; 0];
    static mut DAT_00a08294: i32;
    static mut DAT_009a8070: i16;
    static mut DAT_00a080f0: i16;
    static mut DAT_0099f988: i16;
    static mut DAT_0099f98e: i16;
    static mut DAT_009a20b0: i32;
    static mut DAT_009cd26c: i32;
    static mut DAT_009fd438: i32;
    static mut DAT_009a1832: u8;
    static mut DAT_0099f985: u8;
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_00a08104: i32;
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;
    static mut DAT_0051c641: [u8; 0];
    static mut DAT_0051c645: [u8; 0];
    static mut DAT_0051c649: [u8; 0];
    static mut DAT_0051c64d: [u8; 0];
    static mut DAT_0051c651: [u8; 0];
    static mut DAT_0051c655: [u8; 0];
    static mut DAT_0099f984: u8;
    static mut DAT_0099f983: u8;
    static mut DAT_0099f98b: u8;
    static mut DAT_009a1081: u8;
    static mut DAT_009a1080: u8;
    static mut DAT_009a1830: u8;
    static mut DAT_00521f1b: [u8; 0];
    static mut DAT_00524170: [u8; 0];
    static mut DAT_0099f987: u8;
    static mut DAT_0099f990: u8;
    static mut DAT_0099f982: u8;
    static mut DAT_0099f992: u8;
    static mut DAT_009a20ca: [u16; 0];
    static mut DAT_009a20cc: [u8; 0];
    static mut DAT_009a2103: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
    fn FUN_00456ac0() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00458e20(param_1: i32, param_2: i32);
    fn FUN_00425cb0() -> u32;
    fn FUN_0046a8d0(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_0046a870(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_004037b0(param_1: u32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_004269e0(param_1: u32) -> u32;
    fn FUN_00451940(param_1: u32, param_2: u32);
}

macro_rules! rd {
    ($n:ident, $i:expr) => {
        (&raw const $n).cast::<u8>().add(($i) as usize).read()
    };
}
macro_rules! wr {
    ($n:ident, $i:expr, $v:expr) => {
        (&raw mut $n).cast::<u8>().add(($i) as usize).write($v)
    };
}
macro_rules! rd16 {
    ($n:ident, $i:expr) => {
        (&raw const $n)
            .cast::<u8>()
            .add(($i) as usize)
            .cast::<u16>()
            .read_unaligned()
    };
}
macro_rules! wr16 {
    ($n:ident, $i:expr, $v:expr) => {
        (&raw mut $n)
            .cast::<u8>()
            .add(($i) as usize)
            .cast::<u16>()
            .write_unaligned($v)
    };
}
macro_rules! rd32 {
    ($n:ident, $i:expr) => {
        (&raw const $n)
            .cast::<u8>()
            .add(($i) as usize)
            .cast::<i32>()
            .read_unaligned()
    };
}
macro_rules! rdp32 {
    ($a:expr) => {
        (($a) as usize as *const i32).read_unaligned()
    };
}
macro_rules! rdp8 {
    ($a:expr) => {
        (($a) as usize as *const u8).read()
    };
}

#[inline(always)]
unsafe fn udiv(a: u32, b: u32) -> u32 {
    let q: u32;
    // SAFETY: machine divide; faults on zero divisor as the original would.
    unsafe {
        core::arch::asm!("div {d}", d = in(reg) b, inout("eax") a => q, inout("edx") 0u32 => _);
    }
    q
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_unsafe
)]
pub unsafe extern "C" fn FUN_00456190(param_1: u16) -> u64 {
    let extraout_var: u16 = 0;
    let extraout_var_00: u16 = 0;
    let extraout_edx: u32 = 0;
    let extraout_edx_00: u32 = 0;
    let extraout_edx_01: i32 = 0;
    // SAFETY: all globals and callees are the game's own; pointers are
    // computed from the game's tables exactly as the original does.
    unsafe {
        let c4 = DAT_009ed22d;
        let c3 = DAT_009ed22c;
        let c2 = rd!(DAT_009ed228, 3);
        let mut b_var1: i32 = 0;
        let mut u_var13: u32 = 0;
        DAT_0099f98a = 0;
        DAT_0099f98c = 0;
        let mut i_var11: i32 = (DAT_009a1834 as u32).wrapping_mul(0x566) as i32;
        let mut u_var16: u32 = 0;
        let mut u_var15: u16;
        let mut i_var14: i32;
        let mut t_a: i32;
        let mut t_b: i32;
        let mut t_c: i32;

        if rd!(DAT_009d8ccb, i_var11) == 0
            || (DAT_009ed22c == 0 && DAT_009ed22d == 0 && rd!(DAT_009ed228, 3) == 0)
            || rd!(DAT_009d6940, 1) != 0
            || param_1 != 0xffff
        {
            FUN_00458e20(0x8000, DAT_009a1834 as i32);
            DAT_009a1084 = DAT_00a07cdc;
            DAT_009a1088 = DAT_00a07cd8;
            DAT_009a108c = DAT_00a07cd4;
            if c3 != 0 || c4 != 0 || c2 != 0 {
                if DAT_009a2062 == 2 {
                    u_var13 = DAT_009a1834 as u32;
                    i_var11 = u_var13.wrapping_mul(0x566) as i32;
                    if rd!(DAT_009d8c94, i_var11) != 0 {
                        let mut local_10: [u8; 10] = [0; 10];
                        let lp = local_10.as_mut_ptr();
                        let mut b_var12: u8 = 0;
                        if 0 < DAT_009a2000 {
                            loop {
                                let o = (b_var12 as u32).wrapping_mul(0x566);
                                if rd!(DAT_009d8c94, o) != 0 {
                                    let k = rd!(DAT_009d8c88, o) as usize;
                                    lp.add(k).write(lp.add(k).read().wrapping_add(1));
                                }
                                b_var12 = b_var12.wrapping_add(1);
                                if !((b_var12 as u16 as i16) < DAT_009a2000) {
                                    break;
                                }
                            }
                        }
                        if lp.read() == 1 {
                            if rd!(DAT_009d8c83, i_var11) == 0 {
                                RaiseException(0xC0000094, 0, 0, core::ptr::null());
                            }
                            b_var12 = udiv(
                                rd!(DAT_009a20c8, u_var13.wrapping_mul(0x126)) as u32,
                                rd!(DAT_009d8c83, i_var11) as u32,
                            ) as u8;
                            if (rd!(DAT_009d8c94, i_var11) & 1) == 0 {
                                b_var12 = b_var12.wrapping_neg();
                            }
                            let mut b_var5: u8 = 0;
                            if 0 < DAT_009a2000 {
                                loop {
                                    let want =
                                        ((((rd!(DAT_009d8c94, i_var11) as u32).wrapping_add(
                                            (b_var12 as u32).wrapping_sub(3).wrapping_mul(3),
                                        )) & 7)
                                            .wrapping_add(1))
                                            as u8;
                                    if rd!(DAT_009d8c94, (b_var5 as u32).wrapping_mul(0x566))
                                        == want
                                    {
                                        FUN_00458e20(0x8000, b_var5 as i32);
                                        break;
                                    }
                                    b_var5 = b_var5.wrapping_add(1);
                                    if !((b_var5 as u16 as i16) < DAT_009a2000) {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    DAT_009a1084 = DAT_00a07cdc;
                    DAT_009a1088 = DAT_00a07cd8;
                    DAT_009a108c = DAT_00a07cd4;
                    t_a = rd32!(DAT_009d9192, (DAT_009a1834 as u32).wrapping_mul(0x566));
                    if t_a != -1 {
                        u_var15 = FUN_00425cb0() as u16;
                        let u_var7 = FUN_00425cb0();
                        u_var13 = (u_var15 & 0x7fff) as u32;
                        if (u_var7 & 1) == 0 {
                            u_var13 = u_var13.wrapping_neg();
                        }
                        DAT_009a1084 = DAT_009a1084.wrapping_add(u_var13 as i32);
                        u_var15 = FUN_00425cb0() as u16;
                        u_var13 = FUN_00425cb0();
                        if (u_var13 & 1) == 0 {
                            DAT_009a1088 = DAT_009a1088.wrapping_sub((u_var15 & 0x7fff) as i32);
                        } else {
                            DAT_009a1088 = DAT_009a1088.wrapping_add((u_var15 & 0x7fff) as i32);
                        }
                    }
                } else if DAT_00a08294 == 0xe {
                    i_var11 = (DAT_009a1834 as u32).wrapping_mul(0x566) as i32;
                    if rd!(DAT_009d8c94, i_var11) != 0 && rd16!(DAT_009d911c, i_var11) != 0 {
                        FUN_00458e20(0x8001, DAT_009a1834 as i32);
                        DAT_009a1084 = DAT_00a07cdc;
                        DAT_009a1088 = DAT_00a07cd8;
                        DAT_009a108c = DAT_00a07cd4;
                    }
                }
            }
            if rd16!(DAT_009d9122, (DAT_009a1834 as u32).wrapping_mul(0x566)) == 0 {
                DAT_009a8070 = 0;
                DAT_00a080f0 = 0x4000;
            } else {
                FUN_00458e20(0x8004, DAT_009a1834 as i32);
                FUN_0046a8d0(
                    DAT_00a07cdc.wrapping_sub(DAT_009a1084),
                    DAT_00a07cd8.wrapping_sub(DAT_009a1088),
                    DAT_00a07cd4.wrapping_sub(DAT_009a108c),
                );
            }
            DAT_0099f988 = DAT_009a8070;
            DAT_0099f98e = DAT_00a080f0;
            t_a = rd32!(DAT_009d9192, (DAT_009a1834 as u32).wrapping_mul(0x566));
            if rd!(DAT_009d6940, 1) == 0
                && rd!(DAT_009d8ccb, (DAT_009a1834 as u32).wrapping_mul(0x566)) == 0
                && (c3 != 0 || c4 != 0 || c2 != 0)
                && param_1 == 0xffff
                && t_a == -1
            {
                DAT_009a8070 = (DAT_009a8070 as i32).wrapping_sub(0x8000) as i16;
                let s_var6 = (-0x8000i32).wrapping_sub(DAT_00a080f0 as i32) as i16;
                DAT_00a080f0 = s_var6;
                FUN_0046a870(0xffff, DAT_009a8070 as i32, s_var6 as i32);
                DAT_009a20b0 = DAT_009a20b0.wrapping_mul(8);
                DAT_009cd26c = DAT_009cd26c.wrapping_mul(8);
                DAT_009fd438 = DAT_009fd438.wrapping_mul(8);
                DAT_009a1084 = DAT_009a1084.wrapping_add(DAT_009a20b0);
                DAT_009a1088 = DAT_009a1088.wrapping_add(DAT_009cd26c);
                DAT_009a108c = DAT_009a108c.wrapping_add(DAT_009fd438);
                DAT_0099f98a = 1;
            }
            DAT_009a1832 = rd!(DAT_009d8c8d, (DAT_009a1834 as u32).wrapping_mul(0x566));
            DAT_0099f985 = rd!(DAT_009d8c8e, (DAT_009a1834 as u32).wrapping_mul(0x566));
        } else {
            u_var13 = DAT_00a07cec as u32;
            if (u_var13 as i32) < DAT_009a1fe0 {
                u_var13 = 0;
                u_var15 = DAT_00a07cec;
                u_var16 = 0;
                loop {
                    let base = (u_var15 as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(DAT_009a1fe8);
                    if rdp8!(base.wrapping_add(3)) != 0 {
                        t_a = rdp32!(base.wrapping_add(0x1f));
                        DAT_00a08104 = rdp32!((t_a as u32).wrapping_add(0xa9));
                        if rdp8!(base.wrapping_add(0x16)) == rd!(DAT_009d8cca, i_var11) {
                            if rdp8!((DAT_00a08104 as u32).wrapping_add(5)) == 0xff {
                                b_var1 = 1;
                                u_var16 = u_var15 as u32;
                                break;
                            }
                        }
                    }
                    u_var15 = u_var15.wrapping_add(1);
                    if !((u_var15 as i32) < DAT_009a1fe0) {
                        b_var1 = 0;
                        break;
                    }
                }
            }
            if b_var1 == 0 {
                return (((u_var13 as u64) << 32) | (i_var11 as u32 as u64))
                    & 0xffffffffffff0000u64;
            }
            i_var11 = (u_var16 & 0xffff).wrapping_mul(0x23) as i32;
            t_a = rdp32!(DAT_009a1fe8.wrapping_add(i_var11 as u32).wrapping_add(0x1f));
            DAT_00a08104 = rdp32!((t_a as u32).wrapping_add(0xa9));
            i_var14 =
                (rdp8!((DAT_00a08104 as u32).wrapping_add(4)) as u32).wrapping_mul(0x101) as i32;
            t_a = rd32!(DAT_0051c641, i_var14);
            t_b = rd32!(DAT_0051c645, i_var14);
            t_c = rd32!(DAT_0051c649, i_var14);
            FUN_004037b0(DAT_009a1fe8.wrapping_add(i_var11 as u32), t_a, t_b, t_c);
            let i_var10 = DAT_009a1fe8.wrapping_add(i_var11 as u32);
            DAT_009a1084 = rdp32!(i_var10.wrapping_add(4)).wrapping_add(DAT_009d8a58);
            DAT_009a1088 = rdp32!(i_var10.wrapping_add(8)).wrapping_add(DAT_009d8a54);
            DAT_009a108c = rdp32!(i_var10.wrapping_add(0xc)).wrapping_add(DAT_009d8b60);
            t_a = rd32!(DAT_0051c64d, i_var14);
            t_b = rd32!(DAT_0051c651, i_var14);
            t_c = rd32!(DAT_0051c655, i_var14);
            FUN_004037b0(i_var10, t_a, t_b, t_c);
            let i_var11b = DAT_009a1fe8.wrapping_add(i_var11 as u32);
            DAT_00a07cdc = rdp32!(i_var11b.wrapping_add(4)).wrapping_add(DAT_009d8a58);
            DAT_00a07cd8 = rdp32!(i_var11b.wrapping_add(8)).wrapping_add(DAT_009d8a54);
            DAT_00a07cd4 = rdp32!(i_var11b.wrapping_add(0xc)).wrapping_add(DAT_009d8b60);
            i_var11 = i_var11b as i32;
            FUN_0046a8d0(
                DAT_00a07cdc.wrapping_sub(DAT_009a1084),
                DAT_00a07cd8.wrapping_sub(DAT_009a1088),
                DAT_00a07cd4.wrapping_sub(DAT_009a108c),
            );
            DAT_0099f98c = 1;
            DAT_0099f988 = DAT_009a8070;
            DAT_0099f98e = DAT_00a080f0;
            DAT_009a1832 = 0;
            if 3 < rd!(DAT_009d8c83, (DAT_009a1834 as u32).wrapping_mul(0x566)) {
                DAT_009a1832 = 6;
            }
            DAT_0099f985 = 0;
        }

        let mut u_var7 = DAT_009a1834 as u32;
        u_var13 = u_var7.wrapping_mul(0x566);
        DAT_0099f984 = rd!(DAT_009d8c87, u_var13);
        DAT_0099f983 = rd!(DAT_009d8c88, u_var13);
        DAT_0099f98b = rd!(DAT_009d8c84, u_var13);
        DAT_009a1081 = rd!(DAT_009d9155, u_var13);
        i_var14 = 0;
        DAT_009a1080 = 0;
        DAT_009a1830 = rd!(
            DAT_00521f1b,
            (rd!(DAT_00524170, rd!(DAT_009d8c82, u_var13)) as u32).wrapping_mul(0x1a)
        );
        DAT_0099f987 = rd!(DAT_009d8c89, u_var13);
        i_var11 = (((((rd!(DAT_00524170, rd!(DAT_009d8c82, u_var13)) as u32).wrapping_mul(0x19))
            >> 8)
            << 8)
            | (DAT_0099f987 as u32)) as i32;
        if param_1 == 0xffff {
            DAT_0099f990 = 0;
            DAT_0099f982 = 0xff;
            if rd!(DAT_009d8c83, u_var13) != 0 {
                loop {
                    let a = DAT_009a1834 as u32;
                    if rd16!(DAT_009a20cc, a.wrapping_mul(0x126))
                        < (&raw const DAT_009a20ca)
                            .cast::<u16>()
                            .add(a.wrapping_mul(0x93) as usize)
                            .read()
                    {
                        let u_var8 = FUN_00456ac0();
                        if (u_var8 as u16 as i16) == -1 {
                            return (((extraout_edx as u64) << 32) | (u_var8 as u64))
                                & 0xffffffffffff0000u64;
                        }
                        i_var14 = i_var14.wrapping_add(1);
                        u_var13 = DAT_009a1834 as u32;
                        let o = u_var13.wrapping_mul(0x126);
                        wr16!(DAT_009a20cc, o, rd16!(DAT_009a20cc, o).wrapping_add(1));
                        if rd!(DAT_009d8c80, u_var13.wrapping_mul(0x566)) == DAT_0099f990 {
                            wr!(DAT_009a2103, o, rd!(DAT_009a2103, o).wrapping_add(1));
                        }
                    }
                    DAT_0099f990 = DAT_0099f990.wrapping_add(1);
                    i_var11 = (DAT_009a1834 as u32).wrapping_mul(0x2b3) as i32;
                    u_var13 = DAT_0099f990 as u32;
                    if !(u_var13
                        < rd!(DAT_009d8c83, (DAT_009a1834 as u32).wrapping_mul(0x566)) as u32)
                    {
                        break;
                    }
                }
            }
        } else {
            u_var13 = u_var7.wrapping_mul(0x126);
            if rd16!(DAT_009a20cc, u_var13)
                < (&raw const DAT_009a20ca)
                    .cast::<u16>()
                    .add(u_var7.wrapping_mul(0x93) as usize)
                    .read()
            {
                DAT_0099f990 = param_1 as u8;
                let u_var8 = FUN_00456ac0();
                if (u_var8 as u16 as i16) == -1 {
                    return (((extraout_edx_00 as u64) << 32) | (u_var8 as u64))
                        & 0xffffffffffff0000u64;
                }
                i_var14 = 1;
                u_var7 = DAT_009a1834 as u32;
                let o = u_var7.wrapping_mul(0x126);
                wr16!(DAT_009a20cc, o, rd16!(DAT_009a20cc, o).wrapping_add(1));
                i_var11 = u_var7.wrapping_mul(0x2b3) as i32;
                u_var13 = DAT_0099f990 as u32;
                if rd!(DAT_009d8c80, u_var7.wrapping_mul(0x566)) as u32 == u_var13 {
                    wr!(DAT_009a2103, o, rd!(DAT_009a2103, o).wrapping_add(1));
                }
            }
        }
        let mut u_var9: u16 = (u_var13 >> 16) as u16;
        if (c3 != 0 || c4 != 0 || c2 != 0) && param_1 == 0xffff && i_var14 != 0 {
            u_var13 = FUN_004269e0((((u_var13 >> 8) << 8) | (DAT_0099f992 as u32)) & 0xffff00ff);
            FUN_00451940(
                ((extraout_var_00 as u32) << 16) | (DAT_009a1834 as u32),
                u_var13,
            );
            i_var11 = extraout_edx_01;
            u_var9 = extraout_var;
        }
        ((i_var11 as u32 as u64) << 32) | ((((u_var9 as u32) << 16) | 1u32) as u64)
    }
}
