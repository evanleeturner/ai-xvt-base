// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0099943c: i32;
    static mut DAT_00999432: [u8; 4];
    static mut _DAT_005181c0: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00486980(param_1: f32, param_2: f32, param_3: u32, param_4: i32);
}

unsafe fn rd(a: i32) -> i32 {
    // SAFETY: address inside the game's own structure.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

unsafe fn wr(a: i32, v: i32) {
    // SAFETY: address inside the game's own structure.
    unsafe { (a as usize as *mut i32).write_unaligned(v) }
}

fn pp(a: i32) -> *mut u32 {
    a as usize as *mut u32
}

unsafe fn rf(p: *mut u32, i: usize) -> f32 {
    // SAFETY: p points at the game's vertex record.
    unsafe { p.add(i).cast::<f32>().read_unaligned() }
}

unsafe fn wf(p: *mut u32, i: usize, v: f32) {
    // SAFETY: p points at the game's vertex record.
    unsafe { p.add(i).cast::<f32>().write_unaligned(v) }
}

unsafe fn ru(p: *mut u32, i: usize) -> u32 {
    // SAFETY: p points at the game's vertex record.
    unsafe { p.add(i).read_unaligned() }
}

unsafe fn wu(p: *mut u32, i: usize, v: u32) {
    // SAFETY: p points at the game's vertex record.
    unsafe { p.add(i).write_unaligned(v) }
}

unsafe fn ent(p: i32, i: i32) -> *mut u32 {
    // SAFETY: reads the pointer table inside the game's structure.
    unsafe { pp(rd(p.wrapping_add(0x50).wrapping_add(i.wrapping_mul(4)))) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00471410(param_1: i32) {
    // SAFETY: all memory touched is the game's own structure, vertex
    // records and globals, accessed where the original accesses them.
    unsafe {
        let p = param_1;
        let mut local_1c: i32 = rd(p.wrapping_add(100));
        let mut p6: *mut u32 = ent(p, 0);
        let mut p7: *mut u32 = p6;
        let mut p5: *mut u32 = core::ptr::null_mut();
        let mut i_var3: i32;
        let mut i_var8: i32;
        let mut u_var9: u32;
        let mut u_var10: u32;
        let mut b11: bool;
        let mut local_14: f32 = 0.0;
        let mut local_10: f32 = 0.0;
        let mut local_8: u32 = 0;
        let mut local_4: u32 = 0;

        if 1 < local_1c {
            i_var3 = local_1c.wrapping_sub(1);
            let mut idx: i32 = 1;
            loop {
                p5 = ent(p, idx);
                if (ru(p5, 1) as i32) < (ru(p7, 1) as i32) {
                    p7 = p5;
                }
                if (ru(p6, 0) as i32) < (ru(p5, 0) as i32) {
                    p6 = p5;
                }
                idx = idx.wrapping_add(1);
                i_var3 = i_var3.wrapping_sub(1);
                if i_var3 == 0 {
                    break;
                }
            }
        }
        u_var10 = ru(p7, 1);
        wr(p.wrapping_add(0x40), u_var10 as i32);
        u_var9 = ru(p6, 0);
        wr(p.wrapping_add(0x44), u_var9 as i32);
        i_var3 = u_var9.wrapping_sub(u_var10) as i32;
        if DAT_0099943c <= i_var3 {
            wr(p.wrapping_add(0x44), u_var10 as i32);
            return;
        }
        i_var8 = 0;
        DAT_0099943c = DAT_0099943c.wrapping_sub(i_var3);
        i_var3 = rd(p.wrapping_add(100));
        let tmp_base: u32 = (&raw const DAT_00999432).cast::<u32>().read_unaligned();
        wr(
            p.wrapping_add(0x68),
            tmp_base.wrapping_add((DAT_0099943c as u32).wrapping_mul(4)) as i32,
        );
        b11 = i_var3 == 0;
        if 0 < i_var3 {
            loop {
                let e = ent(p, i_var8);
                if p7 != e && ru(e, 1) == ru(p7, 1) {
                    p6 = ent(p, i_var8);
                    break;
                }
                i_var8 = i_var8.wrapping_add(1);
                if !(i_var8 < i_var3) {
                    break;
                }
            }
            b11 = i_var8 == i_var3;
        }
        if b11 {
            wr(p.wrapping_add(0x44), rd(p.wrapping_add(0x40)));
            return;
        }
        if rf(p6, 2) < rf(p7, 2) || (rf(p6, 2) == rf(p7, 2) && rf(p6, 4) < rf(p7, 4)) {
            p5 = p7;
            p7 = p6;
        } else {
            p5 = p6;
        }
        wr(p.wrapping_add(0x38), p7 as usize as i32);
        u_var10 = ru(p7, 1);
        u_var9 = ru(p5, 0);
        if (ru(p7, 0) as i32) < (ru(p5, 0) as i32) {
            u_var9 = ru(p7, 0);
        }
        i_var3 = u_var9.wrapping_sub(u_var10) as i32;
        local_14 = rf(p7, 2);
        local_10 = rf(p5, 2);
        local_8 = ru(p7, 3);
        local_4 = ru(p5, 3);

        macro_rules! fin {
            () => {{
                wf(p7, 2, local_14);
                wf(p5, 2, local_10);
                wu(p7, 3, local_8);
                wu(p5, 3, local_4);
                return;
            }};
        }

        if _DAT_005181c0 < local_10 - local_14 {
            loop {
                i_var3 = i_var3.wrapping_sub(1);
                u_var9 = u_var10.wrapping_add(1);
                wf(
                    p.wrapping_add(0x34) as usize as *mut u32,
                    0,
                    (rf(p5, 3) - rf(p7, 3)) / (rf(p5, 2) - rf(p7, 2)),
                );
                FUN_00486980(rf(p7, 2), rf(p5, 2), u_var10, p);
                if i_var3 < 1 {
                    break;
                }
                wf(p7, 2, rf(p7, 4) + rf(p7, 2));
                wf(p7, 3, rf(p7, 5) + rf(p7, 3));
                wf(p5, 2, rf(p5, 4) + rf(p5, 2));
                wf(p5, 3, rf(p5, 5) + rf(p5, 3));
                u_var10 = u_var9;
            }
        } else {
            let f_var1 = i_var3 as f32;
            wf(
                p.wrapping_add(0x34) as usize as *mut u32,
                0,
                ((f_var1 * rf(p5, 5) + rf(p5, 3)) - (f_var1 * rf(p7, 5) + rf(p7, 3)))
                    / ((f_var1 * rf(p5, 4) + rf(p5, 2)) - (f_var1 * rf(p7, 4) + rf(p7, 2))),
            );
            loop {
                i_var3 = i_var3.wrapping_sub(1);
                u_var9 = u_var10.wrapping_add(1);
                FUN_00486980(rf(p7, 2), rf(p5, 2), u_var10, p);
                if i_var3 < 1 {
                    break;
                }
                wf(p7, 2, rf(p7, 2) + rf(p7, 4));
                wf(p7, 3, rf(p7, 5) + rf(p7, 3));
                wf(p5, 2, rf(p5, 2) + rf(p5, 4));
                wf(p5, 3, rf(p5, 5) + rf(p5, 3));
                u_var10 = u_var9;
            }
        }

        loop {
            if local_1c < 1 {
                fin!();
            }
            'l853: {
                if ru(p5, 0) == ru(p7, 0) {
                    local_1c = local_1c.wrapping_sub(2);
                    if local_1c != 0 {
                        wf(p7, 2, local_14);
                        wf(p5, 2, local_10);
                        i_var8 = 0;
                        wu(p7, 3, local_8);
                        wu(p5, 3, local_4);
                        i_var3 = rd(p.wrapping_add(100));
                        b11 = i_var3 == 0;
                        if 0 < i_var3 {
                            loop {
                                if ru(ent(p, i_var8), 1) == u_var9 {
                                    p7 = ent(p, i_var8);
                                    break;
                                }
                                i_var8 = i_var8.wrapping_add(1);
                                if !(i_var8 < i_var3) {
                                    break;
                                }
                            }
                            b11 = i_var8 == i_var3;
                        }
                        if !b11 {
                            i_var8 = i_var8.wrapping_add(1);
                            if i_var8 < i_var3 {
                                loop {
                                    if ru(ent(p, i_var8), 1) == u_var9 {
                                        p5 = ent(p, i_var8);
                                        break;
                                    }
                                    i_var8 = i_var8.wrapping_add(1);
                                    if !(i_var8 < i_var3) {
                                        break;
                                    }
                                }
                            }
                            if i_var8 != i_var3 {
                                if rf(p5, 2) < rf(p7, 2)
                                    || (rf(p5, 2) == rf(p7, 2) && rf(p5, 4) < rf(p7, 4))
                                {
                                    p6 = p7;
                                    p7 = p5;
                                } else {
                                    p6 = p5;
                                }
                                wr(p.wrapping_add(0x38), p7 as usize as i32);
                                local_8 = ru(p7, 3);
                                local_14 = rf(p7, 2);
                                local_10 = rf(p6, 2);
                                local_4 = ru(p6, 3);
                                break 'l853;
                            }
                        }
                        wr(p.wrapping_add(0x44), u_var9 as i32);
                    }
                    fin!();
                }
                local_1c = local_1c.wrapping_sub(1);
                p6 = p5;
                if u_var9 == ru(p7, 0) {
                    i_var8 = 0;
                    wf(p7, 2, local_14);
                    wu(p7, 3, local_8);
                    i_var3 = rd(p.wrapping_add(100));
                    b11 = i_var3 == 0;
                    if 0 < i_var3 {
                        loop {
                            if ru(ent(p, i_var8), 1) == u_var9 {
                                p7 = ent(p, i_var8);
                                break;
                            }
                            i_var8 = i_var8.wrapping_add(1);
                            if !(i_var8 < i_var3) {
                                break;
                            }
                        }
                        b11 = i_var8 == i_var3;
                    }
                    if b11 {
                        wr(p.wrapping_add(0x44), u_var9 as i32);
                        fin!();
                    }
                    wr(p.wrapping_add(0x38), p7 as usize as i32);
                    local_14 = rf(p7, 2);
                    local_8 = ru(p7, 3);
                    wf(p5, 2, rf(p5, 4) + rf(p5, 2));
                    wf(p5, 3, rf(p5, 5) + rf(p5, 3));
                } else {
                    i_var8 = 0;
                    wf(p5, 2, local_10);
                    wu(p5, 3, local_4);
                    i_var3 = rd(p.wrapping_add(100));
                    b11 = i_var3 == 0;
                    if 0 < i_var3 {
                        loop {
                            if ru(ent(p, i_var8), 1) == u_var9 {
                                p5 = ent(p, i_var8);
                                break;
                            }
                            i_var8 = i_var8.wrapping_add(1);
                            if !(i_var8 < i_var3) {
                                break;
                            }
                        }
                        b11 = i_var8 == i_var3;
                        p6 = p5;
                    }
                    p5 = p6;
                    if b11 {
                        wr(p.wrapping_add(0x44), u_var9 as i32);
                        fin!();
                    }
                    local_10 = rf(p6, 2);
                    local_4 = ru(p6, 3);
                    wf(p7, 2, rf(p7, 4) + rf(p7, 2));
                    wf(p7, 3, rf(p7, 5) + rf(p7, 3));
                }
            }
            // LAB_00471853
            if local_1c == 1 {
                return;
            }
            p5 = p6;
            if local_1c == 2 {
                if ru(p6, 0) != ru(p7, 0) {
                    return;
                }
                i_var3 = ru(p7, 0).wrapping_sub(u_var9) as i32;
                u_var10 = u_var9;
                if _DAT_005181c0
                    < ((rf(p6, 4) * (i_var3 as f32) + rf(p6, 2))
                        - (rf(p7, 4) * (i_var3 as f32) + rf(p7, 2)))
                {
                    loop {
                        i_var3 = i_var3.wrapping_sub(1);
                        u_var9 = u_var10.wrapping_add(1);
                        wf(
                            p.wrapping_add(0x34) as usize as *mut u32,
                            0,
                            (rf(p6, 3) - rf(p7, 3)) / (rf(p6, 2) - rf(p7, 2)),
                        );
                        FUN_00486980(rf(p7, 2), rf(p6, 2), u_var10, p);
                        if i_var3 < 1 {
                            break;
                        }
                        wf(p7, 2, rf(p7, 2) + rf(p7, 4));
                        wf(p7, 3, rf(p7, 5) + rf(p7, 3));
                        wf(p6, 2, rf(p6, 2) + rf(p6, 4));
                        wf(p6, 3, rf(p6, 5) + rf(p6, 3));
                        u_var10 = u_var9;
                    }
                } else {
                    wf(
                        p.wrapping_add(0x34) as usize as *mut u32,
                        0,
                        (rf(p6, 3) - rf(p7, 3)) / (rf(p6, 2) - rf(p7, 2)),
                    );
                    loop {
                        i_var3 = i_var3.wrapping_sub(1);
                        u_var9 = u_var10.wrapping_add(1);
                        FUN_00486980(rf(p7, 2), rf(p6, 2), u_var10, p);
                        if i_var3 < 1 {
                            break;
                        }
                        wf(p7, 2, rf(p7, 2) + rf(p7, 4));
                        wf(p7, 3, rf(p7, 5) + rf(p7, 3));
                        wf(p6, 2, rf(p6, 2) + rf(p6, 4));
                        wf(p6, 3, rf(p6, 5) + rf(p6, 3));
                        u_var10 = u_var9;
                    }
                }
            } else {
                u_var10 = u_var9;
                if (ru(p7, 0) as i32) < (ru(p6, 0) as i32) {
                    i_var3 = ru(p7, 0).wrapping_sub(u_var9) as i32;
                } else {
                    i_var3 = ru(p6, 0).wrapping_sub(u_var9) as i32;
                }
                loop {
                    i_var3 = i_var3.wrapping_sub(1);
                    u_var9 = u_var10.wrapping_add(1);
                    wf(
                        p.wrapping_add(0x34) as usize as *mut u32,
                        0,
                        (rf(p6, 3) - rf(p7, 3)) / (rf(p6, 2) - rf(p7, 2)),
                    );
                    FUN_00486980(rf(p7, 2), rf(p6, 2), u_var10, p);
                    if i_var3 < 1 {
                        break;
                    }
                    wf(p7, 2, rf(p7, 2) + rf(p7, 4));
                    wf(p7, 3, rf(p7, 5) + rf(p7, 3));
                    wf(p6, 2, rf(p6, 2) + rf(p6, 4));
                    wf(p6, 3, rf(p6, 5) + rf(p6, 3));
                    u_var10 = u_var9;
                }
            }
        }
    }
}
