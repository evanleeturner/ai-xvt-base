// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006a885e: i32;
    static mut DAT_006a884a: i32;
    static mut DAT_006a8858: i32;
    static mut DAT_006a8874: i32;
    static mut DAT_00732f3c: f32;
    static mut _DAT_005201dc: f32;
    static mut _DAT_005201d4: f32;
    static mut _DAT_005201e0: f32;
    static mut _DAT_005201e4: f32;
    static mut _DAT_006a8a08: f32;
    static mut DAT_007091ae: u16;
    static mut DAT_006d4984: u16;
    static mut DAT_00709ffc: i32;
    static mut DAT_006a88b0: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00414d00(param_1: i32, param_2: *mut u32, param_3: i32);
    fn FUN_00414fe0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_00414070(
        param_1: *mut i32,
        param_2: i32,
        param_3: *mut f32,
        param_4: *mut f32,
        param_5: *mut f32,
    );
}

unsafe fn gf(p: *mut i32, i: usize) -> f32 {
    // SAFETY: caller supplies a valid game structure pointer.
    unsafe { p.add(i).cast::<f32>().read_unaligned() }
}

unsafe fn sf(p: *mut i32, i: usize, v: f32) {
    // SAFETY: caller supplies a valid game structure pointer.
    unsafe { p.add(i).cast::<f32>().write_unaligned(v) }
}

unsafe fn gi(p: *mut i32, i: usize) -> i32 {
    // SAFETY: caller supplies a valid game structure pointer.
    unsafe { p.add(i).read_unaligned() }
}

fn ad(x: u32) -> usize {
    x as usize
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004842b0(param_1: *mut i32) {
    // SAFETY: all globals and pointers are the game's own memory, accessed
    // exactly where the original does.
    unsafe {
        let mut l44: [f32; 3];
        let mut i_var5 = DAT_006a885e;
        let mut p9 = ad((gi(param_1, 0x2e) as u32)
            .wrapping_mul(0x70)
            .wrapping_add(DAT_006a884a as u32)) as *mut i32;
        param_1.add(0x2f).write_unaligned(DAT_006a885e);
        let mut i_var4: i32 = 0;
        let mut p7 =
            ad((DAT_006a8858 as u32).wrapping_add((i_var5 as u32).wrapping_mul(0x18))) as *mut f32;
        i_var5 = 0;
        param_1.add(0x32).write_unaligned(0);
        if 0 < gi(param_1, 0x1e) {
            loop {
                i_var5 = i_var5.wrapping_add(4);
                i_var4 = i_var4.wrapping_add(1);
                let a = (DAT_006a8874 as u32)
                    .wrapping_add(0xfffffffc)
                    .wrapping_add(i_var5 as u32);
                (ad(a) as *mut i32).write_unaligned(-1);
                if !(i_var4 < gi(param_1, 0x1e)) {
                    break;
                }
            }
        }
        let mut local_20: i32 = 0;
        if 0 < gi(param_1, 0x31) {
            loop {
                FUN_00414d00(
                    p9 as usize as i32,
                    ad((gi(param_1, 0x26) as u32)
                        .wrapping_add((gi(p9, 0) as u32).wrapping_mul(0x18)))
                        as *mut u32,
                    param_1.add(2) as usize as i32,
                );
                i_var5 = gi(param_1, 0x28);
                p9.add(0x12).write_unaligned(0);
                let mut local_38: f32 = 0.0;
                let mut local_24: i32 = 0;
                sf(p9, 0x13, DAT_00732f3c);
                let mut p3 = ad((gi(p9, 0) as u32)
                    .wrapping_mul(0x40)
                    .wrapping_add(i_var5 as u32)) as *mut i32;
                let mut p8: *mut i32;
                loop {
                    p8 = p3;
                    i_var5 = gi(p8, 0);
                    let local_34i: i32 = gi(p8, 0xc);
                    if i_var5 == -1 {
                        break;
                    }
                    let local_4c: f32;
                    i_var4 =
                        ((ad((DAT_006a8874 as u32).wrapping_add((i_var5 as u32).wrapping_mul(4))))
                            as *mut i32)
                            .read_unaligned();
                    if i_var4 == -1 {
                        let local_1c = (i_var5 as u32).wrapping_mul(0xc) as i32;
                        (ad((DAT_006a8874 as u32).wrapping_add((i_var5 as u32).wrapping_mul(4)))
                            as *mut i32)
                            .write_unaligned(gi(param_1, 0x32));
                        param_1
                            .add(0x32)
                            .write_unaligned(gi(param_1, 0x32).wrapping_add(1));
                        let p6 = ad((gi(param_1, 0x1f) as u32).wrapping_add(local_1c as u32))
                            as *mut f32;
                        l44 = [
                            p6.read_unaligned(),
                            p6.add(1).read_unaligned(),
                            p6.add(2).read_unaligned(),
                        ];
                        FUN_00414fe0(l44.as_mut_ptr(), param_1.add(5).cast::<f32>());
                        l44[0] = gf(param_1, 2) + l44[0];
                        l44[1] = gf(param_1, 3) + l44[1];
                        l44[2] = gf(param_1, 4) + l44[2];
                        if _DAT_005201dc <= l44[2] {
                            p7.add(2).write_unaligned(DAT_00732f3c / l44[2]);
                            p7.write_unaligned(p7.add(2).read_unaligned() * l44[0]);
                            p7.add(1)
                                .write_unaligned(p7.add(2).read_unaligned() * l44[1]);
                            p7.write_unaligned((DAT_007091ae >> 1) as f32 + p7.read_unaligned());
                            let t = ((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc as u32)
                                as i32;
                            p7.add(1)
                                .write_unaligned(t as f32 + p7.add(1).read_unaligned());
                            local_4c = p7.add(2).read_unaligned();
                        } else {
                            p7.add(2).write_unaligned(l44[2] - _DAT_005201dc);
                            p7.write_unaligned(l44[0]);
                            p7.add(1).write_unaligned(l44[1]);
                            p9.add(3).write_unaligned(-1);
                            local_4c = DAT_00732f3c;
                        }
                        FUN_00414070(
                            param_1,
                            p7 as usize as i32,
                            ad((gi(param_1, 0x21) as u32)
                                .wrapping_add((local_34i as u32).wrapping_mul(0xc)))
                                as *mut f32,
                            ad((gi(param_1, 0x1f) as u32).wrapping_add(local_1c as u32))
                                as *mut f32,
                            &raw mut DAT_006a88b0,
                        );
                        p7 = p7.add(6);
                    } else {
                        i_var4 = (gi(param_1, 0x2f) as u32).wrapping_add(i_var4 as u32) as i32;
                        let base =
                            (DAT_006a8858 as u32).wrapping_add((i_var4 as u32).wrapping_mul(0x18));
                        if (ad(base.wrapping_add(8)) as *mut u32).read_unaligned() < 0x80000001 {
                            local_4c = (ad(base.wrapping_add(8)) as *mut f32).read_unaligned();
                        } else {
                            p9.add(3).write_unaligned(-1);
                            local_4c = DAT_00732f3c;
                        }
                    }
                    local_38 = local_38 + local_4c;
                    if gf(p9, 0x12) < local_4c {
                        sf(p9, 0x12, local_4c);
                    }
                    if local_4c < gf(p9, 0x13) {
                        sf(p9, 0x13, local_4c);
                    }
                    local_24 = local_24.wrapping_add(1);
                    p3 = p8.add(1);
                    if !(local_24 < 4) {
                        break;
                    }
                }
                if gi(param_1, 0x20) != 0 {
                    i_var5 = gi(p8, 5);
                    let p6 = ad((gi(param_1, 0x1f) as u32)
                        .wrapping_add((gi(p8.offset(-3), 0) as u32).wrapping_mul(0xc)))
                        as *mut f32;
                    l44 = [
                        p6.read_unaligned(),
                        p6.add(1).read_unaligned(),
                        p6.add(2).read_unaligned(),
                    ];
                    FUN_00414fe0(l44.as_mut_ptr(), param_1.add(5).cast::<f32>());
                    l44[0] = gf(param_1, 2) + l44[0];
                    l44[1] = gf(param_1, 3) + l44[1];
                    l44[2] = gf(param_1, 4) + l44[2];
                    let q = gi(param_1, 0x20) as u32;
                    let qa = (ad(q
                        .wrapping_add(4)
                        .wrapping_add((i_var5 as u32).wrapping_mul(8)))
                        as *mut f32)
                        .read_unaligned();
                    let qb = (ad(q.wrapping_add((i_var5 as u32).wrapping_mul(8))) as *mut f32)
                        .read_unaligned();
                    sf(p9, 10, (l44[0] - qa * gf(p9, 7)) - qb * gf(p9, 4));
                    let qa = (ad((gi(param_1, 0x20) as u32)
                        .wrapping_add(4)
                        .wrapping_add((i_var5 as u32).wrapping_mul(8)))
                        as *mut f32)
                        .read_unaligned();
                    let qb =
                        (ad((gi(param_1, 0x20) as u32)
                            .wrapping_add((i_var5 as u32).wrapping_mul(8)))
                            as *mut f32)
                            .read_unaligned();
                    sf(p9, 0xb, (l44[1] - qa * gf(p9, 8)) - qb * gf(p9, 5));
                    let qa = (ad((gi(param_1, 0x20) as u32)
                        .wrapping_add(4)
                        .wrapping_add((i_var5 as u32).wrapping_mul(8)))
                        as *mut f32)
                        .read_unaligned();
                    let qb =
                        (ad((gi(param_1, 0x20) as u32)
                            .wrapping_add((i_var5 as u32).wrapping_mul(8)))
                            as *mut f32)
                            .read_unaligned();
                    sf(p9, 0xc, (l44[2] - qa * gf(p9, 9)) - qb * gf(p9, 6));
                    let fs18 = gf(p9, 0xc) * gf(p9, 8) - gf(p9, 9) * gf(p9, 0xb);
                    let fs14 = gf(p9, 9) * gf(p9, 10) - gf(p9, 0xc) * gf(p9, 7);
                    let fs10 = gf(p9, 0xb) * gf(p9, 7) - gf(p9, 10) * gf(p9, 8);
                    let fsc = gf(p9, 6) * gf(p9, 0xb) - gf(p9, 0xc) * gf(p9, 5);
                    let fs8 = gf(p9, 0xc) * gf(p9, 4) - gf(p9, 10) * gf(p9, 6);
                    let fs4 = gf(p9, 5) * gf(p9, 10) - gf(p9, 0xb) * gf(p9, 4);
                    let fs30 = gf(p9, 9) * gf(p9, 5) - gf(p9, 6) * gf(p9, 8);
                    let fs2c = gf(p9, 6) * gf(p9, 7) - gf(p9, 9) * gf(p9, 4);
                    let mut fs28 = gf(p9, 8) * gf(p9, 4) - gf(p9, 5) * gf(p9, 7);
                    let ab = |x: f32| if x < 0.0 { -x } else { x };
                    if ab(fs30) == 0.0 && ab(fs2c) == 0.0 && ab(fs28) == 0.0 {
                        fs28 = 1.0;
                    }
                    let f1 = _DAT_005201dc
                        / (gf(p9, 0xb) * fs2c + gf(p9, 10) * fs30 + gf(p9, 0xc) * fs28);
                    let f2 = _DAT_006a8a08 * f1;
                    sf(p9, 4, fs18 * f2);
                    sf(p9, 5, fs14 * f2);
                    sf(p9, 6, fs10 * f1);
                    sf(p9, 7, fsc * f2);
                    sf(p9, 8, fs8 * f2);
                    sf(p9, 9, fs4 * f1);
                    sf(p9, 10, fs30 * f2);
                    sf(p9, 0xb, f2 * fs2c);
                    sf(p9, 0xc, f1 * fs28);
                    sf(p9, 6, gf(p9, 6) - (DAT_007091ae >> 1) as f32 * gf(p9, 4));
                    let t = ((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc as u32) as i32;
                    sf(p9, 6, gf(p9, 6) - t as f32 * gf(p9, 5));
                    sf(p9, 9, gf(p9, 9) - (DAT_007091ae >> 1) as f32 * gf(p9, 7));
                    let t = ((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc as u32) as i32;
                    sf(p9, 9, gf(p9, 9) - t as f32 * gf(p9, 8));
                    sf(
                        p9,
                        0xc,
                        gf(p9, 0xc) - (DAT_007091ae >> 1) as f32 * gf(p9, 10),
                    );
                    let t = ((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc as u32) as i32;
                    sf(p9, 0xc, gf(p9, 0xc) - t as f32 * gf(p9, 0xb));
                    let mut local_34 = gf(p9, 8) * gf(p9, 4) - gf(p9, 5) * gf(p9, 7);
                    if local_34 < _DAT_005201d4 {
                        local_34 = -local_34;
                    }
                    let _ = local_34;
                    let mut f1 = _DAT_005201e4;
                    if gi(p8, 0) == -1 {
                        f1 = _DAT_005201e0;
                    }
                    local_38 = f1 / local_38;
                    let l10 = local_38 as i64;
                    p9.add(0x1b).write_unaligned(l10 as i32);
                }
                p9 = p9.add(0x1c);
                local_20 = local_20.wrapping_add(1);
                if !(local_20 < gi(param_1, 0x31)) {
                    break;
                }
            }
        }
        DAT_006a885e = DAT_006a885e.wrapping_add(gi(param_1, 0x32));
    }
}
