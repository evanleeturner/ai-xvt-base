// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b740: u8;
    static mut DAT_0071b730: u8;
    static mut DAT_0071b785: u8;
    static mut DAT_0071b789: u8;
    static mut DAT_0071b78d: u8;
    static mut DAT_0071b791: u8;
    static mut DAT_0071b795: u8;
    static mut DAT_0071b799: u8;
    static mut DAT_0052b13f: u8;
    static mut DAT_0052b13d: u8;
    static mut DAT_0052b141: u8;
    static mut DAT_0052b143: u8;
    static mut DAT_0052b145: u8;
    static mut DAT_0052b147: u8;
    static mut DAT_0052b149: u8;
    static mut DAT_0052b09d: u8;
    static mut DAT_0052686c: u8;
    static mut DAT_00525bf7: u8;
    static mut DAT_005248c7: u8;
    static mut DAT_00524968: u8;
    static mut DAT_005249c0: u8;
    static mut DAT_0071ecf4: u32;
    static mut DAT_0071e690: u32;
    static mut DAT_006d4988: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_0070ab18: i32;
    static mut DAT_0070ab14: i32;
    static mut DAT_0070ac20: i32;
    static mut DAT_0070a1d2: i16;
    static mut DAT_0071e6be: u16;
    static mut DAT_00732a38: i32;
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_006d4a50: i32;
    static mut DAT_006ffbec: i32;
    static mut DAT_0072f4e8: i32;
    static mut DAT_0071b71c: i32;
    static mut DAT_006da9f0: i16;
    static mut DAT_00732e40: i16;
    static mut DAT_006db720: i32;
    static mut DAT_006db704: i32;
    static mut DAT_006db708: i32;
    static mut DAT_007092c1: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a7aa0(a: u32) -> u32;
    fn FUN_0042a230(c: i32, d: i32, e: i32, f: i32);
    fn FUN_00421df0(a: u32, b: i32);
    fn FUN_00412700(a: u32, b: i32);
    fn FUN_0049f9c0(a: u32, b: u32, c: i32);
    fn FUN_0044ad00(a: u32) -> i32;
    fn FUN_0041a750(a: u32, b: i32);
    fn FUN_0042b150(a: i32) -> i32;
    fn FUN_00483410(a: i16, b: i16, c: i32);
    fn FUN_00413ca0(c: i32, d: u32) -> u64;
    fn FUN_00421c70(a: u32, b: i32);
    fn FUN_00449b30(a: u32, b: u32) -> i32;
    fn FUN_00449cb0(a: u32, b: u32) -> i32;
    fn FUN_00449bf0(a: u32, b: u32) -> i32;
    fn FUN_00418cd0(a: i32, b: i32, c: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00401000();
    fn FUN_00478470() -> u8;
}

unsafe fn rd<T>(p: *const u8) -> T {
    // SAFETY: the pointer addresses the game's own memory, read where the original reads.
    unsafe { p.cast::<T>().read_unaligned() }
}

unsafe fn wr<T>(p: *mut u8, v: T) {
    // SAFETY: the pointer addresses the game's own memory, written where the original writes.
    unsafe { p.cast::<T>().write_unaligned(v) }
}

fn off(p: *mut u8, o: i32) -> *mut u8 {
    p.wrapping_byte_offset(o as isize)
}

unsafe fn rp(p: *const u8) -> *mut u8 {
    // SAFETY: reads a stored 32-bit pointer from the game's memory.
    unsafe { rd::<u32>(p) as usize as *mut u8 }
}

fn ip(v: i32) -> *mut u8 {
    v as u32 as usize as *mut u8
}

unsafe fn gp(o: i32) -> *mut u8 {
    // SAFETY: DAT_006d4988 is the game's own global.
    unsafe { ip(DAT_006d4988.wrapping_add(o)) }
}

unsafe fn clamp_add(p: *mut u8, d: i32) {
    // SAFETY: p points into the game's object record.
    unsafe {
        let s: i32 = rd(p);
        let n = s.wrapping_add(d);
        wr(p, n);
        if n < -0x1000000 {
            wr(p, 0xff000000u32);
        }
        let s2: i32 = rd(p);
        if 0x1000000 < s2 {
            wr(p, 0x1000000i32);
        }
    }
}

unsafe fn upd(i2: i32, id: u32) {
    // SAFETY: all addresses are the game's own tables and globals.
    unsafe {
        let a = gp(id.wrapping_mul(0x23) as i32);
        let r = FUN_004a7aa0((((id >> 8) << 8) | u32::from(rd::<u8>(off(a, 3)))) & 0xffff00ff);
        let i17 = ((r & 0xffff).wrapping_mul(0xef)) as i32;
        let u: u32 = rd(off(&raw mut DAT_0071b730, i2));
        let i18 = (u.wrapping_mul(0x23) as i32).wrapping_add(DAT_006d4988);
        let s16: i16 = rd(off(&raw mut DAT_0052b13f, i17));
        let s16b: i16 = rd(off(&raw mut DAT_0052b13d, i17));
        FUN_0042a230(i18, 0, s16 as i32, s16b as i32);
        let u: u32 = rd(off(&raw mut DAT_0071b785, i2));
        wr(off(&raw mut DAT_0071b791, i2), u);
        let a18 = DAT_0070ab18;
        let u: u32 = rd(off(&raw mut DAT_0071b789, i2));
        wr(off(&raw mut DAT_0071b795, i2), u);
        let a14 = DAT_0070ab14;
        let u: u32 = rd(off(&raw mut DAT_0071b78d, i2));
        wr(off(&raw mut DAT_0071b799, i2), u);
        let c20 = DAT_0070ac20;
        wr(off(&raw mut DAT_0071b785, i2), a18);
        wr(off(&raw mut DAT_0071b789, i2), a14);
        wr(off(&raw mut DAT_0071b78d, i2), c20);
    }
}

unsafe fn adj(p: *mut u8, i13: i32, u16v: u32, s24: i16) -> i32 {
    // SAFETY: p points into the game's object record; the global is the game's own.
    unsafe {
        let s12: i16 =
            if 0u32.wrapping_sub(i13 as u32) == u16v || (0u32.wrapping_sub(u16v) as i32) < i13 {
                if i13 <= u16v as i32 { i13 as i16 } else { s24 }
            } else {
                s24.wrapping_neg()
            };
        let prod = (s12 as i32 as u32).wrapping_mul(DAT_0071e6be as u32) as i32;
        let mut i14 = prod / 0xec;
        if i14 == 0 {
            i14 = i13;
        }
        wr(p, (i13 as u32).wrapping_sub(i14 as u32));
        i14
    }
}

unsafe fn apply_vel(p_e: *mut u8, u19: u32) {
    // SAFETY: p_e is a game object record; callee and globals are the game's own.
    unsafe {
        let p_i = rp(off(p_e, 0x1f));
        let s: i16 = rd(off(p_i, 0x8a));
        DAT_006d4a50 = FUN_00413ca0(s as i32, u19) as i32;
        let p_i = rp(off(p_e, 0x1f));
        let s: i16 = rd(off(p_i, 0x8c));
        DAT_006ffbec = FUN_00413ca0(s as i32, u19) as i32;
        let p_i = rp(off(p_e, 0x1f));
        let s: i16 = rd(off(p_i, 0x8e));
        DAT_0072f4e8 = FUN_00413ca0(s as i32, u19) as i32;
        clamp_add(off(p_e, 4), DAT_006d4a50);
        clamp_add(off(p_e, 8), DAT_006ffbec);
        clamp_add(off(p_e, 0xc), DAT_0072f4e8);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "stdcall" fn FUN_004253a0() {
    // SAFETY: every global, table and callee is the game's own memory and code;
    // object pointers are read from the game's records where the original reads them.
    unsafe {
        let mut s_stack_1c: i16 = 0;
        let mut k: u16 = 0;
        loop {
            let mut uv = k as u32;
            let i2 = uv.wrapping_mul(0x5bd) as i32;
            if rd::<u8>(off(&raw mut DAT_0071b740, i2)) != 0 && DAT_0071ecf4 != uv {
                uv = rd(off(&raw mut DAT_0071b730, i2));
                if uv != 0xffffffff {
                    let a = gp(uv.wrapping_mul(0x23) as i32);
                    if rd::<u8>(off(a, 2)) == 0
                        && (DAT_0071e690 == 0xffffffff || DAT_0071e690 == uv)
                    {
                        upd(i2, uv);
                    }
                }
            }
            k = k.wrapping_add(1);
            if !(k < 8) {
                break;
            }
        }

        let u22 = DAT_0071ecf4 & 0xffff;
        let i2 = u22.wrapping_mul(0x5bd) as i32;
        let id: u32 = rd(off(&raw mut DAT_0071b730, i2));
        if id != 0xffffffff {
            let a = gp(id.wrapping_mul(0x23) as i32);
            if rd::<u8>(off(a, 2)) == 0 && (DAT_0071e690 == 0xffffffff || DAT_0071e690 == id) {
                upd(i2, id);
            }
        }

        let u_var28 = DAT_0071e6be;
        let s_var9 = DAT_0070a1d2;
        let mut b_var8 = false;
        let mut u_stack_1e: u16 = 0;
        if 0 < DAT_00732a38 {
            loop {
                if DAT_0071e690 != 0xffffffff {
                    if b_var8 {
                        DAT_0070a1d2 = s_var9;
                        DAT_0071e6be = u_var28;
                        return;
                    }
                    b_var8 = true;
                    u_stack_1e = DAT_0071e690 as u16;
                }
                'next: {
                    let mut u_var16 = u_stack_1e as u32;
                    let i_var17 = u_var16.wrapping_mul(0x23) as i32;
                    let p_e = gp(i_var17);
                    DAT_0070a1d2 = s_var9;
                    DAT_0071e6be = u_var28;
                    let mut p_i = rp(off(p_e, 0x1f));
                    if !p_i.is_null() {
                        let i18: i32 = rd(off(p_i, 2));
                        if i18 != 0 {
                            DAT_0071e6be =
                                u_var28.wrapping_add(
                                    (DAT_0071b71c as i16).wrapping_sub(i18 as i16) as u16,
                                );
                            if DAT_0071e6be != 0 {
                                let u19 = DAT_0071e6be as u32;
                                DAT_0070a1d2 = (0xecu32 / u19) as i16;
                                if DAT_0070a1d2 == 0 {
                                    DAT_0070a1d2 = 1;
                                }
                                let s32: i32 = rd(off(p_i, 2));
                                wr(off(p_i, 2), s32.wrapping_add(u19 as i32));
                            } else {
                                if DAT_0071e690 == 0xffffffff {
                                    let u: u32 = rd(off(p_e, 0x1b));
                                    if u != 0xffffffff {
                                        if DAT_007092c1 != 0 && DAT_0071ecf4 == u {
                                            FUN_00401000();
                                        }
                                        let s: i32 = rd(off(p_e, 4));
                                        wr(off(p_i, 6), s);
                                        let s: u32 = rd(off(p_e, 8));
                                        wr(off(p_i, 10), s);
                                        let s: i32 = rd(off(p_e, 0xc));
                                        wr(off(p_i, 0xe), s);
                                    }
                                }
                                break 'next;
                            }
                        }
                    }
                    if rd::<u8>(off(p_e, 3)) == 0 {
                        break 'next;
                    }
                    let u_var3: u8 = rd(off(p_e, 2));
                    let u6: u16 = rd(off(p_i, 0x7f));
                    let run = if u6 != 0 {
                        let mut u21 = u6.wrapping_sub(DAT_0071e6be);
                        if u6 < u21 {
                            u21 = 0;
                        }
                        wr(off(p_i, 0x7f), u21);
                        u21 == 0
                    } else {
                        false
                    };
                    if run {
                        match u_var3 {
                            0 => {
                                FUN_00421df0(
                                    ((s_stack_1c as u16 as u32) << 16) | u_stack_1e as u32,
                                    1,
                                );
                                let b10 = FUN_00478470();
                                FUN_00412700(u_var16, ((b10 & 1) as i32) + 0x7f);
                                FUN_0049f9c0(
                                    u_stack_1e as u32,
                                    u32::from(rd::<u8>(off(gp(i_var17), 0x16))),
                                    2,
                                );
                            }
                            1..=5 => {
                                let i18 = FUN_0044ad00(u32::from(rd::<u8>(off(p_e, 3))));
                                if i18 == 0 {
                                    FUN_0041a750(
                                        ((s_stack_1c as u16 as u32) << 16) | u_stack_1e as u32,
                                        1,
                                    );
                                    FUN_00412700(u_var16, 0x82);
                                    let pq = rp(off(gp(i_var17), 0x1f));
                                    let u: u16 = rd(off(
                                        &raw mut DAT_0052686c,
                                        (u32::from(rd::<u8>(off(p_e, 3))).wrapping_mul(0x16))
                                            as i32,
                                    ));
                                    wr(off(pq, 1), (u >> 9) as u8);
                                } else {
                                    FUN_00412700(u_var16, 0x82);
                                }
                                FUN_0049f9c0(
                                    u_stack_1e as u32,
                                    u32::from(rd::<u8>(off(gp(i_var17), 0x16))),
                                    2,
                                );
                            }
                            6 | 7 => {
                                if rd::<u8>(off(
                                    &raw mut DAT_00525bf7,
                                    i32::from(rd::<u8>(off(p_e, 3))),
                                )) != 0
                                {
                                    FUN_00412700(u_var16, -0x7f);
                                } else {
                                    wr(off(p_e, 3), 0u8);
                                    break 'next;
                                }
                            }
                            0xb => {
                                let b10 = FUN_00478470();
                                FUN_00412700(u_var16, (((b10 & 1) + 0x7f) as i8) as i32);
                            }
                            _ => {
                                wr(off(p_e, 3), 0u8);
                                break 'next;
                            }
                        }
                    }
                    // LAB_0042587d
                    if DAT_0071e690 == 0xffffffff {
                        if DAT_007092c1 != 0 && rd::<u32>(off(gp(i_var17), 0x1b)) == DAT_0071ecf4 {
                            FUN_00401000();
                        }
                        p_i = rp(off(p_e, 0x1f));
                        let s: i32 = rd(off(p_e, 4));
                        wr(off(p_i, 6), s);
                        let s: u32 = rd(off(p_e, 8));
                        wr(off(p_i, 10), s);
                        let s: i32 = rd(off(p_e, 0xc));
                        wr(off(p_i, 0xe), s);
                    }
                    p_i = rp(off(p_e, 0x1f));
                    let mut s_var24: i16 = rd(off(p_i, 0x77));
                    let mut p_s = off(p_i, 0x77);
                    if s_var24 != 0 {
                        let mut p_o: *mut u8 = core::ptr::null_mut();
                        let mut cond = false;
                        if (u_var16 as i32) < DAT_006d4980 {
                            let pq = rp(off(gp(i_var17), 0x1f));
                            p_o = rp(off(pq, 0xa7));
                            let s: i16 = rd(off(p_o, 0x70));
                            cond = s != -1;
                        }
                        if cond {
                            let mut clear = false;
                            if s_var24 < 0 {
                                s_stack_1c = 0xec;
                                let s16: i16 = (((DAT_0071e6be as u32) << 12) / 0xec) as i16;
                                let s16 = s16.wrapping_add(s_var24);
                                wr(p_s, s16);
                                p_i = rp(off(p_e, 0x1f));
                                p_s = off(p_i, 0x77);
                                let s16: i16 = rd(p_s);
                                if -1 < s16 {
                                    clear = true;
                                }
                            } else {
                                s_stack_1c = -0xec;
                                let q = (((DAT_0071e6be as u32) << 12) as i32) / -0xec;
                                let s16 = (q as i16).wrapping_add(s_var24);
                                wr(p_s, s16);
                                p_i = rp(off(p_e, 0x1f));
                                p_s = off(p_i, 0x77);
                                let s16: i16 = rd(p_s);
                                if s16 < 1 {
                                    clear = true;
                                }
                            }
                            if clear {
                                wr(p_s, 0i16);
                                p_i = rp(off(p_e, 0x1f));
                                s_var24 = rd(off(p_i, 0x77));
                                wr(off(p_o, 0x70), 0xffffu16);
                            }
                        }
                        let i13 = (s_var24 as i32 as u32).wrapping_mul(DAT_0071e6be as u32) as i32;
                        let s16: i16 = rd(off(p_e, 0x14));
                        let s16 = (s16 as i32)
                            .wrapping_add(((i13 / 0xec) as i16 as i32).wrapping_mul(4))
                            as i16;
                        wr(off(p_e, 0x14), s16);
                        p_i = rp(off(p_e, 0x1f));
                        wr(off(p_i, 0x90), 1u8);
                    }
                    let mut u_var19: u32 = 0;
                    p_i = rp(off(p_e, 0x1f));
                    let us16: u16 = rd(off(p_i, 0x79));
                    if us16 != 0 {
                        let t = ((us16 as u32).wrapping_mul(0x1234).wrapping_add(0x80) as i32) >> 8;
                        let i13 = (t as u32).wrapping_mul(DAT_0071e6be as u32) as i32;
                        u_var19 = ((i13 / 0xec) as u32) & 0xffff;
                    }
                    match u_var3 {
                        0..=4 => {
                            let p_o = rp(off(p_i, 0xa7));
                            let mut i_var13 = FUN_0042b150(p_o as usize as u32 as i32);
                            s_stack_1c = i_var13 as i16;
                            p_i = rp(off(p_e, 0x1f));
                            if rd::<u8>(off(p_i, 0x89)) != 0 {
                                let s16: i16 = rd(off(p_e, 0x12));
                                let s16b: i16 = rd(off(p_e, 0x10));
                                FUN_00483410(s16, s16b, p_e as usize as u32 as i32);
                            }
                            p_i = rp(off(p_e, 0x1f));
                            let s: i16 = rd(off(p_i, 0x8a));
                            DAT_006d4a50 = FUN_00413ca0(s as i32, u_var19) as i32;
                            p_i = rp(off(p_e, 0x1f));
                            let s: i16 = rd(off(p_i, 0x8c));
                            DAT_006ffbec = FUN_00413ca0(s as i32, u_var19) as i32;
                            p_i = rp(off(p_e, 0x1f));
                            let s: i16 = rd(off(p_i, 0x8e));
                            DAT_0072f4e8 = FUN_00413ca0(s as i32, u_var19) as i32;
                            let s16: i16 = rd(off(p_o, 0x1df));
                            if s16 != 0 {
                                let b34: i8 = rd(off(ip(i_var13), 0x34));
                                if b34 == 0x12 {
                                    u_var16 = 0xfa;
                                } else if b34 == 0x1e {
                                    u_var16 = 0x2ee;
                                } else {
                                    let o = (u32::from(rd::<u8>(off(p_o, 4))).wrapping_mul(0xef))
                                        as i32;
                                    let u: u16 = rd(off(&raw mut DAT_0052b09d, o));
                                    u_var16 = u as u32;
                                }
                                i_var13 = rd(off(p_o, 0xb1));
                                let s_var24 = u_var16 as i16;
                                if i_var13 != 0 {
                                    s_stack_1c = 0xec;
                                    let d = adj(off(p_o, 0xb1), i_var13, u_var16, s_var24);
                                    DAT_006d4a50 = DAT_006d4a50.wrapping_add(d);
                                }
                                i_var13 = rd(off(p_o, 0xb5));
                                if i_var13 != 0 {
                                    s_stack_1c = 0xec;
                                    let d = adj(off(p_o, 0xb5), i_var13, u_var16, s_var24);
                                    DAT_006ffbec = DAT_006ffbec.wrapping_add(d);
                                }
                                let u22: u32 = rd(off(p_o, 0xb9));
                                if u22 != 0 {
                                    if 0u32.wrapping_sub(u22) == u_var16
                                        || (0u32.wrapping_sub(u_var16) as i32) < (u22 as i32)
                                    {
                                        if (u22 as i32) <= (u_var16 as i32) {
                                            u_var16 = u22 & 0xffff;
                                        }
                                    } else {
                                        u_var16 = s_var24.wrapping_neg() as u16 as u32;
                                    }
                                    let i13 = ((u_var16 as i16 as i32 as u32)
                                        .wrapping_mul(DAT_0071e6be as u32))
                                        as i32;
                                    u_var16 = (i13 / 0xec) as u32;
                                    if u_var16 == 0 {
                                        u_var16 = u22;
                                    }
                                    wr(off(p_o, 0xb9), u22.wrapping_sub(u_var16));
                                    DAT_0072f4e8 = DAT_0072f4e8.wrapping_add(u_var16 as i32);
                                }
                            }
                            clamp_add(off(p_e, 4), DAT_006d4a50);
                            clamp_add(off(p_e, 8), DAT_006ffbec);
                            clamp_add(off(p_e, 0xc), DAT_0072f4e8);
                            let us16: u16 = rd(off(p_o, 0x66));
                            if us16 != 0xffff {
                                let i20 = (us16 as u32).wrapping_mul(0x23) as i32;
                                let mut p_t = gp(i20);
                                let mut p_q = rp(off(p_t, 0x1f));
                                if !p_q.is_null() {
                                    let pq = rp(off(p_q, 0xa7));
                                    let i13 =
                                        (u32::from(rd::<u8>(off(pq, 4))).wrapping_mul(0xef)) as i32;
                                    let o4 = (u32::from(rd::<u8>(off(p_o, 4))).wrapping_mul(0xef))
                                        as i32;
                                    if rd::<u8>(off(p_t, 2)) < 3 {
                                        let s16: i16 = rd(off(&raw mut DAT_0052b147, o4));
                                        let s16b: i16 = rd(off(&raw mut DAT_0052b143, i13));
                                        u_var16 =
                                            (s16 as i32).wrapping_sub(s16b as i32) as u16 as u32;
                                    } else if rd::<u8>(off(gp(i_var17), 2)) < 3 {
                                        let s16: i16 = rd(off(&raw mut DAT_0052b149, o4));
                                        let s16b: i16 = rd(off(&raw mut DAT_0052b143, i13));
                                        u_var16 =
                                            (s16 as i32).wrapping_sub(s16b as i32) as u16 as u32;
                                    } else {
                                        let s16: i16 = rd(off(&raw mut DAT_0052b149, o4));
                                        let s16b: i16 = rd(off(&raw mut DAT_0052b145, i13));
                                        u_var16 =
                                            (s16 as i32).wrapping_sub(s16b as i32) as u16 as u32;
                                    }
                                    let u: u16 = rd(off(&raw mut DAT_0052b141, i13));
                                    FUN_0042a230(
                                        p_e as usize as u32 as i32,
                                        0,
                                        (u_var16 as i16) as i32,
                                        u as i32,
                                    );
                                    p_t = gp(i20);
                                    p_q = rp(off(p_t, 0x1f));
                                    let s: i32 = rd(off(p_t, 4));
                                    wr(off(p_q, 6), s);
                                    p_q = rp(off(p_t, 0x1f));
                                    let s: i32 = rd(off(p_t, 8));
                                    wr(off(p_q, 10), s);
                                    let p_u = rp(off(p_t, 0x1f));
                                    let s: i32 = rd(off(p_t, 0xc));
                                    wr(off(p_u, 0xe), s);
                                    let s: i32 = rd(off(p_e, 4));
                                    wr(off(p_t, 4), s.wrapping_add(DAT_0070ab18));
                                    let s: i32 = rd(off(p_e, 8));
                                    wr(off(p_t, 8), s.wrapping_add(DAT_0070ab14));
                                    let s: i32 = rd(off(p_e, 0xc));
                                    wr(off(p_t, 0xc), s.wrapping_add(DAT_0070ac20));
                                }
                            }
                        }
                        6 | 7 => {
                            let p_p = rp(off(p_i, 0xa3));
                            let b10: u8 = rd(off(p_p, 1));
                            let us16: u16 = if b10 != 0 { rd(off(p_p, 4)) } else { 0xffff };
                            if b10 != 0 && us16 != 0xffff {
                                let u22 = us16 as u32;
                                if (u22 as i32) < DAT_00732a38
                                    && rd::<u8>(off(gp(3), u22.wrapping_mul(0x23) as i32)) == 0
                                {
                                    FUN_00412700(u_var16, 0x81);
                                    break 'next;
                                }
                                let u6: u16 = rd(off(p_p, 2));
                                let mut b_var7 = false;
                                let b4: u8 = rd(off(
                                    &raw mut DAT_005248c7,
                                    i32::from(rd::<u8>(off(p_e, 3))),
                                ));
                                if (u22 as i32) < DAT_006d4980 {
                                    let pq = rp(off(gp(u22.wrapping_mul(0x23) as i32), 0x1f));
                                    let p_q = rp(off(pq, 0xa7));
                                    let c: i8 = rd(off(p_q, 0x233));
                                    if c == 3 {
                                        let c2: i8 = rd(off(p_q, 0x237));
                                        if c2 != 0 {
                                            b_var7 = true;
                                        }
                                    }
                                }
                                if !b_var7 {
                                    FUN_00421c70(u22, 0);
                                    let mut alt = true;
                                    if (u22 as i32) < DAT_006d4980 {
                                        let i18 = u22.wrapping_mul(0x23) as i32;
                                        let i13 = DAT_006d4988.wrapping_add(i18);
                                        let pq = rp(off(ip(i13), 0x1f));
                                        if rd::<u8>(pq) == 0 {
                                            alt = false;
                                            let b5: u8 = rd(off(ip(i13), 3));
                                            if b5 == 0x35 {
                                                let u22b: u32;
                                                if u6 == 0xffff {
                                                    DAT_0070ab18 = FUN_00449b30(0x35, 0);
                                                    DAT_0070ab14 = FUN_00449cb0(
                                                        u32::from(rd::<u8>(off(gp(3), i18))),
                                                        0,
                                                    );
                                                    u22b = 0;
                                                } else {
                                                    u22b = u6 as u32;
                                                    DAT_0070ab18 = FUN_00449b30(0x35, u22b);
                                                    DAT_0070ab14 = FUN_00449cb0(
                                                        u32::from(rd::<u8>(off(gp(3), i18))),
                                                        u22b,
                                                    );
                                                }
                                                let i13b = FUN_00449bf0(
                                                    u32::from(rd::<u8>(off(gp(3), i18))),
                                                    u22b,
                                                );
                                                DAT_0070ab14 = DAT_0070ab14 >> 1;
                                                DAT_0070ab18 = DAT_0070ab18 >> 1;
                                                DAT_0070ac20 =
                                                    (0u32.wrapping_sub(i13b as u32) as i32) >> 1;
                                                FUN_0042a230(
                                                    DAT_006d4988.wrapping_add(i18),
                                                    (DAT_0070ab18 as i16) as i32,
                                                    (DAT_0070ab14 as i16) as i32,
                                                    (DAT_0070ac20 as i16) as i32,
                                                );
                                                DAT_0070ab18 = DAT_0070ab18.wrapping_mul(2);
                                                DAT_0070ab14 = DAT_0070ab14.wrapping_mul(2);
                                                DAT_0070ac20 = DAT_0070ac20.wrapping_mul(2);
                                            } else {
                                                let s24: i16;
                                                let s12: i16;
                                                let u22b: u32;
                                                if u6 == 0xffff {
                                                    let u15 = FUN_00449bf0(b5 as u32, 0) as u32;
                                                    s24 = (u15 as i16).wrapping_neg();
                                                    let u15 = FUN_00449cb0(b5 as u32, 0) as u32;
                                                    s12 = u15 as i16;
                                                    u22b = 0;
                                                } else {
                                                    u22b = u6 as u32;
                                                    let u15 = FUN_00449bf0(b5 as u32, u22b) as u32;
                                                    s24 = (u15 as i16).wrapping_neg();
                                                    let u15 = FUN_00449cb0(b5 as u32, u22b) as u32;
                                                    s12 = u15 as i16;
                                                }
                                                let u15 = FUN_00449b30(b5 as u32, u22b) as u32;
                                                FUN_0042a230(
                                                    i13,
                                                    (u15 as i16) as i32,
                                                    s12 as i32,
                                                    s24 as i32,
                                                );
                                            }
                                            DAT_0070ab18 = DAT_0070ab18.wrapping_add(DAT_00732a2c);
                                            DAT_0070ab14 = DAT_0070ab14.wrapping_add(DAT_00732a28);
                                            DAT_0070ac20 = DAT_0070ac20.wrapping_add(DAT_00732a24);
                                        }
                                    }
                                    if alt {
                                        DAT_0070ab18 = DAT_00732a2c;
                                        DAT_0070ab14 = DAT_00732a28;
                                        DAT_0070ac20 = DAT_00732a24;
                                    }
                                    FUN_00421c70(u_var16, 0);
                                    DAT_0070ab18 = DAT_0070ab18.wrapping_sub(DAT_00732a2c);
                                    DAT_0070ab14 = DAT_0070ab14.wrapping_sub(DAT_00732a28);
                                    DAT_0070ac20 = DAT_0070ac20.wrapping_sub(DAT_00732a24);
                                    FUN_00418cd0(DAT_0070ab18, DAT_0070ab14, DAT_0070ac20);
                                    let s16: i16 = rd(off(p_e, 0x10));
                                    let mut s26 = DAT_006da9f0.wrapping_sub(s16);
                                    let i18 = (((b10 as u16).wrapping_add(b4 as u16)) as u32)
                                        .wrapping_mul(2)
                                        as i32;
                                    let u: u16 = rd(off(&raw mut DAT_00524968, i18));
                                    u_var16 =
                                        (((u as u32).wrapping_mul(DAT_0071e6be as u32) as i32)
                                            / 0xec) as u32;
                                    let mut s12 = u_var16 as i16;
                                    let mut s24 = s26;
                                    if s26 < 0 {
                                        s24 = s26.wrapping_neg();
                                    }
                                    if ((u_var16 & 0xffff) as i32) < (s24 as i32) {
                                        if s26 < 0 {
                                            s12 = s12.wrapping_neg();
                                        }
                                        let s16: i16 = rd(off(p_e, 0x10));
                                        wr(off(p_e, 0x10), s16.wrapping_add(s12));
                                        p_i = rp(off(p_e, 0x1f));
                                        let mut p_s = off(p_i, 0x79);
                                        let mut us: u16 = rd(p_s);
                                        if 200 < us {
                                            let u: u16 = rd(off(&raw mut DAT_005249c0, i18));
                                            let q = ((u as u32).wrapping_mul(DAT_0071e6be as u32)
                                                as i32)
                                                / -0xec;
                                            us = us.wrapping_add(q as i16 as u16);
                                            wr(p_s, us);
                                            p_i = rp(off(p_e, 0x1f));
                                            p_s = off(p_i, 0x79);
                                            us = rd(p_s);
                                            if us < 200 {
                                                us = 200;
                                                wr(p_s, us);
                                            }
                                        }
                                    } else {
                                        wr(off(p_e, 0x10), DAT_006da9f0);
                                        p_i = rp(off(p_e, 0x1f));
                                        let p_s = off(p_i, 0x79);
                                        let us: u16 = rd(off(p_i, 0x79));
                                        let u: u16 = rd(off(p_p, 6));
                                        if us < u {
                                            let s16: i16 = rd(p_s);
                                            let u: u16 = rd(off(&raw mut DAT_005249c0, i18));
                                            let q = ((u as u32).wrapping_mul(DAT_0071e6be as u32)
                                                as i32)
                                                / 0xec;
                                            wr(p_s, s16.wrapping_add(q as i16));
                                        }
                                    }
                                    s_stack_1c = rd(off(p_e, 0x12));
                                    s26 = DAT_00732e40.wrapping_sub(s_stack_1c);
                                    let u: u16 = rd(off(&raw mut DAT_00524968, i18));
                                    u_var16 =
                                        (((u as u32).wrapping_mul(DAT_0071e6be as u32) as i32)
                                            / 0xec) as u32;
                                    let mut s12 = u_var16 as i16;
                                    let mut s24 = s26;
                                    if s26 < 0 {
                                        s24 = s26.wrapping_neg();
                                    }
                                    if ((u_var16 & 0xffff) as i32) < (s24 as i32) {
                                        if s26 < 0 {
                                            s12 = s12.wrapping_neg();
                                        }
                                        wr(off(p_e, 0x12), s_stack_1c.wrapping_add(s12));
                                    } else {
                                        wr(off(p_e, 0x12), DAT_00732e40);
                                    }
                                    p_i = rp(off(p_e, 0x1f));
                                    wr(off(p_i, 0x90), 1u8);
                                    let v: u8 = rd(off(p_i, 0x90));
                                    wr(off(p_i, 0x89), v);
                                    let s16: i16 = rd(off(p_e, 0x12));
                                    let s16b: i16 = rd(off(p_e, 0x10));
                                    FUN_00483410(s16, s16b, p_e as usize as u32 as i32);
                                    p_i = rp(off(p_e, 0x1f));
                                    wr(off(p_i, 0x8a), DAT_006db720 as i16);
                                    p_i = rp(off(p_e, 0x1f));
                                    wr(off(p_i, 0x8c), DAT_006db704 as i16);
                                    p_i = rp(off(p_e, 0x1f));
                                    wr(off(p_i, 0x8e), DAT_006db708 as i16);
                                }
                            }
                            // LAB_00426241
                            p_i = rp(off(p_e, 0x1f));
                            if rd::<u8>(off(p_i, 0x89)) != 0 {
                                let s16: i16 = rd(off(p_e, 0x12));
                                let s16b: i16 = rd(off(p_e, 0x10));
                                FUN_00483410(s16, s16b, p_e as usize as u32 as i32);
                            }
                            apply_vel(p_e, u_var19);
                        }
                        0xb | 0xd => {
                            if rd::<u8>(off(p_i, 0x89)) != 0 {
                                let s16: i16 = rd(off(p_e, 0x12));
                                let s16b: i16 = rd(off(p_e, 0x10));
                                FUN_00483410(s16, s16b, p_e as usize as u32 as i32);
                            }
                            apply_vel(p_e, u_var19);
                        }
                        _ => {}
                    }
                }
                u_stack_1e = u_stack_1e.wrapping_add(1);
                if !((u_stack_1e as i32) < DAT_00732a38) {
                    break;
                }
                continue;
            }
        }
        DAT_0070a1d2 = s_var9;
        DAT_0071e6be = u_var28;
    }
}
