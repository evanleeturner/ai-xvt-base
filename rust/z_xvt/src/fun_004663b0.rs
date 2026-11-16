// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525544: i32;
    static mut DAT_007091ae: u16;
    static mut DAT_006a888e: i32;
    static mut DAT_006a882a: *mut u32;
    static mut DAT_006a882e: *mut u32;
}

#[inline(always)]
unsafe fn ri(p: *mut u32, i: usize) -> i32 {
    // SAFETY: p points at the game's record words, read where the original reads.
    unsafe { p.wrapping_add(i).cast::<i32>().read_unaligned() }
}
#[inline(always)]
unsafe fn wi(p: *mut u32, i: usize, v: i32) {
    // SAFETY: p points at the game's record words, written where the original writes.
    unsafe { p.wrapping_add(i).cast::<i32>().write_unaligned(v) }
}
#[inline(always)]
unsafe fn ru(p: *mut u32, i: usize) -> u32 {
    // SAFETY: as above.
    unsafe { p.wrapping_add(i).read_unaligned() }
}
#[inline(always)]
unsafe fn rf(p: *mut u32, i: usize) -> f32 {
    // SAFETY: as above.
    unsafe { p.wrapping_add(i).cast::<f32>().read_unaligned() }
}
#[inline(always)]
unsafe fn wf(p: *mut u32, i: usize, v: f32) {
    // SAFETY: as above.
    unsafe { p.wrapping_add(i).cast::<f32>().write_unaligned(v) }
}
#[inline(always)]
unsafe fn rp(p: *mut u32) -> *mut u32 {
    // SAFETY: as above; the first word of a record is its next pointer.
    unsafe { p.read_unaligned() as usize as *mut u32 }
}
#[inline(always)]
unsafe fn wp(p: *mut u32, v: *mut u32) {
    // SAFETY: as above.
    unsafe { p.write_unaligned(v as usize as u32) }
}
#[inline(always)]
unsafe fn rd(p: *const f32) -> f32 {
    // SAFETY: p points at a float in the game's records.
    unsafe { p.read_unaligned() }
}
#[inline(always)]
unsafe fn rdi(a: i32) -> i32 {
    // SAFETY: a is an address inside the game's records.
    unsafe { (a as *const i32).read_unaligned() }
}
#[inline(always)]
unsafe fn pf(b: i32, o: i32) -> f32 {
    // SAFETY: b + o is a float field of the game's records.
    unsafe { (b.wrapping_add(o) as *const f32).read_unaligned() }
}
#[inline(always)]
unsafe fn slot(s: i32, p3: u32) -> *mut u32 {
    // SAFETY: s is a game record holding a slot table and its base index.
    unsafe {
        let base = rdi(s.wrapping_add(0x68)) as u32;
        let idx = p3
            .wrapping_sub(rdi(s.wrapping_add(0x40)) as u32)
            .wrapping_mul(4);
        base.wrapping_add(idx) as usize as *mut u32
    }
}
#[inline(always)]
unsafe fn clr(s: i32, p3: u32) {
    // SAFETY: the slot lies in the record's own table.
    unsafe { slot(s, p3).write_unaligned(0) }
}
#[inline(always)]
unsafe fn tbl_get(p3: u32) -> *mut u32 {
    // SAFETY: the table base is the game's global; the entry is the game's.
    unsafe {
        let t: i32 = (&raw const DAT_006a888e).read_unaligned();
        let a = (t as u32).wrapping_add(p3.wrapping_mul(4)) as usize as *const u32;
        a.read_unaligned() as usize as *mut u32
    }
}
#[inline(always)]
unsafe fn tbl_set(p3: u32, v: *mut u32) {
    // SAFETY: as above.
    unsafe {
        let t: i32 = (&raw const DAT_006a888e).read_unaligned();
        let a = (t as u32).wrapping_add(p3.wrapping_mul(4)) as usize as *mut u32;
        a.write_unaligned(v as usize as u32)
    }
}
#[inline(always)]
fn f2i(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_unsafe,
    unreachable_code
)]
pub unsafe extern "C" fn FUN_004663b0(param_1: f32, param_2: f32, param_3: u32, param_4: i32) {
    // SAFETY: all memory touched is the game's own records and globals,
    // accessed exactly where the original accesses it.
    unsafe {
        let null: *mut u32 = core::ptr::null_mut();
        let mut i9: i32 = 0;
        let mut i10: i32 = 0;
        let mut i11: i32 = 0;
        let mut i12: i32 = 0;
        let mut i14: i32 = 0;
        let mut i16: i32 = 0;
        let mut i18: i32 = 0;
        let mut u13: u32;
        let mut p15: *mut u32 = null;
        let mut p17: *mut u32 = null;
        let mut p19: *mut u32 = null;
        let mut p20: *mut u32 = null;
        let mut p21: *mut u32 = null;
        let mut p22: *mut u32 = null;
        let mut pi1: *mut u32 = null;
        let mut pi2: *mut u32 = null;
        let mut pi3: *mut u32 = null;
        let mut pf4: *const f32 = core::ptr::null();
        let mut f6: f32;
        let mut f7: f32;
        let mut f8: f32;
        let mut local_28: *mut u32 = null;
        let mut local_24: i32;
        let mut lc: u32;

        macro_rules! unl1 {
            () => {{
                clr(ri(pi3, 0), param_3);
                if p20.is_null() {
                    tbl_set(param_3, rp(p21));
                    local_28 = p20;
                } else {
                    wp(p20, rp(p21));
                    local_28 = p20;
                }
            }};
        }
        macro_rules! unl2 {
            () => {{
                wp(local_28, rp(p21));
                p20 = rp(p21);
                p21 = local_28;
            }};
        }
        macro_rules! reins {
            () => {{
                p20 = rp(p21);
                if !p20.is_null() && ri(p20, 1) < ri(pi1, 0) {
                    wp(local_28, p20);
                    i9 = ri(pi1, 0);
                    if i9 < ri(p20, 2) {
                        i16 = ru(p20, 2).wrapping_sub(i9 as u32) as i32;
                        wi(pi1, 0, i9.wrapping_add(i16));
                        wf(p21, 3, (i16 as f32) * rf(p21, 4) + rf(p21, 3));
                    }
                    if ri(pi1, 0) < ri(pi3, 0) {
                        p17 = rp(p20);
                        if !p17.is_null() {
                            loop {
                                p19 = p17;
                                i9 = ri(pi1, 0);
                                p17 = p19;
                                if !(ri(p19, 1) < i9) {
                                    break;
                                }
                                if i9 < ri(p19, 2) {
                                    i16 = ru(p19, 2).wrapping_sub(i9 as u32) as i32;
                                    wi(pi1, 0, i9.wrapping_add(i16));
                                    wf(p21, 3, (i16 as f32) * rf(p21, 4) + rf(p21, 3));
                                }
                                if ri(pi3, 0) <= ri(pi1, 0) {
                                    break;
                                }
                                p17 = rp(p19);
                                p20 = p19;
                                if p17.is_null() {
                                    break;
                                }
                            }
                        }
                        if ri(pi1, 0) < ri(pi3, 0) {
                            wp(p20, p21);
                            wp(p21, p17);
                        } else {
                            clr(ri(pi2, 0), param_3);
                        }
                        p20 = rp(local_28);
                        p21 = local_28;
                    } else {
                        clr(ri(pi2, 0), param_3);
                        p20 = rp(local_28);
                        p21 = local_28;
                    }
                }
            }};
        }

        clr(param_4, param_3);
        if param_1.to_bits() < 0x80000001u32 {
            local_24 = f2i(param_1);
            if (local_24 as f32) != param_1 {
                local_24 = local_24.wrapping_add(1);
            }
        } else {
            local_24 = 0;
        }
        if param_2.to_bits() < 0x80000001u32 {
            lc = f2i(param_2) as u32;
            if ((lc as i32) as f32) != param_2 {
                lc = lc.wrapping_add(1);
            }
        } else {
            lc = 0;
        }
        u13 = DAT_007091ae as u32;
        if (u13 as i32) <= (lc as i32) {
            lc = u13;
        }
        if (lc as i32) <= local_24 {
            return;
        }
        if (u13 as i32) <= local_24 {
            return;
        }
        if DAT_00525544 != 0 && (param_3 & 1) != 0 {
            return;
        }
        local_28 = null;
        p21 = tbl_get(param_3);
        p20 = local_28;
        i9 = local_24;
        'outer: while !p21.is_null() {
            i16 = ri(p21, 2);
            pi1 = p21.wrapping_add(2);
            local_28 = p21;
            local_24 = i9;
            'body: {
                if !(i9 < i16) {
                    break 'body;
                }
                i12 = ri(p21, 1);
                pi2 = p21.wrapping_add(1);
                if i9 < i12 {
                    break 'outer;
                }
                i10 = ri(p21, 5);
                pi3 = p21.wrapping_add(5);
                'j4a: {
                    if pf(param_4, 0x48) <= pf(i10, 0x4c) {
                        break 'j4a;
                    }
                    if pf(i10, 0x48) <= pf(param_4, 0x4c) {
                        if i16 <= lc as i32 {
                            wi(pi1, 0, i9);
                            if ri(pi2, 0) == i9 {
                                unl1!();
                            }
                        }
                        break 'body;
                    }
                    f6 = pf(param_4, 0x28) * (i9 as f32)
                        + pf(param_4, 0x2c) * ((param_3 as i32) as f32)
                        + pf(param_4, 0x30);
                    pf4 = i10.wrapping_add(0x28) as *const f32;
                    f7 = pf(i10, 0x28) * (i9 as f32)
                        + ((param_3 as i32) as f32) * pf(i10, 0x2c)
                        + pf(i10, 0x30);
                    if f6 <= f7 {
                        if rd(pf4) < pf(param_4, 0x28) {
                            if i16 < lc as i32 {
                                local_24 = i16;
                                let d = i16.wrapping_sub(i9) as f32;
                                if d * pf(param_4, 0x28) + f6 <= rd(pf4) * d + f7 {
                                    break 'body;
                                }
                            } else {
                                let d = lc.wrapping_sub(i9 as u32) as i32 as f32;
                                if d * pf(param_4, 0x28) + f6 <= rd(pf4) * d + f7 {
                                    return;
                                }
                            }
                            i16 = f2i((f7 - f6) / (pf(param_4, 0x28) - rd(pf4)));
                            if i16 < 0 {
                                i16 = 0;
                            }
                            i16 = i9.wrapping_add(i16);
                        }
                        break 'j4a;
                    }
                    if rd(pf4) <= pf(param_4, 0x28) {
                        if i16 <= lc as i32 {
                            wi(pi1, 0, i9);
                            if ri(pi2, 0) == i9 {
                                unl1!();
                            }
                        }
                        break 'body;
                    }
                    'j50: {
                        if (lc as i32) < i16 {
                            let d = lc.wrapping_sub(i9 as u32) as i32 as f32;
                            if d * pf(param_4, 0x28) + f6 < rd(pf4) * d + f7 {
                                i16 = f2i((f6 - f7) / (rd(pf4) - pf(param_4, 0x28)));
                                if i16 < 0 {
                                    i16 = 0;
                                }
                                u13 = i16.wrapping_add(i9) as u32;
                                if (lc as i32) < i16.wrapping_add(i9) {
                                    u13 = lc;
                                }
                                break 'j50;
                            }
                            break 'body;
                        }
                        i10 = i16.wrapping_sub(i9);
                        let d = i10 as f32;
                        if d * rd(pf4) + f7 <= pf(param_4, 0x28) * d + f6 {
                            wi(pi1, 0, i9);
                            if ri(pi2, 0) == i9 {
                                unl1!();
                            }
                            break 'body;
                        }
                        i11 = f2i(((rd(pf4) * d + f7) - (pf(param_4, 0x28) * d + f6))
                            / (rd(pf4) - pf(param_4, 0x28)));
                        if i11 < 0 {
                            i11 = 0;
                        }
                        if i10 < i11 {
                            i11 = i10;
                        }
                        u13 = i16.wrapping_sub(i11) as u32;
                        i10 = (u13 as i32).wrapping_sub(i9);
                        i14 = i9.wrapping_sub(i12);
                        i18 = lc.wrapping_sub(i16 as u32) as i32;
                        if i11 < i14 || i10 < i14 || i18 < i14 {
                            if i14 < i11 || i10 < i11 || i18 < i11 {
                                if i10 <= i14 && i10 <= i11 && i10 <= i18 {
                                    break 'j4a;
                                }
                                break 'j50;
                            }
                            wi(pi1, 0, i9);
                            if ri(pi2, 0) == i9 {
                                unl1!();
                            }
                            break 'body;
                        }
                        i11 = i16.wrapping_sub(i12).wrapping_sub(i11);
                        wi(pi2, 0, i11.wrapping_add(i12));
                        wf(p21, 3, (i11 as f32) * rf(p21, 4) + rf(p21, 3));
                        p15 = rp(p21);
                        if p15.is_null() || ri(pi2, 0) < ri(p15, 1) {
                            break 'outer;
                        }
                        if p20.is_null() {
                            tbl_set(param_3, p15);
                        } else {
                            wp(p20, p15);
                        }
                        i16 = ri(pi2, 0);
                        if i16 < ri(p15, 2) {
                            i12 = ru(p15, 2).wrapping_sub(i16 as u32) as i32;
                            wi(pi2, 0, i16.wrapping_add(i12));
                            wf(p21, 3, (i12 as f32) * rf(p21, 4) + rf(p21, 3));
                        }
                        let mut do_clr = false;
                        if ri(pi2, 0) < ri(pi1, 0) {
                            p17 = rp(p15);
                            p19 = p17;
                            if !p17.is_null() {
                                loop {
                                    i16 = ri(pi2, 0);
                                    p19 = p17;
                                    if !(ri(p17, 1) < i16) {
                                        break;
                                    }
                                    if i16 < ri(p17, 2) {
                                        i12 = ru(p17, 2).wrapping_sub(i16 as u32) as i32;
                                        wi(pi2, 0, i16.wrapping_add(i12));
                                        wf(p21, 3, (i12 as f32) * rf(p21, 4) + rf(p21, 3));
                                    }
                                    if ri(pi1, 0) <= ri(pi2, 0) {
                                        break;
                                    }
                                    p19 = rp(p17);
                                    p15 = p17;
                                    p17 = p19;
                                    if p19.is_null() {
                                        break;
                                    }
                                }
                            }
                            if ri(pi1, 0) <= ri(pi2, 0) {
                                do_clr = true;
                            } else {
                                wp(p15, p21);
                                wp(p21, p19);
                            }
                        } else {
                            do_clr = true;
                        }
                        if do_clr {
                            clr(ri(pi3, 0), param_3);
                        }
                        if p20.is_null() {
                            p21 = tbl_get(param_3);
                        } else {
                            p21 = rp(p20);
                        }
                        continue 'outer;
                    }
                    lc = u13;
                    if (lc as i32) <= i9 {
                        return;
                    }
                    break 'body;
                }
                local_24 = i16;
                if (lc as i32) <= i16 {
                    return;
                }
            }
            p21 = rp(p21);
            p20 = local_28;
            i9 = local_24;
        }

        p15 = (&raw const DAT_006a882a).read_unaligned();
        p17 = p15.wrapping_add(6);
        p22 = (&raw const DAT_006a882e).read_unaligned();
        if p15.wrapping_add(6) == p22 {
            p17 = p15;
        }
        (&raw mut DAT_006a882a).write_unaligned(p17);
        wi(p15, 1, i9);
        wu_(p15, 2, lc);
        wu_(p15, 5, param_4 as u32);
        if rdi(param_4.wrapping_add(0x38)) == 0 {
            wu_(p15, 3, 0);
        } else {
            wf(p15, 3, i9 as f32);
            wf(p15, 3, rf(p15, 3) - pf(rdi(param_4.wrapping_add(0x38)), 8));
            wf(p15, 3, rf(p15, 3) * pf(param_4, 0x34));
            wf(
                p15,
                3,
                pf(rdi(param_4.wrapping_add(0x38)), 0xc) + rf(p15, 3),
            );
        }
        wu_(p15, 4, rdi(param_4.wrapping_add(0x34)) as u32);
        wp(slot(param_4, param_3), p15);
        if p20.is_null() {
            tbl_set(param_3, p15);
        } else {
            wp(p20, p15);
        }
        local_28 = p15;
        wp(p15, p21);
        if p21.is_null() {
            return;
        }
        loop {
            i9 = ri(p21, 1);
            pi1 = p21.wrapping_add(1);
            i16 = ri(p15, 2);
            if i16 <= i9 {
                break;
            }
            i12 = ri(p21, 5);
            pi2 = p21.wrapping_add(5);
            if pf(i12, 0x4c) < pf(param_4, 0x48) {
                if pf(param_4, 0x4c) < pf(i12, 0x48) {
                    f6 = pf(param_4, 0x28) * (i9 as f32)
                        + pf(param_4, 0x2c) * ((param_3 as i32) as f32)
                        + pf(param_4, 0x30);
                    pf4 = i12.wrapping_add(0x28) as *const f32;
                    f7 = pf(i12, 0x28) * (i9 as f32)
                        + ((param_3 as i32) as f32) * pf(i12, 0x2c)
                        + pf(i12, 0x30);
                    if f7 < f6 {
                        pi3 = p21.wrapping_add(2);
                        if pf(param_4, 0x28) < rd(pf4) {
                            if i16 < ri(pi3, 0) {
                                i16 = i16.wrapping_sub(i9);
                                f8 = i16 as f32;
                                if f8 * pf(param_4, 0x28) + f6 < rd(pf4) * f8 + f7 {
                                    i12 = f2i((f6 - f7) / (rd(pf4) - pf(param_4, 0x28)));
                                    if i12 < 0 {
                                        i12 = 0;
                                    }
                                    if i16 < i12 {
                                        i12 = i16;
                                    }
                                    wi(pi1, 0, i9.wrapping_add(i12));
                                    wf(p21, 3, (i12 as f32) * rf(p21, 4) + rf(p21, 3));
                                    reins!();
                                } else {
                                    wi(pi1, 0, i9.wrapping_add(i16));
                                    wf(p21, 3, rf(p21, 4) * f8 + rf(p21, 3));
                                    reins!();
                                }
                            } else {
                                i16 = ri(pi3, 0).wrapping_sub(i9);
                                f8 = i16 as f32;
                                if pf(param_4, 0x28) * f8 + f6 < f8 * rd(pf4) + f7 {
                                    i12 = f2i(((rd(pf4) * f8 + f7)
                                        - (pf(param_4, 0x28) * f8 + f6))
                                        / (rd(pf4) - pf(param_4, 0x28)));
                                    if i12 < 0 {
                                        i12 = 0;
                                    }
                                    i16 = i16.wrapping_sub(i12);
                                    if i16 < 0 {
                                        i16 = 0;
                                    }
                                    wi(pi1, 0, i9.wrapping_add(i16));
                                    wf(p21, 3, (i16 as f32) * rf(p21, 4) + rf(p21, 3));
                                    if ri(pi1, 0) < ri(pi3, 0) {
                                        reins!();
                                    } else {
                                        clr(ri(pi2, 0), param_3);
                                        unl2!();
                                    }
                                } else {
                                    clr(i12, param_3);
                                    unl2!();
                                }
                            }
                        } else if i16 < ri(pi3, 0) {
                            wi(pi1, 0, i9.wrapping_add(i16.wrapping_sub(i9)));
                            wf(
                                p21,
                                3,
                                (i16.wrapping_sub(i9) as f32) * rf(p21, 4) + rf(p21, 3),
                            );
                            reins!();
                        } else {
                            clr(i12, param_3);
                            unl2!();
                        }
                    } else if rd(pf4) < pf(param_4, 0x28) {
                        i12 = ri(p21, 2);
                        pi3 = p21.wrapping_add(2);
                        if i12 < i16 {
                            let d = i12.wrapping_sub(i9) as f32;
                            if d * rd(pf4) + f7 < pf(param_4, 0x28) * d + f6 {
                                i9 = f2i(((pf(param_4, 0x28) * d + f6) - (rd(pf4) * d + f7))
                                    / (pf(param_4, 0x28) - rd(pf4)));
                                if i9 < 0 {
                                    i9 = 0;
                                }
                                wi(pi3, 0, i12.wrapping_sub(i9));
                                if ri(pi1, 0) < i12.wrapping_sub(i9) {
                                    p20 = rp(p21);
                                } else {
                                    clr(ri(pi2, 0), param_3);
                                    unl2!();
                                }
                            } else {
                                p20 = rp(p21);
                            }
                        } else {
                            i10 = i16.wrapping_sub(i9);
                            f8 = i10 as f32;
                            if f8 * pf(param_4, 0x28) + f6 <= rd(pf4) * f8 + f7 {
                                wu_(p15, 2, ru(p21, 1));
                                return;
                            }
                            i11 = f2i(((pf(param_4, 0x28) * f8 + f6) - (rd(pf4) * f8 + f7))
                                / (pf(param_4, 0x28) - rd(pf4)));
                            if i11 < 0 {
                                i11 = 0;
                            }
                            if i10 < i11 {
                                i11 = i10;
                            }
                            if i11 < i10.wrapping_sub(i11) && i11 < i12.wrapping_sub(i16) {
                                wu_(p15, 2, ru(p21, 1));
                                return;
                            }
                            if i10.wrapping_sub(i11) < i12.wrapping_sub(i16) {
                                wi(pi1, 0, i10.wrapping_add(i9));
                                wf(p21, 3, rf(p21, 4) * f8 + rf(p21, 3));
                                reins!();
                            } else {
                                wi(pi3, 0, i16.wrapping_sub(i11));
                                p20 = rp(p21);
                            }
                        }
                    } else {
                        if i16 <= ri(p21, 2) {
                            wu_(p15, 2, ru(p21, 1));
                            return;
                        }
                        p20 = rp(p21);
                    }
                } else {
                    pi3 = p21.wrapping_add(2);
                    if i16 < ri(p21, 2) {
                        wi(pi1, 0, i9.wrapping_add(i16.wrapping_sub(i9)));
                        wf(
                            p21,
                            3,
                            (i16.wrapping_sub(i9) as f32) * rf(p21, 4) + rf(p21, 3),
                        );
                        reins!();
                    } else {
                        clr(i12, param_3);
                        unl2!();
                    }
                }
            } else {
                if i16 <= ri(p21, 2) {
                    wu_(p15, 2, ru(p21, 1));
                    return;
                }
                p20 = rp(p21);
            }
            local_28 = p21;
            p21 = p20;
            if p20.is_null() {
                return;
            }
        }
    }
}

#[inline(always)]
unsafe fn wu_(p: *mut u32, i: usize, v: u32) {
    // SAFETY: p points at the game's record words, written where the original writes.
    unsafe { p.wrapping_add(i).write_unaligned(v) }
}
