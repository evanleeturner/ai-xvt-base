// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233c0: i32;
    static mut _DAT_00a08150: i32;
    static mut _DAT_00a081e0: i32;
    static mut _DAT_00a081e4: i32;
    static mut _DAT_00518098: f32;
    static mut _DAT_0051809c: f32;
    static mut _DAT_005180a0: f32;
    static mut _DAT_005180a8: f32;
    static mut _DAT_005180ac: f32;
    static mut _DAT_005180b0: f64;
    static mut _DAT_005180b8: f32;
    static mut _DAT_005180bc: f32;
    static mut _DAT_005180c0: f32;
    static mut DAT_005235f0: i32;
    static mut DAT_009fd3b0: [i32; 0];
    static mut DAT_00527e90: i32;
    static mut DAT_005233bc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00473580(param_1: i32, param_2: *mut f32, param_3: *mut f32) -> i32;
}

fn abs_bits(x: f32) -> f32 {
    if x.to_bits() > 0x80000000 { -x } else { x }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004201f0(
    param_1: *mut i32,
    param_2: i32,
    param_3: *mut f32,
    param_4: *mut f32,
    in_stack_00000014: *mut f32,
) {
    // SAFETY: all pointers and globals belong to the game; they are read and
    // written in the same order and at the same offsets as the original.
    unsafe {
        let out = (param_2.wrapping_add(0xc)) as *mut u32;
        let outf = out as *mut f32;
        let obj = param_1.read_unaligned();
        let kind = ((obj.wrapping_add(2)) as *const i8).read();
        if kind == 7 || kind == 6 {
            out.write_unaligned(0x3f800000);
            return;
        }
        if DAT_005233c0 == 0 {
            out.write_unaligned(0x3ecccccd);
        } else {
            let v = ((_DAT_00a08150 as f32) * _DAT_00518098 * param_3.add(2).read_unaligned()
                + (_DAT_00a081e0 as f32) * _DAT_00518098 * param_3.read_unaligned()
                + (_DAT_00a081e4 as f32) * _DAT_00518098 * param_3.add(1).read_unaligned())
                * _DAT_0051809c;
            outf.write_unaligned(v);
            if out.read_unaligned() < 0x80000001 {
                let mut local_c: [f32; 3] = [0.0; 3];
                local_c[2] = (_DAT_00a08150 as f32) + param_4.add(2).read_unaligned();
                local_c[0] = (_DAT_00a081e0 as f32) + param_4.read_unaligned();
                local_c[1] = (_DAT_00a081e4 as f32) + param_4.add(1).read_unaligned();
                let r = FUN_00473580(obj, param_4, local_c.as_mut_ptr());
                if r != 0 {
                    out.write_unaligned(0);
                }
            } else {
                out.write_unaligned(0);
            }
        }
        let mut local_10: i32 = 0;
        if 0 < DAT_005235f0 {
            let mut piVar7 = (&raw mut DAT_009fd3b0) as *mut i32;
            loop {
                'cont: {
                    let mut local_c: [f32; 3] = [0.0; 3];
                    local_c[0] = piVar7.read_unaligned() as f32;
                    local_c[1] = piVar7.add(1).read_unaligned() as f32;
                    local_c[2] = piVar7.add(2).read_unaligned() as f32;
                    let fvar1 = local_c[0] - param_4.read_unaligned();
                    let fvar2 = local_c[1] - param_4.add(1).read_unaligned();
                    let fvar3 = local_c[2] - param_4.add(2).read_unaligned();
                    let mut local_14 = param_3.add(2).read_unaligned() * fvar3
                        + param_3.add(1).read_unaligned() * fvar2
                        + param_3.read_unaligned() * fvar1;
                    if !((DAT_00527e90 != 0 || local_14.to_bits() < 0x80000001)
                        && FUN_00473580(obj, param_4, local_c.as_mut_ptr()) == 0)
                    {
                        break 'cont;
                    }
                    let mut fstack_24 = abs_bits(fvar1);
                    let local_28 = abs_bits(fvar2);
                    let local_2c = abs_bits(fvar3);
                    let fvar4: f32;
                    if fstack_24 < local_28 || fstack_24 < local_2c {
                        if local_28 < fstack_24 || local_28 < local_2c {
                            fvar4 = (local_28 + fstack_24) * _DAT_005180a8 + local_2c;
                        } else {
                            fvar4 = (local_2c + fstack_24) * _DAT_005180a8 + local_28;
                        }
                    } else {
                        fvar4 = (local_28 + local_2c) * _DAT_005180a8 + fstack_24;
                    }
                    if DAT_00527e90 != 0 {
                        if local_14 / fvar4 < _DAT_005180ac {
                            break 'cont;
                        }
                        local_14 = fvar4 * (_DAT_005180b0 as f32);
                    }
                    let mut local_30: f32;
                    if DAT_005233bc == 0 {
                        local_30 = 0.0;
                    } else {
                        let a =
                            (in_stack_00000014.read_unaligned() - param_4.read_unaligned()) + fvar1;
                        let b = (in_stack_00000014.add(1).read_unaligned()
                            - param_4.add(1).read_unaligned())
                            + fvar2;
                        let c = (in_stack_00000014.add(2).read_unaligned()
                            - param_4.add(2).read_unaligned())
                            + fvar3;
                        let f1 = param_3.read_unaligned() * a;
                        let f3 = param_3.add(1).read_unaligned() * b;
                        let f2 = param_3.add(2).read_unaligned() * c;
                        fstack_24 = abs_bits(a);
                        let l28 = abs_bits(b);
                        let l2c = abs_bits(c);
                        let fvar5: f32;
                        if fstack_24 < l28 || fstack_24 < l2c {
                            if l28 < fstack_24 || l28 < l2c {
                                fvar5 = (l28 + fstack_24) * _DAT_005180bc;
                                fstack_24 = l2c;
                            } else {
                                fvar5 = (l2c + fstack_24) * _DAT_005180bc;
                                fstack_24 = l28;
                            }
                        } else {
                            fvar5 = (l28 + l2c) * _DAT_005180bc;
                        }
                        local_30 =
                            ((f2 + f1 + f3) * _DAT_005180b8) / (fstack_24 * _DAT_005180c0 + fvar5);
                        if local_30 < _DAT_005180b8 {
                            local_30 = 0.0;
                        } else {
                            local_30 = local_30 * local_30 * local_30;
                            local_30 = local_30 * local_30;
                            local_30 = local_30 * local_30;
                            local_30 = local_30 * local_30;
                            local_30 = local_30 * local_30;
                        }
                    }
                    local_30 = local_30 + local_14 / (fvar4 * fvar4);
                    if _DAT_005180a0 < local_30 {
                        let w = (piVar7.add(3).read_unaligned() as f32) * local_30
                            + outf.read_unaligned();
                        outf.write_unaligned(w);
                        if 0x3f7fffff < (out.read_unaligned() as i32) {
                            out.write_unaligned(0x3f800000);
                            return;
                        }
                    }
                }
                piVar7 = piVar7.add(4);
                local_10 = local_10.wrapping_add(1);
                if DAT_005235f0 <= local_10 {
                    return;
                }
            }
        }
    }
}
