// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732f3c: u32;
    static mut _DAT_005201b0: f32;
    static mut DAT_006a884a: i32;
    static mut DAT_006a885e: i32;
    static mut DAT_006a8858: i32;
    static mut DAT_006a8874: i32;
    static mut DAT_007091ae: u16;
    static mut DAT_006d4984: u16;
    static mut DAT_00709ffc: u32;
    static mut DAT_006a88b0: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
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
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045cff0(param_1: *mut i32, param_2: i32) {
    let _ = param_2;
    // SAFETY: all globals and callees are the game's own; param_1 and the
    // tables it points to are the caller's memory, accessed as the original does.
    unsafe {
        let pf_param1 = param_1 as *mut f32;
        let tmp_885e: i32 = (&raw const DAT_006a885e).read_unaligned();
        let tmp_884a: i32 = (&raw const DAT_006a884a).read_unaligned();

        let i_var2 = tmp_885e;
        let mut i_var4: u32 = 0;
        let u_var1 = DAT_00732f3c as u64;
        let mut pi_var8 = ((*param_1.add(0x2e) as u32)
            .wrapping_mul(0x70)
            .wrapping_add(tmp_884a as u32)) as *mut i32;
        let mut pf_var9 = pi_var8 as *mut f32;
        *param_1.add(0x2f) = tmp_885e;
        let i_var3_0 = DAT_006a8858;
        *param_1.add(0x32) = 0;
        let mut pf_var6 =
            (i_var3_0 as u32).wrapping_add((i_var2 as u32).wrapping_mul(0x18)) as *mut f32;
        let mut i_var3: i32 = 0;
        let local_8: f32 = ((u_var1 as f32) / *pf_param1.add(4)) * _DAT_005201b0;
        if 0 < *param_1.add(0x1e) {
            loop {
                i_var4 = i_var4.wrapping_add(4);
                i_var3 = i_var3.wrapping_add(1);
                let addr = (DAT_006a8874 as u32)
                    .wrapping_add(-4i32 as u32)
                    .wrapping_add(i_var4) as *mut i32;
                addr.write_unaligned(-1);
                if !(i_var3 < *param_1.add(0x1e)) {
                    break;
                }
            }
        }
        let mut local_14: i32 = 0;
        if 0 < *param_1.add(0x31) {
            loop {
                let i_var3b = *param_1.add(0x28);
                *pf_var9.add(0x12) = 0.0;
                let mut pi_var7 = ((*pi_var8 as u32)
                    .wrapping_mul(0x40)
                    .wrapping_add(i_var3b as u32)) as *mut i32;
                let mut local_18: i32 = 0;
                *pf_var9.add(0x13) = DAT_00732f3c as f32;
                loop {
                    let mut local_10: i32 = pi_var7.read_unaligned();
                    let local_c: i32 = pi_var7.add(8).read_unaligned();
                    let local_4: i32 = pi_var7.add(0xc).read_unaligned();
                    pi_var7 = pi_var7.add(1);
                    if local_10 == -1 {
                        break;
                    }
                    let slot = (DAT_006a8874 as u32).wrapping_add((local_10 as u32).wrapping_mul(4))
                        as *mut i32;
                    let i_var3c: i32 = slot.read_unaligned();
                    let local_2c: f32;
                    if i_var3c == -1 {
                        slot.write_unaligned(*param_1.add(0x32));
                        local_10 = (local_10 as u32).wrapping_mul(0xc) as i32;
                        let pf_var5 =
                            ((*param_1.add(0x1f)) as u32).wrapping_add(local_10 as u32) as *mut f32;
                        *param_1.add(0x32) = (*param_1.add(0x32)).wrapping_add(1);
                        let mut v: [f32; 3] = [
                            pf_var5.read_unaligned(),
                            pf_var5.add(1).read_unaligned(),
                            pf_var5.add(2).read_unaligned(),
                        ];
                        FUN_00414fe0(v.as_mut_ptr(), param_1.add(5) as *mut f32);
                        v[0] = *pf_param1.add(2) + v[0];
                        v[1] = *pf_param1.add(3) + v[1];
                        v[2] = *pf_param1.add(4) + v[2] + _DAT_005201b0;
                        *pf_var6.add(2) = local_8 / v[2];
                        *pf_var6 = *pf_var6.add(2) * v[0];
                        *pf_var6.add(1) = *pf_var6.add(2) * v[1];
                        *pf_var6 = ((DAT_007091ae >> 1) as f32) + *pf_var6;
                        *pf_var6.add(1) = ((((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc))
                            as i32 as f32)
                            + *pf_var6.add(1);
                        local_2c = *pf_var6.add(2);
                        FUN_00414070(
                            param_1,
                            pf_var6 as usize as i32,
                            ((*param_1.add(0x21)) as u32)
                                .wrapping_add((local_4 as u32).wrapping_mul(0xc))
                                as *mut f32,
                            ((*param_1.add(0x1f)) as u32).wrapping_add(local_10 as u32) as *mut f32,
                            &raw mut DAT_006a88b0,
                        );
                        let t = (*param_1.add(0x20)) as u32;
                        *pf_var6.add(4) = ((t.wrapping_add((local_c as u32).wrapping_mul(8)))
                            as *const f32)
                            .read_unaligned();
                        *pf_var6.add(5) = ((t
                            .wrapping_add(4)
                            .wrapping_add((local_c as u32).wrapping_mul(8)))
                            as *const f32)
                            .read_unaligned();
                        pf_var6 = pf_var6.add(6);
                    } else {
                        let a = (DAT_006a8858 as u32).wrapping_add(8).wrapping_add(
                            ((*param_1.add(0x2f)) as u32)
                                .wrapping_add(i_var3c as u32)
                                .wrapping_mul(0x18),
                        );
                        local_2c = (a as *const f32).read_unaligned();
                    }
                    if *pf_var9.add(0x12) < local_2c {
                        *pf_var9.add(0x12) = local_2c;
                    }
                    if local_2c < *pf_var9.add(0x13) {
                        *pf_var9.add(0x13) = local_2c;
                    }
                    local_18 = local_18.wrapping_add(1);
                    if !(local_18 < 4) {
                        break;
                    }
                }
                pi_var8 = pi_var8.add(0x1c);
                pf_var9 = pf_var9.add(0x1c);
                local_14 = local_14.wrapping_add(1);
                if !(local_14 < *param_1.add(0x31)) {
                    break;
                }
            }
        }
        let mut t: i32 = (&raw const DAT_006a885e).read_unaligned();
        t = ((t as u32).wrapping_add(*param_1.add(0x32) as u32)) as i32;
        (&raw mut DAT_006a885e).write_unaligned(t);
    }
}
