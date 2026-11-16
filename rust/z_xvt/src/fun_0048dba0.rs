// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00602678: i32;
    static mut DAT_006026a0: i32;
    static mut DAT_0052a468: i32;
    static mut DAT_0060268c: i32;
    static mut DAT_006026bc: i32;
    static mut _DAT_005201d4: f32;
    static mut _DAT_005201dc: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048e380(param_1: *mut f32, param_2: i32, param_3: i32);
}

unsafe fn vx(i: i32, o: u32) -> f32 {
    // SAFETY: the vertex table is the game's own memory.
    unsafe {
        let a = (DAT_00602678 as u32)
            .wrapping_add(o)
            .wrapping_add((i as u32).wrapping_mul(12));
        (a as *const f32).read_unaligned()
    }
}

unsafe fn uv(i: i32, o: u32) -> f32 {
    // SAFETY: the UV table is the game's own memory.
    unsafe {
        let a = (DAT_006026a0 as u32)
            .wrapping_add(o)
            .wrapping_add((i as u32).wrapping_mul(8));
        (a as *const f32).read_unaligned()
    }
}

unsafe fn bad(p: *mut f32, s10: f32) -> bool {
    // SAFETY: p points at the caller's output buffer.
    unsafe {
        s10 == 0.0
            || (p.add(1).read_unaligned() * p.add(1).read_unaligned()
                + p.read_unaligned() * p.read_unaligned()
                + p.add(2).read_unaligned() * p.add(2).read_unaligned())
                == _DAT_005201d4
    }
}

fn sqrtf(mut x: f32) -> f32 {
    // SAFETY: plain sqrtss on a register value.
    unsafe {
        core::arch::asm!("sqrtss {0}, {0}", inout(xmm_reg) x, options(pure, nomem, nostack));
    }
    x
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048dba0(param_1: *mut f32, param_2: i32, param_3: i32, param_4: i32) {
    let mut param_1 = param_1;
    // SAFETY: all pointers and globals belong to the game; the original
    // reads and writes them at the same places.
    unsafe {
        if DAT_00602678 == 0 {
            return;
        }
        let piVar11 = (param_2 as u32).wrapping_add(4) as *const i32;
        let mut piVar14 = piVar11;
        if 0 < param_3 {
            let mut iVar13 = param_3;
            loop {
                let iVar2 = piVar14.add(1).read_unaligned();
                let iVar3 = piVar14.read_unaligned();
                let p2 = piVar14.add(2).read_unaligned();
                let mut fVar6 = vx(iVar2, 8) - vx(iVar3, 8);
                let mut fVar4 = vx(iVar2, 0) - vx(iVar3, 0);
                let mut fVar9 = vx(iVar2, 4) - vx(iVar3, 4);
                let mut fVar5 = vx(iVar2, 0) - vx(p2, 0);
                let mut fVar7 = vx(iVar2, 4) - vx(p2, 4);
                let mut fVar8 = vx(iVar2, 8) - vx(p2, 8);
                param_1.write_unaligned(fVar8 * fVar9 - fVar7 * fVar6);
                param_1
                    .add(1)
                    .write_unaligned(fVar5 * fVar6 - fVar8 * fVar4);
                param_1
                    .add(2)
                    .write_unaligned(fVar4 * fVar7 - fVar5 * fVar9);
                fVar4 = param_1.add(1).read_unaligned() * param_1.add(1).read_unaligned()
                    + param_1.read_unaligned() * param_1.read_unaligned()
                    + param_1.add(2).read_unaligned() * param_1.add(2).read_unaligned();
                let mut do_norm = true;
                if fVar4 == _DAT_005201d4 {
                    do_norm = false;
                    let iVar1 = piVar14.add(3).read_unaligned();
                    if iVar1 != -1 {
                        let iVar3 = piVar14.read_unaligned();
                        fVar6 = vx(iVar1, 8) - vx(iVar3, 8);
                        fVar4 = vx(iVar1, 0) - vx(iVar3, 0);
                        fVar9 = vx(iVar1, 4) - vx(iVar3, 4);
                        fVar5 = vx(iVar1, 0) - vx(p2, 0);
                        fVar7 = vx(iVar1, 4) - vx(p2, 4);
                        fVar8 = vx(iVar1, 8) - vx(p2, 8);
                        param_1.write_unaligned(fVar8 * fVar9 - fVar7 * fVar6);
                        param_1
                            .add(1)
                            .write_unaligned(fVar5 * fVar6 - fVar8 * fVar4);
                        param_1
                            .add(2)
                            .write_unaligned(fVar4 * fVar7 - fVar5 * fVar9);
                        fVar4 = param_1.add(1).read_unaligned() * param_1.add(1).read_unaligned()
                            + param_1.read_unaligned() * param_1.read_unaligned()
                            + param_1.add(2).read_unaligned() * param_1.add(2).read_unaligned();
                        if fVar4 != _DAT_005201d4 {
                            do_norm = true;
                        }
                    }
                }
                if do_norm {
                    fVar4 = _DAT_005201dc / sqrtf(fVar4);
                    param_1.write_unaligned(param_1.read_unaligned() * fVar4);
                    param_1
                        .add(1)
                        .write_unaligned(param_1.add(1).read_unaligned() * fVar4);
                    param_1
                        .add(2)
                        .write_unaligned(fVar4 * param_1.add(2).read_unaligned());
                }
                if DAT_0052a468 != 0 {
                    param_1.write_unaligned(-param_1.read_unaligned());
                    param_1
                        .add(1)
                        .write_unaligned(-param_1.add(1).read_unaligned());
                    param_1
                        .add(2)
                        .write_unaligned(-param_1.add(2).read_unaligned());
                }
                param_1 = param_1.add(3);
                iVar13 = iVar13.wrapping_sub(1);
                piVar14 = piVar14.add(0x10);
                if iVar13 == 0 {
                    break;
                }
            }
        }
        if DAT_006026a0 == 0 {
            param_1 = param_1.wrapping_add((param_3 as u32).wrapping_mul(0x18) as usize);
        } else if 0 < param_3 {
            let mut q = piVar11;
            let mut iStack_4 = param_3;
            let mut fVar4: f32;
            let mut fVar5: f32;
            let mut fVar6: f32;
            let mut fStack_1c: f32;
            let mut fStack_18: f32;
            let mut fStack_14: f32;
            let mut fStack_10: f32;
            let mut fStack_c: f32;
            let mut fStack_8: f32;
            loop {
                let r = |k: usize| -> i32 { q.add(k).read_unaligned() };
                let iVar13 = r(0);
                let mut iVar1 = r(1);
                fVar4 = vx(iVar13, 0) - vx(iVar1, 0);
                fVar5 = vx(iVar13, 4) - vx(iVar1, 4);
                fVar6 = vx(iVar13, 8) - vx(iVar1, 8);
                iVar1 = r(8);
                fStack_1c = vx(iVar13, 8) - vx(r(2), 8);
                fStack_c = vx(iVar13, 0) - vx(r(2), 0);
                fStack_8 = vx(iVar13, 4) - vx(r(2), 4);
                fStack_18 = uv(iVar1, 0) - uv(r(9), 0);
                let mut fVar7 = uv(iVar1, 4) - uv(r(9), 4);
                let mut fVar8 = uv(iVar1, 4) - uv(r(10), 4);
                fStack_14 = uv(iVar1, 0) - uv(r(10), 0);
                fStack_10 = fStack_14 * fVar7 - fStack_18 * fVar8;
                param_1.write_unaligned(fStack_c * fVar7 - fVar4 * fVar8);
                param_1
                    .add(1)
                    .write_unaligned(fStack_8 * fVar7 - fVar8 * fVar5);
                param_1
                    .add(2)
                    .write_unaligned(fVar7 * fStack_1c - fVar8 * fVar6);
                let mut need_fb = false;
                if bad(param_1, fStack_10) {
                    if r(3) == -1 {
                        need_fb = true;
                    } else {
                        let iVar1 = r(0);
                        fStack_c = vx(iVar1, 0) - vx(r(3), 0);
                        fStack_8 = vx(iVar1, 4) - vx(r(3), 4);
                        fVar5 = vx(iVar1, 4) - vx(r(1), 4);
                        fVar4 = vx(iVar1, 0) - vx(r(1), 0);
                        fVar6 = vx(iVar1, 8) - vx(r(1), 8);
                        fStack_1c = vx(iVar1, 8) - vx(r(3), 8);
                        let iVar13 = r(8);
                        fVar7 = uv(iVar13, 4) - uv(r(9), 4);
                        fStack_18 = uv(iVar13, 0) - uv(r(9), 0);
                        fStack_14 = uv(iVar13, 0) - uv(r(11), 0);
                        fVar8 = uv(iVar13, 4) - uv(r(11), 4);
                        fStack_10 = fStack_14 * fVar7 - fStack_18 * fVar8;
                        param_1.write_unaligned(fStack_c * fVar7 - fVar4 * fVar8);
                        param_1
                            .add(1)
                            .write_unaligned(fStack_8 * fVar7 - fVar8 * fVar5);
                        param_1
                            .add(2)
                            .write_unaligned(fVar7 * fStack_1c - fVar8 * fVar6);
                        if bad(param_1, fStack_10) {
                            let iVar1 = r(0);
                            let iVar2 = r(2);
                            fVar5 = vx(iVar1, 4) - vx(iVar2, 4);
                            fVar4 = vx(iVar1, 0) - vx(iVar2, 0);
                            fVar6 = vx(iVar1, 8) - vx(iVar2, 8);
                            fStack_c = vx(iVar1, 0) - vx(r(3), 0);
                            fStack_8 = vx(iVar1, 4) - vx(r(3), 4);
                            fStack_1c = vx(iVar1, 8) - vx(r(3), 8);
                            let iVar13 = r(8);
                            fStack_18 = uv(iVar13, 0) - uv(r(10), 0);
                            fVar7 = uv(iVar13, 4) - uv(r(10), 4);
                            fVar8 = uv(iVar13, 4) - uv(r(11), 4);
                            fStack_14 = uv(iVar13, 0) - uv(r(11), 0);
                            fStack_10 = fStack_14 * fVar7 - fStack_18 * fVar8;
                            param_1.write_unaligned(fStack_c * fVar7 - fVar4 * fVar8);
                            param_1
                                .add(1)
                                .write_unaligned(fStack_8 * fVar7 - fVar8 * fVar5);
                            param_1
                                .add(2)
                                .write_unaligned(fVar7 * fStack_1c - fVar8 * fVar6);
                            if bad(param_1, fStack_10) {
                                let iVar13 = r(1);
                                let iVar1 = r(2);
                                fVar4 = vx(iVar13, 0) - vx(iVar1, 0);
                                fVar5 = vx(iVar13, 4) - vx(iVar1, 4);
                                fVar6 = vx(iVar13, 8) - vx(iVar1, 8);
                                let iVar1 = r(9);
                                fStack_1c = vx(iVar13, 8) - vx(r(3), 8);
                                fStack_c = vx(iVar13, 0) - vx(r(3), 0);
                                fStack_8 = vx(iVar13, 4) - vx(r(3), 4);
                                fStack_18 = uv(iVar1, 0) - uv(r(10), 0);
                                fVar7 = uv(iVar1, 4) - uv(r(10), 4);
                                fVar8 = uv(iVar1, 4) - uv(r(11), 4);
                                fStack_14 = uv(iVar1, 0) - uv(r(11), 0);
                                fStack_10 = fStack_14 * fVar7 - fStack_18 * fVar8;
                                param_1.write_unaligned(fStack_c * fVar7 - fVar4 * fVar8);
                                param_1
                                    .add(1)
                                    .write_unaligned(fStack_8 * fVar7 - fVar8 * fVar5);
                                param_1
                                    .add(2)
                                    .write_unaligned(fVar7 * fStack_1c - fVar8 * fVar6);
                                if bad(param_1, fStack_10) {
                                    need_fb = true;
                                }
                            }
                        }
                    }
                }
                if need_fb {
                    fStack_14 = 1.0;
                    fStack_10 = 1.0;
                    param_1.write_unaligned(fStack_c);
                    fStack_18 = 0.0;
                    param_1.add(1).write_unaligned(fStack_8);
                    param_1.add(2).write_unaligned(fStack_1c);
                }
                fStack_10 = _DAT_005201dc / fStack_10;
                param_1.write_unaligned(param_1.read_unaligned() * fStack_10);
                param_1
                    .add(1)
                    .write_unaligned(param_1.add(1).read_unaligned() * fStack_10);
                fVar7 = -fStack_10;
                param_1
                    .add(2)
                    .write_unaligned(param_1.add(2).read_unaligned() * fStack_10);
                param_1
                    .add(3)
                    .write_unaligned((fStack_18 * fStack_c - fStack_14 * fVar4) * fVar7);
                param_1
                    .add(4)
                    .write_unaligned((fStack_18 * fStack_8 - fStack_14 * fVar5) * fVar7);
                param_1
                    .add(5)
                    .write_unaligned(fVar7 * (fStack_18 * fStack_1c - fStack_14 * fVar6));
                if fStack_18 == 0.0 && fStack_14 == 0.0 {
                    param_1.add(4).write_unaligned(1.0);
                }
                param_1 = param_1.add(6);
                q = q.add(0x10);
                iStack_4 = iStack_4.wrapping_sub(1);
                if iStack_4 == 0 {
                    break;
                }
            }
        }
        if ((param_4 as u32).wrapping_add(0x84) as *const i32).read_unaligned() != 0 {
            DAT_0060268c = 0;
            return;
        }
        DAT_0060268c = DAT_006026bc;
        FUN_0048e380(param_1, param_2, param_3);
    }
}
