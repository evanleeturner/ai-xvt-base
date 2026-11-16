// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00999462: [u8; 4];
    static mut DAT_0099944a: [u8; 4];
    static mut DAT_0099945c: i32;
    static mut DAT_00999478: i32;
    static mut DAT_00a081ec: f32;
    static mut DAT_009d682c: u16;
    static mut DAT_009a1fe4: u16;
    static mut DAT_009d7680: i32;
    static mut _DAT_00518060: f32;
    static mut _DAT_00518054: f32;
    static mut _DAT_00518074: f32;
    static mut _DAT_00518078: f32;
    static mut _DAT_0099940c: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00420db0(param_1: i32, param_2: *mut f32, param_3: i32);
    fn FUN_00420fe0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_004201f0(param_1: *mut i32, param_2: i32, param_3: *mut f32, param_4: *mut f32);
}

#[inline(always)]
unsafe fn ri(a: u32) -> i32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn rf(a: u32) -> f32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const f32).read_unaligned() }
}

#[inline(always)]
unsafe fn wi(a: u32, v: i32) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *mut i32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn pi(p: *mut i32, k: usize) -> i32 {
    // SAFETY: p points at the game's int record.
    unsafe { p.add(k).read_unaligned() }
}

#[inline(always)]
unsafe fn half_w() -> f32 {
    // SAFETY: the game's globals.
    unsafe { (i32::from(DAT_009d682c >> 1)) as f32 }
}

#[inline(always)]
unsafe fn half_h() -> f32 {
    // SAFETY: the game's globals.
    unsafe { ((u32::from(DAT_009a1fe4 >> 1)).wrapping_add(DAT_009d7680 as u32) as i32) as f32 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "C" fn FUN_004084e0(param_1: *mut i32) {
    // SAFETY: all pointers and globals belong to the game; accesses mirror the original.
    unsafe {
        let mut iVar5: i32 = i32::from_le_bytes(*(&raw mut DAT_00999462));
        let mut iVar4: i32 = i32::from_le_bytes(*(&raw mut DAT_0099944a));
        let mut piVar10: *mut i32 = (iVar4 as u32)
            .wrapping_add((pi(param_1, 0x2e) as u32).wrapping_mul(0x70))
            as usize as *mut i32;

        macro_rules! g {
            ($k:expr) => {
                piVar10.cast::<f32>().add($k).read_unaligned()
            };
        }
        macro_rules! s {
            ($k:expr, $v:expr) => {{
                let v__: f32 = $v;
                piVar10.cast::<f32>().add($k).write_unaligned(v__)
            }};
        }

        param_1.add(0x2f).write_unaligned(iVar5);
        let mut iVar6: i32 = DAT_0099945c;
        iVar4 = 0;
        param_1.add(0x32).write_unaligned(0);
        let mut pfVar8: u32 = (iVar6 as u32).wrapping_add((iVar5 as u32).wrapping_mul(0x18));
        iVar6 = 0;
        if 0 < pi(param_1, 0x1e) {
            loop {
                iVar4 = iVar4.wrapping_add(4);
                iVar6 = iVar6.wrapping_add(1);
                wi(
                    (DAT_00999478 as u32)
                        .wrapping_add(0xFFFFFFFC)
                        .wrapping_add(iVar4 as u32),
                    -1,
                );
                if !(iVar6 < pi(param_1, 0x1e)) {
                    break;
                }
            }
        }
        let mut local_20: i32 = 0;
        if 0 < pi(param_1, 0x31) {
            loop {
                FUN_00420db0(
                    piVar10 as usize as i32,
                    (pi(param_1, 0x26) as u32)
                        .wrapping_add((pi(piVar10, 0) as u32).wrapping_mul(0x18))
                        as usize as *mut f32,
                    param_1.add(2) as usize as i32,
                );
                iVar6 = pi(param_1, 0x28);
                piVar10.add(0x12).write_unaligned(0);
                let mut local_3c: f32 = 0.0;
                let mut local_24: i32 = 0;
                s!(0x13, DAT_00a081ec);
                let mut piVar3: *mut i32 = (pi(piVar10, 0) as u32)
                    .wrapping_mul(0x40)
                    .wrapping_add(iVar6 as u32) as usize
                    as *mut i32;
                let mut piVar9: *mut i32 = piVar3;
                let mut local_38: i32;
                let mut local_34: f32;
                let mut local_50: f32;
                let mut vec: [f32; 3] = [0.0; 3];
                loop {
                    piVar9 = piVar3;
                    iVar6 = pi(piVar9, 0);
                    local_38 = pi(piVar9, 8);
                    local_34 = f32::from_bits(pi(piVar9, 0xc) as u32);
                    if iVar6 == -1 {
                        break;
                    }
                    let slot = (DAT_00999478 as u32).wrapping_add((iVar6 as u32).wrapping_mul(4));
                    iVar5 = ri(slot);
                    if iVar5 == -1 {
                        wi(slot, pi(param_1, 0x32));
                        let local_1c: i32 = (iVar6 as u32).wrapping_mul(0xc) as i32;
                        param_1
                            .add(0x32)
                            .write_unaligned(pi(param_1, 0x32).wrapping_add(1));
                        let a7 = (pi(param_1, 0x1f) as u32).wrapping_add(local_1c as u32);
                        vec[0] = rf(a7);
                        vec[1] = rf(a7.wrapping_add(4));
                        vec[2] = rf(a7.wrapping_add(8));
                        FUN_00420fe0(vec.as_mut_ptr(), param_1.add(5).cast::<f32>());
                        vec[0] = f32::from_bits(pi(param_1, 2) as u32) + vec[0];
                        vec[1] = f32::from_bits(pi(param_1, 3) as u32) + vec[1];
                        vec[2] = f32::from_bits(pi(param_1, 4) as u32) + vec[2];
                        if _DAT_00518060 <= vec[2] {
                            let d = DAT_00a081ec / vec[2];
                            wi(pfVar8.wrapping_add(8), d.to_bits() as i32);
                            let x = rf(pfVar8.wrapping_add(8)) * vec[0];
                            wi(pfVar8, x.to_bits() as i32);
                            let y = rf(pfVar8.wrapping_add(8)) * vec[1];
                            wi(pfVar8.wrapping_add(4), y.to_bits() as i32);
                            let x2 = half_w() + rf(pfVar8);
                            wi(pfVar8, x2.to_bits() as i32);
                            let y2 = half_h() + rf(pfVar8.wrapping_add(4));
                            wi(pfVar8.wrapping_add(4), y2.to_bits() as i32);
                            local_50 = rf(pfVar8.wrapping_add(8));
                        } else {
                            let z = vec[2] - _DAT_00518060;
                            wi(pfVar8.wrapping_add(8), z.to_bits() as i32);
                            wi(pfVar8, vec[0].to_bits() as i32);
                            wi(pfVar8.wrapping_add(4), vec[1].to_bits() as i32);
                            piVar10.add(3).write_unaligned(-1);
                            local_50 = DAT_00a081ec;
                        }
                        iVar4 = local_34.to_bits() as i32;
                        FUN_004201f0(
                            param_1,
                            pfVar8 as i32,
                            (pi(param_1, 0x21) as u32)
                                .wrapping_add((iVar4 as u32).wrapping_mul(0xc))
                                as usize as *mut f32,
                            (pi(param_1, 0x1f) as u32).wrapping_add(local_1c as u32) as usize
                                as *mut f32,
                        );
                        let b = (pi(param_1, 0x20) as u32)
                            .wrapping_add((local_38 as u32).wrapping_mul(8));
                        wi(pfVar8.wrapping_add(16), rf(b).to_bits() as i32);
                        wi(
                            pfVar8.wrapping_add(20),
                            rf(b.wrapping_add(4)).to_bits() as i32,
                        );
                        pfVar8 = pfVar8.wrapping_add(24);
                    } else {
                        iVar5 = pi(param_1, 0x2f).wrapping_add(iVar5);
                        let base =
                            (DAT_0099945c as u32).wrapping_add((iVar5 as u32).wrapping_mul(0x18));
                        if (ri(base.wrapping_add(8)) as u32) < 0x80000001 {
                            local_50 = rf(base.wrapping_add(8));
                        } else {
                            piVar10.add(3).write_unaligned(-1);
                            local_50 = DAT_00a081ec;
                        }
                    }
                    local_3c = local_3c + local_50;
                    if g!(0x12) < local_50 {
                        s!(0x12, local_50);
                    }
                    if local_50 < g!(0x13) {
                        s!(0x13, local_50);
                    }
                    local_24 = local_24.wrapping_add(1);
                    piVar3 = piVar9.add(1);
                    if !(local_24 < 4) {
                        break;
                    }
                }
                if pi(param_1, 0x20) != 0 {
                    local_38 = pi(piVar9, 5);
                    let a7 = (pi(param_1, 0x1f) as u32)
                        .wrapping_add((pi(piVar9, 0).wrapping_sub(0) as u32).wrapping_mul(0));
                    let a7 = a7;
                    let _ = a7;
                    let a7 = (pi(param_1, 0x1f) as u32)
                        .wrapping_add((piVar9.sub(3).read_unaligned() as u32).wrapping_mul(0xc));
                    vec[0] = rf(a7);
                    vec[1] = rf(a7.wrapping_add(4));
                    vec[2] = rf(a7.wrapping_add(8));
                    FUN_00420fe0(vec.as_mut_ptr(), param_1.add(5).cast::<f32>());
                    vec[0] = f32::from_bits(pi(param_1, 2) as u32) + vec[0];
                    vec[1] = f32::from_bits(pi(param_1, 3) as u32) + vec[1];
                    vec[2] = f32::from_bits(pi(param_1, 4) as u32) + vec[2];
                    let tb =
                        (pi(param_1, 0x20) as u32).wrapping_add((local_38 as u32).wrapping_mul(8));
                    s!(
                        10,
                        (vec[0] - rf(tb.wrapping_add(4)) * g!(7)) - rf(tb) * g!(4)
                    );
                    s!(
                        0xb,
                        (vec[1] - rf(tb.wrapping_add(4)) * g!(8)) - rf(tb) * g!(5)
                    );
                    s!(
                        0xc,
                        (vec[2] - rf(tb.wrapping_add(4)) * g!(9)) - rf(tb) * g!(6)
                    );
                    let local_18 = g!(0xc) * g!(8) - g!(9) * g!(0xb);
                    let local_14 = g!(9) * g!(10) - g!(0xc) * g!(7);
                    let local_10 = g!(0xb) * g!(7) - g!(10) * g!(8);
                    let local_c = g!(6) * g!(0xb) - g!(0xc) * g!(5);
                    let local_8 = g!(0xc) * g!(4) - g!(10) * g!(6);
                    let local_4 = g!(5) * g!(10) - g!(0xb) * g!(4);
                    let local_30 = g!(9) * g!(5) - g!(6) * g!(8);
                    let local_2c = g!(6) * g!(7) - g!(9) * g!(4);
                    let mut local_28 = g!(8) * g!(4) - g!(5) * g!(7);
                    if (if local_30 < 0.0 { -local_30 } else { local_30 }) == 0.0
                        && (if local_2c < 0.0 { -local_2c } else { local_2c }) == 0.0
                        && (if local_28 < 0.0 { -local_28 } else { local_28 }) == 0.0
                    {
                        local_28 = 1.0;
                    }
                    let mut fVar1: f32 = _DAT_00518060
                        / (g!(10) * local_30 + g!(0xb) * local_2c + g!(0xc) * local_28);
                    let fVar2: f32 = _DAT_0099940c * fVar1;
                    s!(4, local_18 * fVar2);
                    s!(5, local_14 * fVar2);
                    s!(6, local_10 * fVar1);
                    s!(7, local_c * fVar2);
                    s!(8, local_8 * fVar2);
                    s!(9, local_4 * fVar1);
                    s!(10, local_30 * fVar2);
                    s!(0xb, fVar2 * local_2c);
                    s!(0xc, fVar1 * local_28);
                    s!(6, g!(6) - half_w() * g!(4));
                    s!(6, g!(6) - half_h() * g!(5));
                    s!(9, g!(9) - half_w() * g!(7));
                    s!(9, g!(9) - half_h() * g!(8));
                    s!(0xc, g!(0xc) - half_w() * g!(10));
                    s!(0xc, g!(0xc) - half_h() * g!(0xb));
                    local_34 = g!(8) * g!(4) - g!(5) * g!(7);
                    if local_34 < _DAT_00518054 {
                        local_34 = -local_34;
                    }
                    fVar1 = _DAT_00518078;
                    if pi(piVar9, 0) == -1 {
                        fVar1 = _DAT_00518074;
                    }
                    local_3c = fVar1 / local_3c;
                    piVar10.add(0x1b).write_unaligned((local_3c as i64) as i32);
                }
                piVar10 = piVar10.add(0x1c);
                local_20 = local_20.wrapping_add(1);
                if !(local_20 < pi(param_1, 0x31)) {
                    break;
                }
            }
        }
        iVar5 = i32::from_le_bytes(*(&raw mut DAT_00999462));
        iVar5 = iVar5.wrapping_add(pi(param_1, 0x32));
        *(&raw mut DAT_00999462) = iVar5.to_le_bytes();
    }
}
