// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0060f1c0: i32;
    static mut DAT_0060f1e8: i32;
    static mut DAT_005271d0: i32;
    static mut DAT_0060f1d4: i32;
    static mut DAT_0060f204: i32;
    static mut _DAT_005181b8: f32;
    static mut _DAT_005181c0: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sqrt(x: f64) -> f64;
    fn FUN_00479ce0(param_1: *mut f32, param_2: i32, param_3: i32);
}

unsafe fn vp(i: i32) -> *const f32 {
    // SAFETY: address computed from the game's vertex table, as the original does.
    unsafe { (DAT_0060f1c0 as u32).wrapping_add((i as u32).wrapping_mul(0xc)) as *const f32 }
}

unsafe fn v(i: i32, off: u32) -> f32 {
    // SAFETY: reads the game's vertex table at the computed address.
    unsafe { ((vp(i) as u32).wrapping_add(off) as *const f32).read_unaligned() }
}

unsafe fn up(i: i32) -> *const f32 {
    // SAFETY: address computed from the game's uv table, as the original does.
    unsafe { (DAT_0060f1e8 as u32).wrapping_add((i as u32).wrapping_mul(8)) as *const f32 }
}

unsafe fn u(i: i32, off: u32) -> f32 {
    // SAFETY: reads the game's uv table at the computed address.
    unsafe { ((up(i) as u32).wrapping_add(off) as *const f32).read_unaligned() }
}

unsafe fn rf(p: *const f32, k: usize) -> f32 {
    // SAFETY: caller supplies a valid pointer into the game's tables.
    unsafe { p.add(k).read_unaligned() }
}

unsafe fn ri(p: *const i32, k: usize) -> i32 {
    // SAFETY: caller supplies a valid face record pointer.
    unsafe { p.add(k).read_unaligned() }
}

fn fabs_(x: f32) -> f32 {
    if x < 0.0 { -x } else { x }
}

unsafe fn sumsq(p: *mut f32) -> f32 {
    // SAFETY: p points at the caller's output buffer.
    unsafe {
        let a = rf(p, 1) * rf(p, 1);
        let b = rf(p, 0) * rf(p, 0);
        let c = rf(p, 2) * rf(p, 2);
        a + b + c
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "C" fn FUN_00479510(
    mut param_1: *mut f32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) {
    // SAFETY: all pointers are the game's own memory, accessed as the original does.
    unsafe {
        if DAT_0060f1c0 == 0 {
            return;
        }
        let piVar11 = (param_2.wrapping_add(4)) as *const i32;
        let mut piVar12 = piVar11;
        let mut iVar14 = param_3;
        let mut fVar4: f32 = 0.0;
        let mut fVar5: f32 = 0.0;
        let mut fVar6: f32 = 0.0;
        let mut fVar7: f32 = 0.0;
        let mut fVar8: f32 = 0.0;
        let mut fVar9: f32;
        let mut fStack_1c: f32 = 0.0;
        let mut fStack_18: f32 = 0.0;
        let mut fStack_14: f32 = 0.0;
        let mut fStack_10: f32 = 0.0;
        let mut fStack_c: f32 = 0.0;
        let mut fStack_8: f32 = 0.0;

        if 0 < param_3 {
            loop {
                let iVar2 = ri(piVar12, 1);
                let iVar3 = ri(piVar12, 0);
                fVar6 = v(iVar2, 8) - v(iVar3, 8);
                fVar4 = v(iVar2, 0) - v(iVar3, 0);
                fVar9 = v(iVar2, 4) - v(iVar3, 4);
                fVar5 = v(iVar2, 0) - v(ri(piVar12, 2), 0);
                fVar7 = v(iVar2, 4) - v(ri(piVar12, 2), 4);
                fVar8 = v(iVar2, 8) - v(ri(piVar12, 2), 8);
                *param_1 = fVar8 * fVar9 - fVar7 * fVar6;
                *param_1.add(1) = fVar5 * fVar6 - fVar8 * fVar4;
                *param_1.add(2) = fVar7 * fVar4 - fVar5 * fVar9;
                fVar4 = sumsq(param_1);
                let mut norm = false;
                if fVar4 == _DAT_005181b8 {
                    let iVar1 = ri(piVar12, 3);
                    if iVar1 != -1 {
                        let iVar3 = ri(piVar12, 0);
                        fVar6 = v(iVar1, 8) - v(iVar3, 8);
                        fVar4 = v(iVar1, 0) - v(iVar3, 0);
                        fVar9 = v(iVar1, 4) - v(iVar3, 4);
                        fVar5 = v(iVar1, 0) - v(ri(piVar12, 2), 0);
                        fVar7 = v(iVar1, 4) - v(ri(piVar12, 2), 4);
                        fVar8 = v(iVar1, 8) - v(ri(piVar12, 2), 8);
                        *param_1 = fVar8 * fVar9 - fVar7 * fVar6;
                        *param_1.add(1) = fVar5 * fVar6 - fVar8 * fVar4;
                        *param_1.add(2) = fVar7 * fVar4 - fVar5 * fVar9;
                        fVar4 = sumsq(param_1);
                        if fVar4 != _DAT_005181b8 {
                            norm = true;
                        }
                    }
                } else {
                    norm = true;
                }
                if norm {
                    fVar4 = _DAT_005181c0 / (sqrt(fVar4 as f64) as f32);
                    *param_1 = *param_1 * fVar4;
                    *param_1.add(1) = *param_1.add(1) * fVar4;
                    *param_1.add(2) = fVar4 * *param_1.add(2);
                }
                if DAT_005271d0 != 0 {
                    *param_1 = -*param_1;
                    *param_1.add(1) = -*param_1.add(1);
                    *param_1.add(2) = -*param_1.add(2);
                }
                param_1 = param_1.add(3);
                iVar14 = iVar14.wrapping_sub(1);
                piVar12 = piVar12.add(0x10);
                if iVar14 == 0 {
                    break;
                }
            }
        }

        if DAT_0060f1e8 == 0 {
            param_1 =
                param_1.wrapping_offset(((param_3 as u32).wrapping_mul(0x18)) as i32 as isize);
        } else if 0 < param_3 {
            let mut p = piVar11;
            let mut iStack_4 = param_3;
            loop {
                let mut iVar14 = ri(p, 0);
                let mut iVar1 = ri(p, 1);
                fVar4 = v(iVar14, 0) - v(iVar1, 0);
                fVar7 = v(iVar14, 4) - v(iVar1, 4);
                fVar6 = v(iVar14, 8) - v(iVar1, 8);
                iVar1 = ri(p, 8);
                let mut pfVar10 = vp(ri(p, 2));
                fStack_1c = v(iVar14, 8) - rf(pfVar10, 2);
                fStack_c = v(iVar14, 0) - rf(pfVar10, 0);
                fStack_8 = v(iVar14, 4) - rf(pfVar10, 1);
                fStack_18 = u(iVar1, 0) - u(ri(p, 9), 0);
                fVar5 = u(iVar1, 4) - u(ri(p, 9), 4);
                pfVar10 = up(ri(p, 10));
                fVar8 = u(iVar1, 4) - rf(pfVar10, 1);
                fStack_14 = u(iVar1, 0) - rf(pfVar10, 0);
                fStack_10 = fStack_14 * fVar5 - fStack_18 * fVar8;
                *param_1 = fStack_c * fVar5 - fVar8 * fVar4;
                *param_1.add(1) = fStack_8 * fVar5 - fVar8 * fVar7;
                *param_1.add(2) = fVar5 * fStack_1c - fVar8 * fVar6;

                'outer: {
                    'bcb: {
                        if fabs_(fStack_10) == 0.0 || sumsq(param_1) == _DAT_005181b8 {
                            if ri(p, 3) == -1 {
                                break 'bcb;
                            } else {
                                iVar1 = ri(p, 0);
                                pfVar10 = vp(ri(p, 3));
                                fStack_c = v(iVar1, 0) - rf(pfVar10, 0);
                                fStack_8 = v(iVar1, 4) - rf(pfVar10, 1);
                                let mut pfVar13 = vp(ri(p, 1));
                                fVar7 = v(iVar1, 4) - rf(pfVar13, 1);
                                fVar4 = v(iVar1, 0) - rf(pfVar13, 0);
                                fVar6 = v(iVar1, 8) - rf(pfVar13, 2);
                                fStack_1c = v(iVar1, 8) - rf(pfVar10, 2);
                                iVar14 = ri(p, 8);
                                fStack_18 = u(iVar14, 0) - u(ri(p, 9), 0);
                                fVar5 = u(iVar14, 4) - u(ri(p, 9), 4);
                                pfVar10 = up(ri(p, 0xb));
                                fVar8 = u(iVar14, 4) - rf(pfVar10, 1);
                                fStack_14 = u(iVar14, 0) - rf(pfVar10, 0);
                                fStack_10 = fStack_14 * fVar5 - fStack_18 * fVar8;
                                *param_1 = fStack_c * fVar5 - fVar8 * fVar4;
                                *param_1.add(1) = fStack_8 * fVar5 - fVar8 * fVar7;
                                *param_1.add(2) = fVar5 * fStack_1c - fVar8 * fVar6;
                                if fabs_(fStack_10) == 0.0 || sumsq(param_1) == _DAT_005181b8 {
                                    iVar1 = ri(p, 0);
                                    let iVar2 = ri(p, 2);
                                    fVar7 = v(iVar1, 4) - v(iVar2, 4);
                                    fVar4 = v(iVar1, 0) - v(iVar2, 0);
                                    fVar6 = v(iVar1, 8) - v(iVar2, 8);
                                    let iVar2 = ri(p, 8);
                                    pfVar10 = vp(ri(p, 3));
                                    fStack_c = v(iVar1, 0) - rf(pfVar10, 0);
                                    fStack_8 = v(iVar1, 4) - rf(pfVar10, 1);
                                    fStack_1c = v(iVar1, 8) - rf(pfVar10, 2);
                                    fStack_18 = u(iVar2, 0) - u(ri(p, 10), 0);
                                    fVar5 = u(iVar2, 4) - u(ri(p, 10), 4);
                                    pfVar10 = up(ri(p, 0xb));
                                    fVar8 = u(iVar2, 4) - rf(pfVar10, 1);
                                    fStack_14 = u(iVar2, 0) - rf(pfVar10, 0);
                                    fStack_10 = fStack_14 * fVar5 - fStack_18 * fVar8;
                                    *param_1 = fStack_c * fVar5 - fVar8 * fVar4;
                                    *param_1.add(1) = fStack_8 * fVar5 - fVar8 * fVar7;
                                    *param_1.add(2) = fVar5 * fStack_1c - fVar8 * fVar6;
                                    if fabs_(fStack_10) == 0.0 || sumsq(param_1) == _DAT_005181b8 {
                                        iVar14 = ri(p, 1);
                                        iVar1 = ri(p, 2);
                                        fVar4 = v(iVar14, 0) - v(iVar1, 0);
                                        fVar7 = v(iVar14, 4) - v(iVar1, 4);
                                        fVar6 = v(iVar14, 8) - v(iVar1, 8);
                                        pfVar13 = vp(ri(p, 3));
                                        pfVar10 = up(ri(p, 9));
                                        fStack_1c = v(iVar14, 8) - rf(pfVar13, 2);
                                        fStack_c = v(iVar14, 0) - rf(pfVar13, 0);
                                        fStack_8 = v(iVar14, 4) - rf(pfVar13, 1);
                                        fVar5 = rf(pfVar10, 1) - u(ri(p, 10), 4);
                                        fStack_18 = rf(pfVar10, 0) - u(ri(p, 10), 0);
                                        pfVar13 = up(ri(p, 0xb)) as *const f32 as *const f32;
                                        fStack_14 = rf(pfVar10, 0) - rf(pfVar13, 0);
                                        fVar8 = rf(pfVar10, 1) - rf(pfVar13, 1);
                                        fStack_10 = fStack_14 * fVar5 - fStack_18 * fVar8;
                                        *param_1 = fStack_c * fVar5 - fVar8 * fVar4;
                                        *param_1.add(1) = fStack_8 * fVar5 - fVar8 * fVar7;
                                        *param_1.add(2) = fVar5 * fStack_1c - fVar8 * fVar6;
                                        if fabs_(fStack_10) == 0.0
                                            || sumsq(param_1) == _DAT_005181b8
                                        {
                                            break 'bcb;
                                        }
                                    }
                                }
                            }
                        }
                        break 'outer;
                    }
                    fStack_14 = 1.0;
                    fStack_10 = 1.0;
                    *param_1 = fStack_c;
                    fStack_18 = 0.0;
                    *param_1.add(1) = fStack_8;
                    *param_1.add(2) = fStack_1c;
                }

                fStack_10 = _DAT_005181c0 / fStack_10;
                *param_1 = *param_1 * fStack_10;
                *param_1.add(1) = *param_1.add(1) * fStack_10;
                fVar5 = -fStack_10;
                *param_1.add(2) = *param_1.add(2) * fStack_10;
                *param_1.add(3) = (fStack_18 * fStack_c - fStack_14 * fVar4) * fVar5;
                *param_1.add(4) = (fStack_18 * fStack_8 - fStack_14 * fVar7) * fVar5;
                *param_1.add(5) = fVar5 * (fStack_18 * fStack_1c - fStack_14 * fVar6);
                if fabs_(fStack_18) == 0.0 && fabs_(fStack_14) == 0.0 {
                    *param_1.add(4) = 1.0;
                }
                param_1 = param_1.add(6);
                p = p.add(0x10);
                iStack_4 = iStack_4.wrapping_sub(1);
                if iStack_4 == 0 {
                    break;
                }
            }
        }

        if ((param_4 as u32).wrapping_add(0x84) as *const i32).read_unaligned() != 0 {
            DAT_0060f1d4 = 0;
            return;
        }
        DAT_0060f1d4 = DAT_0060f204;
        FUN_00479ce0(param_1, param_2, param_3);
    }
}
