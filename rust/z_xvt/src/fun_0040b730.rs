// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e794: i8;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b730: [u8; 1];
    static mut DAT_006d4980: u32;
    static mut DAT_006d4988: i32;
    static mut DAT_00732a3c: u32;
    static mut DAT_006d4a44: i8;
    static mut DAT_00523a68: [u8; 1];
    static mut DAT_00523a69: u8;
    static mut DAT_00523a6a: u8;
    static mut DAT_00523a79: u8;
    static mut DAT_00523a7d: u8;
    static mut DAT_00523a80: [u8; 1];
    static mut DAT_00523a81: u8;
    static mut DAT_00523a98: [u8; 1];
    static mut DAT_00523a97: u8;
    static mut DAT_0073b7e0: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004784d0() -> u16;
    fn FUN_0040c4c0(a: u32, b: i8, c: i8, d: i8, e: i16);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

unsafe fn udiv(a: u32, b: u32) -> u32 {
    let q: u32;
    // SAFETY: the machine's own unsigned divide; a zero divisor faults as in the original.
    unsafe {
        core::arch::asm!("div {d}", d = in(reg) b, inout("eax") a => q, inout("edx") 0u32 => _, options(nomem, nostack));
    }
    q
}

unsafe fn tbl(i: u32) -> u8 {
    // SAFETY: the game's own table.
    unsafe {
        (&raw mut DAT_0073b7e0)
            .cast::<u8>()
            .wrapping_add(i as usize)
            .read()
    }
}

unsafe fn tbl_inc(i: u32) {
    // SAFETY: the game's own table.
    unsafe {
        let p = (&raw mut DAT_0073b7e0)
            .cast::<u8>()
            .wrapping_add(i as usize);
        p.write(p.read().wrapping_add(1));
    }
}

unsafe fn pick(count: u32, rnd: u32, off: u32, lim: u8) -> u32 {
    // SAFETY: the game's own table and the OS exception call.
    unsafe {
        if count == 0 {
            RaiseException(0xC0000094u32, 0, 0, core::ptr::null());
        }
        let mut v = udiv(rnd & 0xffff, count);
        if (v as i16) == 0 {
            v = 0;
        } else {
            v = (rnd & 0xffff) % v;
        }
        if lim != 0 && lim <= tbl(off.wrapping_add(v)) {
            let mut c = count;
            loop {
                if c == 0 {
                    v = 0xffff_ffff;
                    break;
                }
                v = v.wrapping_add(1);
                if (count as i32) <= (v as i32) {
                    v = 0;
                }
                c = c.wrapping_sub(1);
                if !(lim <= tbl(off.wrapping_add(v))) {
                    break;
                }
            }
        }
        v
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040b730(
    param_1: i32,
    mut param_2: u32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
    mut param_6: u16,
) -> u32 {
    // SAFETY: all globals, callees and pointers are the game's own memory,
    // accessed where and in the order the original does.
    unsafe {
        if DAT_0085e794 == 0 {
            return 0;
        }
        if param_1 == -1 {
            return 0;
        }
        if param_1 != DAT_0071ecf4 {
            return 0;
        }
        let mut i_var8: i32 = (&raw mut DAT_0071b730)
            .cast::<u8>()
            .wrapping_add((param_1 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<i32>()
            .read_unaligned();
        if i_var8 == -1 {
            return 0;
        }
        if param_6 != 0xffff {
            if DAT_0085e794 == 1 {
                param_6 >>= 1;
            }
            let u_var2 = FUN_004784d0();
            if param_6 <= u_var2 {
                return 0;
            }
        }
        let pbase: *mut u8 = (DAT_006d4988 as u32 as usize) as *mut u8;
        let mut au: [u32; 6] = [0; 6];
        let aub = au.as_mut_ptr();
        let mut u_stack_24: u32;
        if param_2 == 0xffff_ffff {
            u_stack_24 = 0;
            if DAT_00732a3c < DAT_006d4980 {
                let mut p7 = aub;
                let mut pc10 =
                    pbase.wrapping_add(DAT_00732a3c.wrapping_mul(0x23).wrapping_add(3) as usize);
                let mut u_var12 = DAT_00732a3c;
                loop {
                    let pc = pc10.wrapping_add(0x1c).cast::<u32>().read_unaligned() as usize
                        as *const i8;
                    let iv = pc10.wrapping_add(0x18).cast::<i32>().read_unaligned();
                    if pc10.read() != 0
                        && pc.read() == 0
                        && iv != param_1
                        && pbase
                            .wrapping_add(
                                (i_var8 as u32).wrapping_mul(0x23).wrapping_add(0x16) as usize
                            )
                            .read()
                            == pc10.wrapping_add(0x13).read()
                    {
                        p7.write(u_var12);
                        p7 = p7.wrapping_add(1);
                        u_stack_24 = u_stack_24.wrapping_add(1);
                    }
                    pc10 = pc10.wrapping_add(0x23);
                    u_var12 = u_var12.wrapping_add(1);
                    if !(u_var12 < DAT_006d4980) {
                        break;
                    }
                }
            }
            if u_stack_24 == 0 {
                if param_3 == 0xc {
                    FUN_0040c4c0(0x25, 0, 0, 0, -1);
                    FUN_0040c4c0(0x25, 0, 0, 0, -1);
                }
                return 0;
            }
            let u_var2 = FUN_004784d0();
            let q = udiv(u_var2 as u32, u_stack_24 & 0xffff) as u16;
            let u_var12: u32 = if q == 0 {
                0
            } else {
                (u_var2 as u32) % (q as u32)
            };
            param_2 = aub.wrapping_add(u_var12 as usize).read();
            if param_5 == param_2 {
                let mut u_var3 = u_var12.wrapping_add(1);
                if u_stack_24 <= u_var3 {
                    u_var3 = 0;
                }
                if u_var3 == u_var12 {
                    return 0;
                }
            }
        } else {
            if (DAT_006d4980 as i32) <= (param_2 as i32) {
                return 0;
            }
            if param_3 != 1
                && pbase
                    .wrapping_add((i_var8 as u32).wrapping_mul(0x23).wrapping_add(0x16) as usize)
                    .read()
                    != pbase
                        .wrapping_add(param_2.wrapping_mul(0x23).wrapping_add(0x16) as usize)
                        .read()
            {
                return 0;
            }
        }
        let i_tmp = pbase
            .wrapping_add(param_2.wrapping_mul(0x23).wrapping_add(0x1f) as usize)
            .cast::<i32>()
            .read_unaligned();
        let pp = ((i_tmp as u32).wrapping_add(0xa7) as usize as *const u32).read_unaligned()
            as usize as *mut u8;
        let mut u_stack_20 = pp.cast::<u32>().read_unaligned();
        if 6 < u_stack_20 {
            u_stack_20 = 0;
        }
        let mut u_stack_1c: u32 = 0;
        au[0] = (pp.wrapping_add(0xb0).read() as u32).wrapping_mul(0x61);
        i_var8 = au[0].wrapping_add(0x6f) as i32;
        let s_var11: i16;
        if param_5 == 0xffff_ffff || (DAT_006d4980 as i32) <= (param_5 as i32) {
            s_var11 = -1;
        } else {
            let psv9 = pbase.wrapping_add(param_5.wrapping_mul(0x23) as usize);
            s_var11 = psv9.cast::<i16>().read_unaligned();
            let i_tmp2 = psv9.wrapping_add(0x1f).cast::<i32>().read_unaligned();
            let pb = ((i_tmp2 as u32).wrapping_add(0xa7) as usize as *const u32).read_unaligned()
                as usize as *mut u8;
            u_stack_1c = pb.wrapping_add(0xb0).read() as u32;
        }
        let p3 = param_3 as u32;
        let u_var12: u32 = (&raw mut DAT_00523a68)
            .cast::<u8>()
            .wrapping_add(p3 as usize)
            .read() as u32;
        let c80: u8 = (&raw mut DAT_00523a80)
            .cast::<u8>()
            .wrapping_add(p3 as usize)
            .read();
        let c98: u8 = (&raw mut DAT_00523a98)
            .cast::<u8>()
            .wrapping_add(p3 as usize)
            .read();
        let c_var13 = param_3 as i8;
        let i8u = i_var8 as u32;
        if param_4 == -1 {
            match param_3 {
                3 | 5 => {
                    if DAT_006d4a44 != 0 {
                        return 0;
                    }
                    let count = c80 as u32;
                    au[0] = count;
                    let rnd = FUN_004784d0() as u32;
                    let off = u_stack_1c.wrapping_mul(0x61).wrapping_add(u_var12);
                    let u_var3 = pick(count, rnd, off, c98);
                    if u_var3 == 0xffff_ffff {
                        return 0;
                    }
                    if u_stack_20 != 0 {
                        tbl_inc(
                            u_var3
                                .wrapping_add(u_stack_1c.wrapping_mul(0x61))
                                .wrapping_add(u_var12),
                        );
                        FUN_0040c4c0(
                            (DAT_00523a6a as u32)
                                .wrapping_add(u_stack_20)
                                .wrapping_add(i8u)
                                .wrapping_sub(1),
                            1,
                            2,
                            0,
                            s_var11,
                        );
                    }
                    FUN_0040c4c0(
                        u_var12.wrapping_add(i8u).wrapping_add(u_var3),
                        1,
                        c_var13,
                        1,
                        s_var11,
                    );
                    return 1;
                }
                6 => {
                    if DAT_006d4a44 != 0 {
                        return 0;
                    }
                    let count = c80 as u32;
                    let rnd = FUN_004784d0() as u32;
                    let off = u_stack_1c.wrapping_mul(0x61).wrapping_add(u_var12);
                    let u_var3 = pick(count, rnd, off, c98);
                    if u_var3 == 0xffff_ffff {
                        return 0;
                    }
                    tbl_inc(u_var3.wrapping_add(off));
                    FUN_0040c4c0(
                        u_var12.wrapping_add(i8u).wrapping_add(u_var3),
                        1,
                        c_var13,
                        0,
                        s_var11,
                    );
                    return 1;
                }
                9 | 10 | 0x15 => {
                    let count = c80 as u32;
                    let rnd = FUN_004784d0() as u32;
                    let off = u_stack_1c.wrapping_mul(0x61).wrapping_add(u_var12);
                    let u_var3 = pick(count, rnd, off, c98);
                    if u_var3 == 0xffff_ffff {
                        return 0;
                    }
                    tbl_inc(u_var3.wrapping_add(off));
                    FUN_0040c4c0(
                        u_var12.wrapping_add(i8u).wrapping_add(u_var3),
                        1,
                        c_var13,
                        0,
                        s_var11,
                    );
                    return 1;
                }
                0xc => {
                    let u_var2 = FUN_004784d0();
                    let q = u_var2 >> 2;
                    let u_var3: u32 = if q == 0 {
                        0
                    } else {
                        (u_var2 as u32) % (q as u32)
                    };
                    FUN_0040c4c0(
                        u_var12.wrapping_add(i8u).wrapping_add(u_var3),
                        1,
                        c_var13,
                        0,
                        s_var11,
                    );
                    return 1;
                }
                0x10 => {
                    let count = c80 as u32;
                    au[0] = count;
                    let rnd = FUN_004784d0() as u32;
                    let off = u_stack_1c.wrapping_mul(0x61).wrapping_add(u_var12);
                    let u_var3 = pick(count, rnd, off, c98);
                    if u_var3 == 0xffff_ffff {
                        return 0;
                    }
                    tbl_inc(u_var3.wrapping_add(off));
                    FUN_0040c4c0(
                        u_var12.wrapping_add(i8u).wrapping_add(u_var3),
                        1,
                        c_var13,
                        0,
                        s_var11,
                    );
                    FUN_0040c4c0(
                        (DAT_00523a79 as u32)
                            .wrapping_add(u_stack_1c)
                            .wrapping_add(i8u),
                        1,
                        0x11,
                        1,
                        s_var11,
                    );
                    return 1;
                }
                _ => {}
            }
        } else {
            if param_3 == 1 {
                if u_stack_20 != 0 {
                    FUN_0040c4c0(
                        ((&raw mut DAT_00523a68).cast::<u8>().read() as u32)
                            .wrapping_add(u_stack_20)
                            .wrapping_add(i8u)
                            .wrapping_sub(1),
                        1,
                        0,
                        0,
                        s_var11,
                    );
                }
                let u_var2 = FUN_004784d0();
                if u_var2 < 0x5555 {
                    FUN_0040c4c0((DAT_00523a69 as u32).wrapping_add(i8u), 1, 1, 1, s_var11);
                }
                if param_4 < (DAT_00523a81 as i32) {
                    FUN_0040c4c0(
                        u_var12.wrapping_add(i8u).wrapping_add(param_4 as u32),
                        1,
                        1,
                        2,
                        s_var11,
                    );
                    return 1;
                }
                return 0;
            }
            if param_3 != 0x17 {
                if param_4 < (c80 as i32) {
                    FUN_0040c4c0(
                        u_var12.wrapping_add(i8u).wrapping_add(param_4 as u32),
                        1,
                        c_var13,
                        0,
                        s_var11,
                    );
                    return 1;
                }
                return 0;
            }
            let u_var2 = FUN_004784d0();
            let b_var1 = DAT_00523a97;
            if u_var2 < 0x8000 {
                let u_var6 = FUN_004784d0() as u32;
                if b_var1 == 0 {
                    RaiseException(0xC0000094u32, 0, 0, core::ptr::null());
                }
                let mut u_var3 = udiv(u_var6 & 0xffff, b_var1 as u32);
                if (u_var3 as i16) == 0 {
                    u_var3 = 0;
                } else {
                    u_var3 = (u_var6 & 0xffff) % u_var3;
                }
                FUN_0040c4c0(
                    (DAT_00523a7d as u32).wrapping_add(i8u).wrapping_add(u_var3),
                    1,
                    0x15,
                    0,
                    s_var11,
                );
            }
            if (DAT_00523a97 as i32) <= param_4 {
                return 0;
            }
            FUN_0040c4c0(
                i8u.wrapping_add(u_var12).wrapping_add(param_4 as u32),
                1,
                0x17,
                1,
                s_var11,
            );
            tbl_inc((param_4 as u32).wrapping_add(u_var12).wrapping_add(au[0]));
        }
        1
    }
}
