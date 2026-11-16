// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d682c: u16;
    static mut DAT_00523404: i32;
    static mut DAT_00999492: [u8; 4];
    static mut DAT_0099942a: [u8; 4];
    static mut DAT_0099942e: [u8; 4];
}

#[inline(always)]
unsafe fn w(a: u32, i: u32) -> u32 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (a.wrapping_add(i.wrapping_mul(4)) as usize as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wi(a: u32, i: u32) -> i32 {
    // SAFETY: see `w`.
    unsafe { w(a, i) as i32 }
}

#[inline(always)]
unsafe fn sw(a: u32, i: u32, v: u32) {
    // SAFETY: the address is the game's own memory, written where the original writes.
    unsafe { (a.wrapping_add(i.wrapping_mul(4)) as usize as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn fl(a: u32, i: u32) -> f32 {
    // SAFETY: see `w`.
    unsafe { f32::from_bits(w(a, i)) }
}

#[inline(always)]
unsafe fn sf(a: u32, i: u32, v: f32) {
    // SAFETY: see `sw`.
    unsafe { sw(a, i, v.to_bits()) }
}

#[inline(always)]
unsafe fn tbl() -> u32 {
    // SAFETY: the global is the game's own memory.
    unsafe { (&raw const DAT_00999492).cast::<u32>().read_unaligned() }
}

#[inline(always)]
unsafe fn clear_slot(owner: u32, p3: u32) {
    // SAFETY: the owner record is the game's own memory.
    unsafe {
        sw(w(owner, 26), p3.wrapping_sub(w(owner, 16)), 0);
    }
}

#[inline(always)]
unsafe fn unlink(owner: u32, p3: u32, p20: u32, cur: u32) {
    // SAFETY: the lists are the game's own memory.
    unsafe {
        clear_slot(owner, p3);
        if p20 == 0 {
            let t = tbl();
            sw(t, p3, w(cur, 0));
        } else {
            sw(p20, 0, w(cur, 0));
        }
    }
}

unsafe fn merge(cur: u32, l24: u32, p3: u32, p20: &mut u32, p21: &mut u32) {
    // SAFETY: the lists are the game's own memory.
    unsafe {
        let pn = w(cur, 0);
        *p20 = pn;
        if pn == 0 || wi(pn, 1) >= wi(cur, 1) {
            return;
        }
        sw(l24, 0, pn);
        let i10 = wi(cur, 1);
        if i10 < wi(pn, 2) {
            let d = wi(pn, 2).wrapping_sub(i10);
            sw(cur, 1, i10.wrapping_add(d) as u32);
            sf(cur, 3, (d as f32) * fl(cur, 4) + fl(cur, 3));
        }
        let mut q20 = pn;
        let mut p17: u32 = 0;
        let mut go919 = false;
        if wi(cur, 1) < wi(cur, 2) {
            p17 = w(pn, 0);
            if p17 != 0 {
                loop {
                    let p19 = p17;
                    let i = wi(cur, 1);
                    if !(wi(p19, 1) < i) {
                        break;
                    }
                    if i < wi(p19, 2) {
                        let d = wi(p19, 2).wrapping_sub(i);
                        sw(cur, 1, i.wrapping_add(d) as u32);
                        sf(cur, 3, (d as f32) * fl(cur, 4) + fl(cur, 3));
                    }
                    if wi(cur, 2) <= wi(cur, 1) {
                        break;
                    }
                    p17 = w(p19, 0);
                    q20 = p19;
                    if p17 == 0 {
                        break;
                    }
                }
            }
            if wi(cur, 1) < wi(cur, 2) {
                go919 = true;
            }
        }
        if go919 {
            sw(q20, 0, cur);
            sw(cur, 0, p17);
        } else {
            clear_slot(w(cur, 5), p3);
        }
        *p20 = w(l24, 0);
        *p21 = l24;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_mut, unused_variables)]
pub unsafe extern "C" fn FUN_00486980(param_1: f32, param_2: f32, param_3: u32, param_4: i32) {
    // SAFETY: all pointers are the game's own span records and tables, accessed
    // where the original accesses them.
    unsafe {
        let fp = param_4 as u32;
        clear_slot(fp, param_3);
        let mut l18: i32;
        if param_1.to_bits() < 0x80000001 {
            l18 = param_1 as i64 as i32;
            if (l18 as f32) != param_1 {
                l18 = l18.wrapping_add(1);
            }
        } else {
            l18 = 0;
        }
        let mut l20: i32;
        if param_2.to_bits() < 0x80000001 {
            l20 = param_2 as i64 as i32;
            if (l20 as f32) != param_2 {
                l20 = l20.wrapping_add(1);
            }
        } else {
            l20 = 0;
        }
        let u9 = DAT_009d682c as i32;
        if u9 <= l20 {
            l20 = u9;
        }
        if l20 <= l18 {
            return;
        }
        if u9 <= l18 {
            return;
        }
        if DAT_00523404 != 0 && (param_3 & 1) != 0 {
            return;
        }
        let mut l24: u32 = 0;
        let t0 = tbl();
        let mut p21 = w(t0, param_3);
        let mut p20 = l24;
        let mut i10 = l18;
        let p3f = param_3 as i32 as f32;
        'outer: while p21 != 0 {
            let cur = p21;
            let mut i16 = wi(cur, 2);
            l24 = cur;
            l18 = i10;
            'adv: {
                if !(i10 < i16) {
                    break 'adv;
                }
                let i13 = wi(cur, 1);
                if i10 < i13 {
                    break 'outer;
                }
                let own = w(cur, 5);
                if fl(fp, 18) <= fl(own, 19) {
                    l18 = i16;
                    if l20 <= i16 {
                        return;
                    }
                    break 'adv;
                }
                if fl(own, 18) <= fl(fp, 19) {
                    if i16 <= l20 {
                        sw(cur, 2, i10 as u32);
                        if wi(cur, 1) == i10 {
                            unlink(own, param_3, p20, cur);
                            l24 = p20;
                        }
                    }
                    break 'adv;
                }
                let fp10 = fl(fp, 10);
                let f6 = fp10 * (i10 as f32) + fl(fp, 11) * p3f + fl(fp, 12);
                let pf = fl(own, 10);
                let f7 = fl(own, 10) * (i10 as f32) + p3f * fl(own, 11) + fl(own, 12);
                if f6 <= f7 {
                    if pf < fp10 {
                        if i16 < l20 {
                            l18 = i16;
                            let d = i16.wrapping_sub(i10) as f32;
                            if d * fp10 + f6 <= pf * d + f7 {
                                break 'adv;
                            }
                        } else {
                            let d = l20.wrapping_sub(i10) as f32;
                            if d * fp10 + f6 <= pf * d + f7 {
                                return;
                            }
                        }
                        let v = ((f7 - f6) / (fp10 - pf)) as i64;
                        let mut t = (v as i32).wrapping_add(1);
                        if t < 0 {
                            t = 0;
                        }
                        i16 = i10.wrapping_add(t);
                    }
                    l18 = i16;
                    if l20 <= i16 {
                        return;
                    }
                    break 'adv;
                }
                if pf <= fp10 {
                    if i16 <= l20 {
                        sw(cur, 2, i10 as u32);
                        if wi(cur, 1) == i10 {
                            unlink(own, param_3, p20, cur);
                            l24 = p20;
                        }
                    }
                    break 'adv;
                }
                if l20 < i16 {
                    let d = l20.wrapping_sub(i10) as f32;
                    if d * fp10 + f6 < pf * d + f7 {
                        let v = ((f6 - f7) / (pf - fp10)) as i64;
                        let mut t = (v as i32).wrapping_add(1);
                        if t < 0 {
                            t = 0;
                        }
                        let mut u = t.wrapping_add(i10);
                        if l20 < u {
                            u = l20;
                        }
                        l20 = u;
                        if l20 <= i10 {
                            return;
                        }
                    }
                    break 'adv;
                }
                let n = i16.wrapping_sub(i10);
                let nf = n as f32;
                if nf * pf + f7 <= fp10 * nf + f6 {
                    sw(cur, 2, i10 as u32);
                    if wi(cur, 1) == i10 {
                        unlink(w(cur, 5), param_3, p20, cur);
                        l24 = p20;
                    }
                    break 'adv;
                }
                let v = ((f6 - f7) / (pf - fp10)) as i64;
                let mut i12 = v as i32;
                if i12 < 0 {
                    i12 = 0;
                }
                if n < i12 {
                    i12 = n;
                }
                let n2 = n.wrapping_sub(i12);
                let i14 = i10.wrapping_sub(i13);
                let i18 = l20.wrapping_sub(i16);
                if i12 < i14 || n2 < i14 || i18 < i14 {
                    if i14 < i12 || n2 < i12 || i18 < i12 {
                        if n2 <= i14 && n2 <= i12 && n2 <= i18 {
                            l18 = i16;
                            if l20 <= i16 {
                                return;
                            }
                            break 'adv;
                        }
                        let u = i16.wrapping_sub(i12);
                        l20 = u;
                        if l20 <= i10 {
                            return;
                        }
                        break 'adv;
                    }
                    sw(cur, 2, i10 as u32);
                    if wi(cur, 1) == i10 {
                        unlink(w(cur, 5), param_3, p20, cur);
                        l24 = p20;
                    }
                    break 'adv;
                }
                let k = i16.wrapping_sub(i13).wrapping_sub(i12);
                sw(cur, 1, k.wrapping_add(i13) as u32);
                sf(cur, 3, (k as f32) * fl(cur, 4) + fl(cur, 3));
                let p15 = w(cur, 0);
                if p15 == 0 || wi(cur, 1) < wi(p15, 1) {
                    break 'outer;
                }
                if p20 == 0 {
                    let t = tbl();
                    sw(t, param_3, p15);
                } else {
                    sw(p20, 0, p15);
                }
                let s = wi(cur, 1);
                if s < wi(p15, 2) {
                    let d = wi(p15, 2).wrapping_sub(s);
                    sw(cur, 1, s.wrapping_add(d) as u32);
                    sf(cur, 3, (d as f32) * fl(cur, 4) + fl(cur, 3));
                }
                let mut p15m = p15;
                let mut clr = true;
                if wi(cur, 1) < wi(cur, 2) {
                    let mut p17 = w(p15m, 0);
                    let mut p19 = p17;
                    if p17 != 0 {
                        loop {
                            let i = wi(cur, 1);
                            p19 = p17;
                            if !(wi(p17, 1) < i) {
                                break;
                            }
                            if i < wi(p17, 2) {
                                let d = wi(p17, 2).wrapping_sub(i);
                                sw(cur, 1, i.wrapping_add(d) as u32);
                                sf(cur, 3, (d as f32) * fl(cur, 4) + fl(cur, 3));
                            }
                            if wi(cur, 2) <= wi(cur, 1) {
                                break;
                            }
                            p19 = w(p17, 0);
                            p15m = p17;
                            p17 = p19;
                            if p19 == 0 {
                                break;
                            }
                        }
                    }
                    if !(wi(cur, 2) <= wi(cur, 1)) {
                        sw(p15m, 0, cur);
                        sw(cur, 0, p19);
                        clr = false;
                    }
                }
                if clr {
                    clear_slot(w(cur, 5), param_3);
                }
                if p20 == 0 {
                    let t = tbl();
                    p21 = w(t, param_3);
                } else {
                    p21 = w(p20, 0);
                }
                continue 'outer;
            }
            p21 = w(cur, 0);
            p20 = l24;
            i10 = l18;
        }

        let p15 = (&raw const DAT_0099942a).cast::<u32>().read_unaligned();
        let pend = (&raw const DAT_0099942e).cast::<u32>().read_unaligned();
        let mut p17 = p15.wrapping_add(24);
        if p17 == pend {
            p17 = p15;
        }
        (&raw mut DAT_0099942a).cast::<u32>().write_unaligned(p17);
        sw(p15, 1, i10 as u32);
        sw(p15, 2, l20 as u32);
        sw(p15, 5, param_4 as u32);
        if wi(fp, 14) == 0 {
            sw(p15, 3, 0);
        } else {
            sf(p15, 3, i10 as f32);
            sf(p15, 3, fl(p15, 3) - fl(w(fp, 14), 2));
            sf(p15, 3, fl(p15, 3) * fl(fp, 13));
            sf(p15, 3, fl(w(fp, 14), 3) + fl(p15, 3));
        }
        sw(p15, 4, w(fp, 13));
        sw(w(fp, 26), param_3.wrapping_sub(w(fp, 16)), p15);
        if p20 == 0 {
            let t = tbl();
            sw(t, param_3, p15);
        } else {
            sw(p20, 0, p15);
        }
        l24 = p15;
        sw(p15, 0, p21);
        if p21 != 0 {
            loop {
                let cur = p21;
                let i10 = wi(cur, 1);
                let mut i16 = wi(p15, 2);
                if i16 <= i10 {
                    break;
                }
                let own = w(cur, 5);
                if fl(own, 19) < fl(fp, 18) {
                    if fl(fp, 19) < fl(own, 18) {
                        let fp10 = fl(fp, 10);
                        let f6 = fp10 * (i10 as f32) + fl(fp, 11) * p3f + fl(fp, 12);
                        let pf = fl(own, 10);
                        let f7 = fl(own, 10) * (i10 as f32) + p3f * fl(own, 11) + fl(own, 12);
                        if f7 < f6 {
                            if fp10 < pf {
                                if i16 < wi(cur, 2) {
                                    i16 = i16.wrapping_sub(i10);
                                    let f8 = i16 as f32;
                                    if f8 * fp10 + f6 < pf * f8 + f7 {
                                        let v = ((f6 - f7) / (pf - fp10)) as i64;
                                        let mut k = v as i32;
                                        if k < 0 {
                                            k = 0;
                                        }
                                        if i16 < k {
                                            k = i16;
                                        }
                                        sw(cur, 1, i10.wrapping_add(k) as u32);
                                        sf(cur, 3, (k as f32) * fl(cur, 4) + fl(cur, 3));
                                        merge(cur, l24, param_3, &mut p20, &mut p21);
                                    } else {
                                        sw(cur, 1, i10.wrapping_add(i16) as u32);
                                        sf(cur, 3, fl(cur, 4) * f8 + fl(cur, 3));
                                        merge(cur, l24, param_3, &mut p20, &mut p21);
                                    }
                                } else {
                                    let mut e = wi(cur, 2).wrapping_sub(i10);
                                    let f8 = e as f32;
                                    if fp10 * f8 + f6 < f8 * pf + f7 {
                                        let v = ((f6 - f7) / (pf - fp10)) as i64;
                                        let mut k = v as i32;
                                        if k < 0 {
                                            k = 0;
                                        }
                                        e = e.wrapping_sub(k);
                                        if e < 0 {
                                            e = 0;
                                        }
                                        sw(cur, 1, i10.wrapping_add(e) as u32);
                                        sf(cur, 3, (e as f32) * fl(cur, 4) + fl(cur, 3));
                                        if wi(cur, 1) < wi(cur, 2) {
                                            merge(cur, l24, param_3, &mut p20, &mut p21);
                                        } else {
                                            clear_slot(w(cur, 5), param_3);
                                            let nx = w(cur, 0);
                                            sw(l24, 0, nx);
                                            p20 = nx;
                                            p21 = l24;
                                        }
                                    } else {
                                        clear_slot(own, param_3);
                                        let nx = w(cur, 0);
                                        sw(l24, 0, nx);
                                        p20 = nx;
                                        p21 = l24;
                                    }
                                }
                            } else if i16 < wi(cur, 2) {
                                let d = i16.wrapping_sub(i10);
                                sw(cur, 1, i10.wrapping_add(d) as u32);
                                sf(cur, 3, (d as f32) * fl(cur, 4) + fl(cur, 3));
                                merge(cur, l24, param_3, &mut p20, &mut p21);
                            } else {
                                clear_slot(own, param_3);
                                let nx = w(cur, 0);
                                sw(l24, 0, nx);
                                p20 = nx;
                                p21 = l24;
                            }
                        } else if pf < fp10 {
                            let i13 = wi(cur, 2);
                            if i13 < i16 {
                                let e = i13.wrapping_sub(i10) as f32;
                                if e * pf + f7 < fp10 * e + f6 {
                                    let v = ((f7 - f6) / (fp10 - pf)) as i64;
                                    let mut k = v as i32;
                                    if k < 0 {
                                        k = 0;
                                    }
                                    sw(cur, 2, i13.wrapping_sub(k) as u32);
                                    if wi(cur, 1) < i13.wrapping_sub(k) {
                                        p20 = w(cur, 0);
                                    } else {
                                        clear_slot(w(cur, 5), param_3);
                                        let nx = w(cur, 0);
                                        sw(l24, 0, nx);
                                        p20 = nx;
                                        p21 = l24;
                                    }
                                } else {
                                    p20 = w(cur, 0);
                                }
                            } else {
                                let n = i16.wrapping_sub(i10);
                                let f8 = n as f32;
                                if f8 * fp10 + f6 <= pf * f8 + f7 {
                                    sw(p15, 2, w(cur, 1));
                                    return;
                                }
                                let v = ((f7 - f6) / (fp10 - pf)) as i64;
                                let mut k = v as i32;
                                if k < 0 {
                                    k = 0;
                                }
                                if n < k {
                                    k = n;
                                }
                                if k < n.wrapping_sub(k) && k < i13.wrapping_sub(i16) {
                                    sw(p15, 2, w(cur, 1));
                                    return;
                                }
                                if n.wrapping_sub(k) < i13.wrapping_sub(i16) {
                                    sw(cur, 1, n.wrapping_add(i10) as u32);
                                    sf(cur, 3, fl(cur, 4) * f8 + fl(cur, 3));
                                    merge(cur, l24, param_3, &mut p20, &mut p21);
                                } else {
                                    sw(cur, 2, i16.wrapping_sub(k) as u32);
                                    p20 = w(cur, 0);
                                }
                            }
                        } else {
                            if i16 <= wi(cur, 2) {
                                sw(p15, 2, w(cur, 1));
                                return;
                            }
                            p20 = w(cur, 0);
                        }
                    } else if i16 < wi(cur, 2) {
                        let d = i16.wrapping_sub(i10);
                        sw(cur, 1, i10.wrapping_add(d) as u32);
                        sf(cur, 3, (d as f32) * fl(cur, 4) + fl(cur, 3));
                        merge(cur, l24, param_3, &mut p20, &mut p21);
                    } else {
                        clear_slot(own, param_3);
                        let nx = w(cur, 0);
                        sw(l24, 0, nx);
                        p20 = nx;
                        p21 = l24;
                    }
                } else {
                    if i16 <= wi(cur, 2) {
                        sw(p15, 2, w(cur, 1));
                        return;
                    }
                    p20 = w(cur, 0);
                }
                l24 = p21;
                p21 = p20;
                if p20 == 0 {
                    return;
                }
            }
        }
    }
}
