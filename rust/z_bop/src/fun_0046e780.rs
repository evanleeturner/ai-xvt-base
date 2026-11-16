// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052702c: i32;
    static mut DAT_00527028: u32;
    static mut DAT_0099f6e0: u32;
    static mut DAT_0099f6e8: u32;
    static mut DAT_0099f6ec: u32;
    static mut DAT_0099f6f8: u8;
    static mut DAT_005599a8: u32;
    static mut DAT_005599ac: u32;
    static mut DAT_005599b0: u32;
    static mut DAT_005599b4: i8;
    static mut DAT_005599b5: i8;
    static mut DAT_005599b6: u8;
    static mut DAT_005599b7: i8;
    static mut DAT_005599b8: i32;
    static mut DAT_005599bc: u32;
    static mut DAT_00999bec: u8;
    static mut DAT_00999bf4: u8;
    static mut DAT_00999bfc: u8;
    static mut DAT_00999e0c: u32;
    static mut DAT_00999e10: u32;
    static mut DAT_00999e18: u8;
    static mut _DAT_0060f1bc: u32;
    static mut DAT_00523418: i32;
    static mut DAT_00999524: i32;
    static mut DAT_00bb736f: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046c830();
    fn FUN_0046fae0();
    fn FUN_0046fee0(param_1: u32) -> u32;
    fn FUN_0046fce0(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: u32) -> u32;
    fn FUN_0046f470(param_1: i32, param_2: *mut u32, param_3: i32);
    fn FUN_0046c220(param_1: *const i8);
    fn FUN_0046fdf0(param_1: u32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

unsafe fn rec(i: u32) -> *mut u32 {
    // SAFETY: address arithmetic inside the game's own record table.
    unsafe { (&raw mut DAT_005599a8).add(i.wrapping_mul(0x84) as usize) }
}

unsafe fn rec_ac(i: u32) -> *mut u32 {
    // SAFETY: address arithmetic inside the game's own record table.
    unsafe { (&raw mut DAT_005599ac).add(i.wrapping_mul(0x84) as usize) }
}

unsafe fn rec_b0(i: u32) -> *mut u32 {
    // SAFETY: address arithmetic inside the game's own record table.
    unsafe { (&raw mut DAT_005599b0).add(i.wrapping_mul(0x84) as usize) }
}

unsafe fn rec_b8(i: u32) -> *mut i32 {
    // SAFETY: address arithmetic inside the game's own record table.
    unsafe { (&raw mut DAT_005599b8).add(i.wrapping_mul(0x84) as usize) }
}

unsafe fn rec_bc(i: u32) -> *mut u32 {
    // SAFETY: address arithmetic inside the game's own record table.
    unsafe { (&raw mut DAT_005599bc).add(i.wrapping_mul(0x84) as usize) }
}

unsafe fn fb4(off: i32) -> *mut i8 {
    // SAFETY: byte offset inside the game's own record table.
    unsafe { (&raw mut DAT_005599b4).offset(off as isize) }
}

unsafe fn fb5(off: i32) -> *mut i8 {
    // SAFETY: byte offset inside the game's own record table.
    unsafe { (&raw mut DAT_005599b5).offset(off as isize) }
}

unsafe fn fb6(off: i32) -> *mut u8 {
    // SAFETY: byte offset inside the game's own record table.
    unsafe { (&raw mut DAT_005599b6).offset(off as isize) }
}

unsafe fn fb7(off: i32) -> *mut i8 {
    // SAFETY: byte offset inside the game's own record table.
    unsafe { (&raw mut DAT_005599b7).offset(off as isize) }
}

unsafe fn e0c(i: u32) -> *mut u32 {
    // SAFETY: address arithmetic inside the game's own player table.
    unsafe { (&raw mut DAT_00999e0c).add(i.wrapping_mul(0x8e) as usize) }
}

unsafe fn e10(i: u32) -> *mut u32 {
    // SAFETY: address arithmetic inside the game's own player table.
    unsafe { (&raw mut DAT_00999e10).add(i.wrapping_mul(0x8e) as usize) }
}

unsafe fn e18(off: u32) -> *mut i32 {
    // SAFETY: byte offset inside the game's own player table.
    unsafe { (&raw mut DAT_00999e18).byte_add(off as usize).cast::<i32>() }
}

unsafe fn seq_ptr(off: u32, u4: u32, u5: u32) -> *mut u32 {
    // SAFETY: byte offset inside the game's own player table.
    unsafe {
        let base = if u4 != 0 {
            &raw mut DAT_00999bf4
        } else if u5 != 0 {
            &raw mut DAT_00999bfc
        } else {
            &raw mut DAT_00999bec
        };
        base.byte_add(off as usize).cast::<u32>()
    }
}

unsafe fn seq_get(off: u32, u4: u32, u5: u32) -> i32 {
    // SAFETY: reads the game's own player table.
    unsafe { seq_ptr(off, u4, u5).read_unaligned() as i32 }
}

unsafe fn seq_set(off: u32, u4: u32, u5: u32, v: u32) {
    // SAFETY: writes the game's own player table.
    unsafe { seq_ptr(off, u4, u5).write_unaligned(v) }
}

unsafe fn copy_rec(src: *const u32) {
    // SAFETY: copies one 0x84-dword record into the game's receive buffer.
    unsafe {
        let mut s = src;
        let mut d = &raw mut DAT_0099f6e8;
        let mut n = 0x84;
        while n != 0 {
            *d = *s;
            s = s.add(1);
            d = d.add(1);
            n -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046e780(param_1: *mut u32, param_2: *mut u32) -> *mut u8 {
    // SAFETY: the callees and globals are the game's own; param_1 and
    // param_2 are the caller's output slots; the scratch arrays are local.
    unsafe {
        FUN_0046c830();
        FUN_0046fae0();
        if DAT_0052702c == 0 {
            return core::ptr::null_mut();
        }
        let mut ab2c8 = [0u8; 40];
        let mut ab2f0 = [0u8; 40];
        let mut ab228 = [0x5au8; 40];
        let mut ab2a0 = [0u8; 120];
        let mut us200 = [0u32; 3];
        let p2c8 = ab2c8.as_mut_ptr();
        let p2f0 = ab2f0.as_mut_ptr();
        let p228 = ab228.as_mut_ptr();
        let p2a0 = ab2a0.as_mut_ptr();

        let mut us32c: u32 = DAT_00527028;
        let mut us328: u32 = 0;
        let mut us314: u32 = 0;
        let mut is310: i32;
        let mut u17: u32 = DAT_0099f6e0;
        let mut i13: i32 = DAT_0052702c;

        if (DAT_0099f6e0 as i32) > 0 {
            let mut u4 = DAT_0099f6e0;
            let mut p14 = &raw mut DAT_00999bf4;
            let mut p15 = p2a0;
            loop {
                *p15 = *p14;
                u4 = u4.wrapping_sub(1);
                *p15.add(1) = *p14.wrapping_sub(8);
                *p15.add(2) = *p14.wrapping_add(8);
                p14 = p14.wrapping_add(0x238);
                p15 = p15.add(3);
                if u4 == 0 {
                    break;
                }
            }
        }

        loop {
            DAT_0099f6e0 = u17;
            if i13 < 1 {
                u17 = DAT_00527028;
                i13 = DAT_0052702c;
                if 0x3fe < DAT_0052702c {
                    while 0 < i13 {
                        let i18: i32 = u17.wrapping_mul(0x210) as i32;
                        if *rec(u17) == 0 {
                            u17 = u17.wrapping_add(1);
                        } else {
                            let c2 = *fb5(i18);
                            let a4 = (c2 == 0) as u32;
                            let a5 = (c2 == 2) as u32;
                            let u4 = *fb6(i18) as u32;
                            let u10 = FUN_0046fee0(*rec(u17));
                            if u10 < DAT_0099f6e0 && u10 < 0x28 {
                                let v = seq_get(u10.wrapping_mul(0x238), a4, a5);
                                us328 = (v as u32).wrapping_add(1);
                                if (us328 as i32) > 0x7f {
                                    us328 = 0;
                                }
                            }
                            let i11 = u4.wrapping_sub(us328) as i32;
                            if u10 < DAT_0099f6e0 && (i11 < -0x1b || (-1 < i11 && i11 < 100)) {
                                if *fb4(i18) != 0 {
                                    FUN_0046c220(b"(NMR) \0".as_ptr().cast());
                                    seq_set(u10.wrapping_mul(0x238), a4, a5, u4);
                                    _DAT_0060f1bc = u4;
                                    *e10(u10) = (*e10(u10)).wrapping_add(1);
                                    copy_rec(rec(u17));
                                    FUN_0046fdf0(u17);
                                    *param_1 = DAT_0099f6e8;
                                    *param_2 = DAT_0099f6ec;
                                    return &raw mut DAT_0099f6f8;
                                }
                                u17 = u17.wrapping_add(1);
                            } else {
                                let r = FUN_0046fdf0(u17);
                                if r != 0 {
                                    u17 = u17.wrapping_add(1);
                                }
                            }
                        }
                        if (u17 as i32) > 0x3ff {
                            u17 = 0;
                        }
                        i13 = i13.wrapping_sub(1);
                    }
                }
                return core::ptr::null_mut();
            }

            let i18: i32 = us32c.wrapping_mul(0x210) as i32;
            let p20: *mut i32 = rec(us32c).cast::<i32>();
            'tail: {
                'joined: {
                    if *rec(us32c) == 0 {
                        if DAT_00527028 == us32c {
                            copy_rec(rec(us32c));
                            *param_1 = *rec(us32c);
                            *param_2 = *rec_ac(us32c);
                            DAT_00527028 = DAT_00527028.wrapping_add(1);
                            DAT_0052702c = DAT_0052702c.wrapping_sub(1);
                            if (DAT_00527028 as i32) > 0x3ff {
                                DAT_00527028 = 0;
                            }
                            return &raw mut DAT_0099f6f8;
                        }
                        break 'joined;
                    }
                    let mut b12: u8 = *fb6(i18);
                    let u10: u32 = b12 as u32;
                    let u4: u32 = (*fb5(i18) == 0) as u32;
                    let u5: u32 = (*fb5(i18) == 2) as u32;
                    let u6: u32 = FUN_0046fee0(*rec(us32c));
                    let off6: u32 = u6.wrapping_mul(0x238);
                    if u17 != DAT_0099f6e0 && u6 < 0x28 {
                        let k = u6.wrapping_mul(3) as usize;
                        *p2a0.add(k) = 0x7f;
                        *p2a0.add(k + 1) = 0x7f;
                        *p2a0.add(k + 2) = 0x7f;
                    }
                    if u6 < DAT_0099f6e0 {
                        if *p228.add(u6 as usize) < *p2f0.add(u6 as usize) {
                            break 'joined;
                        }
                        if *fb7(i18) == 0 {
                            *p2f0.add(u6 as usize) = (*p2f0.add(u6 as usize)).wrapping_add(1);
                        }
                        us328 = (seq_get(off6, u4, u5) as u32).wrapping_add(1);
                        if (us328 as i32) > 0x7f {
                            us328 = 0;
                        }
                        let k = if u4 != 0 {
                            0
                        } else if u5 != 0 {
                            2
                        } else {
                            1
                        };
                        let bv1 = *p2a0.add(u6.wrapping_mul(3) as usize + k);
                        us314 = (bv1 as u32).wrapping_add(1);
                        if 0x7f < us314 {
                            us314 = 0;
                        }
                    }
                    let i11: i32 = *rec_b8(us32c);
                    let mut i7: i32 = u10.wrapping_sub(us328) as i32;
                    if u6 < DAT_0099f6e0 && (i7 < -0x1c || (-1 < i7 && i7 < 100)) {
                        if us328 == u10 && us314 == us328 {
                            *e0c(u6) = timeGetTime();
                            seq_set(off6, u4, u5, u10);
                            if i11 != 1 {
                                _DAT_0060f1bc = u10;
                                *e10(u6) = (*e10(u6)).wrapping_add(1);
                            } else {
                                _DAT_0060f1bc = u10;
                                if DAT_00bb736f != 1 {
                                    _DAT_0060f1bc = u10;
                                    *e10(u6) = (*e10(u6)).wrapping_add(1);
                                }
                            }
                            copy_rec(rec(us32c));
                            FUN_0046fdf0(us32c);
                            *param_1 = DAT_0099f6e8;
                            *param_2 = DAT_0099f6ec;
                            return &raw mut DAT_0099f6f8;
                        }
                        if i11 == 1 && DAT_00bb736f == 1 {
                            *e0c(u6) = timeGetTime();
                            seq_set(off6, u4, u5, u10);
                            _DAT_0060f1bc = u10;
                            copy_rec(rec(us32c));
                            FUN_0046fdf0(us32c);
                            *param_1 = DAT_0099f6e8;
                            *param_2 = DAT_0099f6ec;
                            return &raw mut DAT_0099f6f8;
                        }
                        if *fb7(i18) == 0 {
                            i7 = 0;
                            if u4 == 0 {
                                i7 = 2 - (u5 == 0) as i32;
                            }
                            i7 = (u6.wrapping_mul(3) as i32).wrapping_add(i7);
                            u17 = (*p2a0.offset(i7 as isize) as u32).wrapping_add(1);
                            *p2a0.offset(i7 as isize) = b12;
                            if 0x7f < u17 {
                                u17 = 0;
                            }
                            is310 = u10.wrapping_sub(u17) as i32;
                            if is310 < 0 {
                                is310 = is310.wrapping_add(0x80);
                            }
                            let mut b3: u8 = 0;
                            i7 = (is310 as u32).wrapping_mul(DAT_00523418 as u32) as i32;
                            if *p2f0.add(u6 as usize) < 0x7f {
                                *p2f0.add(u6 as usize) =
                                    (*p2f0.add(u6 as usize)).wrapping_add(is310 as u8);
                            }
                            let mut u16v: u32 = u17;
                            if *p2c8.add(u6 as usize) < 0x1a {
                                while u16v != u10 {
                                    let u8v = FUN_0046fce0(us32c, u16v, u4, u5, u6);
                                    if (u8v as i32) < 0x400 && -1 < (u8v as i32) {
                                        if u16v == us328 {
                                            *e0c(u6) = timeGetTime();
                                            copy_rec(rec(u8v));
                                            FUN_0046fdf0(u8v);
                                            seq_set(off6, u4, u5, us328);
                                            _DAT_0060f1bc = us328;
                                            *e10(u6) = (*e10(u6)).wrapping_add(1);
                                            *param_1 = DAT_0099f6e8;
                                            *param_2 = DAT_0099f6ec;
                                            if is310 < 2 {
                                                *fb4(i18) = 0;
                                                *rec_b0(us32c) = 0;
                                            }
                                            return &raw mut DAT_0099f6f8;
                                        }
                                        is310 = is310.wrapping_sub(1);
                                    } else {
                                        'rto: {
                                            let c2 = *fb4(i18);
                                            *p2c8.add(u6 as usize) =
                                                (*p2c8.add(u6 as usize)).wrapping_add(1);
                                            if c2 == 0 {
                                                us200[2] = 0;
                                                if u4 == 0 {
                                                    us200[2] = 2u32.wrapping_sub((u5 == 0) as u32);
                                                }
                                                let i23: i32;
                                                if i11 == 2 {
                                                    us200[0] = 0x33;
                                                    i23 = *p20;
                                                    us200[1] = (*rec_bc(us32c) & 0x7fffffff)
                                                        .wrapping_sub(i7 as u32);
                                                    us200[2] = u16v;
                                                } else {
                                                    i23 = *p20;
                                                    us200[0] = 0x38;
                                                    us200[1] = u16v;
                                                }
                                                FUN_0046f470(i23, us200.as_mut_ptr(), 0xc);
                                                let p = e18(off6);
                                                p.write_unaligned(
                                                    p.read_unaligned().wrapping_add(1),
                                                );
                                            } else {
                                                let dv = timeGetTime();
                                                let tmo: u32;
                                                if DAT_00999524 == 0 {
                                                    b12 = ((0i32.wrapping_sub((i11 == 2) as i32)
                                                        as u32
                                                        & 2)
                                                    .wrapping_add(3))
                                                        as u8;
                                                    tmo = 1000u32.wrapping_shl(
                                                        (*fb4(i18) as i32 as u32) & 0x1f,
                                                    );
                                                } else {
                                                    b12 = 0;
                                                    tmo = (((i11 == 2) as u32)
                                                        .wrapping_mul(0xffffffff)
                                                        & 37000)
                                                        .wrapping_add(3000);
                                                }
                                                if dv.wrapping_sub(*rec_b0(us32c)) <= tmo {
                                                    break 'rto;
                                                }
                                                FUN_0046c220(b"(RTO) \0".as_ptr().cast());
                                                if b12 < (*fb4(i18) as u8) {
                                                    FUN_0046c220(b"(TMR) \0".as_ptr().cast());
                                                    *e0c(u6) = timeGetTime();
                                                    loop {
                                                        u16v = FUN_0046fce0(us32c, u17, u4, u5, u6);
                                                        if !((u16v as i32) < 0
                                                            || (0x400 < (u16v as i32)))
                                                        {
                                                            break;
                                                        }
                                                        if u17 == u10 {
                                                            seq_set(off6, u4, u5, u10);
                                                            _DAT_0060f1bc = u10;
                                                            *e10(u6) = (*e10(u6)).wrapping_add(1);
                                                            copy_rec(rec(us32c));
                                                            FUN_0046fdf0(us32c);
                                                            *param_1 = DAT_0099f6e8;
                                                            *param_2 = DAT_0099f6ec;
                                                            return &raw mut DAT_0099f6f8;
                                                        }
                                                        u17 = u17.wrapping_add(1);
                                                        if (u17 as i32) > 0x7f {
                                                            u17 = 0;
                                                        }
                                                    }
                                                    copy_rec(rec(u16v));
                                                    FUN_0046fdf0(u16v);
                                                    seq_set(off6, u4, u5, u17);
                                                    _DAT_0060f1bc = u17;
                                                    *e10(u6) = (*e10(u6)).wrapping_add(1);
                                                    *param_1 = DAT_0099f6e8;
                                                    *param_2 = DAT_0099f6ec;
                                                    return &raw mut DAT_0099f6f8;
                                                }
                                                us200[2] = 0;
                                                if u4 == 0 {
                                                    us200[2] = 2u32.wrapping_sub((u5 == 0) as u32);
                                                }
                                                let i23: i32;
                                                if i11 == 2 {
                                                    us200[0] = 0x33;
                                                    i23 = *p20;
                                                    us200[1] = (*rec_bc(us32c) & 0x7fffffff)
                                                        .wrapping_sub(i7 as u32);
                                                    us200[2] = u16v;
                                                } else {
                                                    i23 = *p20;
                                                    us200[0] = 0x38;
                                                    us200[1] = u16v;
                                                }
                                                FUN_0046f470(i23, us200.as_mut_ptr(), 0xc);
                                            }
                                            b3 = 1;
                                        }
                                    }
                                    i7 = i7.wrapping_sub(DAT_00523418);
                                    u16v = u16v.wrapping_add(1);
                                    if (u16v as i32) > 0x7f {
                                        u16v = 0;
                                    }
                                }
                                if is310 < 1 {
                                    *fb4(i18) = 0;
                                    *rec_b0(us32c) = 0;
                                } else if b3 != 0 {
                                    *fb4(i18) = (*fb4(i18)).wrapping_add(1);
                                    *rec_b0(us32c) = timeGetTime();
                                }
                            }
                        } else if DAT_00527028 == us32c && FUN_0046fdf0(us32c) == 0 {
                            break 'tail;
                        }
                        break 'joined;
                    }
                    let r = FUN_0046fdf0(us32c);
                    if r != 0 {
                        break 'joined;
                    }
                    break 'tail;
                }
                us32c = us32c.wrapping_add(1);
                if (us32c as i32) > 0x3ff {
                    us32c = 0;
                }
            }
            u17 = DAT_0099f6e0;
            i13 = i13.wrapping_sub(1);
        }
    }
}
