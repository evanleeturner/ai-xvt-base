// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006a885e: i32;
    static mut DAT_006a884a: i32;
    static mut DAT_006a8858: i32;
    static mut DAT_006a8874: i32;
    static mut DAT_00732f3c: f32;
    static mut _DAT_00520194: f32;
    static mut _DAT_00520190: f32;
    static mut _DAT_005201ac: f32;
    static mut _DAT_005201a8: f32;
    static mut _DAT_006a8a08: f32;
    static mut DAT_006a88b0: f32;
    static mut DAT_007091ae: u16;
    static mut DAT_006d4984: u16;
    static mut DAT_00709ffc: i32;
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

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables)]
pub unsafe extern "C" fn FUN_0045c910(param_1: *mut i32) {
    // SAFETY: param_1 points at the game's structure; every other pointer is
    // derived from the game's own tables and globals exactly as the original
    // does, and the callees are the game's own.
    unsafe {
        let p1 = param_1;
        let pf1 = param_1 as *mut f32;

        let tmp_885e = (&raw mut DAT_006a885e).read_unaligned();
        let tmp_884a = (&raw mut DAT_006a884a).read_unaligned();

        let mut i_var5 = tmp_885e;
        let mut piVar10 = ((*p1.add(0x2e) as u32)
            .wrapping_mul(0x70)
            .wrapping_add(tmp_884a as u32)) as *mut i32;
        let mut pf10 = piVar10 as *mut f32;
        *p1.add(0x2f) = tmp_885e;
        let mut i_var6 = DAT_006a8858;
        let mut i_var4: i32 = 0;
        *p1.add(0x32) = 0;
        let mut pf_var8 =
            (i_var6 as u32).wrapping_add((i_var5 as u32).wrapping_mul(0x18)) as *mut f32;
        i_var6 = 0;
        if 0 < *p1.add(0x1e) {
            loop {
                i_var4 = i_var4.wrapping_add(4);
                i_var6 = i_var6.wrapping_add(1);
                let a = (DAT_006a8874 as u32)
                    .wrapping_sub(4)
                    .wrapping_add(i_var4 as u32) as *mut i32;
                *a = -1;
                if !(i_var6 < *p1.add(0x1e)) {
                    break;
                }
            }
        }
        let mut local_20: i32 = 0;
        if 0 < *p1.add(0x31) {
            loop {
                FUN_00414d00(
                    piVar10 as usize as i32,
                    (*p1.add(0x26) as u32).wrapping_add((*piVar10 as u32).wrapping_mul(0x18))
                        as *mut u32,
                    p1.add(2) as usize as i32,
                );
                i_var6 = *p1.add(0x28);
                *piVar10.add(0x12) = 0;
                let mut local_3c: f32 = 0.0;
                let mut local_24: i32 = 0;
                *pf10.add(0x13) = DAT_00732f3c;
                let mut pi_var3 = (*piVar10 as u32)
                    .wrapping_mul(0x40)
                    .wrapping_add(i_var6 as u32) as *mut i32;
                let mut pi_var9: *mut i32;
                let mut local_50: f32 = 0.0;
                let mut local_38: i32;
                loop {
                    pi_var9 = pi_var3;
                    i_var6 = *pi_var9;
                    local_38 = *pi_var9.add(8);
                    let local_34i = *pi_var9.add(0xc);
                    if i_var6 == -1 {
                        break;
                    }
                    let slot = (DAT_006a8874 as u32).wrapping_add((i_var6 as u32).wrapping_mul(4))
                        as *mut i32;
                    i_var5 = *slot;
                    if i_var5 == -1 {
                        *slot = *p1.add(0x32);
                        let local_1c = (i_var6 as u32).wrapping_mul(0xc) as i32;
                        *p1.add(0x32) = (*p1.add(0x32) as u32).wrapping_add(1) as i32;
                        let pf_var7 =
                            (*p1.add(0x1f) as u32).wrapping_add(local_1c as u32) as *mut f32;
                        let mut v: [f32; 3] = [*pf_var7, *pf_var7.add(1), *pf_var7.add(2)];
                        FUN_00414fe0(v.as_mut_ptr(), p1.add(5) as *mut f32);
                        v[0] = *pf1.add(2) + v[0];
                        v[1] = *pf1.add(3) + v[1];
                        v[2] = *pf1.add(4) + v[2];
                        if _DAT_00520194 <= v[2] {
                            *pf_var8.add(2) = DAT_00732f3c / v[2];
                            *pf_var8 = *pf_var8.add(2) * v[0];
                            *pf_var8.add(1) = *pf_var8.add(2) * v[1];
                            *pf_var8 = ((DAT_007091ae >> 1) as f32) + *pf_var8;
                            *pf_var8.add(1) = ((((DAT_006d4984 >> 1) as u32)
                                .wrapping_add(DAT_00709ffc as u32))
                                as i32 as f32)
                                + *pf_var8.add(1);
                            local_50 = *pf_var8.add(2);
                        } else {
                            *pf_var8.add(2) = v[2] - _DAT_00520194;
                            *pf_var8 = v[0];
                            *pf_var8.add(1) = v[1];
                            *piVar10.add(3) = -1;
                            local_50 = DAT_00732f3c;
                        }
                        FUN_00414070(
                            p1,
                            pf_var8 as usize as i32,
                            (*p1.add(0x21) as u32)
                                .wrapping_add((local_34i as u32).wrapping_mul(0xc))
                                as *mut f32,
                            (*p1.add(0x1f) as u32).wrapping_add(local_1c as u32) as *mut f32,
                            &raw mut DAT_006a88b0,
                        );
                        let u_off = (local_38 as u32).wrapping_mul(8);
                        *pf_var8.add(4) =
                            *((*p1.add(0x20) as u32).wrapping_add(u_off) as *const f32);
                        *pf_var8.add(5) =
                            *((*p1.add(0x20) as u32).wrapping_add(4).wrapping_add(u_off)
                                as *const f32);
                        pf_var8 = pf_var8.add(6);
                    } else {
                        i_var5 = (*p1.add(0x2f) as u32).wrapping_add(i_var5 as u32) as i32;
                        let base =
                            (DAT_006a8858 as u32).wrapping_add((i_var5 as u32).wrapping_mul(0x18));
                        if *(base.wrapping_add(8) as *const u32) < 0x80000001 {
                            local_50 = *(base.wrapping_add(8) as *const f32);
                        } else {
                            *piVar10.add(3) = -1;
                            local_50 = DAT_00732f3c;
                        }
                    }
                    local_3c = local_3c + local_50;
                    if *pf10.add(0x12) < local_50 {
                        *pf10.add(0x12) = local_50;
                    }
                    if local_50 < *pf10.add(0x13) {
                        *pf10.add(0x13) = local_50;
                    }
                    local_24 = local_24.wrapping_add(1);
                    pi_var3 = pi_var9.add(1);
                    if !(local_24 < 4) {
                        break;
                    }
                }
                if *p1.add(0x20) != 0 {
                    local_38 = *pi_var9.add(5);
                    let pf_var7 = (*p1.add(0x1f) as u32)
                        .wrapping_add(((*pi_var9.offset(-3)) as u32).wrapping_mul(0xc))
                        as *mut f32;
                    let mut v: [f32; 3] = [*pf_var7, *pf_var7.add(1), *pf_var7.add(2)];
                    FUN_00414fe0(v.as_mut_ptr(), p1.add(5) as *mut f32);
                    v[0] = *pf1.add(2) + v[0];
                    v[1] = *pf1.add(3) + v[1];
                    v[2] = *pf1.add(4) + v[2];
                    let u_off = (local_38 as u32).wrapping_mul(8);
                    let q0 = *((*p1.add(0x20) as u32).wrapping_add(u_off) as *const f32);
                    let q1 =
                        *((*p1.add(0x20) as u32).wrapping_add(4).wrapping_add(u_off) as *const f32);
                    *pf10.add(10) = (v[0] - q1 * *pf10.add(7)) - q0 * *pf10.add(4);
                    *pf10.add(0xb) = (v[1] - q1 * *pf10.add(8)) - q0 * *pf10.add(5);
                    *pf10.add(0xc) = (v[2] - q1 * *pf10.add(9)) - q0 * *pf10.add(6);
                    let f_stack_18 = *pf10.add(0xc) * *pf10.add(8) - *pf10.add(9) * *pf10.add(0xb);
                    let f_stack_14 = *pf10.add(9) * *pf10.add(10) - *pf10.add(0xc) * *pf10.add(7);
                    let f_stack_10 = *pf10.add(0xb) * *pf10.add(7) - *pf10.add(10) * *pf10.add(8);
                    let f_stack_c = *pf10.add(6) * *pf10.add(0xb) - *pf10.add(0xc) * *pf10.add(5);
                    let f_stack_8 = *pf10.add(0xc) * *pf10.add(4) - *pf10.add(10) * *pf10.add(6);
                    let f_stack_4 = *pf10.add(5) * *pf10.add(10) - *pf10.add(0xb) * *pf10.add(4);
                    let f_stack_30 = *pf10.add(9) * *pf10.add(5) - *pf10.add(6) * *pf10.add(8);
                    let f_stack_2c = *pf10.add(6) * *pf10.add(7) - *pf10.add(9) * *pf10.add(4);
                    let mut f_stack_28 = *pf10.add(8) * *pf10.add(4) - *pf10.add(5) * *pf10.add(7);
                    let abs = |x: f32| if x < 0.0 { -x } else { x };
                    if abs(f_stack_30) == 0.0 && abs(f_stack_2c) == 0.0 && abs(f_stack_28) == 0.0 {
                        f_stack_28 = 1.0;
                    }
                    let mut f_var1 = _DAT_00520194
                        / (*pf10.add(0xb) * f_stack_2c
                            + *pf10.add(10) * f_stack_30
                            + *pf10.add(0xc) * f_stack_28);
                    let f_var2 = _DAT_006a8a08 * f_var1;
                    *pf10.add(4) = f_stack_18 * f_var2;
                    *pf10.add(5) = f_stack_14 * f_var2;
                    *pf10.add(6) = f_stack_10 * f_var1;
                    *pf10.add(7) = f_stack_c * f_var2;
                    *pf10.add(8) = f_stack_8 * f_var2;
                    *pf10.add(9) = f_stack_4 * f_var1;
                    *pf10.add(10) = f_stack_30 * f_var2;
                    *pf10.add(0xb) = f_var2 * f_stack_2c;
                    *pf10.add(0xc) = f_var1 * f_stack_28;
                    *pf10.add(6) = *pf10.add(6) - ((DAT_007091ae >> 1) as f32) * *pf10.add(4);
                    *pf10.add(6) = *pf10.add(6)
                        - ((((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc as u32)) as i32
                            as f32)
                            * *pf10.add(5);
                    *pf10.add(9) = *pf10.add(9) - ((DAT_007091ae >> 1) as f32) * *pf10.add(7);
                    *pf10.add(9) = *pf10.add(9)
                        - ((((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc as u32)) as i32
                            as f32)
                            * *pf10.add(8);
                    *pf10.add(0xc) = *pf10.add(0xc) - ((DAT_007091ae >> 1) as f32) * *pf10.add(10);
                    *pf10.add(0xc) = *pf10.add(0xc)
                        - ((((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc as u32)) as i32
                            as f32)
                            * *pf10.add(0xb);
                    let mut local_34 = *pf10.add(8) * *pf10.add(4) - *pf10.add(5) * *pf10.add(7);
                    if local_34 < _DAT_00520190 {
                        local_34 = -local_34;
                    }
                    f_var1 = _DAT_005201ac;
                    if *pi_var9 == -1 {
                        f_var1 = _DAT_005201a8;
                    }
                    local_3c = f_var1 / local_3c;
                    *piVar10.add(0x1b) = local_3c as i32;
                }
                piVar10 = piVar10.add(0x1c);
                pf10 = piVar10 as *mut f32;
                local_20 = local_20.wrapping_add(1);
                if !(local_20 < *p1.add(0x31)) {
                    break;
                }
            }
        }
        let mut t = (&raw mut DAT_006a885e).read_unaligned();
        t = (t as u32).wrapping_add(*p1.add(0x32) as u32) as i32;
        (&raw mut DAT_006a885e).write_unaligned(t);
    }
}
