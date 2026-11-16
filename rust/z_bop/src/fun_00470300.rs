// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0099944a: i32;
    static mut DAT_00999462: i32;
    static mut DAT_0099945c: i32;
    static mut DAT_00999478: i32;
    static mut DAT_00a081ec: i32;
    static mut DAT_009d7680: i32;
    static mut DAT_009d682c: u16;
    static mut DAT_009a1fe4: u16;
    static mut _DAT_005181b8: f32;
    static mut _DAT_005181c0: f32;
    static mut _DAT_005181c4: f32;
    static mut _DAT_005181c8: f32;
    static mut _DAT_0099940c: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00420db0(param_1: i32, param_2: *mut f32, param_3: i32);
    fn FUN_00420fe0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_004201f0(param_1: *mut i32, param_2: i32, param_3: *mut f32, param_4: *mut f32);
}

#[inline(always)]
fn pa(a: i32) -> usize {
    a as u32 as usize
}

#[inline(always)]
unsafe fn rd_i(a: i32) -> i32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (pa(a) as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd_f(a: i32) -> f32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (pa(a) as *const f32).read_unaligned() }
}

#[inline(always)]
unsafe fn g(p: *mut i32, k: usize) -> f32 {
    // SAFETY: p points at the game's 0x70-byte record.
    unsafe { p.cast::<f32>().add(k).read_unaligned() }
}

#[inline(always)]
unsafe fn s(p: *mut i32, k: usize, v: f32) {
    // SAFETY: p points at the game's 0x70-byte record.
    unsafe { p.cast::<f32>().add(k).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00470300(param_1: *mut i32) {
    // SAFETY: all pointers and globals belong to the game; accessed as the original does.
    unsafe {
        let p1 = |k: usize| -> i32 { param_1.add(k).read_unaligned() };
        let iv5 = DAT_00999462;
        let mut piv10: *mut i32 =
            pa(p1(0x2e).wrapping_mul(0x70).wrapping_add(DAT_0099944a)) as *mut i32;
        param_1.add(0x2f).write_unaligned(iv5);
        let iv6 = DAT_0099945c;
        let mut iv4: i32 = 0;
        param_1.add(0x32).write_unaligned(0);
        let mut pf8: *mut f32 = pa(iv6.wrapping_add(iv5.wrapping_mul(0x18))) as *mut f32;
        let mut iv6: i32 = 0;
        if 0 < p1(0x1e) {
            loop {
                iv4 = iv4.wrapping_add(4);
                iv6 = iv6.wrapping_add(1);
                (pa(DAT_00999478.wrapping_add(-4).wrapping_add(iv4)) as *mut i32)
                    .write_unaligned(-1);
                if !(iv6 < p1(0x1e)) {
                    break;
                }
            }
        }
        let mut local_20: i32 = 0;
        if 0 < p1(0x31) {
            loop {
                FUN_00420db0(
                    piv10 as usize as i32,
                    pa(p1(0x26).wrapping_add(piv10.read_unaligned().wrapping_mul(0x18)))
                        as *mut f32,
                    param_1.add(2) as usize as i32,
                );
                let iv6b = p1(0x28);
                piv10.add(0x12).write_unaligned(0);
                let mut local_38: f32 = 0.0;
                let mut local_24: i32 = 0;
                s(piv10, 0x13, DAT_00a081ec as f32);
                let mut piv3: *mut i32 =
                    pa(piv10.read_unaligned().wrapping_mul(0x40).wrapping_add(iv6b)) as *mut i32;
                let mut piv9: *mut i32;
                let mut local_34: f32;
                loop {
                    piv9 = piv3;
                    let iv6c = piv9.read_unaligned();
                    local_34 = f32::from_bits(piv9.add(0xc).read_unaligned() as u32);
                    if iv6c == -1 {
                        break;
                    }
                    let iv5c = rd_i(DAT_00999478.wrapping_add(iv6c.wrapping_mul(4)));
                    let local_4c: f32;
                    if iv5c == -1 {
                        let local_1c = iv6c.wrapping_mul(0xc);
                        (pa(DAT_00999478.wrapping_add(iv6c.wrapping_mul(4))) as *mut i32)
                            .write_unaligned(p1(0x32));
                        param_1.add(0x32).write_unaligned(p1(0x32).wrapping_add(1));
                        let pf7 = pa(p1(0x1f).wrapping_add(local_1c)) as *const f32;
                        let mut v: [f32; 3] = [
                            pf7.read_unaligned(),
                            pf7.add(1).read_unaligned(),
                            pf7.add(2).read_unaligned(),
                        ];
                        FUN_00420fe0(v.as_mut_ptr(), param_1.add(5).cast::<f32>());
                        v[0] = (p1(2) as f32) + v[0];
                        v[1] = (p1(3) as f32) + v[1];
                        v[2] = (p1(4) as f32) + v[2];
                        if _DAT_005181c0 <= v[2] {
                            pf8.add(2).write_unaligned((DAT_00a081ec as f32) / v[2]);
                            pf8.write_unaligned(pf8.add(2).read_unaligned() * v[0]);
                            pf8.add(1)
                                .write_unaligned(pf8.add(2).read_unaligned() * v[1]);
                            pf8.write_unaligned(
                                ((DAT_009d682c >> 1) as i32 as f32) + pf8.read_unaligned(),
                            );
                            pf8.add(1).write_unaligned(
                                (((DAT_009a1fe4 >> 1) as i32).wrapping_add(DAT_009d7680) as f32)
                                    + pf8.add(1).read_unaligned(),
                            );
                            local_4c = pf8.add(2).read_unaligned();
                        } else {
                            pf8.add(2).write_unaligned(v[2] - _DAT_005181c0);
                            pf8.write_unaligned(v[0]);
                            pf8.add(1).write_unaligned(v[1]);
                            piv10.add(3).write_unaligned(-1);
                            local_4c = DAT_00a081ec as f32;
                        }
                        FUN_004201f0(
                            param_1,
                            pf8 as usize as i32,
                            pa(
                                p1(0x21)
                                    .wrapping_add((local_34.to_bits() as i32).wrapping_mul(0xc)),
                            ) as *mut f32,
                            pa(p1(0x1f).wrapping_add(local_1c)) as *mut f32,
                        );
                        pf8 = pf8.add(6);
                    } else {
                        let iv5d = p1(0x2f).wrapping_add(iv5c);
                        let base = DAT_0099945c;
                        if (rd_i(base.wrapping_add(8).wrapping_add(iv5d.wrapping_mul(0x18))) as u32)
                            < 0x80000001u32
                        {
                            local_4c =
                                rd_f(base.wrapping_add(iv5d.wrapping_mul(0x18)).wrapping_add(8));
                        } else {
                            piv10.add(3).write_unaligned(-1);
                            local_4c = DAT_00a081ec as f32;
                        }
                    }
                    local_38 = local_38 + local_4c;
                    if g(piv10, 0x12) < local_4c {
                        s(piv10, 0x12, local_4c);
                    }
                    if local_4c < g(piv10, 0x13) {
                        s(piv10, 0x13, local_4c);
                    }
                    local_24 = local_24.wrapping_add(1);
                    piv3 = piv9.add(1);
                    if !(local_24 < 4) {
                        break;
                    }
                }
                if p1(0x20) != 0 {
                    let iv6d = piv9.add(5).read_unaligned();
                    let pf7 =
                        pa(p1(0x1f).wrapping_add(piv9.sub(3).read_unaligned().wrapping_mul(0xc)))
                            as *const f32;
                    let mut v: [f32; 3] = [
                        pf7.read_unaligned(),
                        pf7.add(1).read_unaligned(),
                        pf7.add(2).read_unaligned(),
                    ];
                    FUN_00420fe0(v.as_mut_ptr(), param_1.add(5).cast::<f32>());
                    v[0] = (p1(2) as f32) + v[0];
                    v[1] = (p1(3) as f32) + v[1];
                    v[2] = (p1(4) as f32) + v[2];
                    let x4 = rd_f(p1(0x20).wrapping_add(4).wrapping_add(iv6d.wrapping_mul(8)));
                    let x0 = rd_f(p1(0x20).wrapping_add(iv6d.wrapping_mul(8)));
                    s(piv10, 10, (v[0] - x4 * g(piv10, 7)) - x0 * g(piv10, 4));
                    let x4 = rd_f(p1(0x20).wrapping_add(4).wrapping_add(iv6d.wrapping_mul(8)));
                    let x0 = rd_f(p1(0x20).wrapping_add(iv6d.wrapping_mul(8)));
                    s(piv10, 0xb, (v[1] - x4 * g(piv10, 8)) - x0 * g(piv10, 5));
                    let x4 = rd_f(p1(0x20).wrapping_add(4).wrapping_add(iv6d.wrapping_mul(8)));
                    let x0 = rd_f(p1(0x20).wrapping_add(iv6d.wrapping_mul(8)));
                    s(piv10, 0xc, (v[2] - x4 * g(piv10, 9)) - x0 * g(piv10, 6));
                    let local_18 = g(piv10, 0xc) * g(piv10, 8) - g(piv10, 9) * g(piv10, 0xb);
                    let local_14 = g(piv10, 9) * g(piv10, 10) - g(piv10, 0xc) * g(piv10, 7);
                    let local_10 = g(piv10, 0xb) * g(piv10, 7) - g(piv10, 10) * g(piv10, 8);
                    let local_c = g(piv10, 6) * g(piv10, 0xb) - g(piv10, 0xc) * g(piv10, 5);
                    let local_8 = g(piv10, 0xc) * g(piv10, 4) - g(piv10, 10) * g(piv10, 6);
                    let local_4 = g(piv10, 5) * g(piv10, 10) - g(piv10, 0xb) * g(piv10, 4);
                    let local_30 = g(piv10, 9) * g(piv10, 5) - g(piv10, 6) * g(piv10, 8);
                    let local_2c = g(piv10, 6) * g(piv10, 7) - g(piv10, 9) * g(piv10, 4);
                    let mut local_28 = g(piv10, 8) * g(piv10, 4) - g(piv10, 5) * g(piv10, 7);
                    let ab = |x: f32| if x < 0.0 { -x } else { x };
                    if ab(local_30) == 0.0 && ab(local_2c) == 0.0 && ab(local_28) == 0.0 {
                        local_28 = 1.0;
                    }
                    let f1 = _DAT_005181c0
                        / (g(piv10, 0xb) * local_2c
                            + g(piv10, 10) * local_30
                            + g(piv10, 0xc) * local_28);
                    let f2 = _DAT_0099940c * f1;
                    s(piv10, 4, local_18 * f2);
                    s(piv10, 5, local_14 * f2);
                    s(piv10, 6, local_10 * f1);
                    s(piv10, 7, local_c * f2);
                    s(piv10, 8, local_8 * f2);
                    s(piv10, 9, local_4 * f1);
                    s(piv10, 10, local_30 * f2);
                    s(piv10, 0xb, f2 * local_2c);
                    s(piv10, 0xc, f1 * local_28);
                    let hx = (DAT_009d682c >> 1) as i32 as f32;
                    s(piv10, 6, g(piv10, 6) - hx * g(piv10, 4));
                    let hy = ((DAT_009a1fe4 >> 1) as i32).wrapping_add(DAT_009d7680) as f32;
                    s(piv10, 6, g(piv10, 6) - hy * g(piv10, 5));
                    let hx = (DAT_009d682c >> 1) as i32 as f32;
                    s(piv10, 9, g(piv10, 9) - hx * g(piv10, 7));
                    let hy = ((DAT_009a1fe4 >> 1) as i32).wrapping_add(DAT_009d7680) as f32;
                    s(piv10, 9, g(piv10, 9) - hy * g(piv10, 8));
                    let hx = (DAT_009d682c >> 1) as i32 as f32;
                    s(piv10, 0xc, g(piv10, 0xc) - hx * g(piv10, 10));
                    let hy = ((DAT_009a1fe4 >> 1) as i32).wrapping_add(DAT_009d7680) as f32;
                    s(piv10, 0xc, g(piv10, 0xc) - hy * g(piv10, 0xb));
                    local_34 = g(piv10, 8) * g(piv10, 4) - g(piv10, 5) * g(piv10, 7);
                    if local_34 < _DAT_005181b8 {
                        local_34 = -local_34;
                    }
                    let mut f1 = _DAT_005181c8;
                    if piv9.read_unaligned() == -1 {
                        f1 = _DAT_005181c4;
                    }
                    local_38 = f1 / local_38;
                    piv10.add(0x1b).write_unaligned(local_38 as i32);
                }
                piv10 = piv10.add(0x1c);
                local_20 = local_20.wrapping_add(1);
                if !(local_20 < p1(0x31)) {
                    break;
                }
            }
        }
        let t = DAT_00999462;
        DAT_00999462 = t.wrapping_add(p1(0x32));
    }
}
