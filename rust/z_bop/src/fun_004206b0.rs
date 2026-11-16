// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005180a0: f32;
    static mut _DAT_00518098: f32;
    static mut _DAT_005180a8: f32;
    static mut _DAT_005180b8: f32;
    static mut _DAT_005180bc: f32;
    static mut _DAT_005180c0: f32;
    static mut _DAT_005180c4: f32;
    static mut _DAT_005180c8: f32;
    static mut _DAT_005180cc: f32;
    static mut _DAT_005180d0: f32;
    static mut DAT_0051c00c: i32;
    static mut DAT_0051c010: i32;
    static mut DAT_0051c014: *mut i32;
    static mut DAT_005235f0: i32;
    static mut DAT_005233bc: i32;
    static mut DAT_00550be0: [f32; 0];
    static mut DAT_00550c10: [f32; 0];
    static mut DAT_009fd3b0: [i32; 0];
    static mut _DAT_00550c00: f32;
    static mut _DAT_00550c04: f32;
    static mut _DAT_00550c08: f32;
    static mut DAT_00550c70: f32;
    static mut DAT_00550c74: f32;
    static mut DAT_00550c78: f32;
    static mut _DAT_00a081e4: i32;
    static mut _DAT_00a08150: i32;
    static mut _DAT_00a081e0: i32;
    static mut DAT_009d682c: u16;
    static mut DAT_009a1fe4: u16;
    static mut DAT_009d7680: i32;
    static mut _DAT_0099940c: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044f880(param_1: i32);
    fn FUN_00420fe0(param_1: *mut f32, param_2: *mut f32);
}

#[inline(always)]
fn fabs_q(x: f32) -> f32 {
    if 0x80000000u32 < x.to_bits() { -x } else { x }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004206b0(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: f32,
) -> f64 {
    // SAFETY: all globals and callees belong to the game; param_1 and the
    // structures it points to are the caller's memory, accessed as the original does.
    unsafe {
        let piv1 = param_1.add(1).read_unaligned() as u32 as *mut i32;
        if piv1.is_null() {
            return _DAT_005180a0 as f64;
        }
        let first = piv1.read_unaligned();
        if first != DAT_0051c00c {
            FUN_0044f880(first);
            DAT_0051c010 = DAT_005235f0;
            DAT_0051c00c = piv1.read_unaligned();
            let mut local_3c: i32 = 0;
            if 0 < DAT_005235f0 {
                let mut pf9 = (&raw mut DAT_00550be0).cast::<f32>();
                let mut pi11 = (&raw mut DAT_009fd3b0).cast::<i32>();
                let mut pf8 = (&raw mut DAT_00550c10).cast::<f32>();
                loop {
                    pf8.write_unaligned(pi11.read_unaligned() as f32);
                    pf8.add(1)
                        .write_unaligned(pi11.add(1).read_unaligned() as f32);
                    pf8.add(2)
                        .write_unaligned(pi11.add(2).read_unaligned() as f32);
                    FUN_00420fe0(pf8, piv1.add(5).cast::<f32>());
                    local_3c = local_3c.wrapping_add(1);
                    let v = piv1.add(2).read_unaligned() as f32 + pf8.read_unaligned();
                    pf8.write_unaligned(v);
                    let v = piv1.add(3).read_unaligned() as f32 + pf8.add(1).read_unaligned();
                    pf8.add(1).write_unaligned(v);
                    let v = piv1.add(4).read_unaligned() as f32 + pf8.add(2).read_unaligned();
                    pf8.add(2).write_unaligned(v);
                    if DAT_005235f0 <= local_3c {
                        break;
                    }
                    pf9.write_unaligned(pi11.add(3).read_unaligned() as f32);
                    pf9 = pf9.add(1);
                    pi11 = pi11.add(4);
                    pf8 = pf8.add(3);
                }
                pf9.write_unaligned(pi11.add(3).read_unaligned() as f32);
            }
            _DAT_00550c04 = (_DAT_00a081e4 as f32) * _DAT_00518098;
            _DAT_00550c08 = (_DAT_00a08150 as f32) * _DAT_00518098;
            _DAT_00550c00 = (_DAT_00a081e0 as f32) * _DAT_00518098;
            FUN_00420fe0(&raw mut _DAT_00550c00, piv1.add(5).cast::<f32>());
        }
        let local_14: f32 = _DAT_005180c4 / param_4;
        let local_1c: f32 = ((param_2 as u32).wrapping_sub((DAT_009d682c >> 1) as u32) as i32)
            as f32
            * local_14
            * _DAT_0099940c;
        let local_18: f32 = ((((param_3 as u32).wrapping_sub((DAT_009a1fe4 >> 1) as u32))
            .wrapping_sub(DAT_009d7680 as u32)) as i32) as f32
            * local_14
            * _DAT_0099940c;
        let local_c: f32;
        let local_8: f32;
        let local_4: f32;
        if param_1 == DAT_0051c014 {
            local_c = DAT_00550c70;
            local_8 = DAT_00550c74;
            local_4 = DAT_00550c78;
        } else {
            DAT_0051c014 = param_1;
            let base = (piv1.add(0x25).read_unaligned() as u32)
                .wrapping_add((param_1.read_unaligned() as u32).wrapping_mul(0xc));
            let mut v: [f32; 3] = [0.0; 3];
            v[0] = (base as *const f32).read_unaligned();
            v[1] = (base.wrapping_add(4) as *const f32).read_unaligned();
            v[2] = (base.wrapping_add(8) as *const f32).read_unaligned();
            FUN_00420fe0(v.as_mut_ptr(), piv1.add(5).cast::<f32>());
            local_c = v[0];
            local_8 = v[1];
            local_4 = v[2];
        }
        let mut local_10: f32 = 0.0;
        if DAT_005233bc != 0 {
            let f2 = -local_1c;
            let f3 = -local_18;
            let f4 = -local_14;
            let mut l40 = fabs_q(f2);
            let l44 = fabs_q(f3);
            let l48 = fabs_q(f4);
            let f5: f32;
            if (l40 < l44) || (l40 < l48) {
                if (l44 < l40) || (l44 < l48) {
                    f5 = (l44 + l40) * _DAT_005180c8;
                    l40 = l48;
                } else {
                    f5 = (l48 + l40) * _DAT_005180c8;
                    l40 = l44;
                }
            } else {
                f5 = (l48 + l44) * _DAT_005180c8;
            }
            let f5 = _DAT_005180c4 / (l40 * _DAT_005180cc + f5);
            let mut l38: f32 = (_DAT_00550c08 + f5 * f4) * local_4
                + (_DAT_00550c04 + f3 * f5) * local_8
                + (_DAT_00550c00 + f2 * f5) * local_c;
            if l38 <= _DAT_005180a0 {
                l38 = 0.0;
            } else {
                l38 = l38 * _DAT_005180b8;
                l38 = l38 * l38 * l38;
                l38 = l38 * l38;
                l38 = l38 * l38;
                l38 = l38 * l38;
                l38 = l38 * l38;
            }
            l38 = l38 * _DAT_005180d0;
            if _DAT_005180a0 < l38 {
                local_10 = l38;
                if 0x3f7fffff < (l38.to_bits() as i32) {
                    DAT_00550c70 = local_c;
                    DAT_00550c74 = local_8;
                    DAT_00550c78 = local_4;
                    return _DAT_005180c4 as f64;
                }
            }
        }
        let mut pf9 = (&raw mut DAT_00550c10).cast::<f32>();
        let mut i10: i32 = 0;
        if 0 < DAT_0051c010 {
            let mut pf8 = (&raw mut DAT_00550be0).cast::<f32>();
            loop {
                let mut f2 = pf9.read_unaligned() - local_1c;
                let mut f3 = pf9.add(1).read_unaligned() - local_18;
                let mut f4 = pf9.add(2).read_unaligned() - local_14;
                let f5 = local_4 * f4 + local_8 * f3 + local_c * f2;
                if _DAT_005180a0 < f5 {
                    let mut l40 = fabs_q(f2);
                    let mut l44 = fabs_q(f3);
                    let mut l48 = fabs_q(f4);
                    let f6: f32;
                    if (l40 < l44) || (l40 < l48) {
                        if (l44 < l40) || (l44 < l48) {
                            f6 = (l44 + l40) * _DAT_005180a8 + l48;
                        } else {
                            f6 = (l48 + l40) * _DAT_005180a8 + l44;
                        }
                    } else {
                        f6 = (l48 + l44) * _DAT_005180a8 + l40;
                    }
                    let mut l38: f32;
                    if DAT_005233bc == 0 {
                        l38 = 0.0;
                    } else {
                        l40 = f2 - local_1c;
                        l44 = f3 - local_18;
                        l48 = f4 - local_14;
                        f2 = local_c * l40;
                        f4 = local_8 * l44;
                        f3 = local_4 * l48;
                        l40 = fabs_q(l40);
                        l44 = fabs_q(l44);
                        l48 = fabs_q(l48);
                        let f7: f32;
                        if (l40 < l44) || (l40 < l48) {
                            if (l44 < l40) || (l44 < l48) {
                                f7 = (l44 + l40) * _DAT_005180bc;
                                l40 = l48;
                            } else {
                                f7 = (l48 + l40) * _DAT_005180bc;
                                l40 = l44;
                            }
                        } else {
                            f7 = (l48 + l44) * _DAT_005180bc;
                        }
                        l38 = _DAT_005180c4 / (l40 * _DAT_005180c0 + f7);
                        f2 = l38 * (f3 + f4 + f2) * _DAT_005180b8;
                        if f2 < _DAT_005180a0 {
                            l38 = 0.0;
                        } else {
                            f2 = f2 * f2 * f2;
                            f2 = f2 * f2;
                            f2 = f2 * f2;
                            f2 = f2 * f2;
                            l38 = f2 * f2 * l38;
                        }
                    }
                    l38 = f5 / (f6 * f6) + l38;
                    if _DAT_005180a0 < l38 {
                        local_10 = pf8.read_unaligned() * l38 + local_10;
                        if _DAT_005180c4 <= local_10 {
                            local_10 = 1.0;
                            break;
                        }
                    }
                }
                pf8 = pf8.add(1);
                i10 = i10.wrapping_add(1);
                pf9 = pf9.add(3);
                if !(i10 < DAT_0051c010) {
                    break;
                }
            }
        }
        DAT_00550c70 = local_c;
        DAT_00550c74 = local_8;
        DAT_00550c78 = local_4;
        local_10 as f64
    }
}
