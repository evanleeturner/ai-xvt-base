// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006a883c: i32;
    static mut DAT_006a8832: [u8; 4];
    static mut _DAT_005201dc: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004663b0(param_1: f32, param_2: f32, param_3: u32, param_4: i32);
}

unsafe fn g(p: *mut u32, i: usize) -> u32 {
    // SAFETY: p points at the game's vertex record.
    unsafe { p.add(i).read_unaligned() }
}
unsafe fn gf(p: *mut u32, i: usize) -> f32 {
    // SAFETY: p points at the game's vertex record.
    unsafe { p.add(i).cast::<f32>().read_unaligned() }
}
unsafe fn sw(p: *mut u32, i: usize, v: u32) {
    // SAFETY: p points at the game's vertex record.
    unsafe { p.add(i).write_unaligned(v) }
}
unsafe fn sf(p: *mut u32, i: usize, v: f32) {
    // SAFETY: p points at the game's vertex record.
    unsafe { p.add(i).cast::<f32>().write_unaligned(v) }
}
unsafe fn rd(b: i32, o: i32) -> u32 {
    // SAFETY: b points at the caller's structure.
    unsafe { (b.wrapping_add(o) as u32 as usize as *const u32).read_unaligned() }
}
unsafe fn wr(b: i32, o: i32, v: u32) {
    // SAFETY: b points at the caller's structure.
    unsafe { (b.wrapping_add(o) as u32 as usize as *mut u32).write_unaligned(v) }
}
unsafe fn wrf(b: i32, o: i32, v: f32) {
    // SAFETY: b points at the caller's structure.
    unsafe { (b.wrapping_add(o) as u32 as usize as *mut f32).write_unaligned(v) }
}
unsafe fn slot(b: i32, i: i32) -> *mut u32 {
    // SAFETY: b points at the caller's structure.
    unsafe { rd(b, 0x50i32.wrapping_add(i.wrapping_mul(4))) as usize as *mut u32 }
}
unsafe fn find(b: i32, start: i32, n: i32, key: u32) -> (i32, *mut u32) {
    let mut i = start;
    // SAFETY: slots below n hold valid vertex pointers.
    unsafe {
        while i < n {
            let s = slot(b, i);
            if g(s, 1) == key {
                return (i, s);
            }
            i += 1;
        }
    }
    (i, core::ptr::null_mut())
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "C" fn FUN_004853c0(param_1: i32) {
    // SAFETY: param_1 is the game's polygon structure; its slots point at
    // vertex records; globals and callee are the game's own.
    unsafe {
        let mut local_1c = rd(param_1, 100) as i32;
        let mut p6 = slot(param_1, 0);
        let mut p7 = p6;
        let mut p5: *mut u32;
        if 1 < local_1c {
            let mut iv3 = local_1c.wrapping_sub(1);
            let mut k = 1;
            loop {
                p5 = slot(param_1, k);
                if (g(p5, 1) as i32) < (g(p7, 1) as i32) {
                    p7 = p5;
                }
                if (g(p6, 0) as i32) < (g(p5, 0) as i32) {
                    p6 = p5;
                }
                k += 1;
                iv3 = iv3.wrapping_sub(1);
                if iv3 == 0 {
                    break;
                }
            }
        }
        let mut u10 = g(p7, 1);
        wr(param_1, 0x40, u10);
        let mut u9 = g(p6, 0);
        wr(param_1, 0x44, u9);
        let mut iv3 = u9.wrapping_sub(u10) as i32;
        if DAT_006a883c <= iv3 {
            wr(param_1, 0x44, u10);
            return;
        }
        DAT_006a883c = DAT_006a883c.wrapping_sub(iv3);
        iv3 = rd(param_1, 100) as i32;
        let tmp_g = u32::from_le_bytes(core::ptr::read_unaligned(&raw const DAT_006a8832));
        wr(
            param_1,
            0x68,
            tmp_g.wrapping_add((DAT_006a883c as u32).wrapping_mul(4)),
        );
        let mut i8_ = 0i32;
        let mut b11 = iv3 == 0;
        if 0 < iv3 {
            loop {
                let s = slot(param_1, i8_);
                if s != p7 && g(s, 1) == g(p7, 1) {
                    p6 = slot(param_1, i8_);
                    break;
                }
                i8_ += 1;
                if !(i8_ < iv3) {
                    break;
                }
            }
            b11 = i8_ == iv3;
        }
        if b11 {
            let v = rd(param_1, 0x40);
            wr(param_1, 0x44, v);
            return;
        }
        if gf(p6, 2) < gf(p7, 2) || (gf(p6, 2) == gf(p7, 2) && gf(p6, 4) < gf(p7, 4)) {
            p5 = p7;
            p7 = p6;
        } else {
            p5 = p6;
        }
        wr(param_1, 0x38, p7 as usize as u32);
        u10 = g(p7, 1);
        u9 = g(p5, 0);
        if (g(p7, 0) as i32) < (g(p5, 0) as i32) {
            u9 = g(p7, 0);
        }
        iv3 = u9.wrapping_sub(u10) as i32;
        let mut local_14 = gf(p7, 2);
        let mut local_10 = gf(p5, 2);
        let mut local_8 = g(p7, 3);
        let mut local_4 = g(p5, 3);
        if _DAT_005201dc < local_10 - local_14 {
            loop {
                iv3 = iv3.wrapping_sub(1);
                u9 = u10.wrapping_add(1);
                wrf(
                    param_1,
                    0x34,
                    (gf(p5, 3) - gf(p7, 3)) / (gf(p5, 2) - gf(p7, 2)),
                );
                FUN_004663b0(gf(p7, 2), gf(p5, 2), u10, param_1);
                if iv3 < 1 {
                    break;
                }
                sf(p7, 2, gf(p7, 4) + gf(p7, 2));
                sf(p7, 3, gf(p7, 5) + gf(p7, 3));
                sf(p5, 2, gf(p5, 4) + gf(p5, 2));
                sf(p5, 3, gf(p5, 5) + gf(p5, 3));
                u10 = u9;
            }
        } else {
            let f1 = iv3 as f32;
            wrf(
                param_1,
                0x34,
                ((f1 * gf(p5, 5) + gf(p5, 3)) - (f1 * gf(p7, 5) + gf(p7, 3)))
                    / ((f1 * gf(p5, 4) + gf(p5, 2)) - (f1 * gf(p7, 4) + gf(p7, 2))),
            );
            loop {
                iv3 = iv3.wrapping_sub(1);
                u9 = u10.wrapping_add(1);
                FUN_004663b0(gf(p7, 2), gf(p5, 2), u10, param_1);
                if iv3 < 1 {
                    break;
                }
                sf(p7, 2, gf(p7, 2) + gf(p7, 4));
                sf(p7, 3, gf(p7, 3) + gf(p7, 5));
                sf(p5, 2, gf(p5, 2) + gf(p5, 4));
                sf(p5, 3, gf(p5, 5) + gf(p5, 3));
                u10 = u9;
            }
        }
        'outer: loop {
            'fin: {
                'to803: {
                    if local_1c < 1 {
                        break 'fin;
                    }
                    if g(p5, 0) == g(p7, 0) {
                        local_1c -= 2;
                        if local_1c == 0 {
                            break 'fin;
                        }
                        sf(p7, 2, local_14);
                        sf(p5, 2, local_10);
                        sw(p7, 3, local_8);
                        sw(p5, 3, local_4);
                        iv3 = rd(param_1, 100) as i32;
                        'f7c: {
                            let (idx, s) = find(param_1, 0, iv3, u9);
                            i8_ = idx;
                            if idx != iv3 {
                                p7 = s;
                            }
                            if idx == iv3 {
                                break 'f7c;
                            }
                            i8_ += 1;
                            if i8_ < iv3 {
                                let (idx2, s2) = find(param_1, i8_, iv3, u9);
                                i8_ = idx2;
                                if idx2 != iv3 {
                                    p5 = s2;
                                }
                            }
                            if i8_ != iv3 {
                                if gf(p5, 2) < gf(p7, 2)
                                    || (gf(p5, 2) == gf(p7, 2) && gf(p5, 4) < gf(p7, 4))
                                {
                                    p6 = p7;
                                    p7 = p5;
                                } else {
                                    p6 = p5;
                                }
                                wr(param_1, 0x38, p7 as usize as u32);
                                local_8 = g(p7, 3);
                                local_14 = gf(p7, 2);
                                local_10 = gf(p6, 2);
                                local_4 = g(p6, 3);
                                break 'to803;
                            }
                        }
                        wr(param_1, 0x44, u9);
                        break 'fin;
                    }
                    local_1c -= 1;
                    p6 = p5;
                    if g(p7, 0) == u9 {
                        sf(p7, 2, local_14);
                        sw(p7, 3, local_8);
                        iv3 = rd(param_1, 100) as i32;
                        let (idx, s) = find(param_1, 0, iv3, u9);
                        if idx != iv3 {
                            p7 = s;
                        }
                        if idx == iv3 {
                            wr(param_1, 0x44, u9);
                            break 'fin;
                        }
                        wr(param_1, 0x38, p7 as usize as u32);
                        local_14 = gf(p7, 2);
                        local_8 = g(p7, 3);
                        sf(p5, 2, gf(p5, 4) + gf(p5, 2));
                        sf(p5, 3, gf(p5, 5) + gf(p5, 3));
                    } else {
                        sf(p5, 2, local_10);
                        sw(p5, 3, local_4);
                        iv3 = rd(param_1, 100) as i32;
                        let (idx, s) = find(param_1, 0, iv3, u9);
                        if idx != iv3 {
                            p5 = s;
                            p6 = p5;
                        }
                        p5 = p6;
                        if idx == iv3 {
                            wr(param_1, 0x44, u9);
                            break 'fin;
                        }
                        local_10 = gf(p6, 2);
                        local_4 = g(p6, 3);
                        sf(p7, 2, gf(p7, 4) + gf(p7, 2));
                        sf(p7, 3, gf(p7, 5) + gf(p7, 3));
                    }
                }
                // LAB_00485803
                if local_1c == 1 {
                    return;
                }
                p5 = p6;
                if local_1c == 2 {
                    if g(p6, 0) != g(p7, 0) {
                        return;
                    }
                    iv3 = g(p7, 0).wrapping_sub(u9) as i32;
                    u10 = u9;
                    let f1 = iv3 as f32;
                    if _DAT_005201dc < ((gf(p6, 4) * f1 + gf(p6, 2)) - (gf(p7, 4) * f1 + gf(p7, 2)))
                    {
                        loop {
                            iv3 = iv3.wrapping_sub(1);
                            u9 = u10.wrapping_add(1);
                            wrf(
                                param_1,
                                0x34,
                                (gf(p6, 3) - gf(p7, 3)) / (gf(p6, 2) - gf(p7, 2)),
                            );
                            FUN_004663b0(gf(p7, 2), gf(p6, 2), u10, param_1);
                            if iv3 < 1 {
                                break;
                            }
                            sf(p7, 2, gf(p7, 2) + gf(p7, 4));
                            sf(p7, 3, gf(p7, 3) + gf(p7, 5));
                            sf(p6, 2, gf(p6, 2) + gf(p6, 4));
                            sf(p6, 3, gf(p6, 5) + gf(p6, 3));
                            u10 = u9;
                        }
                    } else {
                        wrf(
                            param_1,
                            0x34,
                            (gf(p6, 3) - gf(p7, 3)) / (gf(p6, 2) - gf(p7, 2)),
                        );
                        loop {
                            iv3 = iv3.wrapping_sub(1);
                            u9 = u10.wrapping_add(1);
                            FUN_004663b0(gf(p7, 2), gf(p6, 2), u10, param_1);
                            if iv3 < 1 {
                                break;
                            }
                            sf(p7, 2, gf(p7, 2) + gf(p7, 4));
                            sf(p7, 3, gf(p7, 3) + gf(p7, 5));
                            sf(p6, 2, gf(p6, 2) + gf(p6, 4));
                            sf(p6, 3, gf(p6, 5) + gf(p6, 3));
                            u10 = u9;
                        }
                    }
                } else {
                    u10 = u9;
                    if (g(p7, 0) as i32) < (g(p6, 0) as i32) {
                        iv3 = g(p7, 0).wrapping_sub(u9) as i32;
                    } else {
                        iv3 = g(p6, 0).wrapping_sub(u9) as i32;
                    }
                    loop {
                        iv3 = iv3.wrapping_sub(1);
                        u9 = u10.wrapping_add(1);
                        wrf(
                            param_1,
                            0x34,
                            (gf(p6, 3) - gf(p7, 3)) / (gf(p6, 2) - gf(p7, 2)),
                        );
                        FUN_004663b0(gf(p7, 2), gf(p6, 2), u10, param_1);
                        if iv3 < 1 {
                            break;
                        }
                        sf(p7, 2, gf(p7, 2) + gf(p7, 4));
                        sf(p7, 3, gf(p7, 3) + gf(p7, 5));
                        sf(p6, 2, gf(p6, 2) + gf(p6, 4));
                        sf(p6, 3, gf(p6, 5) + gf(p6, 3));
                        u10 = u9;
                    }
                }
                continue 'outer;
            }
            // LAB_0048597f
            sf(p7, 2, local_14);
            sf(p5, 2, local_10);
            sw(p7, 3, local_8);
            sw(p5, 3, local_4);
            return;
        }
    }
}
