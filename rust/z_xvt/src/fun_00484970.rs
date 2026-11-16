// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006a884a: i32;
    static mut DAT_006a885e: i32;
    static mut DAT_006a8858: i32;
    static mut DAT_006a8874: i32;
    static mut DAT_007091ae: u16;
    static mut DAT_006d4984: u16;
    static mut DAT_00709ffc: i32;
    static mut DAT_00732f3c: u32;
    static mut DAT_006a88b0: f32;
    static mut _DAT_005201e8: f32;
    static mut _DAT_005201dc: f32;
    static mut _DAT_005201d4: f32;
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

unsafe fn rf(p: *mut f32, i: usize) -> f32 {
    // SAFETY: caller guarantees p+i points at a readable float.
    unsafe { p.add(i).read_unaligned() }
}

unsafe fn wf(p: *mut f32, i: usize, v: f32) {
    // SAFETY: caller guarantees p+i points at a writable float.
    unsafe { p.add(i).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00484970(param_1: *mut i32) {
    // SAFETY: all pointers are the game's own structures and globals; they are
    // read and written exactly where the original does.
    unsafe {
        let ri = |i: usize| -> i32 { param_1.add(i).read_unaligned() };
        let g_885e: i32 = (&raw mut DAT_006a885e).read_unaligned();
        let g_884a: i32 = (&raw mut DAT_006a884a).read_unaligned();
        let mut iv1: i32 = g_885e;
        let mut iv7: i32 = 0;
        let mut local_20: f32 = f32::from_bits(DAT_00732f3c);
        let mut f_stack_1c: f32 = 0.0;
        let u_var2: u64 = DAT_00732f3c as u64;
        let mut pi11: *mut i32 = (ri(0x2e) as u32)
            .wrapping_mul(0x70)
            .wrapping_add(g_884a as u32) as *mut i32;
        let pf1: *mut f32 = param_1 as *mut f32;
        param_1.add(0x2f).write_unaligned(g_885e);
        let mut iv6: i32 = DAT_006a8858;
        param_1.add(0x32).write_unaligned(0);
        let mut pf9: *mut f32 =
            (iv6 as u32).wrapping_add((iv1 as u32).wrapping_mul(0x18)) as *mut f32;
        iv6 = 0;
        let local_10: f32 = ((u_var2 as f32) / rf(pf1, 4)) * _DAT_005201e8;
        if 0 < ri(0x1e) {
            loop {
                iv7 = iv7.wrapping_add(4);
                iv6 = iv6.wrapping_add(1);
                ((DAT_006a8874 as u32)
                    .wrapping_sub(4)
                    .wrapping_add(iv7 as u32) as *mut i32)
                    .write_unaligned(-1);
                if !(iv6 < ri(0x1e)) {
                    break;
                }
            }
        }
        let mut local_24: i32 = 0;
        if 0 < ri(0x31) {
            loop {
                let pf11: *mut f32 = pi11 as *mut f32;
                FUN_00414d00(
                    pi11 as i32,
                    (ri(0x26) as u32)
                        .wrapping_add((pi11.read_unaligned() as u32).wrapping_mul(0x18))
                        as *mut u32,
                    param_1.add(2) as i32,
                );
                iv6 = ri(0x28);
                pi11.add(0x12).write_unaligned(0);
                let mut local_28: i32 = 0;
                wf(pf11, 0x13, u_var2 as f32);
                let mut pi5: *mut i32 = (pi11.read_unaligned() as u32)
                    .wrapping_mul(0x40)
                    .wrapping_add(iv6 as u32) as *mut i32;
                let mut pi10: *mut i32;
                let mut local_4c: f32;
                let mut local_38: i32;
                loop {
                    pi10 = pi5;
                    iv6 = pi10.read_unaligned();
                    local_38 = pi10.add(0xc).read_unaligned();
                    if iv6 == -1 {
                        break;
                    }
                    iv1 = ((DAT_006a8874 as u32).wrapping_add((iv6 as u32).wrapping_mul(4))
                        as *mut i32)
                        .read_unaligned();
                    if iv1 == -1 {
                        ((DAT_006a8874 as u32).wrapping_add((iv6 as u32).wrapping_mul(4))
                            as *mut i32)
                            .write_unaligned(ri(0x32));
                        param_1.add(0x32).write_unaligned(ri(0x32).wrapping_add(1));
                        let local_14: i32 = ((iv6 as u32).wrapping_mul(0xc)) as i32;
                        let pf8: *mut f32 =
                            (ri(0x1f) as u32).wrapping_add(local_14 as u32) as *mut f32;
                        let mut local_44: f32 = rf(pf8, 0);
                        let mut local_40: f32 = rf(pf8, 1);
                        let mut local_3c: f32 = rf(pf8, 2);
                        FUN_00414fe0(&raw mut local_44, param_1.add(5) as *mut f32);
                        local_44 = rf(pf1, 2) + local_44;
                        local_40 = rf(pf1, 3) + local_40;
                        local_3c = rf(pf1, 4) + local_3c + _DAT_005201e8;
                        wf(pf9, 2, local_10 / local_3c);
                        wf(pf9, 0, rf(pf9, 2) * local_44);
                        wf(pf9, 1, rf(pf9, 2) * local_40);
                        wf(pf9, 0, ((DAT_007091ae >> 1) as i32 as f32) + rf(pf9, 0));
                        wf(
                            pf9,
                            1,
                            (((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc as u32) as i32
                                as f32)
                                + rf(pf9, 1),
                        );
                        local_4c = rf(pf9, 2);
                        FUN_00414070(
                            param_1,
                            pf9 as i32,
                            (ri(0x21) as u32).wrapping_add((local_38 as u32).wrapping_mul(0xc))
                                as *mut f32,
                            (ri(0x1f) as u32).wrapping_add(local_14 as u32) as *mut f32,
                            &raw mut DAT_006a88b0,
                        );
                        pf9 = pf9.add(6);
                    } else {
                        local_4c = ((DAT_006a8858 as u32)
                            .wrapping_add(8)
                            .wrapping_add(((ri(0x2f)).wrapping_add(iv1) as u32).wrapping_mul(0x18))
                            as *mut f32)
                            .read_unaligned();
                    }
                    if rf(pf11, 0x12) < local_4c {
                        wf(pf11, 0x12, local_4c);
                    }
                    if local_4c < rf(pf11, 0x13) {
                        wf(pf11, 0x13, local_4c);
                    }
                    local_28 = local_28.wrapping_add(1);
                    pi5 = pi10.add(1);
                    if !(local_28 < 4) {
                        break;
                    }
                }
                if ri(0x20) != 0 {
                    iv1 = pi10.add(5).read_unaligned();
                    let idx3 = pi10.offset(-3).read_unaligned();
                    let mut local_44: f32 = ((ri(0x1f) as u32)
                        .wrapping_add((idx3 as u32).wrapping_mul(0xc))
                        as *mut f32)
                        .read_unaligned();
                    iv6 = (ri(0x1f) as u32).wrapping_add((idx3 as u32).wrapping_mul(0xc)) as i32;
                    let mut local_40: f32 =
                        ((iv6 as u32).wrapping_add(4) as *mut f32).read_unaligned();
                    let mut local_3c: f32 =
                        ((iv6 as u32).wrapping_add(8) as *mut f32).read_unaligned();
                    FUN_00414fe0(&raw mut local_44, param_1.add(5) as *mut f32);
                    local_44 = rf(pf1, 2) + local_44;
                    local_40 = rf(pf1, 3) + local_40;
                    local_3c = rf(pf1, 4) + local_3c + _DAT_005201e8;
                    let pf8: *mut f32 =
                        (ri(0x20) as u32).wrapping_add((iv1 as u32).wrapping_mul(8)) as *mut f32;
                    wf(
                        pf11,
                        10,
                        (local_44 - rf(pf8, 1) * rf(pf11, 7)) - rf(pf8, 0) * rf(pf11, 4),
                    );
                    wf(
                        pf11,
                        0xb,
                        (local_40 - rf(pf8, 1) * rf(pf11, 8)) - rf(pf8, 0) * rf(pf11, 5),
                    );
                    wf(
                        pf11,
                        0xc,
                        (local_3c - rf(pf8, 1) * rf(pf11, 9)) - rf(pf8, 0) * rf(pf11, 6),
                    );
                    local_20 = rf(pf11, 0xc) * rf(pf11, 8) - rf(pf11, 9) * rf(pf11, 0xb);
                    f_stack_1c = rf(pf11, 9) * rf(pf11, 10) - rf(pf11, 0xc) * rf(pf11, 7);
                    let f_stack_18 = rf(pf11, 0xb) * rf(pf11, 7) - rf(pf11, 10) * rf(pf11, 8);
                    let f_stack_c = rf(pf11, 6) * rf(pf11, 0xb) - rf(pf11, 0xc) * rf(pf11, 5);
                    let f_stack_8 = rf(pf11, 0xc) * rf(pf11, 4) - rf(pf11, 10) * rf(pf11, 6);
                    let f_stack_4 = rf(pf11, 5) * rf(pf11, 10) - rf(pf11, 0xb) * rf(pf11, 4);
                    let f_stack_34 = rf(pf11, 9) * rf(pf11, 5) - rf(pf11, 6) * rf(pf11, 8);
                    let f_stack_30 = rf(pf11, 6) * rf(pf11, 7) - rf(pf11, 9) * rf(pf11, 4);
                    let mut f_stack_2c = rf(pf11, 8) * rf(pf11, 4) - rf(pf11, 5) * rf(pf11, 7);
                    let ab = |x: f32| -> f32 { if x < 0.0 { -x } else { x } };
                    if ab(f_stack_34) == 0.0 && ab(f_stack_30) == 0.0 && ab(f_stack_2c) == 0.0 {
                        f_stack_2c = 1.0;
                    }
                    let f_var3: f32 = _DAT_005201dc
                        / (rf(pf11, 0xb) * f_stack_30
                            + rf(pf11, 10) * f_stack_34
                            + rf(pf11, 0xc) * f_stack_2c);
                    let f_var4: f32 = f_var3 / local_10;
                    wf(pf11, 4, local_20 * f_var4);
                    wf(pf11, 5, f_stack_1c * f_var4);
                    wf(pf11, 6, f_stack_18 * f_var3);
                    wf(pf11, 7, f_stack_c * f_var4);
                    wf(pf11, 8, f_stack_8 * f_var4);
                    wf(pf11, 9, f_stack_4 * f_var3);
                    wf(pf11, 10, f_stack_34 * f_var4);
                    wf(pf11, 0xb, f_var4 * f_stack_30);
                    wf(pf11, 0xc, f_var3 * f_stack_2c);
                    let half_w = (DAT_007091ae >> 1) as i32 as f32;
                    let half_h = ((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc as u32)
                        as i32 as f32;
                    wf(pf11, 6, rf(pf11, 6) - half_w * rf(pf11, 4));
                    wf(pf11, 6, rf(pf11, 6) - half_h * rf(pf11, 5));
                    wf(pf11, 9, rf(pf11, 9) - half_w * rf(pf11, 7));
                    wf(pf11, 9, rf(pf11, 9) - half_h * rf(pf11, 8));
                    wf(pf11, 0xc, rf(pf11, 0xc) - half_w * rf(pf11, 10));
                    wf(pf11, 0xc, rf(pf11, 0xc) - half_h * rf(pf11, 0xb));
                    let mut local_38f: f32 = rf(pf11, 8) * rf(pf11, 4) * local_3c * local_3c;
                    if local_38f < _DAT_005201d4 {
                        local_38f = -local_38f;
                    }
                    let l12: i32 = local_38f as i32;
                    pi11.add(0x1b).write_unaligned(l12);
                    local_38f = rf(pf11, 5) * rf(pf11, 7) * local_3c * local_3c;
                    if local_38f < _DAT_005201d4 {
                        local_38f = -local_38f;
                    }
                    let l13: i32 = local_38f as i32;
                    pi11.add(0x1b).write_unaligned(l13.wrapping_add(l12));
                }
                pi11 = pi11.add(0x1c);
                local_24 = local_24.wrapping_add(1);
                if !(local_24 < ri(0x31)) {
                    break;
                }
            }
        }
        let _ = (local_20, f_stack_1c);
        let mut g: i32 = (&raw mut DAT_006a885e).read_unaligned();
        g = g.wrapping_add(ri(0x32));
        (&raw mut DAT_006a885e).write_unaligned(g);
    }
}
